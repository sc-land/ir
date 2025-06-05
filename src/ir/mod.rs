mod alveolus;
mod larvie;
mod flora;
mod seal;
mod casts;

use sc_dsl::dsl::ast::genome::Genome;
use sc_dsl::dsl::ast::anatomy::Anatomy;
use sc_dsl::dsl::parser::tree::Tree;
use serde::{Deserialize, Serialize};
use crate::ir::alveolus::Alveolus;
use crate::ir::larvie::Larvie;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct IR {
    pub alveolus: Vec<Alveolus>,
}

impl IR {
    pub fn from_sc(tree: Tree) -> Self {
        let mut alveolus = Vec::new();
        for gene in tree.sc.fly.strand.genome {
            match gene {
                Genome::Anatomy(anatomy) => {
                    match anatomy {
                        Anatomy::Bug(bug) => {
                            alveolus.push(Alveolus::Larvie(Larvie::from_bug(bug)));
                        },
                    }
                },
                Genome::Behavior(_) => {}
            }
        }
        Self { alveolus }
    }
}
