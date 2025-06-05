use sc_dsl::dsl::ast::bug::Bug;
use serde::{Deserialize, Serialize};

use crate::ir::{casts::Casts, instincts::Instinct};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Larvie {
    pub primor: String,
    pub casts: Vec<Casts>,
    pub instincts: Vec<Instinct>,
}

impl Larvie {
    pub fn from_bug(bug: Bug) -> Self {
        let primor = bug.specie.clone();
        let mut casts: Vec<Casts> = vec![];
        let mut instincts: Vec<Instinct> = vec![];
        for gene in bug.genes {
            casts.push(Casts::from_gene(gene));
        }
        for ethics in bug.ethics {
            instincts.push(Instinct::from_ethics(ethics));
        }
        Self { primor, casts, instincts }
    }
}

#[cfg(test)]
mod tests {
    use sc_dsl::dsl::ast::bug::Bug;
    use crate::ir::flora::Flora;
    use crate::ir::larvie::Larvie;

    #[test]
    fn test_larvie() {
        let input = "bug Bird gene energy Int end".to_string();
        let bug = Bug::from_string(input);
        let larvie = Larvie::from_bug(bug.clone());
        assert_eq!(bug.specie, larvie.primor);
        assert_eq!(bug.genes.len(), larvie.casts.len());
        for (gene, cast) in bug.genes.iter().zip(larvie.casts.iter()) {
            assert_eq!(gene.tag.raw, cast.primor);
            // Valida a equivalência entre cast.flora e gene.specie
            let expected_flora = Flora::from_specie(gene.specie.clone());
            assert_eq!(cast.flora, expected_flora);
        }
    }
}
