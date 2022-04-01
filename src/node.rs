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
