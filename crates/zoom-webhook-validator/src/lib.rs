use hmac::Mac;

type HmacSha256 = hmac::Hmac<sha2::Sha256>;

#[derive(Debug)]
pub struct ZoomSignedWebhookValidator {
    mac: HmacSha256,
}

impl ZoomSignedWebhookValidator {
    pub fn new(secret_token: &str) -> Self {
        let mac = HmacSha256::new_from_slice(secret_token.as_bytes()).unwrap();
        Self { mac }
    }
}

impl ZoomSignedWebhookValidator {
    pub fn encrypt(&self, token: &str) -> String {
        let mut mac = self.mac.clone();
        mac.update(token.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        format!("{:x}", code_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zoom_sample() {
        let validator = ZoomSignedWebhookValidator::new("test");
        let token = "qgg8vlvZRS6UYooatFL8Aw";
        let value = validator.encrypt(token);
        insta::assert_snapshot!(value);
    }
}
