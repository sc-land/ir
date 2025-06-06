use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Instinct {
    pub echo: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_instinct_creation() {
        let instinct = Instinct {
            echo: "test_action".to_string(),
        };

        assert_eq!(instinct.echo, "test_action");
    }

    #[test]
    fn test_instinct_serialization() {
        let instinct = Instinct {
            echo: "serializable_action".to_string(),
        };

        let json = serde_json::to_string(&instinct).unwrap();
        let deserialized: Instinct = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.echo, instinct.echo);
    }

    #[test]
    fn test_instinct_clone_and_eq() {
        let original = Instinct {
            echo: "original_action".to_string(),
        };

        let cloned = original.clone();
        assert_eq!(original, cloned);

        let different = Instinct {
            echo: "different_action".to_string(),
        };
        assert_ne!(original, different);
    }

    #[test]
    fn test_instinct_empty_echo() {
        let instinct = Instinct {
            echo: "".to_string(),
        };

        assert_eq!(instinct.echo, "");

        let json = serde_json::to_string(&instinct).unwrap();
        let deserialized: Instinct = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.echo, "");
    }

    #[test]
    fn test_instinct_unicode_support() {
        let instinct = Instinct {
            echo: "создать_用户_🚀".to_string(),
        };

        let json = serde_json::to_string(&instinct).unwrap();
        let deserialized: Instinct = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.echo, instinct.echo);
    }

    #[test]
    fn test_instinct_vector() {
        let instincts = vec![
            Instinct { echo: "action1".to_string() },
            Instinct { echo: "action2".to_string() },
            Instinct { echo: "action3".to_string() },
        ];

        let json = serde_json::to_string(&instincts).unwrap();
        let deserialized: Vec<Instinct> = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.len(), 3);
        assert_eq!(deserialized[0].echo, "action1");
        assert_eq!(deserialized[1].echo, "action2");
        assert_eq!(deserialized[2].echo, "action3");
    }
}
