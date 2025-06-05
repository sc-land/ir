use sc_dsl::dsl::ast::gene::Gene;
use serde::{Deserialize, Serialize};
use crate::ir::flora::Flora;
use crate::ir::seal::Seal;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Casts {
    pub primor: String,
    pub flora: Flora,
    pub seals: Vec<Seal>,
}

impl Casts {
    pub fn from_gene(gene: Gene) -> Self {
        let primor = gene.tag.raw.clone();
        let flora = Flora::from_specie(gene.specie);
        let seals: Vec<Seal> = vec![Seal::Vital];
        Self { primor, flora, seals }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use sc_dsl::dsl::ast::bug::Bug;
    use crate::ir::casts::Casts;
    use crate::ir::flora::Flora;
    use crate::ir::seal::Seal;

    #[test]
    fn test_casts() {
        let input = "bug Bird gene energy Int end".to_string();
        let bug = Bug::from_string(input);
        let gene = bug.genes[0].clone();
        let casts = Casts::from_gene(gene.clone());

        assert_eq!(gene.tag.raw, casts.primor);
        assert_eq!(Flora::from_specie(gene.specie.clone()), casts.flora);

        if let Flora::Int = casts.flora {
            // Teste passou
        } else {
            panic!("Flora não é do tipo esperado");
        }

        assert_eq!(casts.seals, vec![Seal::Vital]);
    }
}
