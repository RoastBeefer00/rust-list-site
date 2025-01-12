use axum::{
    extract::{FromRef, FromRequestParts},
    http::{self, request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use firebase_auth::{FirebaseAuthState, FirebaseProvider};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FirebaseUser {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
    pub auth_time: u64,
    pub user_id: String,
    pub provider_id: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub firebase: FirebaseProvider,
}

fn get_bearer_token(header: &str) -> Option<String> {
    let prefix_len = "Bearer ".len();

    match header.len() {
        l if l < prefix_len => None,
        _ => Some(header[prefix_len..].to_string()),
    }
}

// #[async_trait]
impl<S> FromRequestParts<S> for FirebaseUser
where
    FirebaseAuthState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = UnauthorizedResponse;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let store = FirebaseAuthState::from_ref(state);

        let auth_header = parts
            .headers
            .get(http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");

        let bearer = get_bearer_token(auth_header).map_or(
            Err(UnauthorizedResponse {
                msg: "Missing Bearer Token".to_string(),
            }),
            Ok,
        )?;

        // let pem = b"-----BEGIN CERTIFICATE-----MIIDHDCCAgSgAwIBAgIIeQcTtsb5b8cwDQYJKoZIhvcNAQEFBQAwMTEvMC0GA1UEAwwmc2VjdXJldG9rZW4uc3lzdGVtLmdzZXJ2aWNlYWNjb3VudC5jb20wHhcNMjUwMTAzMDczMjUyWhcNMjUwMTE5MTk0NzUyWjAxMS8wLQYDVQQDDCZzZWN1cmV0b2tlbi5zeXN0ZW0uZ3NlcnZpY2VhY2NvdW50LmNvbTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAK8YEDB/FoyW53qbGHi3znbgltkAQaiyIAi7GdQOkhqM3R2KKd8xtt4dpArV20QFvj6n+/couKSuCLZfYSP/sRIM3z8mNUGugni5Gx0jBm8f7HkPUA6yrSmADlA2Ie9roTPk832VY7YW2ppBvR2dnG00cYgUNVJAqabSRFZT+q6NDvixSPtHCGWr+ympHgzuVe2a++BcQkP6rgnbdF2+bA3aWaEgnAwRc6yUMLqETBEEJoDaWsmChbqxdwSLX0/08r0oeVlPewnQVRjsjcY826rnDtUjVhjFHicZp3Lr8ageD5oXVqIhPWKhXTOY1dX7UiW7UytTi2piw55N2dCdrBUCAwEAAaM4MDYwDAYDVR0TAQH/BAIwADAOBgNVHQ8BAf8EBAMCB4AwFgYDVR0lAQH/BAwwCgYIKwYBBQUHAwIwDQYJKoZIhvcNAQEFBQADggEBAEerVERnHAoEYEKITnIthsHra6A5Ef/ZYk/iBG1iN1zPz29U2gdIZeblesEsVDmMNacPOVAS2MMHTkiMA5CyoqhQP4aFXnYKD2EYNVkST0Y+Gx6laQdD85ZyhpPWx9jSKd9BAn8S2DYId78JqE0okAVOGSP4vdNWd3kTIgdMsDF1iO+hf7P4y9Nv/1xJTjhyXLetxokziDU8la+IrPX70O33wU1xwv0Z3rRTCe4psS+YTI9efU44fF6jY+orqgz55mr39KjUJ1QA6z6Q5dJYoJF9VnOMuN1jZb2hL6M9l7fq/V50VeqF/oHV4Y2XZuic9ZeGRcKrLTxCpSKztneAMpg=-----END CERTIFICATE-----";
        // let project_id = "r-j-magenta-carrot-42069".to_owned();
        // let audience = "r-j-magenta-carrot-42069".to_owned();
        // let issuer = format!("https://securetoken.google.com/{}", project_id);
        // let mut validation = Validation::new(Algorithm::RS256);
        // validation.set_audience(&[audience.to_owned()]);
        // validation.set_issuer(&[issuer.to_owned()]);
        // let decoding_key = DecodingKey::from_rsa_pem(pem).unwrap();
        // match decode::<FirebaseUser>(&bearer, &decoding_key, &validation) {
        //     Err(e) => Err(UnauthorizedResponse {
        //         msg: format!("Failed to verify Token: {}", e),
        //     }),
        //     Ok(current_user) => Ok(current_user.claims),
        // }

        match store.firebase_auth.verify(&bearer) {
            Err(e) => Err(UnauthorizedResponse {
                msg: format!("Failed to verify Token: {}", e),
            }),
            Ok(current_user) => Ok(current_user),
        }
    }
}

pub struct UnauthorizedResponse {
    msg: String,
}

impl IntoResponse for UnauthorizedResponse {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, self.msg).into_response()
    }
}
