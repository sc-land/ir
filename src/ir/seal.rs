use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Seal {
    Vital,  // NotNull
    Core,   // PrimaryKey
    Root,   // Unique
}
