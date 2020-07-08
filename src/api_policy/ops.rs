use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, EnumVariantNames};

#[derive(
    EnumString, EnumVariantNames, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash,
)]
#[strum(serialize_all = "snake_case")]
pub enum Operation {
    List,
    Fetch,
    Create,
    Update,
    Replace,
    Delete,
}
impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::Operation;

    #[test]
    fn test_operation_fmt() {
        assert_eq!("List", Operation::List.to_string());
    }
}
