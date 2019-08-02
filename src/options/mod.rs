use std::string::ToString;

pub struct Options {
    pub line_limit: usize,
    pub indent: Indent,
    pub keyword_case: Case,
    pub body_delimiter: Whitespace,
    pub subquery_delimiter: Whitespace,
    pub query_delimiter: Whitespace,
    pub long_comma_delimiter: Comma,
    pub short_comma_delimiter: Comma,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            line_limit: 95,
            indent: Indent(Whitespace::Space(4)),
            keyword_case: Case::Upper,
            body_delimiter: Whitespace::NewLine(1),
            subquery_delimiter: Whitespace::NewLine(2),
            query_delimiter: Whitespace::NewLine(3),
            long_comma_delimiter: Comma {
                before: Whitespace::NewLine(1),
                after: Whitespace::Space(1),
            },
            short_comma_delimiter: Comma {
                before: Whitespace::None,
                after: Whitespace::Space(1),
            }
        }
    }
}

impl Options {
    fn adheres_to_line_limit(&self, s: &str) -> bool {
        s.lines().into_iter().all(|line| line.len() <= self.line_limit)
    }

    fn join_alternatives(
        &self,
        alternatives: &[&impl Delimiter],
        prefix: &str,
        items: &[String],
        suffix: &str,
        indent: usize,
    ) -> Option<String> {
        let n = alternatives.len();

        alternatives.into_iter()
            .enumerate()
            .map(|(i, alt)| {
                (
                    (n - i),
                    format!(
                        "{}{}{}",
                        prefix,
                        alt.join_with_indent(items, &self.indent, indent),
                        suffix
                    )
                )
            })
            .filter(|(j, option)| self.adheres_to_line_limit(&option) || *j == 0)
            .map(|(_, option)| option)
            .next()
    }

    pub fn join_with_commas(
        &self,
        prefix: &str,
        items: &[String],
        suffix: &str,
        indent: usize,
    ) -> String {
        let alternatives = [&self.short_comma_delimiter, &self.long_comma_delimiter];

        self.join_alternatives(
            &alternatives,
            prefix,
            items,
            suffix,
            indent,
        ).expect("Expected a string to be returned since the number of alternatives is greater than zero")
    }
}

pub enum Case {
    Lower,
    Upper,
}

impl Case {
    pub fn apply(&self, s: &str) -> String {
        match self {
            Case::Upper => s.to_uppercase(),
            Case::Lower => s.to_lowercase(),
        }
    }
}

pub trait Delimiter: ToString + Sized {
    fn join(&self, items: &[String]) -> String {
        items.join(&self.to_string())
    }
    fn join_with_indent(&self, items: &[String], indent: &Indent, level: usize) -> String {
        let delimiter = indent.apply(&self.to_string(), level);
        items.join(&delimiter)
    }
}

pub enum Whitespace {
    None,
    Space(u32),
    Tab(u32),
    NewLine(u32),
    Combo(Vec<Whitespace>),
}

impl ToString for Whitespace {
    fn to_string(&self) -> String {
        match self {
            Whitespace::None => "".to_string(),
            Whitespace::Space(n) => {
                (0..*n).into_iter().map(|_| " ").collect::<String>()
            },
            Whitespace::Tab(n) => {
                (0..*n).into_iter().map(|_| "\t").collect::<String>()
            },
            Whitespace::NewLine(n) => {
                (0..*n).into_iter().map(|_| "\n").collect::<String>()
            },
            Whitespace::Combo(whitespace_sequence) => {
                whitespace_sequence.iter().map(|w| w.to_string()).collect::<String>()
            }
        }
    }
}

impl Delimiter for Whitespace {}

pub struct Comma {
    before: Whitespace,
    after: Whitespace,
}

impl ToString for Comma {
    fn to_string(&self) -> String {
        format!("{},{}", self.before.to_string(), self.after.to_string())
    }
}

impl Delimiter for Comma {}

pub struct Indent(Whitespace);

impl Indent {
    pub fn apply(&self, s: &str, i: usize) -> String {
        let space = self.0.to_string();
        let repeated_space = (0..i).into_iter().map(|_| space.clone()).collect::<String>();
        let newline = format!("\n{}", repeated_space);
        s.replace("\n", &newline)
    }
}
