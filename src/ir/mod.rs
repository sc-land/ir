pub mod alveolus;
pub mod larvie;
pub mod flora;
pub mod seal;
pub mod casts;
pub mod instincts;

use serde::{Deserialize, Serialize};

// Re-export public types
pub use larvie::Larvie;
pub use flora::Flora;
pub use seal::Seal;
pub use casts::Casts;
pub use instincts::Instinct;
pub use alveolus::Alveolus;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct IR {
    pub alveolus: Vec<Alveolus>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_ir_creation() {
        let ir = IR {
            alveolus: vec![],
        };

        assert_eq!(ir.alveolus.len(), 0);
    }

    #[test]
    fn test_ir_with_alveolus() {
        let larvie = Larvie {
            primor: "TestLarvie".to_string(),
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

        let ir = IR {
            alveolus: vec![
                Alveolus::Larvie(larvie.clone()),
                Alveolus::Larvie(larvie),
            ],
        };

        assert_eq!(ir.alveolus.len(), 2);

        match &ir.alveolus[0] {
            Alveolus::Larvie(l) => {
                assert_eq!(l.primor, "TestLarvie");
                assert_eq!(l.casts.len(), 1);
                assert_eq!(l.instincts.len(), 1);
            }
        }
    }

    #[test]
    fn test_ir_serialization() {
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

        let ir = IR {
            alveolus: vec![Alveolus::Larvie(larvie)],
        };

        let json = serde_json::to_string(&ir).unwrap();
        let deserialized: IR = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.alveolus.len(), ir.alveolus.len());
        assert_eq!(deserialized, ir);
    }

    #[test]
    fn test_ir_equality() {
        let larvie1 = Larvie {
            primor: "SameLarvie".to_string(),
            casts: vec![],
            instincts: vec![],
        };

        let larvie2 = Larvie {
            primor: "SameLarvie".to_string(),
            casts: vec![],
            instincts: vec![],
        };

        let larvie3 = Larvie {
            primor: "DifferentLarvie".to_string(),
            casts: vec![],
            instincts: vec![],
        };

        let ir1 = IR {
            alveolus: vec![Alveolus::Larvie(larvie1)],
        };

        let ir2 = IR {
            alveolus: vec![Alveolus::Larvie(larvie2)],
        };

        let ir3 = IR {
            alveolus: vec![Alveolus::Larvie(larvie3)],
        };

        assert_eq!(ir1, ir2);
        assert_ne!(ir1, ir3);
    }

    #[test]
    fn test_ir_clone() {
        let larvie = Larvie {
            primor: "CloneLarvie".to_string(),
            casts: vec![
                Casts {
                    primor: "field".to_string(),
                    flora: Flora::Str,
                    seals: vec![Seal::Core],
                }
            ],
            instincts: vec![
                Instinct { echo: "action".to_string() }
            ],
        };

        let original = IR {
            alveolus: vec![Alveolus::Larvie(larvie)],
        };

        let cloned = original.clone();
        assert_eq!(original, cloned);

        // Ensure deep clone
        assert_eq!(original.alveolus.len(), cloned.alveolus.len());
    }

    #[test]
    fn test_ir_with_multiple_alveolus() {
        let larvie1 = Larvie {
            primor: "Larvie1".to_string(),
            casts: vec![
                Casts {
                    primor: "id".to_string(),
                    flora: Flora::Int,
                    seals: vec![Seal::Core],
                }
            ],
            instincts: vec![
                Instinct { echo: "create".to_string() },
                Instinct { echo: "read".to_string() },
            ],
        };

        let larvie2 = Larvie {
            primor: "Larvie2".to_string(),
            casts: vec![
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
            ],
            instincts: vec![
                Instinct { echo: "update".to_string() },
                Instinct { echo: "delete".to_string() },
            ],
        };

        let ir = IR {
            alveolus: vec![
                Alveolus::Larvie(larvie1),
                Alveolus::Larvie(larvie2),
            ],
        };

        assert_eq!(ir.alveolus.len(), 2);

        // Test serialization of complex structure
        let json = serde_json::to_string_pretty(&ir).unwrap();
        let deserialized: IR = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ir);
    }

    #[test]
    fn test_ir_debug_format() {
        let larvie = Larvie {
            primor: "DebugLarvie".to_string(),
            casts: vec![],
            instincts: vec![],
        };

        let ir = IR {
            alveolus: vec![Alveolus::Larvie(larvie)],
        };

        let debug_str = format!("{:?}", ir);
        assert!(debug_str.contains("IR"));
        assert!(debug_str.contains("alveolus"));
        assert!(debug_str.contains("DebugLarvie"));
    }
}
