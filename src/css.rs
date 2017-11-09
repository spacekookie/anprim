//! A super simple CSS which parser for a very simple browser-engine
//!
//! It only supports a small subset of CSS syntax and doesn't throw errors
//! for everything that could go wrong. Should probably improve that :)

use std::ascii::AsciiExt;


#[derive(Debug)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

#[derive(Debug)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
}

#[derive(Debug)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColourValue(Colour),
}

#[derive(Debug)]
pub enum Unit {
    PX,
}

/* Colour spelt correctly */
#[derive(Debug)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub type Specificity = (usize, usize, usize);

impl Selector {

    /// To find out moar go to 
    ///    http://www.w3.org/TR/selectors/#specificity
    ///
    pub fn specificity(&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();
        return (a, b, c);
    }
}

impl Value {
    pub fn to_px(&self) -> f32 {
        return match *self {
            Value::Length(f, Unit::PX) => f,
            _ => 0.0,
        }
    }
}


/// Parse an entire CSS sheet and return a Stylesheet
pub fn parse(src: String) -> Stylesheet {
    let mut parser = Parser {
        pos: 0,
        input: src,
    };

    return Stylesheet {
        rules: parser.parse_rules()
    }
}


struct Parser {
    pos: usize,
    input: String,
}


impl Parser {

    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();
        loop {
            self.white_genocide();
            if self.eof() {
                break;
            }

            rules.push(self.parse_rule());
        }

        return rules;
    }

    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations()
        }
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.white_genocide();
            match self.next_char() {
                ',' => {
                    self.consume_char();
                    self.white_genocide();
                },
                '{' => break, // Starts a declaration
                c   => panic!("Unexpected character {} in selector list!", c)
            }
        }

        selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
        return selectors
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        assert!(self.consume_char() == '{'); // Safety, yo
        let mut decl = Vec::new();
        loop {
            self.white_genocide();
            if self.next_char() == '}' {
                self.consume_char();
                break;
            }

            decl.push(self.parse_declaration());
        }

        return decl;
    }

    fn parse_declaration(&mut self) -> Declaration {
        let prop_name = self.parse_identifier();
        self.white_genocide();
        assert!(self.consume_char() == ':');
        self.white_genocide();

        /* After the ':' */
        self.white_genocide();
        let value = self.parse_value();
        self.white_genocide();
        assert!(self.consume_char() == ';'); // Must end with ';' eventually

        return Declaration {
            name: prop_name,
            value: value
        }
    }

    fn parse_value(&mut self) -> Value {
        return match self.next_char() {
            '0'...'9' => self.parse_length(),
            '#' => self.parse_colour(),
            _ => Value::Keyword(self.parse_identifier())
        };
    }

    fn parse_length(&mut self) -> Value {
        return Value::Length(self.parse_float(), self.parse_unit());
    }

    fn parse_colour(&mut self) -> Value {

        /* MUST start with a hashtag */
        assert!(self.consume_char() == '#');

        return Value::ColourValue(Colour {
            r: self.parse_hexpair(),
            g: self.parse_hexpair(),
            b: self.parse_hexpair(),
            a: 255
        });
    }

    fn parse_float(&mut self) -> f32 {
        let s = self.consume_while(|c| match c {
            '0'...'9' | '.' => true,
            _ => false
        });

        return s.parse().unwrap();
    }

    fn parse_unit(&mut self) -> Unit {
        return match &*self.parse_identifier().to_ascii_lowercase() {
            "px" => Unit::PX,
            _ => panic!("Unknown unit!")
        }
    }

    fn parse_hexpair(&mut self) -> u8 {
        let s = &self.input[self.pos .. self.pos + 2];
        self.pos += 2;

        /* Return a u8 without really checking if it worked */
        return u8::from_str_radix(s, 16).unwrap();
    }

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut select = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };

        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    select.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    select.class.push(self.parse_identifier());
                }
                '*' => {
                    // Universal select
                    self.consume_char();
                }

                c if valid_identifier_char(c) => {
                    select.tag_name = Some(self.parse_identifier());
                }

                // TODO: Some basic error checking?
                _ => break
            }
        }

        return select;
    }

    fn parse_identifier(&mut self) -> String {
        return self.consume_while(valid_identifier_char);
    }

    /******* Some generic parser functions below *******/

    fn eof(&mut self) -> bool {
        return self.pos >= self.input.len();
    }

    /// KILL ALL WHITES(paces)
    fn white_genocide(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    /// Eat characters from buffer until function tells us we're full
    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    /// Like consume_char but on a diet
    fn next_char(&self) -> char {
        return self.input[self.pos..].chars().next().unwrap();
    }

    /// Consume one character from the input
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }
}


/// Small utility function to determine what's a real char and what's not
fn valid_identifier_char(c: char) -> bool {
    match c {
        'a'...'z' | 'A'...'Z' | '0'...'9' | '-' | '_' => true,
        _ => false
    }
}
