use serde::{Deserialize, Serialize};
use crate::ir::flora::Flora;
use crate::ir::seal::Seal;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Casts {
    pub primor: String,
    pub flora: Flora,
    pub seals: Vec<Seal>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_casts_creation() {
        let cast = Casts {
            primor: "test_field".to_string(),
            flora: Flora::Int,
            seals: vec![Seal::Vital],
        };

        assert_eq!(cast.primor, "test_field");
        assert_eq!(cast.flora, Flora::Int);
        assert_eq!(cast.seals.len(), 1);
        assert_eq!(cast.seals[0], Seal::Vital);
    }

    #[test]
    fn test_casts_with_multiple_seals() {
        let cast = Casts {
            primor: "complex_field".to_string(),
            flora: Flora::Str,
            seals: vec![Seal::Vital, Seal::Core, Seal::Root],
        };

        assert_eq!(cast.seals.len(), 3);
        assert!(cast.seals.contains(&Seal::Vital));
        assert!(cast.seals.contains(&Seal::Core));
        assert!(cast.seals.contains(&Seal::Root));
    }

    #[test]
    fn test_casts_with_no_seals() {
        let cast = Casts {
            primor: "optional_field".to_string(),
            flora: Flora::Bool,
            seals: vec![],
        };

        assert_eq!(cast.seals.len(), 0);
    }

    #[test]
    fn test_casts_serialization() {
        let cast = Casts {
            primor: "serializable_field".to_string(),
            flora: Flora::Bug("Bee".to_string()),
            seals: vec![Seal::Core],
        };

        let json = serde_json::to_string(&cast).unwrap();
        let deserialized: Casts = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.primor, cast.primor);
        assert_eq!(deserialized.flora, cast.flora);
        assert_eq!(deserialized.seals, cast.seals);
    }

    #[test]
    fn test_casts_with_all_flora_types() {
        let flora_types = vec![Flora::Int, Flora::Str, Flora::Bool, Flora::Bug("Bee".to_string())];

        for flora in flora_types {
            let cast = Casts {
                primor: format!("field_{:?}", flora),
                flora: flora.clone(),
                seals: vec![Seal::Vital],
            };

            assert_eq!(cast.flora, flora);
        }
    }

    #[test]
    fn test_casts_equality() {
        let cast1 = Casts {
            primor: "same_field".to_string(),
            flora: Flora::Int,
            seals: vec![Seal::Vital],
        };

        let cast2 = Casts {
            primor: "same_field".to_string(),
            flora: Flora::Int,
            seals: vec![Seal::Vital],
        };

        let cast3 = Casts {
            primor: "different_field".to_string(),
            flora: Flora::Int,
            seals: vec![Seal::Vital],
        };

        assert_eq!(cast1, cast2);
        assert_ne!(cast1, cast3);
    }

    #[test]
    fn test_casts_clone() {
        let original = Casts {
            primor: "original".to_string(),
            flora: Flora::Str,
            seals: vec![Seal::Core, Seal::Root],
        };

        let cloned = original.clone();
        assert_eq!(original, cloned);

        // Ensure deep clone
        assert_eq!(original.seals.len(), cloned.seals.len());
    }
}
