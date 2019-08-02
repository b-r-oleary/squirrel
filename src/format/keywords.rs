use inflector::Inflector;

use crate::options::Options;
use crate::format::base::Format;

#[derive(Debug)]
pub enum Keyword {
    As,
    Asc,
    Desc,
    Limit,
    OrderBy,
    Select,
    With,
}

impl Format for Keyword {
    fn format(&self, opt: &Options, _indent: usize) -> String {
        let keyword_string = format!("{:?}", self);
        let normalized = keyword_string.to_snake_case().replace("_", " ");
        opt.keyword_case.apply(&normalized)
    }
}
