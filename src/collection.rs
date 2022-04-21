use crate::game_tree::{GameTree};

const TREE_START: char = '(';
const TREE_END: char = ')';

pub struct Collection {
    game_trees: Vec<GameTree>,
}

impl Collection {
    pub fn new(source: &str) -> Result<Self, &str> {
        Ok(Self::parse(source)?)
    }

    fn parse(source: &str) -> Result<Self, &str> {
        let mut skip_counter = 0;
        let mut game_trees: Vec<GameTree> = vec![];

        for (index, character) in source.chars().enumerate() {
            let index = index + 1;

            if skip_counter > 0 {
                skip_counter -= 1;
                continue;
            }

            match character {
                TREE_START => {
                    // We encountered a nested GameTree.
                    let remaining_content = source.split_at(index);
                    let leaf_result = GameTree::parse(remaining_content.1)?;
                    game_trees.push(leaf_result.0);
                    skip_counter = leaf_result.1
                }
                // White space (space, tab, carriage return, line feed, vertical tab and so on) may appear
                // anywhere between PropValues, Properties, Nodes, Sequences and GameTrees.
                ' ' | '\n' | '\t' => (),
                _ => todo!(),
            }
        }

        Ok(Collection { game_trees })
    }
}

#[cfg(test)]
mod tests {
    use super::Collection;

    #[test]
    fn can_parse_multiple_game_trees() {
        let content = "( ab ) ( cd )";
        let collection = Collection::new(content).unwrap();

        assert_eq!(collection.game_trees.len(), 2);
        assert_eq!(collection.game_trees.get(0).unwrap().content, "ab");
        assert_eq!(collection.game_trees.get(1).unwrap().content, "cd");
    }
}
