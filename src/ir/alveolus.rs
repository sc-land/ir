
use serde::{Deserialize, Serialize};
use crate::ir::larvie::Larvie;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Alveolus {
    Larvie(Larvie),
}
