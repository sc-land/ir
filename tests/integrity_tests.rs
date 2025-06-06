//! Comprehensive test suite for the IR library
//!
//! This module contains integration tests and unit tests to ensure
//! the integrity of the IR library as an independent component.

use ::ir::*;
use serde_json;

#[cfg(test)]
mod ir_integrity_tests {
    use super::*;

    #[test]
    fn test_ir_basic_creation() {
        let ir = IR {
            alveolus: vec![]
        };

        assert_eq!(ir.alveolus.len(), 0);
        assert_eq!(ir, IR { alveolus: vec![] });
    }

    #[test]
    fn test_ir_with_single_larvie() {
        let larvie = Larvie {
            primor: "User".to_string(),
            casts: vec![
                Casts {
                    primor: "id".to_string(),
                    flora: Flora::Int,
                    seals: vec![Seal::Vital, Seal::Core],
                }
            ],
            instincts: vec![
                Instinct {
                    echo: "create".to_string(),
                }
            ],
        };

        let ir = IR {
            alveolus: vec![Alveolus::Larvie(larvie.clone())]
        };

        assert_eq!(ir.alveolus.len(), 1);
        let Alveolus::Larvie(ref stored_larvie) = ir.alveolus[0];
        assert_eq!(stored_larvie.primor, "User");
        assert_eq!(stored_larvie.casts.len(), 1);
        assert_eq!(stored_larvie.instincts.len(), 1);
    }

    #[test]
    fn test_ir_with_multiple_entities() {
        let user_larvie = Larvie {
            primor: "User".to_string(),
            casts: vec![
                Casts {
                    primor: "id".to_string(),
                    flora: Flora::Int,
                    seals: vec![Seal::Core],
                },
                Casts {
                    primor: "name".to_string(),
                    flora: Flora::Str,
                    seals: vec![Seal::Vital],
                },
            ],
            instincts: vec![
                Instinct { echo: "create".to_string() },
                Instinct { echo: "update".to_string() },
            ],
        };

        let product_larvie = Larvie {
            primor: "Product".to_string(),
            casts: vec![
                Casts {
                    primor: "price".to_string(),
                    flora: Flora::Int,
                    seals: vec![Seal::Vital],
                },
                Casts {
                    primor: "available".to_string(),
                    flora: Flora::Bool,
                    seals: vec![Seal::Root],
                },
            ],
            instincts: vec![
                Instinct { echo: "sell".to_string() },
            ],
        };

        let ir = IR {
            alveolus: vec![
                Alveolus::Larvie(user_larvie),
                Alveolus::Larvie(product_larvie),
            ]
        };

        assert_eq!(ir.alveolus.len(), 2);
    }

    #[test]
    fn test_ir_json_serialization() {
        let larvie = Larvie {
            primor: "TestEntity".to_string(),
            casts: vec![
                Casts {
                    primor: "field1".to_string(),
                    flora: Flora::Str,
                    seals: vec![Seal::Vital],
                }
            ],
            instincts: vec![
                Instinct { echo: "action1".to_string() }
            ],
        };

        let ir = IR {
            alveolus: vec![Alveolus::Larvie(larvie)]
        };

        // Test serialization
        let json = serde_json::to_string(&ir).expect("Should serialize to JSON");
        assert!(json.contains("TestEntity"));
        assert!(json.contains("field1"));
        assert!(json.contains("action1"));

        // Test deserialization
        let deserialized: IR = serde_json::from_str(&json).expect("Should deserialize from JSON");
        assert_eq!(deserialized, ir);
    }

    #[test]
    fn test_ir_json_pretty_serialization() {
        let ir = IR {
            alveolus: vec![
                Alveolus::Larvie(Larvie {
                    primor: "Beautiful".to_string(),
                    casts: vec![
                        Casts {
                            primor: "elegant".to_string(),
                            flora: Flora::Bool,
                            seals: vec![Seal::Core, Seal::Root],
                        }
                    ],
                    instincts: vec![
                        Instinct { echo: "shine".to_string() }
                    ],
                })
            ]
        };

        let pretty_json = serde_json::to_string_pretty(&ir).expect("Should serialize to pretty JSON");

        // Should contain proper formatting
        assert!(pretty_json.contains("{\n"));
        assert!(pretty_json.contains("  "));

        // Should deserialize back correctly
        let deserialized: IR = serde_json::from_str(&pretty_json).expect("Should deserialize from pretty JSON");
        assert_eq!(deserialized, ir);
    }

