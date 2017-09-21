
use domme;
use std::collections::HashMap;


pub fn parse(source: String) -> domme::Node {
    let mut nodes = Parser { pos: 0, input: source }.parse_nodes();

    /* Create a root node if none exists */
    return match nodes.len() {
        1 => nodes.swap_remove(0),
        _ => domme::elem("html".to_string(), HashMap::new(), nodes)
    }
}


/*** Internal implementation of html parser never exposed ***/

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {

    fn parse_nodes(&mut self) -> Vec<domme::Node> {
        let mut nodes = vec!();

        loop {
            self.white_genocide();
            if self.end_of_file() || self.starts_with("</") {
                break;
            }

            nodes.push(self.parse_node());
        }
        return nodes;
    }

    /** Parse a single node */
    fn parse_node(&mut self) -> domme::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _   => self.parse_text()
        }
    }

    /** Parse a text node */
    fn parse_text(&mut self) -> domme::Node {
        return domme::text(self.consume_while(|c| c != '<'));
    }

    /** Parse a single element (including tags and content) */
    fn parse_element(&mut self) -> domme::Node {
        assert!(self.consume_char() == '<');

        /* Handle opening tag */
        let name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        /* Contents */
        let children = self.parse_nodes();

        /* Handle closing tag */
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == name);
        assert!(self.consume_char() == '>');

        return domme::elem(name, attrs, children);
    }

    /** Handles multi-byte chars correctly too */
    fn next_char(&self) -> char {
        return self.input[self.pos..].chars().next().unwrap();
    }

    /** Small util function which...c'mon the name is obvious */
    fn starts_with(&self, s: &str) -> bool {
        return self.input[self.pos..].starts_with(s);
    }

    /** Not gonna explain this one :) */
    fn end_of_file(&self) -> bool {
        return self.pos >= self.input.len();
    }

    /** Return the current character, and advance self.pos to the next character */
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, curr_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return curr_char;
    }

    /** Consume characters until a condition returns false */
    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.end_of_file() && test(self.next_char()) {
            result.push(self.consume_char());
        }

        return result;
    }

    /** Eat all the whitespaces - oh how could I not? */
    fn white_genocide(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    /** Parse tag or attribute name */
    fn parse_tag_name(&mut self) -> String {
        return self.consume_while(|c| match c {
            'a'...'z' | 'A'...'Z' | '0'...'9' => true,
            _ => false,
        });
    }

    /** Parse a single key="value" pair */
    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        return (name, value);
    }

    /** Parse a quoted value */
    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert!(self.consume_char() == open_quote);
        return value;
    }

    fn parse_attributes(&mut self) -> domme::AttributeMap {
        let mut attributes = HashMap::new();

        loop {
            self.white_genocide(); // Mwuhahaha!
            if self.next_char() == '>' {
                break;
            }

            let (key, value) = self.parse_attr();
            attributes.insert(key, value);
        }

        return attributes;
    }
}
