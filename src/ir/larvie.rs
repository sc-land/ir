use sc_dsl::dsl::ast::bug::Bug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Larvie {
    pub primor: String,
}

impl Larvie {
    pub fn from_bug(bug: Bug) -> Self {
        let primor = bug.specie.clone();
        Self { primor }
    }
}

#[cfg(test)]
mod tests {
    use sc_dsl::dsl::ast::bug::Bug;
    use crate::ir::larvie::Larvie;

    #[test]
    fn test_larvie() {
        let input = "bug Bird\n gene energy Int\n end".to_string();
        let bug = Bug::from_string(input);
        let larvie = Larvie::from_bug(bug.clone());
        assert_eq!(bug.specie, larvie.primor);
    }
}
