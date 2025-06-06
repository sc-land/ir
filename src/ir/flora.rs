use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Flora {
    Int,
    Str,
    Bool,
    Bug(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_flora_equality() {
        assert_eq!(Flora::Int, Flora::Int);
        assert_eq!(Flora::Str, Flora::Str);
        assert_eq!(Flora::Bool, Flora::Bool);
        assert_eq!(Flora::Bug("Cat".to_string()), Flora::Bug("Cat".to_string()));

        assert_ne!(Flora::Int, Flora::Str);
    }

    #[test]
    fn test_flora_clone() {
        let original = Flora::Str;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_flora_debug() {
        assert_eq!(format!("{:?}", Flora::Int), "Int");
        assert_eq!(format!("{:?}", Flora::Str), "Str");
        assert_eq!(format!("{:?}", Flora::Bool), "Bool");
        assert_eq!(format!("{:?}", Flora::Bug("Bird".to_string())), "Bug(\"Bird\")");
    }

    #[test]
    fn test_flora_serialization() {
        let variants = vec![Flora::Int, Flora::Str, Flora::Bool, Flora::Bug("Bee".to_string())];

        for variant in variants {
            let json = serde_json::to_string(&variant).unwrap();
            let deserialized: Flora = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, deserialized);
        }
    }

    #[test]
    fn test_flora_json_format() {
        assert_eq!(serde_json::to_string(&Flora::Int).unwrap(), "\"Int\"");
        assert_eq!(serde_json::to_string(&Flora::Str).unwrap(), "\"Str\"");
        assert_eq!(serde_json::to_string(&Flora::Bool).unwrap(), "\"Bool\"");
        assert_eq!(serde_json::to_string(&Flora::Bug("LadyBug".to_string())).unwrap(), "{\"Bug\":\"LadyBug\"}");
    }
}
