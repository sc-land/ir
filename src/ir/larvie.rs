use serde::{Deserialize, Serialize};

use crate::ir::{casts::Casts, instincts::Instinct};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Larvie {
    pub primor: String,
    pub casts: Vec<Casts>,
    pub instincts: Vec<Instinct>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{flora::Flora, seal::Seal};
    use serde_json;

    #[test]
    fn test_larvie_creation() {
        let larvie = Larvie {
            primor: "TestLarvie".to_string(),
            casts: vec![],
            instincts: vec![],
        };

        assert_eq!(larvie.primor, "TestLarvie");
        assert_eq!(larvie.casts.len(), 0);
        assert_eq!(larvie.instincts.len(), 0);
    }

    #[test]
    fn test_larvie_with_fields() {
        let larvie = Larvie {
            primor: "ComplexLarvie".to_string(),
            casts: vec![
                Casts {
                    primor: "field1".to_string(),
                    flora: Flora::Int,
                    seals: vec![Seal::Vital],
                },
                Casts {
                    primor: "field2".to_string(),
                    flora: Flora::Str,
                    seals: vec![Seal::Core],
                },
            ],
            instincts: vec![
                Instinct { echo: "action1".to_string() },
                Instinct { echo: "action2".to_string() },
            ],
        };

        assert_eq!(larvie.casts.len(), 2);
        assert_eq!(larvie.instincts.len(), 2);
        assert_eq!(larvie.casts[0].primor, "field1");
        assert_eq!(larvie.instincts[0].echo, "action1");
    }

    #[test]
    fn test_larvie_serialization() {
        let larvie = Larvie {
            primor: "SerializableLarvie".to_string(),
            casts: vec![
                Casts {
                    primor: "test_field".to_string(),
                    flora: Flora::Bool,
                    seals: vec![Seal::Root],
                }
            ],
            instincts: vec![
                Instinct { echo: "test_action".to_string() }
            ],
        };

        let json = serde_json::to_string(&larvie).unwrap();
        let deserialized: Larvie = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.primor, larvie.primor);
        assert_eq!(deserialized.casts.len(), larvie.casts.len());
        assert_eq!(deserialized.instincts.len(), larvie.instincts.len());
    }

    #[test]
    fn test_larvie_equality() {
        let larvie1 = Larvie {
            primor: "SameLarvie".to_string(),
            casts: vec![
                Casts {
                    primor: "field".to_string(),
                    flora: Flora::Int,
                    seals: vec![Seal::Vital],
                }
            ],
            instincts: vec![
                Instinct { echo: "action".to_string() }
            ],
        };

        let larvie2 = Larvie {
            primor: "SameLarvie".to_string(),
            casts: vec![
                Casts {
                    primor: "field".to_string(),
                    flora: Flora::Int,
                    seals: vec![Seal::Vital],
                }
            ],
            instincts: vec![
                Instinct { echo: "action".to_string() }
            ],
        };

        let larvie3 = Larvie {
            primor: "DifferentLarvie".to_string(),
            casts: vec![],
            instincts: vec![],
        };

        assert_eq!(larvie1, larvie2);
        assert_ne!(larvie1, larvie3);
    }

    #[test]
    fn test_larvie_clone() {
        let original = Larvie {
            primor: "OriginalLarvie".to_string(),
            casts: vec![
                Casts {
                    primor: "field".to_string(),
                    flora: Flora::Str,
                    seals: vec![Seal::Core, Seal::Root],
                }
            ],
            instincts: vec![
                Instinct { echo: "complex_action".to_string() }
            ],
        };

        let cloned = original.clone();
        assert_eq!(original, cloned);

        // Ensure deep clone
        assert_eq!(original.casts.len(), cloned.casts.len());
        assert_eq!(original.instincts.len(), cloned.instincts.len());
    }

    #[test]
    fn test_larvie_with_multiple_casts_and_instincts() {
        let larvie = Larvie {
            primor: "RichLarvie".to_string(),
            casts: vec![
                Casts {
                    primor: "id".to_string(),
                    flora: Flora::Int,
                    seals: vec![Seal::Core, Seal::Vital],
                },
                Casts {
                    primor: "name".to_string(),
                    flora: Flora::Str,
                    seals: vec![Seal::Vital],
                },
                Casts {
                    primor: "active".to_string(),
                    flora: Flora::Bool,
                    seals: vec![Seal::Root],
                },
                Casts {
                    primor: "metadata".to_string(),
                    flora: Flora::Bug("Bee".to_string()),
                    seals: vec![],
                },
            ],
            instincts: vec![
                Instinct { echo: "create".to_string() },
                Instinct { echo: "read".to_string() },
                Instinct { echo: "update".to_string() },
                Instinct { echo: "delete".to_string() },
                Instinct { echo: "activate".to_string() },
                Instinct { echo: "deactivate".to_string() },
            ],
        };

        assert_eq!(larvie.casts.len(), 4);
        assert_eq!(larvie.instincts.len(), 6);

        // Test serialization of complex structure
        let json = serde_json::to_string_pretty(&larvie).unwrap();
        let deserialized: Larvie = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, larvie);
    }
}
