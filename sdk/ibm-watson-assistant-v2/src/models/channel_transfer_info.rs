/*
 * Watson Assistant v2
 *
 * The IBM Watson&trade; Assistant service combines machine learning, natural language understanding, and an integrated dialog editor to create conversation flows between your apps and your users.  The Assistant v2 API provides runtime methods your client application can use to send user input to an assistant and receive a response.
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChannelTransferInfo : Information used by an integration to transfer the conversation to a different channel.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelTransferInfo {
    #[serde(rename = "target")]
    pub target: Box<crate::models::ChannelTransferTarget>,
}

impl ChannelTransferInfo {
    /// Information used by an integration to transfer the conversation to a different channel.
    pub fn new(target: crate::models::ChannelTransferTarget) -> ChannelTransferInfo {
        ChannelTransferInfo {
            target: Box::new(target),
        }
    }
}


