//! I didn't want to call it "style" because I'm an idiot 😂

use css::{Rule, Selector, SimpleSelector, Specificity, Stylesheet, Value};
use domme::{ElementData, Node};
use domme::NodeType::{Element, Text};

use std::collections::HashMap;


type PropertyMap = HashMap<String, Value>;
type MatchedRule<'a> = (Specificity, &'a Rule);

pub struct StyledNode<'a> {
    pub node: &'a Node,
    pub specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}


/// Apply a stylesheet to an entire DOM tree, returning a StyledNode tree
/// 
/// This only deals with specified values and simple selectors for now and 
/// as you might expect...none of it cascades in any way
pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {

    return StyledNode {
        node: root,
        specified_values: match root.node_type {
            Element(ref elem) => specified_values(elem, stylesheet),
            Text(_) => HashMap::new()
        },
        children: root.children.iter().map(|c| style_tree(c, stylesheet)).collect(),

    };

}


/// Match rules to an element from the domme tree
fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    return rule.selectors
        .iter()
        .find(|sel| matches(elem, *sel))
        .map(|sel| (sel.specificity(), rule));
}


fn matching_rules<'a>(elem: &ElementData, sheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    return sheet.rules.iter().filter_map(|rule| match_rule(elem, rule)).collect();
}


fn specified_values(elem: &ElementData, sheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, sheet);

    // Sort by specificity
    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    for (_, rule) in rules {
        for decl in &rule.declarations {
            values.insert(decl.name.clone(), decl.value.clone());
        }
    }

    return values;
}


/// Does any old selector match?
fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Selector::Simple(ref s_select) => match_simple(elem, s_select),
    }
}


/// Run a match compare for SimpleSelectors
fn match_simple(elem: &ElementData, sel: &SimpleSelector) -> bool {
    if sel.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    if sel.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    let ec = elem.classes();
    if sel.class.iter().any(|class| !ec.contains(&**class)) {
        return false;
    }

    // Must be true then
    return true;
}
