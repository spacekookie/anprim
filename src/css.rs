
pub struct Stylesheet {
    rules: Vec<Rule>,
}

pub struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}

pub struct SimpleSelector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Vec<String>,
}

pub enum Selector {
    Simple(SimpleSelector),
}

pub struct Declaration {
    name: String,
    value: Value,
}

pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColourValue(Colour),
}

pub enum Unit {
    PX,
}

/* Colour spelt correctly */
pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub fn parse(source: String) -> Stylesheet {
    let mut parser = Parser {
        pos: 0,
        input: source,
    };
    Stylesheet { rules: parser.parse_rules() }
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
        }

        return rules;
    }

    fn white_genocide(&mut self) {}

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };

        while !self.eof() {}

        return selector;
    }

    fn eof(&mut self) -> bool {
        return false;
    }

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

    /* Like consume_char but with a diet */
    fn next_char(&self) -> char {
        return self.input[self.pos..].chars().next().unwrap();
    }

    /* Consume one character from the input */
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }
}
