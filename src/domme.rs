use std::collections::{HashMap,HashSet};
use std::fmt;

pub type AttributeMap = HashMap<String, String>;


#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Element(ElementData),
    Text(String),
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttributeMap,
}

/** Simple text constructor */
pub fn text(data: String) -> Node {
    return Node { children: Vec::new(), node_type: NodeType::Text(data) }
}

pub fn elem(name: String, attr: AttributeMap, children: Vec<Node>) -> Node {
    return Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attr
        })
    }
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        return self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        return match self.attributes.get("class") {
            Some(clist) => clist.split(' ').collect(),
            None        => HashSet::new()
        }
    }
}

impl fmt::Display for ElementData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "< {} >", self.tag_name);
    }
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match *self {
            NodeType::Element(ref x) => write!(f, "({})", x),
            NodeType::Text(ref x) => write!(f, "({})", x),
        }
    }
}
