/*
 * Watson Assistant v2
 *
 * The IBM Watson&trade; Assistant service combines machine learning, natural language understanding, and an integrated dialog editor to create conversation flows between your apps and your users.  The Assistant v2 API provides runtime methods your client application can use to send user input to an assistant and receive a response.
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BaseMessageContextGlobal : Session context data that is shared by all skills used by the Assistant.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseMessageContextGlobal {
    #[serde(rename = "system", skip_serializing_if = "Option::is_none")]
    pub system: Option<Box<crate::models::MessageContextGlobalSystem>>,
}

impl BaseMessageContextGlobal {
    /// Session context data that is shared by all skills used by the Assistant.
    pub fn new() -> BaseMessageContextGlobal {
        BaseMessageContextGlobal {
            system: None,
        }
    }
}


