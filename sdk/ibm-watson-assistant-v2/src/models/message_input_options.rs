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
pub struct MessageInputOptions {
    /// Whether to restart dialog processing at the root of the dialog, regardless of any previously visited nodes. **Note:** This does not affect `turn_count` or any other context variables.
    #[serde(rename = "restart", skip_serializing_if = "Option::is_none")]
    pub restart: Option<bool>,
    /// Whether to return more than one intent. Set to `true` to return all matching intents.
    #[serde(rename = "alternate_intents", skip_serializing_if = "Option::is_none")]
    pub alternate_intents: Option<bool>,
    #[serde(rename = "spelling", skip_serializing_if = "Option::is_none")]
    pub spelling: Option<Box<crate::models::MessageInputOptionsSpelling>>,
    #[serde(rename = "auto_learn", skip_serializing_if = "Option::is_none")]
    pub auto_learn: Option<Box<crate::models::MessageInputOptionsAutoLearn>>,
    /// Whether to return additional diagnostic information. Set to `true` to return additional information in the `output.debug` property. If you also specify **return_context**=`true`, the returned skill context includes the `system.state` property.
    #[serde(rename = "debug", skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,
    /// Whether to return session context with the response. If you specify `true`, the response includes the `context` property. If you also specify **debug**=`true`, the returned skill context includes the `system.state` property.
    #[serde(rename = "return_context", skip_serializing_if = "Option::is_none")]
    pub return_context: Option<bool>,
    /// Whether to return session context, including full conversation state. If you specify `true`, the response includes the `context` property, and the skill context includes the `system.state` property.  **Note:** If **export**=`true`, the context is returned regardless of the value of **return_context**.
    #[serde(rename = "export", skip_serializing_if = "Option::is_none")]
    pub export: Option<bool>,
}

impl MessageInputOptions {
    pub fn new() -> MessageInputOptions {
        MessageInputOptions {
            restart: None,
            alternate_intents: None,
            spelling: None,
            auto_learn: None,
            debug: None,
            return_context: None,
            export: None,
        }
    }
}

