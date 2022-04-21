// From the SGF spec (https://www.red-bean.com/sgf/sgf4.html):

// The order of property values might change as well.

// Everybody is free to define additional, private properties,
// as long as they do not interfere with the standard properties defined in this document.

// Therefore, it is important to skip unknown properties.
// An application should issue a warning message when skipping unknown or faulty properties.

// Only one of each property is allowed per node, e.g. one cannot have two comments in one node:
// ... ;  C[comment1]  B  [dg]  C[comment2] ; ...
// This is an error.

// Each property has a property type. Property types place restrictions on certain properties,
// e.g. in which nodes they are allowed and with which properties they may be combined.

use crate::errors::SgfParseError;
use std::error::Error;
use std::fmt::Debug;

const PROP_VAL_START: char = '[';
const PROP_VAL_END: char = ']';

enum PropertyType {
    Move,
    Setup,
    Root,
    GameInfo,
}

#[derive(Debug)]
pub enum PropertyValue {
    None,
    // Number with a range.
    Number(u32, u32, u32),
    Real(String),
    // Double values are used for annotation properties.
    // They are called Double because the value is either simple or emphasized.
    // A value of '1' means 'normal'; '2' means that it is emphasized.
    // Example:
    // GB[1] could be displayed as: Good for black
    // GB[2] could be displayed as: Very good for blacka
    Double(bool),
    Color(Color),
    // SimpleText is a simple string. Whitespaces other than space must be converted to space,
    // i.e. there's no newline!
    // Linebreaks preceded by a "\" are converted to "", i.e. they are removed (same as Text type).
    // All other linebreaks are converted to space (no newline on display!!).

    // "\" is the escape character. Any char following "\" is inserted verbatim
    // (exception: whitespaces still have to be converted to space!).
    // Following chars have to be escaped, when used in SimpleText: "]", "\" and ":" (only if used in compose data type).
    // SimpleTexts can be encoded in different charsets. See CA property.
    SimpleText(String),

    // Text is a formatted text. White spaces other than linebreaks are converted to space (e.g. no tab, vertical tab, ..).
    // Linebreaks preceded by a "\" (soft linebreaks are converted to "", i.e. they are removed)
    // Hard line breaks: any other linebreaks encountered

    // "\" is the escape character. Any char following "\" is inserted verbatim
    // (exception: whitespaces still have to be converted to space!).
    // Following chars have to be escaped, when used in Text: "]", "\" and ":" (only if used in compose data type).
    // Texts can be encoded in different charsets. See CA property.
    Text(String),

    // The rest of these are game specific.
    Point,
    Move,
    Stone,
    Compose(Box<PropertyValue>, Box<PropertyValue>),
}

impl PropertyValue {
    fn validate(&self) -> Result<(), SgfParseError> {
        match self {
            PropertyValue::None => Ok(()),
            PropertyValue::Number(val, min, max) => {
                if val < min || val > max {
                    Err(SgfParseError::new(format!(
                        "Value {} not in range (min {}, max {})",
                        val, min, max
                    )))
                } else {
                    Ok(())
                }
            }
            PropertyValue::Real(val) => Ok(()),
            PropertyValue::Double(val) => Ok(()),
            PropertyValue::Color(val) => Ok(()),
            PropertyValue::SimpleText(val) => Ok(()),
            PropertyValue::Text(val) => Ok(()),
            PropertyValue::Point => Ok(()),
            PropertyValue::Move => Ok(()),
            PropertyValue::Stone => Ok(()),
            PropertyValue::Compose(val_1, val_2) => Ok(()),
        }
    }
}

impl PartialEq for PropertyValue {
    fn eq(&self, other: &Self) -> bool {
        match self {
            PropertyValue::Number(a, b, c) => {
                if let PropertyValue::Number(x, y, z) = other {
                    return a == x && b == y && c == z;
                };

                todo!()
            }
            _ => todo!(),
        }
    }
}

pub struct Property {
    pub id: String,
    pub values: Vec<PropertyValue>,
}

impl Property {
    pub fn parse(source: &str) -> Result<(Self, usize), SgfParseError> {
        let mut parse_mode = PropParseMode::ID;
        let mut prop_id = String::new();
        let mut values = vec![];
        let mut prop_id_buffer = String::new();
        let mut prop_val_buffer = String::new();

        for (index, character) in source.chars().enumerate() {
            let index = index + 1;

            match character {
                PROP_VAL_START => {
                    // Property values are starting.
                    // Properties have only one ID, so we are done with the prop_id_buffer.
                    parse_mode = PropParseMode::Value;
                    prop_id = prop_id_buffer.clone();
                }
                PROP_VAL_END => {
                    let val = prop_val_buffer.as_str();
                    let prop_val = Property::get_prop_val(prop_id.as_str(), val)?;
                    prop_val.validate()?;
                    values.push(prop_val);
                    prop_val_buffer.clear();
                }
                // White space (space, tab, carriage return, line feed, vertical tab and so on) may appear
                // anywhere between PropValues, Properties, Nodes, Sequences and GameTrees.
                ' ' | '\n' | '\t' => (),
                other => {
                    if index >= 2 && source.chars().nth(index - 2).unwrap() == PROP_VAL_END {
                        return Ok((
                            Property {
                                id: prop_id,
                                values,
                            },
                            index - 2,
                        ));
                    }
                    match parse_mode {
                        PropParseMode::ID => prop_id_buffer.push(other),
                        PropParseMode::Value => prop_val_buffer.push(other),
                    };
                }
            }
        }

        return Ok((
            Property {
                id: prop_id,
                values,
            },
            source.len(),
        ));
    }

    fn get_prop_val(id: &str, val: &str) -> Result<PropertyValue, SgfParseError> {
        let prop_val = match id {
            "FF" => {
                let converted = match val.parse::<u32>() {
                    Ok(x) => x,
                    Err(err) => Err(SgfParseError::new(err.to_string()))?,
                };
                PropertyValue::Number(converted, 1, 4)
            }
            _ => todo!(),
        };

        Ok(prop_val)
    }
}

enum PropParseMode {
    ID,
    Value,
}

// Property-identifiers are defined as keywords using only uppercase letters.
// Currently there are no more than two uppercase letters per identifier.

#[derive(Debug)]
enum Color {
    White,
    Black,
}

#[cfg(test)]
mod tests {
    use crate::property::{Property, PropertyValue};
    use test_case::test_case;

    #[test]
    fn can_parse_property() {
        let content = "FF[4]";
        let property = Property::parse(content).unwrap().0;

        assert_eq!(property.id, "FF");

        assert_eq!(property.values.len(), 1);

        let val = property.values.get(0).unwrap();
        assert_eq!(*val, PropertyValue::Number(4, 1, 4))
    }

    #[test_case("0" ; "Below min")]
    #[test_case("5" ; "Above max")]
    #[test_case("abcde" ; "Non-number")]
    fn ff_property_validation(val: &str) {
        let content = format!("FF[{}]", val);

        let property = Property::parse(content.as_str());

        assert!(property.is_err());
    }

    #[test]
    fn can_parse_property_multiple_value() {
        let content = "FF[1][2][3][4]";
        let property = Property::parse(content).unwrap().0;

        assert_eq!(property.id, "FF");

        assert_eq!(property.values.len(), 4);

        for i in 0..4 {
            assert_eq!(
                *property.values.get(i).unwrap(),
                PropertyValue::Number((i + 1) as u32, 1, 4)
            )
        }
    }
}
