#[cfg(feature = "jsonschema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "validate")]
use validator::Validate;

#[cfg_attr(feature = "jsonschema", derive(JsonSchema))]
#[cfg_attr(feature = "validate", derive(Validate))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    #[cfg(feature = "validate")]
    #[validate(email, non_control_character)]
    pub(crate) email: String,
    // Never leak this field
    #[serde(skip_serializing)]
    #[cfg(feature = "validate")]
    #[validate(length(min = 8), non_control_character)]
    pub(crate) password: String,

    pub(crate) holder: Option<String>,
}
