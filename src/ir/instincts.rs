use sc_dsl::dsl::ast::ethics::Ethics;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Instinct {
    pub echo: String,
}

impl Instinct {
    pub fn from_ethics(ethics: Ethics) -> Self {
        let echo = ethics.tag.raw.clone();
        Self { echo }
    }
}

#[cfg(test)]
mod tests {
    use sc_dsl::dsl::ast::ethics::Ethics;
    use crate::ir::instincts::Instinct;

    #[test]
    fn test_instinct_from_ethics() {
      let input = "ethics fly".to_string();
      let ethics = Ethics::from_string(input).unwrap();
      let instinct = Instinct::from_ethics(ethics.clone());
      assert_eq!(instinct.echo, ethics.tag.raw);
    }
}
