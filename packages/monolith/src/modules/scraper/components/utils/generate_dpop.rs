use jsonwebtoken::jwk::{CommonParameters, EllipticCurveKeyParameters};
use p256::{ecdsa::SigningKey, elliptic_curve::rand_core::OsRng};
use p256::pkcs8::EncodePrivateKey;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use jsonwebtoken::{encode, jwk::Jwk, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct JWK {
    crv: String,
    kty: String,
    x: String,
    y: String,
}

#[derive(Debug, Serialize)]
struct Claims {
    iat: i64,
    jti: String,
    htu: String,
    htm: String,
    uuid: String,
}

/// Generate a DPOP token for use in "authentication" in Mercari APIs.
pub fn generate_dpop(
    accessed_url: &str,
    http_method: &str,
) -> Result<String, String> {
    let private_key = SigningKey::random(&mut OsRng);
    let public_key = private_key.verifying_key();
    let public_key_bytes = public_key.to_encoded_point(false);
    let x = &public_key_bytes.as_bytes()[1..33];
    let y = &public_key_bytes.as_bytes()[33..65];

    let ec_params = EllipticCurveKeyParameters {
        key_type: jsonwebtoken::jwk::EllipticCurveKeyType::EC,
        curve: jsonwebtoken::jwk::EllipticCurve::P256,
        x: URL_SAFE_NO_PAD.encode(x),
        y: URL_SAFE_NO_PAD.encode(y),
    };
    let common_params = CommonParameters {
        public_key_use: None,
        key_operations: None,
        key_algorithm: None,
        key_id: None,
        x509_url: None,
        x509_chain: None,
        x509_sha1_fingerprint: None,
        x509_sha256_fingerprint: None,
    };
    let jwk = Jwk {
        common: common_params,
        algorithm: jsonwebtoken::jwk::AlgorithmParameters::EllipticCurve(ec_params)
    };
    let claims = Claims {
        iat: Utc::now().timestamp(),
        jti: Uuid::new_v4().to_string(),
        htu: accessed_url.to_string(),
        htm: http_method.to_string(),
        uuid: Uuid::new_v4().to_string(),
    };
    let mut header = Header::new(Algorithm::ES256);
    header.typ = Some("dpop+jwt".to_string());
    header.jwk = Some(jwk);

    let encoded_key = match private_key.to_pkcs8_der() {
        Ok(key) => key,
        Err(err) => return Err(format!("Failed to serialize private key to DER: {err}"))
    };
    match encode(
        &header,
        &claims,
        &EncodingKey::from_ec_der(&encoded_key.to_bytes()),
    ) {
        Ok(token) => Ok(token),
        Err(err) => Err(format!("Failed to encode into JWT: {err}"))
    }
}
