use std::collections::{HashMap,HashSet};
pub type AtrtibuteMap = HashMap<String, String>;


#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

#[derive(Debug)]
enum NodeType {
    Text(String), Element(ElementData)
}


#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AtrtibuteMap,
}


/** Simple text constructor */
fn text(data: String) -> Node {
    return Node { children: Vec::new(), node_type: NodeType::Text(data) }
}


fn elem(name: String, attr: AtrtibuteMap, children: Vec<Node>) -> Node {
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

