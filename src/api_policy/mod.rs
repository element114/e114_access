mod ops;
pub use ops::*;

#[cfg(feature = "jsonschema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "validate")]
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct List {
    pub audiences: Vec<String>,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Access {
    pub granted: bool,
    pub message: String,
}

// This type is on a performance critical path.
#[cfg_attr(feature = "jsonschema", derive(JsonSchema))]
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ApiClaims {
    pub sub: String,
    pub roles: Vec<String>,
    pub audiences: Vec<String>,
    pub exp: u64,
}
impl ApiClaims {
    /// `ApiClaims` object from pre-decoded/verified token
    /// Roles in claims are in the format of `access_type:audience_name/audience_id/object_name`
    /// `resource_key.cannot_be_a_base()` must be true
    /// `resource_key` is in the format of `access_type:object_name`
    /// All audiences in claims are checked against the resource key automatically
    #[must_use]
    pub fn is_authorized(&self, resource_key: &str) -> List {
        log::debug!("{}", resource_key);
        let mut res = List::default();
        if let Ok(resource_key) = url::Url::parse(resource_key) {
            // list:users
            let direct_access = Self::roles_match(self, &resource_key);
            if direct_access.granted {
                res.roles.push(resource_key.to_string());
            }
            // list:organizer/4242/events
            let req_access_type = resource_key.scheme().to_owned();
            let req_document = resource_key.path().to_owned();
            for aud in &self.audiences {
                res.audiences.push(aud.clone());
                if let Ok(full_key) = url::Url::parse(
                    format!("{}:{}/{}", req_access_type, aud, req_document).as_str(),
                ) {
                    let acc = Self::roles_match(&self, &full_key);
                    if acc.granted {
                        res.roles.push(full_key.to_string());
                    }
                }
            }
            if res.roles.is_empty() {
                log::warn!(
                    "{}",
                    serde_json::to_string(&Access {
                        granted: false,
                        message: "Access | no applicable role found!".to_owned(),
                    })
                    .unwrap()
                );
            }
        } else if res.roles.is_empty() {
            log::warn!(
                "{}",
                serde_json::to_string(&Access {
                    granted: false,
                    message: "Access | resource key is invalid!".to_owned(),
                })
                .unwrap()
            );
        }
        res
    }

    fn roles_match(&self, full_key: &url::Url) -> Access {
        // let roles = extract_roles(claims);
        let roles = &self.roles;
        if let Some(perm) = roles.iter().find(|role| {
            let r = url::Url::parse(role).unwrap();
            r.scheme().eq(full_key.scheme()) && r.path().eq(full_key.path())
        }) {
            Access { granted: true, message: format!("Access | granted by key {}", perm) }
        } else {
            Access { granted: false, message: "Access | no applicable role found!".to_owned() }
        }
    }
}

/// This type is used to create and store `ApiRoles`, and then to convert them to their `String` representations.
/// An `ApiRole` is an `action` on an api `path`.
/// It works as an allow-list.
#[cfg_attr(feature = "validate", derive(Validate))]
#[cfg_attr(feature = "jsonschema", derive(JsonSchema))]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApiRole {
    #[cfg(feature = "validate")]
    #[validate(length(min = 1), non_control_character)]
    pub action: String,
    pub path: std::path::PathBuf,
}
impl std::fmt::Display for ApiRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.action, self.path.to_str().unwrap_or_default())
    }
}
impl core::str::FromStr for ApiRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, ':').collect();
        if let Some(action) = parts.get(0) {
            if action.contains('/') {
                Err(format!("Invalid action component:{}.", action))
            } else if let Some(path) = parts.get(1) {
                let path = std::path::PathBuf::from(path);
                Ok(ApiRole { action: (*action).to_string(), path })
            } else {
                Err("Missing path component.".to_owned())
            }
        } else {
            Err("Missing action component.".to_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ApiClaims, ApiRole};
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_api_role_to_string() {
        let r0 = "list:users".to_owned();
        let r1 = "fetch:users/3".to_owned();
        let r2 = "list:organizers/2/events".to_owned();
        let r3 = "fetch:organizers/2/events/3".to_owned();

        let ar0 = ApiRole { action: "list".to_owned(), path: PathBuf::from(r"users") };
        assert_eq!(r0, ar0.to_string());

        let ar1 = ApiRole { action: "fetch".to_owned(), path: PathBuf::from(r"users/3") };
        assert_eq!(r1, ar1.to_string());

        let ar2 =
            ApiRole { action: "list".to_owned(), path: PathBuf::from(r"organizers/2/events") };
        assert_eq!(r2, ar2.to_string());

        let ar3 =
            ApiRole { action: "fetch".to_owned(), path: PathBuf::from(r"organizers/2/events/3") };
        assert_eq!(r3, ar3.to_string());
    }

    #[test]
    fn test_api_role_from_str() {
        let r0 = "list:users";
        let r1 = "fetch:users/3";
        let r2 = "list:organizers/2/events";
        let r3 = "fetch:organizers/2/events/3";

        let ar0 = ApiRole { action: "list".to_owned(), path: PathBuf::from(r"users") };
        assert_eq!(ar0, ApiRole::from_str(r0).unwrap());

        let ar1 = ApiRole { action: "fetch".to_owned(), path: PathBuf::from(r"users/3") };
        assert_eq!(ar1, ApiRole::from_str(r1).unwrap());

        let ar2 =
            ApiRole { action: "list".to_owned(), path: PathBuf::from(r"organizers/2/events") };
        assert_eq!(ar2, ApiRole::from_str(r2).unwrap());

        let ar3 =
            ApiRole { action: "fetch".to_owned(), path: PathBuf::from(r"organizers/2/events/3") };
        assert_eq!(ar3, ApiRole::from_str(r3).unwrap());

        let no_action = "organizers/2/events";
        assert_eq!(
            Err("Invalid action component:organizers/2/events.".to_owned()),
            ApiRole::from_str(no_action)
        );
    }

    #[test]
    fn test_is_authorized() {
        let r0 = "list:users".to_owned();
        let r1 = "fetch:users/3".to_owned();
        let r2 = "list:organizers/2/events".to_owned();
        let r3 = "fetch:organizers/2/events/3".to_owned();

        let c0 = ApiClaims {
            sub: "user@domain.com".to_owned(),
            roles: vec![r0.clone(), r1.clone(), r2.clone(), r3.clone()],
            audiences: vec!["organizers/2".to_owned()],
            exp: 1000,
        };

        let auth = ApiClaims::is_authorized(&c0, &r0);
        assert!(!auth.roles.is_empty());

        let auth = ApiClaims::is_authorized(&c0, &r1);
        assert!(!auth.roles.is_empty());

        // user lacks global permission
        let c1 = ApiClaims {
            sub: "user@domain.com".to_owned(),
            roles: vec![r2.clone(), r3],
            audiences: vec!["organizers/2".to_owned()],
            exp: 1000,
        };

        // access denied
        let auth = ApiClaims::is_authorized(&c1, &r0);
        assert!(auth.roles.is_empty());

        // access denied
        let auth = ApiClaims::is_authorized(&c1, &r1);
        assert!(auth.roles.is_empty());

        // resource_key passed by the derived code is in the format access_type:/object_name
        // resource key != role, roles are compile time, audiences are runtime
        //
        // is_authorized can be used to check if a role exactly matches a resource_key
        // In this case it's treated the same as global keys, audiences remains empty
        let auth = ApiClaims::is_authorized(&c1, &r2);
        assert!(!auth.roles.is_empty());

        // user can list events of organizer 2 because it has the audience and the role
        let r4 = "list:events";
        let auth = ApiClaims::is_authorized(&c1, &r4);
        assert!(!auth.audiences.is_empty());
        assert!(!auth.roles.is_empty());

        // access denied user can't fetch events
        let r5 = "fetch:events";
        let auth = ApiClaims::is_authorized(&c1, &r5);
        assert!(auth.roles.is_empty());

        // user can only fetch events/3
        // Note: derived code doesn't check single document permissions
        // This could be useful for custom endpoints
        let r6 = "fetch:events/3";
        let auth = ApiClaims::is_authorized(&c1, &r6);
        assert!(!auth.audiences.is_empty());
        assert!(!auth.roles.is_empty());
        assert_eq!("organizers/2", auth.audiences.first().unwrap());
        assert_eq!("fetch:organizers/2/events/3", auth.roles.first().unwrap());

        // println!("{:#?}", c1);
        // println!("{:#?}", r0);
        // println!("{:#?}", auth);
    }
}
