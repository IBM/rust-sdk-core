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
pub struct RuntimeResponseTypeUserDefined {
    /// The type of response returned by the dialog node. The specified response type must be supported by the client application or channel.
    #[serde(rename = "response_type")]
    pub response_type: String,
    /// An object containing any properties for the user-defined response type.
    #[serde(rename = "user_defined")]
    pub user_defined: ::std::collections::HashMap<String, serde_json::Value>,
    /// An array of objects specifying channels for which the response is intended. If **channels** is present, the response is intended for a built-in integration and should not be handled by an API client.
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<crate::models::ResponseGenericChannel>>,
}

impl RuntimeResponseTypeUserDefined {
    pub fn new(response_type: String, user_defined: ::std::collections::HashMap<String, serde_json::Value>) -> RuntimeResponseTypeUserDefined {
        RuntimeResponseTypeUserDefined {
            response_type,
            user_defined,
            channels: None,
        }
    }
}

