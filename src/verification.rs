use slack_morphism::signature_verifier::SlackEventSignatureVerifier;
use tower::{Layer, Service};

#[derive(Debug)]
pub struct SlackVerification<T> {
    inner: T,
    slack_verifier: SlackEventSignatureVerifier,
}

impl<T> SlackVerification<T> {
    pub fn new(inner: T, secret: &str) -> SlackVerification<T> {
        SlackVerification {
            inner,
            slack_verifier: SlackEventSignatureVerifier::new(secret),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SlackVerificationLayer {
    secret: String,
    slack_verifier: SlackEventSignatureVerifier,
}

impl SlackVerificationLayer {
    pub fn new(secret: &str) -> Self {
        SlackVerificationLayer {
            secret: secret.to_string(),
            slack_verifier: SlackEventSignatureVerifier::new(secret),
        }
    }
}

impl<S> Layer<S> for SlackVerificationLayer {
    type Service = SlackVerification<S>;

    fn layer(&self, service: S) -> Self::Service {
        SlackVerification::new(service, &self.secret)
    }
}
