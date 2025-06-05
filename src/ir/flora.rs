use sc_dsl::dsl::ast::emitter::Specie;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Flora {
    Int,
    Str,
    Bool,
    Bug(String),
}

impl Flora {
    pub fn from_specie(specie: Specie) -> Self {
        match specie.raw.as_str() {
            "Int" => Flora::Int,
            "Bool" => Flora::Bool,
            "Str" => Flora::Str,
            _ => Flora::Bug(specie.raw),
        }
    }
}
