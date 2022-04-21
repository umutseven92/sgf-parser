

// When numbering nodes starting with zero is suggested.
// Nodes should be numbered in the way they are stored in the file.
// Example (of file above): root=0, a=1, b=2, c=3, d=4, e=5, f=6, g=7, h=8, i=9 and j=10.
// The order of properties in a node is not fixed. It may change every time the file is saved and
// may vary from application to application.
// Applications should not rely on the order of property values.
use crate::property::Property;

pub struct Node {
    properties: Vec<Property>,
}

impl Node {
    fn new(source: &str) -> Result<Self, &str> {
        Ok(Node::parse(source)?.0)
    }

    pub fn parse(source: &str) -> Result<(Self, usize), &str> {
        let mut properties: Vec<Property> = vec![];

        let mut skip_counter = 0;

        for (index, character) in source.chars().enumerate() {
            let index = index + 1;

            let _prop_id_buffer = String::new();
            let _prop_val_buffer = String::new();

            if skip_counter > 0 {
                skip_counter -= 1;
                continue;
            }

            match character {
                // White space (space, tab, carriage return, line feed, vertical tab and so on) may appear
                // anywhere between PropValues, Properties, Nodes, Sequences and GameTrees.
                ' ' | '\n' | '\t' => (),
                _ => {
                    let remaining_content = source.split_at(index);

                    match Property::parse(remaining_content.1) {
                        Ok(prop_result) => {
                            properties.push(prop_result.0);
                            skip_counter = prop_result.1;
                        }
                        Err(_) => break,
                    }
                }
            }
        }

        todo!()
    }
}

#[cfg(test)]
mod tests {
    

    // #[test]
    // fn can_parse_node_single_property() {
    //     let content = "FF[4]";
    //     let node = Node::parse(content).unwrap().0;
    //
    //     assert_eq!(node.properties.len(), 1);
    //
    //     let prop = node.properties.get(0).unwrap();
    //
    //     assert_eq!(prop.id, "FF");
    // }
    //
    // #[test]
    // fn can_parse_node_multiple_property() {
    //     let content = "AB[dd][de]N[Markup]";
    //     let node = Node::parse(content).unwrap().0;
    //
    //     assert_eq!(node.properties.len(), 2);
    // }
}
