use sc_dsl::dsl::parser::tree::Tree;
use crate::ir::IR;

mod ir;


pub fn sc_tree_to_ir(tree: Tree) -> IR {
    let ir = IR::from_sc(tree);
    return ir;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ir_works() {
        let tree = Tree::parse_input("bug Bee
            gene energy Int
            ethics fly
        end".to_string()).unwrap();
        println!("[tree] {:?}", tree);
        let ir = sc_tree_to_ir(tree.clone());
        println!("[ir] {:?}", ir);
    }
}
