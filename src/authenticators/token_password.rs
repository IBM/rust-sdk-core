use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use serde::Deserialize;
use std::env;
use crate::authenticators::base::{ResponseType, TokenResponse, Configs};


