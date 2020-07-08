use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header as JWTHeader, Validation,
};
use serde::{de::DeserializeOwned, Serialize};

#[must_use]
pub fn algorithm() -> Algorithm {
    Algorithm::RS256
}

/// ```ignore
/// let my_pub_key = include_bytes!("web_id_rsa_pub.pem");
/// let token = token_decode(my_claim, my_pub_key);
/// ```
/// # Errors
/// May yield a `jsonwebtoken::errors::Error`.
pub fn jwt_decode<T: DeserializeOwned>(
    token: &str,
    pub_key: &[u8],
) -> Result<jsonwebtoken::TokenData<T>, jsonwebtoken::errors::Error> {
    decode::<T>(&token, &DecodingKey::from_rsa_pem(pub_key)?, &Validation::new(algorithm()))
}

/// ```ignore
/// let my_private_key = include_bytes!("web_id_rsa.pem");
/// let token = token_encode(my_claim, my_private_key);
/// ```
/// # Errors
/// May yield a `jsonwebtoken::errors::Error`.
pub fn jwt_encode<T: Serialize>(
    claims: &T,
    priv_key: &[u8],
) -> Result<String, jsonwebtoken::errors::Error> {
    encode(&JWTHeader::new(algorithm()), claims, &EncodingKey::from_rsa_pem(priv_key)?)
}

#[cfg(test)]
mod tests {
    use super::{jwt_decode, jwt_encode};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    pub(crate) struct IdClaim {
        pub(crate) sub: String,
        // pub(crate) company: String,
        pub(crate) exp: i64,
    }

    #[test]
    fn organizer_id_claim_to_and_from_token() {
        let my_private_key = include_bytes!("test_rsa.pem");
        let my_pub_key = include_bytes!("test_rsa_pub.pem");

        let organizer_id = || String::from("John Doe");
        let non_expired_claim = IdClaim { sub: organizer_id(), exp: i64::max_value() };
        let encoded_token = jwt_encode(&non_expired_claim, my_private_key).unwrap();
        let token_decoded = jwt_decode::<IdClaim>(&encoded_token, my_pub_key).unwrap();
        assert_eq!(non_expired_claim, token_decoded.claims);
    }
}
