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

use std::{collections::HashMap, error::Error, fmt::Debug};

#[derive(PartialEq, Debug)]
enum PropertyType {
    Move,
    Setup,
    Root,
    GameInfo,
}

enum PropertyValue {
    None,
    // Number with a range.
    Number(u32),
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
}

pub struct Property {
    id: PropertyID,
    value: PropertyValue,
    prop_type: PropertyType,
}

impl Property {
    fn validate_range(min: u32, max: u32, val: u32) -> Result<(), String> {
        if val < min || val > max {
            return Err(format!(
                "Value {val} is not in range (between {min} and {max})"
            ));
        }

        Ok(())
    }

    fn new(id: PropertyID) -> Result<Self, String> {
        match id {
            PropertyID::FF(val) => {
                Self::validate_range(1, 4, val)?;
                Ok(Self {
                    id,
                    value: PropertyValue::Number(val),
                    prop_type: PropertyType::Root,
                })
            }
            _ => todo!(),
        }
    }
}

// Property-identifiers are defined as keywords using only uppercase letters.
// Currently there are no more than two uppercase letters per identifier.

#[derive(PartialEq, Debug)]
enum PropertyID {
    FF(u32),
    GM(u32),
}

enum Color {
    White,
    Black,
}

#[cfg(test)]
mod tests {
    use crate::property::{PropertyID, PropertyType};

    use super::Property;

    #[test]
    fn can_validate_range() {
        // Should not error (in range).
        let prop = Property::new(PropertyID::FF(4)).unwrap();

        assert_eq!(prop.id, PropertyID::FF(4));
        assert_eq!(prop.prop_type, PropertyType::Root);

        let prop = Property::new(PropertyID::FF(1)).unwrap();

        assert_eq!(prop.id, PropertyID::FF(1));

        // Should error (out of range).
        let prop = Property::new(PropertyID::FF(6));

        assert!(prop.is_err());

        // Should error (out of range).
        let prop = Property::new(PropertyID::FF(0));

        assert!(prop.is_err());
    }
}
