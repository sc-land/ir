
//! # IR - Intermediate Representation
//!
//! Uma biblioteca Rust para conversão de árvores sintáticas SC-DSL em representação intermediária estruturada.
//!
//! ## Visão Geral
//!
//! O IR (Intermediate Representation) transforma estruturas de dados parseadas pela `sc-dsl`
//! em uma representação otimizada para análise, validação e geração de código.
//!
//! ## Estruturas Principais
//!
//! - [`IR`]: Estrutura raiz contendo todos os alvéolos
//! - [`Alveolus`]: Enum para diferentes tipos de elementos
//! - [`Larvie`]: Entidades com campos e comportamentos
//! - [`Casts`]: Propriedades de campos com tipos e restrições
//! - [`Instinct`]: Aspectos comportamentais das entidades
//!
//! ## Exemplo de Uso
//!
//! ```rust
//! use ir::sc_tree_to_ir;
//! use sc_dsl::dsl::parser::tree::Tree;
//!
//! let input = "bug User gene name Str ethics create end";
//! let tree = Tree::parse_input(input.to_string()).unwrap();
//! let ir = sc_tree_to_ir(tree);
//! println!("{:?}", ir);
//! ```

use sc_dsl::dsl::parser::tree::Tree;
use crate::ir::IR;

mod ir;

/// Converte uma árvore sintática SC-DSL em representação intermediária IR.
///
/// Esta função é o ponto de entrada principal da biblioteca, transformando
/// uma árvore parseada em uma estrutura IR otimizada para análises posteriores.
///
/// # Argumentos
///
/// * `tree` - Árvore sintática parseada pela `sc-dsl`
///
/// # Retorna
///
/// Uma estrutura [`IR`] contendo a representação intermediária completa
///
/// # Exemplo
///
/// ```rust
/// use ir::sc_tree_to_ir;
/// use sc_dsl::dsl::parser::tree::Tree;
///
/// let input = "bug Product gene name Str gene price Int end";
/// let tree = Tree::parse_input(input.to_string()).unwrap();
/// let ir = sc_tree_to_ir(tree);
///
/// assert_eq!(ir.alveolus.len(), 1);
/// ```
pub fn sc_tree_to_ir(tree: Tree) -> IR {
    IR::from_sc(tree)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        let input = "bug Bee gene energy Int ethics fly end";
        let tree = Tree::parse_input(input.to_string()).unwrap();
        let ir = sc_tree_to_ir(tree);
        
        assert_eq!(ir.alveolus.len(), 1);
        // Validação detalhada é feita nos testes dos módulos específicos
    }

    #[test]
    fn test_multiple_entities() {
        let input = "bug User gene name Str end bug Post gene title Str end";
        let tree = Tree::parse_input(input.to_string()).unwrap();
        let ir = sc_tree_to_ir(tree);
        
        assert_eq!(ir.alveolus.len(), 2);
    }
}
