use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Seal {
    Vital,  // NotNull
    Core,   // PrimaryKey
    Root,   // Unique
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_seal_equality() {
        assert_eq!(Seal::Vital, Seal::Vital);
        assert_eq!(Seal::Core, Seal::Core);
        assert_eq!(Seal::Root, Seal::Root);

        assert_ne!(Seal::Vital, Seal::Core);
        assert_ne!(Seal::Core, Seal::Root);
        assert_ne!(Seal::Vital, Seal::Root);
    }

    #[test]
    fn test_seal_clone() {
        let original = Seal::Core;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_seal_debug() {
        assert_eq!(format!("{:?}", Seal::Vital), "Vital");
        assert_eq!(format!("{:?}", Seal::Core), "Core");
        assert_eq!(format!("{:?}", Seal::Root), "Root");
    }

    #[test]
    fn test_seal_serialization() {
        let variants = vec![Seal::Vital, Seal::Core, Seal::Root];

        for variant in variants {
            let json = serde_json::to_string(&variant).unwrap();
            let deserialized: Seal = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, deserialized);
        }
    }

    #[test]
    fn test_seal_vector_serialization() {
        let seals = vec![Seal::Vital, Seal::Core, Seal::Root];
        let json = serde_json::to_string(&seals).unwrap();
        let deserialized: Vec<Seal> = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, seals);
    }

    #[test]
    fn test_empty_seal_vector() {
        let empty_seals: Vec<Seal> = vec![];
        let json = serde_json::to_string(&empty_seals).unwrap();
        let deserialized: Vec<Seal> = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.len(), 0);
    }
}
