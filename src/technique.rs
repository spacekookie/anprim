//! I didn't want to call it "style" because I'm an idiot 😂

use css::{Rule, Selector, SimpleSelector, Specificity, Stylesheet, Value};
use domme::{ElementData, Node, NodeType};
use std::collections::HashMap;


type PropertyMap = HashMap<String, Value>;
type MatchedRule<'a> = (Specificity, &'a Rule);

struct StyledNode<'a> {
    node: &'a Node,
    specified_values: PropertyMap,
    children: Vec<StyledNode<'a>>,
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
