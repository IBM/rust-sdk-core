/*
 * Watson Assistant v2
 *
 * The IBM Watson&trade; Assistant service combines machine learning, natural language understanding, and an integrated dialog editor to create conversation flows between your apps and your users.  The Assistant v2 API provides runtime methods your client application can use to send user input to an assistant and receive a response.
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SearchResultHighlight : An object containing segments of text from search results with query-matching text highlighted using HTML `<em>` tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResultHighlight {
    /// An array of strings containing segments taken from body text in the search results, with query-matching substrings highlighted.
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<String>>,
    /// An array of strings containing segments taken from title text in the search results, with query-matching substrings highlighted.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<String>>,
    /// An array of strings containing segments taken from URLs in the search results, with query-matching substrings highlighted.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<Vec<String>>,
}

impl SearchResultHighlight {
    /// An object containing segments of text from search results with query-matching text highlighted using HTML `<em>` tags.
    pub fn new() -> SearchResultHighlight {
        SearchResultHighlight {
            body: None,
            title: None,
            url: None,
        }
    }
}


