# Documentação Técnica - IR

## Transformações de Dados

### 1. Bug → Larvie

A conversão de `Bug` (entidade da SC-DSL) para `Larvie` (representação IR) envolve:

```rust
impl Larvie {
    pub fn from_bug(bug: Bug) -> Self {
        let primor = bug.specie.clone();
        let mut casts: Vec<Casts> = vec![];
        let mut instincts: Vec<Instinct> = vec![];

        // Conversão de genes para casts
        for gene in bug.genes {
            casts.push(Casts::from_gene(gene));
        }

        // Conversão de ethics para instincts
        for ethics in bug.ethics {
            instincts.push(Instinct::from_ethics(ethics));
        }

        Self { primor, casts, instincts }
    }
}
```

### 2. Gene → Casts

Cada gene é transformado em um cast que define propriedades de campo:

```rust
impl Casts {
    pub fn from_gene(gene: Gene) -> Self {
        let primor = gene.tag.raw.clone();           // Nome do campo
        let flora = Flora::from_specie(gene.specie); // Tipo do campo
        let seals: Vec<Seal> = vec![Seal::Vital];    // Restrições padrão
        Self { primor, flora, seals }
    }
}
```

### 3. Ethics → Instinct

Comportamentos são capturados como instintos:

```rust
impl Instinct {
    pub fn from_ethics(ethics: Ethics) -> Self {
        let echo = ethics.tag.raw.clone();
        Self { echo }
    }
}
```

## Mapeamento de Tipos

O sistema Flora mapeia tipos da SC-DSL para representações IR:

```rust
impl Flora {
    pub fn from_specie(specie: Specie) -> Self {
        match specie.raw.as_str() {
            "Int" => Flora::Int,
            "Bool" => Flora::Bool,
            "Str" => Flora::Str,
            custom => Flora::Bug(custom.to_string()), // Tipos personalizados
        }
    }
}
```

## Casos de Uso

### 1. Análise de Esquemas
O IR permite análise estrutural do código SC-DSL:
- Identificação de tipos utilizados
- Validação de restrições de dados
- Detecção de padrões comportamentais

### 2. Geração de Código
A representação estruturada facilita geração de:
- Esquemas de banco de dados
- Estruturas de dados em outras linguagens
- APIs REST baseadas na estrutura

### 3. Validação Semântica
O IR permite validações avançadas:
- Verificação de tipos
- Consistência de restrições
- Análise de dependências

## Extensibilidade

### Adicionando Novos Tipos Flora
```rust
// Em flora.rs
pub enum Flora {
    Int,
    Str,
    Bool,
    Float,        // Novo tipo
    DateTime,     // Novo tipo
    Bug(String),
}
```

### Adicionando Novas Restrições Seal
```rust
// Em seal.rs
pub enum Seal {
    Vital,        // NotNull
    Core,         // PrimaryKey
    Root,         // Unique
    Foreign,      // ForeignKey - Nova restrição
    Indexed,      // Index - Nova restrição
}
```

### Adicionando Novos Alvéolos
```rust
// Em alveolus.rs
pub enum Alveolus {
    Larvie(Larvie),
    Relation(Relation),  // Novo tipo
    Function(Function),  // Novo tipo
}
```

## Considerações de Performance

1. **Clonagem Eficiente**: Uso de `String` para nomes pode ser otimizado com `Cow<str>` ou `Arc<str>`
2. **Alocações**: Vectors são pré-alocados quando possível
3. **Serialização**: Serde otimiza automaticamente a serialização JSON

## Padrões de Design

1. **From Trait**: Implementação consistente de conversões
2. **Builder Pattern**: Futura implementação para construção manual de IR
3. **Visitor Pattern**: Para análises complexas sobre a estrutura IR
