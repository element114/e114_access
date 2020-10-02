#[cfg(feature = "jsonschema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "validate"))]
#[cfg_attr(feature = "jsonschema", derive(JsonSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Account<T> {
    pub email: String,
    // Never leak this field
    #[serde(skip_serializing)]
    pub password: String,
    pub holder: Option<String>,
    pub account_options: Option<T>,
}

#[cfg(feature = "validate")]
#[cfg_attr(feature = "jsonschema", derive(JsonSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, Validate)]
pub struct Account<T: validator::Validate> {
    #[cfg(feature = "validate")]
    #[validate(email, non_control_character)]
    pub email: String,
    // Never leak this field
    #[serde(skip_serializing)]
    #[cfg(feature = "validate")]
    #[validate(length(min = 8), non_control_character)]
    pub password: String,

    #[cfg(feature = "validate")]
    #[validate(non_control_character)]
    pub holder: Option<String>,

    #[cfg(feature = "validate")]
    #[validate]
    pub account_options: Option<T>,
}
