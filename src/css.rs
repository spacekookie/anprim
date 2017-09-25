
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

    /** KILL ALL WHITES(paces) */
    fn white_genocide(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };

        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    // Universal selector
                    self.consume_char();
                }

                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }

                _ => break
            }
        }

        return selector;
    }

    fn parse_identifier(&mut self) -> String {
        return self.consume_while(valid_identifier_char);
    }

    fn eof(&mut self) -> bool {
        return self.pos >= self.input.len();
    }

    /** Eat characters from buffer until function tells us we're full */
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

    /* Like consume_char but on a diet */
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


/* Small utility function to determine what's a real char and what's not */
fn valid_identifier_char(c: char) -> bool {
    match c {
        'a'...'z' | 'A'...'Z' | '0'...'9' | '-' | '_' => true,
        _ => false
    }
}
