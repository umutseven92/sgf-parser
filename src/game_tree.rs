use crate::chars;
use crate::errors::SgfParseError;
use crate::node::Node;

pub struct GameTree {
    // Called `leaves` instead of `nodes` since `Node` has a specific meaning in SFG files.
    leaves: Vec<GameTree>,
    sequence: Vec<Node>,
}

impl GameTree {
    pub fn parse(source: &str) -> Result<(Self, usize), SgfParseError> {
        let mut leaves: Vec<GameTree> = vec![];
        let mut sequence: Vec<Node> = vec![];

        let mut skip_counter = 0;

        let _counting_node = false;

        for (index, character) in source.chars().enumerate() {
            let index = index + 1;
            if skip_counter > 0 {
                skip_counter -= 1;
                continue;
            }

            match character {
                chars::TREE_START => {
                    // We encountered a nested GameTree.
                    let remaining_content = source.split_at(index);
                    match GameTree::parse(remaining_content.1) {
                        Ok(leaf_result) => {
                            leaves.push(leaf_result.0);
                            skip_counter = leaf_result.1;
                        }
                        Err(_) => break,
                    }
                }
                chars::TREE_END => {
                    return Ok((GameTree { leaves, sequence }, index));
                }
                chars::NODE_START => {
                    // We encountered a Node.
                    let remaining_content = source.split_at(index);
                    let node_result = Node::parse(remaining_content.1)?;

                    sequence.push(node_result.0);
                    skip_counter = node_result.1;
                }
                // White space (space, tab, carriage return, line feed, vertical tab and so on) may appear
                // anywhere between PropValues, Properties, Nodes, Sequences and GameTrees.
                ' ' | '\n' | '\t' => (),
                _ => {
                    todo!()
                }
            }
        }

        return Ok((GameTree { leaves, sequence }, source.len()));
    }
}

#[cfg(test)]
mod tests {
    use super::GameTree;

    #[test]
    fn can_parse_single_game_tree() {
        let content = ";FF[4]";
        let tree = GameTree::parse(content).unwrap().0;

        let node = tree.sequence.get(0).unwrap();
        assert_eq!(node.properties.len(), 1);

        let prop = node.properties.get(0).unwrap();
        assert_eq!(prop.id, "FF");
    }

    #[test]
    fn can_parse_nested_game_tree() {
        let content = ";FF[4] (;AP[windows:95])";
        let tree = GameTree::parse(content).unwrap().0;

        assert_eq!(tree.leaves.len(), 1);

        let nested = tree.leaves.get(0).unwrap();

        assert_eq!(nested.leaves.len(), 0);
    }
    //
    // #[test]
    // fn can_parse_consecutive_nested_game_tree() {
    //     let content = "ab (def) (ghi)";
    //     let tree = GameTree::parse(content).unwrap().0;
    //
    //     assert_eq!(tree.content, "ab");
    //     assert_eq!(tree.leaves.len(), 2);
    //
    //     let first_nested = tree.leaves.get(0).unwrap();
    //
    //     assert_eq!(first_nested.content, "def");
    //     assert_eq!(first_nested.leaves.len(), 0);
    //
    //     let second_nested = tree.leaves.get(1).unwrap();
    //
    //     assert_eq!(second_nested.content, "ghi");
    //     assert_eq!(second_nested.leaves.len(), 0);
    // }
    //
    // #[test]
    // fn can_parse_complex_nested_game_tree() {
    //     let content = "ab (def (ghi)) (jkl(mno(pqr)))";
    //     let tree = GameTree::parse(content).unwrap().0;
    //
    //     assert_eq!(tree.content, "ab");
    //     assert_eq!(tree.leaves.len(), 2);
    //
    //     let nested = tree.leaves.get(0).unwrap();
    //
    //     assert_eq!(nested.content, "def");
    //     assert_eq!(nested.leaves.len(), 1);
    //
    //     let nested = nested.leaves.get(0).unwrap();
    //
    //     assert_eq!(nested.content, "ghi");
    //     assert_eq!(nested.leaves.len(), 0);
    //
    //     let nested = tree.leaves.get(1).unwrap();
    //
    //     assert_eq!(nested.content, "jkl");
    //     assert_eq!(nested.leaves.len(), 1);
    //
    //     let nested = nested.leaves.get(0).unwrap();
    //
    //     assert_eq!(nested.content, "mno");
    //     assert_eq!(nested.leaves.len(), 1);
    //
    //     let nested = nested.leaves.get(0).unwrap();
    //
    //     assert_eq!(nested.content, "pqr");
    //     assert_eq!(nested.leaves.len(), 0);
    // }
}