    #[test]
    fn test_empty_ir_serialization() {
        let empty_ir = IR { alveolus: vec![] };

        let json = serde_json::to_string(&empty_ir).unwrap();
        let deserialized: IR = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.alveolus.len(), 0);
        assert_eq!(deserialized, empty_ir);
    }
}

#[cfg(test)]
mod flora_tests {
    use super::*;

    #[test]
    fn test_flora_variants() {
        assert_eq!(Flora::Int, Flora::Int);
        assert_eq!(Flora::Str, Flora::Str);
        assert_eq!(Flora::Bool, Flora::Bool);
        assert_eq!(Flora::Bug("Ant".to_string()), Flora::Bug("Ant".to_string()));

        assert_ne!(Flora::Int, Flora::Str);
        assert_ne!(Flora::Bool, Flora::Bug("Ant".to_string()));
    }

    #[test]
    fn test_flora_serialization() {
        let variants = vec![Flora::Int, Flora::Str, Flora::Bool, Flora::Bug("Tick".to_string())];

        for variant in variants {
            let json = serde_json::to_string(&variant).unwrap();
            let deserialized: Flora = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, deserialized);
        }
    }

    #[test]
    fn test_flora_debug_display() {
        assert_eq!(format!("{:?}", Flora::Int), "Int");
        assert_eq!(format!("{:?}", Flora::Str), "Str");
        assert_eq!(format!("{:?}", Flora::Bool), "Bool");
        assert_eq!(format!("{:?}", Flora::Bug("Wasp".to_string())), "Bug(\"Wasp\")");
    }

    #[test]
    fn test_flora_clone_and_eq() {
        let original = Flora::Str;
        let cloned = original.clone();
        assert_eq!(original, cloned);

        let different = Flora::Int;
        assert_ne!(original, different);
    }
}

#[cfg(test)]
mod seal_tests {
    use super::*;

    #[test]
    fn test_seal_variants() {
        assert_eq!(Seal::Vital, Seal::Vital);
        assert_eq!(Seal::Core, Seal::Core);
        assert_eq!(Seal::Root, Seal::Root);

        assert_ne!(Seal::Vital, Seal::Core);
        assert_ne!(Seal::Core, Seal::Root);
        assert_ne!(Seal::Vital, Seal::Root);
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
    fn test_seal_multiple_in_vector() {
        let seals = vec![Seal::Vital, Seal::Core, Seal::Root];
        let json = serde_json::to_string(&seals).unwrap();
        let deserialized: Vec<Seal> = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, seals);
    }

    #[test]
    fn test_seal_empty_vector() {
        let empty_seals: Vec<Seal> = vec![];
        let json = serde_json::to_string(&empty_seals).unwrap();
        let deserialized: Vec<Seal> = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.len(), 0);
    }
}

#[cfg(test)]
mod casts_tests {
    use super::*;

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
            flora: Flora::Bug("Roach".to_string()),
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
        let flora_types = vec![Flora::Int, Flora::Str, Flora::Bool, Flora::Bug("Rat".to_string())];

        for flora in flora_types {
            let cast = Casts {
                primor: format!("field_{:?}", flora),
                flora: flora.clone(),
                seals: vec![Seal::Vital],
            };

            assert_eq!(cast.flora, flora);
        }
    }
}

#[cfg(test)]
mod instinct_tests {
    use super::*;

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
}

#[cfg(test)]
mod larvie_tests {
    use super::*;

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
}

#[cfg(test)]
mod alveolus_tests {
    use super::*;

    #[test]
    fn test_alveolus_larvie_variant() {
        let larvie = Larvie {
            primor: "TestLarvie".to_string(),
            casts: vec![],
            instincts: vec![],
        };

        let alveolus = Alveolus::Larvie(larvie.clone());

        let Alveolus::Larvie(ref stored_larvie) = alveolus;
        assert_eq!(stored_larvie.primor, "TestLarvie");
    }

