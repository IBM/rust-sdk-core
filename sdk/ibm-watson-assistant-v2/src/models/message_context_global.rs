/*
 * Watson Assistant v2
 *
 * The IBM Watson&trade; Assistant service combines machine learning, natural language understanding, and an integrated dialog editor to create conversation flows between your apps and your users.  The Assistant v2 API provides runtime methods your client application can use to send user input to an assistant and receive a response.
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageContextGlobal {
    #[serde(rename = "system", skip_serializing_if = "Option::is_none")]
    pub system: Option<Box<crate::models::MessageContextGlobalSystem>>,
    /// The session ID.
    #[serde(rename = "session_id", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

impl MessageContextGlobal {
    pub fn new() -> MessageContextGlobal {
        MessageContextGlobal {
            system: None,
            session_id: None,
        }
    }
}


