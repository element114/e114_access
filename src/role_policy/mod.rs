use crate::api_policy::ApiRole;
#[cfg(feature = "jsonschema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[cfg_attr(feature = "jsonschema", derive(JsonSchema))]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RoleClaims {
    pub sub: String,
    pub roles: Vec<String>,
    pub audiences: Vec<String>,
    pub exp: u64,
}

/// Maps a collection of `ApiRole`s to a group.
/// `machine_name` is the name of the role-map.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RoleMap {
    pub machine_name: String,
    pub group: String,
    api_roles: BTreeSet<ApiRole>,
}
impl RoleMap {
    #[must_use]
    /// A new `Role`.
    /// `@machine_name` The `snake_case` name of the `Role`, keep it short as it will make it into the jwt.
    /// `@group` The `Group` which grants this `Role`.
    pub fn new(machine_name: String, group: String) -> Self {
        Self { machine_name, group, api_roles: BTreeSet::new() }
    }

    /// Add a `ApiRole` to the `RoleMap`.
    /// This mutates the internal state of the `RoleMap`.
    /// If the same `ApiRole` exists, it's overwritten.
    pub fn add_api_role(&mut self, api_role: ApiRole) {
        self.api_roles.insert(api_role);
    }

    #[must_use]
    /// Returns a & to the internal role `Set` for you.
    pub fn all_roles(&self) -> &BTreeSet<ApiRole> {
        &self.api_roles
    }

    #[must_use]
    /// Filter all roles by action in `api_roles`, and returns a `Vec` for you.
    pub fn find_roles(&self, action: &str) -> Vec<&ApiRole> {
        self.api_roles.iter().filter(|r| r.action.eq(action)).collect()
    }
}
