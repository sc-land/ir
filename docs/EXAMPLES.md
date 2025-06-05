# Exemplos de Uso do IR

Este documento demonstra casos de uso práticos do sistema IR.

## Exemplo 1: Entidade Simples

### Entrada SC-DSL
```sc
bug User
    gene id Int
    gene name Str
    gene active Bool
    ethics create
    ethics update
end
```

### Código Rust
```rust
use ir::sc_tree_to_ir;
use sc_dsl::dsl::parser::tree::Tree;

let input = "bug User gene id Int gene name Str gene active Bool ethics create ethics update end";
let tree = Tree::parse_input(input.to_string()).unwrap();
let ir = sc_tree_to_ir(tree);

// Resultado IR
assert_eq!(ir.alveolus.len(), 1);
if let Alveolus::Larvie(larvie) = &ir.alveolus[0] {
    assert_eq!(larvie.primor, "User");
    assert_eq!(larvie.casts.len(), 3);
    assert_eq!(larvie.instincts.len(), 2);
}
```

### Estrutura IR Resultante
```rust
IR {
    alveolus: [
        Alveolus::Larvie(Larvie {
            primor: "User",
            casts: [
                Casts { primor: "id", flora: Flora::Int, seals: [Seal::Vital] },
                Casts { primor: "name", flora: Flora::Str, seals: [Seal::Vital] },
                Casts { primor: "active", flora: Flora::Bool, seals: [Seal::Vital] }
            ],
            instincts: [
                Instinct { echo: "create" },
                Instinct { echo: "update" }
            ]
        })
    ]
}
```

## Exemplo 2: Múltiplas Entidades

### Entrada SC-DSL
```sc
bug Post
    gene title Str
    gene content Str
    gene author User
    ethics publish
end

bug User
    gene name Str
    gene email Str
end
```

### Código Rust
```rust
let input = "bug Post gene title Str gene content Str gene author User ethics publish end bug User gene name Str gene email Str end";
let tree = Tree::parse_input(input.to_string()).unwrap();
let ir = sc_tree_to_ir(tree);

assert_eq!(ir.alveolus.len(), 2);

// Primeira entidade: Post
if let Alveolus::Larvie(post) = &ir.alveolus[0] {
    assert_eq!(post.primor, "Post");
    assert_eq!(post.casts[2].flora, Flora::Bug("User".to_string()));
}

// Segunda entidade: User
if let Alveolus::Larvie(user) = &ir.alveolus[1] {
    assert_eq!(user.primor, "User");
    assert_eq!(user.instincts.len(), 0); // Sem behaviors
}
```

## Exemplo 3: Análise de Tipos

```rust
use ir::{sc_tree_to_ir, ir::{flora::Flora, alveolus::Alveolus}};

fn analyze_types(ir: &IR) -> HashMap<String, Vec<String>> {
    let mut type_usage = HashMap::new();

    for alveolus in &ir.alveolus {
        if let Alveolus::Larvie(larvie) = alveolus {
            let mut types = Vec::new();

            for cast in &larvie.casts {
                let type_name = match &cast.flora {
                    Flora::Int => "Int".to_string(),
                    Flora::Str => "Str".to_string(),
                    Flora::Bool => "Bool".to_string(),
                    Flora::Bug(custom) => custom.clone(),
                };
                types.push(type_name);
            }

            type_usage.insert(larvie.primor.clone(), types);
        }
    }

    type_usage
}

// Uso
let tree = Tree::parse_input(input.to_string()).unwrap();
let ir = sc_tree_to_ir(tree);
let types = analyze_types(&ir);
println!("Tipos utilizados: {:?}", types);
```

## Exemplo 4: Serialização JSON

```rust
use serde_json;

let input = "bug Product gene name Str gene price Int ethics sell end";
let tree = Tree::parse_input(input.to_string()).unwrap();
let ir = sc_tree_to_ir(tree);

// Serializar para JSON
let json = serde_json::to_string_pretty(&ir).unwrap();
println!("{}", json);

// Deserializar de volta
let deserialized: IR = serde_json::from_str(&json).unwrap();
assert_eq!(ir, deserialized);
```

### JSON Resultante
```json
{
  "alveolus": [
    {
      "Larvie": {
        "primor": "Product",
        "casts": [
          {
            "primor": "name",
            "flora": "Str",
            "seals": ["Vital"]
          },
          {
            "primor": "price",
            "flora": "Int",
            "seals": ["Vital"]
          }
        ],
        "instincts": [
          {
            "echo": "sell"
          }
        ]
      }
    }
  ]
}
```

## Exemplo 5: Validação de Restrições

```rust
use ir::ir::seal::Seal;

fn validate_constraints(ir: &IR) -> Vec<String> {
    let mut warnings = Vec::new();

    for alveolus in &ir.alveolus {
        if let Alveolus::Larvie(larvie) = alveolus {
            // Verificar se há campo ID
            let has_id = larvie.casts.iter().any(|cast| {
                cast.primor.to_lowercase().contains("id") ||
                cast.seals.contains(&Seal::Core)
            });

            if !has_id {
                warnings.push(format!("Entidade '{}' não possui campo ID", larvie.primor));
            }

            // Verificar campos obrigatórios
            for cast in &larvie.casts {
                if !cast.seals.contains(&Seal::Vital) {
                    warnings.push(format!("Campo '{}' em '{}' pode ser nulo",
                                        cast.primor, larvie.primor));
                }
            }
        }
    }

    warnings
}
```

## Exemplo 6: Geração de Schema SQL

```rust
fn generate_sql_schema(ir: &IR) -> String {
    let mut sql = String::new();

    for alveolus in &ir.alveolus {
        if let Alveolus::Larvie(larvie) = alveolus {
            sql.push_str(&format!("CREATE TABLE {} (\n", larvie.primor));

            for (i, cast) in larvie.casts.iter().enumerate() {
                let sql_type = match cast.flora {
                    Flora::Int => "INTEGER",
                    Flora::Str => "VARCHAR(255)",
                    Flora::Bool => "BOOLEAN",
                    Flora::Bug(_) => "INTEGER", // Foreign key
                };

                let constraints = if cast.seals.contains(&Seal::Vital) {
                    " NOT NULL"
                } else {
                    ""
                };

                sql.push_str(&format!("  {} {}{}", cast.primor, sql_type, constraints));

                if i < larvie.casts.len() - 1 {
                    sql.push_str(",");
                }
                sql.push_str("\n");
            }

            sql.push_str(");\n\n");
        }
    }

    sql
}
```

Estes exemplos demonstram a versatilidade do sistema IR para análise, transformação e geração de código baseado em estruturas SC-DSL.
