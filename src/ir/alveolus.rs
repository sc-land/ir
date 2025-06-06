use serde::{Deserialize, Serialize};
use crate::ir::larvie::Larvie;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Alveolus {
    Larvie(Larvie),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{flora::Flora, seal::Seal, casts::Casts, instincts::Instinct};
    use serde_json;

    #[test]
    fn test_alveolus_larvie_creation() {
        let larvie = Larvie {
            primor: "TestLarvie".to_string(),
            casts: vec![],
            instincts: vec![],
        };

        let alveolus = Alveolus::Larvie(larvie.clone());

        match alveolus {
            Alveolus::Larvie(l) => {
                assert_eq!(l.primor, "TestLarvie");
                assert_eq!(l.casts.len(), 0);
                assert_eq!(l.instincts.len(), 0);
            }
        }
    }

    #[test]
    fn test_alveolus_with_complex_larvie() {
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
                Instinct { echo: "create".to_string() },
                Instinct { echo: "read".to_string() },
            ],
        };

        let alveolus = Alveolus::Larvie(larvie);

        match alveolus {
            Alveolus::Larvie(l) => {
                assert_eq!(l.primor, "ComplexLarvie");
                assert_eq!(l.casts.len(), 2);
                assert_eq!(l.instincts.len(), 2);
                assert_eq!(l.casts[0].primor, "field1");
                assert_eq!(l.instincts[0].echo, "create");
            }
        }
    }

    #[test]
    fn test_alveolus_serialization() {
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

        let alveolus = Alveolus::Larvie(larvie);

        let json = serde_json::to_string(&alveolus).unwrap();
        let deserialized: Alveolus = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized, alveolus);
    }

    #[test]
    fn test_alveolus_equality() {
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

        let alveolus1 = Alveolus::Larvie(larvie1);
        let alveolus2 = Alveolus::Larvie(larvie2);
        let alveolus3 = Alveolus::Larvie(larvie3);

        assert_eq!(alveolus1, alveolus2);
        assert_ne!(alveolus1, alveolus3);
    }

    #[test]
    fn test_alveolus_clone() {
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

        let original = Alveolus::Larvie(larvie);
        let cloned = original.clone();

        assert_eq!(original, cloned);

        // Ensure deep clone
        match (original, cloned) {
            (Alveolus::Larvie(orig), Alveolus::Larvie(clone)) => {
                assert_eq!(orig.primor, clone.primor);
                assert_eq!(orig.casts.len(), clone.casts.len());
                assert_eq!(orig.instincts.len(), clone.instincts.len());
            }
        }
    }

    #[test]
    fn test_alveolus_debug_format() {
        let larvie = Larvie {
            primor: "DebugLarvie".to_string(),
            casts: vec![],
            instincts: vec![],
        };

        let alveolus = Alveolus::Larvie(larvie);
        let debug_str = format!("{:?}", alveolus);

        assert!(debug_str.contains("Larvie"));
        assert!(debug_str.contains("DebugLarvie"));
    }
}
