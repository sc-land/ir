# IR - Intermediate Representation

Um sistema de representação intermediária para transformar árvores sintáticas da linguagem SC-DSL em estruturas de dados otimizadas.

## 📋 Visão Geral

O IR (Intermediate Representation) é uma biblioteca Rust que converte árvores sintáticas parseadas pela `sc-dsl` em uma representação intermediária estruturada e tipada. Essa representação facilita análises, transformações e geração de código posterior.

## 🏗️ Arquitetura

### Estruturas Principais

- **`IR`**: Estrutura raiz que contém todos os alvéolos do sistema
- **`Alveolus`**: Enum que representa diferentes tipos de elementos (atualmente `Larvie`)
- **`Larvie`**: Representa entidades estruturadas com campos e comportamentos
- **`Casts`**: Define propriedades de campos com tipos e restrições
- **`Instinct`**: Modela aspectos comportamentais das entidades

### Sistema de Tipos

#### Flora (Tipos de Dados)
- `Int`: Números inteiros
- `Str`: Strings/texto
- `Bool`: Valores booleanos
- `Bug(String)`: Tipos personalizados definidos pelo usuário

#### Seal (Restrições)
- `Vital`: NotNull - campo obrigatório
- `Core`: PrimaryKey - chave primária
- `Root`: Unique - valor único

## 🚀 Uso

### Exemplo Básico

```rust
use ir::sc_tree_to_ir;
use sc_dsl::dsl::parser::tree::Tree;

// Parse da entrada SC-DSL
let tree = Tree::parse_input("
    bug Bird
        gene energy Int
        ethics fly
    end
".to_string()).unwrap();

// Conversão para IR
let ir = sc_tree_to_ir(tree);

println!("{:?}", ir);
```

### Estrutura do IR Gerado

```rust
IR {
    alveolus: [
        Alveolus::Larvie(Larvie {
            primor: "Bird",
            casts: [
                Casts {
                    primor: "energy",
                    flora: Flora::Int,
                    seals: [Seal::Vital]
                }
            ],
            instincts: [
                Instinct {
                    echo: "fly"
                }
            ]
        })
    ]
}
```

## 🧪 Testes

Execute os testes com:

```bash
cargo test
```

### Cobertura de Testes

- ✅ Conversão de `Bug` para `Larvie`
- ✅ Conversão de `Gene` para `Casts`
- ✅ Conversão de `Ethics` para `Instinct`
- ✅ Mapeamento de tipos `Specie` para `Flora`
- ✅ Integração completa do pipeline de conversão

## 📊 Diagrama UML

O projeto inclui documentação UML em `doc/ir.puml` que ilustra a estrutura completa do sistema:

```plantuml
@startuml IR
class IR {
  alveolus Alveolus[]
}
enum Alveolus {
  Larvie
}
class Larvie {
  primor Str
  casts Casts[]
  instincts Instinct[]
}
// ... (veja doc/ir.puml para o diagrama completo)
@enduml
```

## 🔧 Dependências

- `sc-dsl`: Parsing e AST da linguagem SC-DSL
- `serde`: Serialização/deserialização
- `serde_json`: Suporte a JSON

## 📂 Estrutura do Projeto

```
src/
├── lib.rs              # API pública
└── ir/
    ├── mod.rs           # Módulo principal do IR
    ├── alveolus.rs      # Enum de tipos de elementos
    ├── larvie.rs        # Estruturas de entidades
    ├── casts.rs         # Propriedades de campos
    ├── instincts.rs     # Aspectos comportamentais
    ├── flora.rs         # Sistema de tipos
    └── seal.rs          # Restrições de dados
```

## 🌟 Funcionalidades

- **Conversão Automática**: Transforma AST de SC-DSL em IR estruturado
- **Sistema de Tipos Robusto**: Suporte a tipos primitivos e personalizados
- **Modelagem Comportamental**: Captura aspectos comportamentais via instintos
- **Restrições de Dados**: Sistema de selos para validação e restrições
- **Serialização**: Suporte completo a JSON via Serde
- **Testes Abrangentes**: Cobertura de testes para todas as funcionalidades

## 📄 Licença

Este projeto está licenciado sob os termos especificados no arquivo LICENSE.
