use axum::{
    body::{Body, BoxBody},
    http::{Request, Response, StatusCode},
    response::IntoResponse,
};
use slack_morphism::signature_verifier::SlackEventSignatureVerifier;
use std::{convert::Infallible, future::Future, pin::Pin};
use tower::Service;

#[derive(Clone)]
pub struct SlackRequestVerifier<S> {
    pub inner: S,
    pub verifier: SlackEventSignatureVerifier,
}

impl<S> Service<Request<Body>> for SlackRequestVerifier<S>
where
    S: Service<Request<Body>, Error = Infallible> + Clone + Send + 'static,
    S::Response: IntoResponse,
    S::Future: Send + 'static,
{
    type Response = Response<BoxBody>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut inner = self.inner.clone();
        let verifier = self.verifier.clone();

        Box::pin(async move {
            let (parts, body) = req.into_parts();

            let hash = match parts
                .headers
                .get(SlackEventSignatureVerifier::SLACK_SIGNED_HASH_HEADER)
            {
                Some(hash_header) => match hash_header.to_str() {
                    Ok(hash_str) => hash_str,
                    Err(_) => return Ok(StatusCode::UNAUTHORIZED.into_response()),
                },
                None => return Ok(StatusCode::UNAUTHORIZED.into_response()),
            };

            let ts = match parts
                .headers
                .get(SlackEventSignatureVerifier::SLACK_SIGNED_TIMESTAMP)
            {
                Some(ts_header) => match ts_header.to_str() {
                    Ok(ts_str) => ts_str,
                    Err(_) => return Ok(StatusCode::UNAUTHORIZED.into_response()),
                },
                None => return Ok(StatusCode::UNAUTHORIZED.into_response()),
            };

            let body_bytes = if let Ok(bytes) = hyper::body::to_bytes(body).await {
                bytes
            } else {
                return Ok(StatusCode::BAD_REQUEST.into_response());
            };

            let body_as_str = match std::str::from_utf8(body_bytes.as_ref()) {
                Ok(byte_str) => byte_str,
                Err(_) => return Ok(StatusCode::BAD_REQUEST.into_response()),
            };

            // check if the request is valid
            match verifier.verify(hash, body_as_str, ts) {
                Ok(_) => {
                    let req = Request::from_parts(parts, Body::from(body_bytes));
                    inner.call(req).await.map(|res| res.into_response())
                }
                Err(_) => return Ok(StatusCode::UNAUTHORIZED.into_response()),
            }
        })
    }
}
