/*
 * Watson Assistant v2
 *
 * The IBM Watson&trade; Assistant service combines machine learning, natural language understanding, and an integrated dialog editor to create conversation flows between your apps and your users.  The Assistant v2 API provides runtime methods your client application can use to send user input to an assistant and receive a response.
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MessageContextSkillSystem : System context data used by the skill.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageContextSkillSystem {
    /// An encoded string that represents the current conversation state. By saving this value and then sending it in the context of a subsequent message request, you can return to an earlier point in the conversation. If you are using stateful sessions, you can also use a stored state value to restore a paused conversation whose session is expired.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl MessageContextSkillSystem {
    /// System context data used by the skill.
    pub fn new() -> MessageContextSkillSystem {
        MessageContextSkillSystem {
            state: None,
        }
    }
}


