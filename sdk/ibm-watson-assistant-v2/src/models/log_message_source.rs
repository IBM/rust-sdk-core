/*
 * Watson Assistant v2
 *
 * The IBM Watson&trade; Assistant service combines machine learning, natural language understanding, and an integrated dialog editor to create conversation flows between your apps and your users.  The Assistant v2 API provides runtime methods your client application can use to send user input to an assistant and receive a response.
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LogMessageSource : An object that identifies the dialog element that generated the error message.


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LogMessageSource {
    #[serde(rename="action")]
    LogMessageSourceAction {
        /// The unique identifier of the action that generated the error message.
        #[serde(rename = "action")]
        action: String,
    },
    #[serde(rename="dialog_node")]
    LogMessageSourceDialogNode {
        /// The unique identifier of the dialog node that generated the error message.
        #[serde(rename = "dialog_node")]
        dialog_node: String,
    },
    #[serde(rename="handler")]
    LogMessageSourceHandler {
        /// The unique identifier of the action that generated the error message.
        #[serde(rename = "action")]
        action: String,
        /// The unique identifier of the step that generated the error message.
        #[serde(rename = "step", skip_serializing_if = "Option::is_none")]
        step: Option<String>,
        /// The unique identifier of the handler that generated the error message.
        #[serde(rename = "handler")]
        handler: String,
    },
    #[serde(rename="step")]
    LogMessageSourceStep {
        /// The unique identifier of the action that generated the error message.
        #[serde(rename = "action")]
        action: String,
        /// The unique identifier of the step that generated the error message.
        #[serde(rename = "step")]
        step: String,
    },
}