    #[test]
    fn test_alveolus_serialization() {
        let larvie = Larvie {
            primor: "SerializableLarvie".to_string(),
            casts: vec![
                Casts {
                    primor: "test".to_string(),
                    flora: Flora::Int,
                    seals: vec![Seal::Vital],
                }
            ],
            instincts: vec![],
        };

        let alveolus = Alveolus::Larvie(larvie);

        let json = serde_json::to_string(&alveolus).unwrap();
        let deserialized: Alveolus = serde_json::from_str(&json).unwrap();

        let (Alveolus::Larvie(original), Alveolus::Larvie(deserialized)) = (&alveolus, &deserialized);
        assert_eq!(original.primor, deserialized.primor);
    }

    #[test]
    fn test_alveolus_clone_and_eq() {
        let larvie = Larvie {
            primor: "CloneTest".to_string(),
            casts: vec![],
            instincts: vec![],
        };

        let original = Alveolus::Larvie(larvie);
        let cloned = original.clone();

        assert_eq!(original, cloned);
    }
}

#[cfg(test)]
mod integration_edge_cases {
    use super::*;

    #[test]
    fn test_deeply_nested_structure() {
        let complex_ir = IR {
            alveolus: vec![
                Alveolus::Larvie(Larvie {
                    primor: "Entity1".to_string(),
                    casts: vec![
                        Casts {
                            primor: "field_with_all_seals".to_string(),
                            flora: Flora::Int,
                            seals: vec![Seal::Vital, Seal::Core, Seal::Root],
                        },
                        Casts {
                            primor: "another_field".to_string(),
                            flora: Flora::Bug("Fox".to_string()),
                            seals: vec![],
                        },
                    ],
                    instincts: vec![
                        Instinct { echo: "complex_action".to_string() },
                    ],
                }),
                Alveolus::Larvie(Larvie {
                    primor: "Entity2".to_string(),
                    casts: vec![],
                    instincts: vec![
                        Instinct { echo: "action1".to_string() },
                        Instinct { echo: "action2".to_string() },
                        Instinct { echo: "action3".to_string() },
                    ],
                }),
            ]
        };

        // Test full serialization roundtrip
        let json = serde_json::to_string_pretty(&complex_ir).unwrap();
        let deserialized: IR = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized, complex_ir);
        assert_eq!(deserialized.alveolus.len(), 2);
    }

    #[test]
    fn test_unicode_support() {
        let unicode_ir = IR {
            alveolus: vec![
                Alveolus::Larvie(Larvie {
                    primor: "Usuario_测试_🚀".to_string(),
                    casts: vec![
                        Casts {
                            primor: "名前".to_string(),
                            flora: Flora::Str,
                            seals: vec![Seal::Vital],
                        }
                    ],
                    instincts: vec![
                        Instinct { echo: "создать_用户".to_string() }
                    ],
                })
            ]
        };

        let json = serde_json::to_string(&unicode_ir).unwrap();
        let deserialized: IR = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized, unicode_ir);
    }

    #[test]
    fn test_large_structure_performance() {
        let mut large_casts = Vec::new();
        let mut large_instincts = Vec::new();

        // Create 100 fields and 50 actions
        for i in 0..100 {
            large_casts.push(Casts {
                primor: format!("field_{}", i),
                flora: match i % 4 {
                    0 => Flora::Int,
                    1 => Flora::Str,
                    2 => Flora::Bool,
                    _ => Flora::Bug("Bee".to_string()),
                },
                seals: vec![Seal::Vital],
            });
        }

        for i in 0..50 {
            large_instincts.push(Instinct {
                echo: format!("action_{}", i),
            });
        }

        let large_ir = IR {
            alveolus: vec![
                Alveolus::Larvie(Larvie {
                    primor: "LargeEntity".to_string(),
                    casts: large_casts,
                    instincts: large_instincts,
                })
            ]
        };

        // Should handle serialization of large structures
        let json = serde_json::to_string(&large_ir).unwrap();
        let deserialized: IR = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.alveolus.len(), 1);
        let Alveolus::Larvie(ref larvie) = deserialized.alveolus[0];
        assert_eq!(larvie.casts.len(), 100);
        assert_eq!(larvie.instincts.len(), 50);
    }
}
