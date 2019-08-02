use sqlparser::ast::{Select, SelectItem, TableWithJoins, TableFactor, Join};

use crate::options::{Delimiter, Options};
use crate::format::base::Format;
use crate::format::keywords::Keyword;

fn format_select_projection(s: &Select, opt: &Options, indent: usize) -> String {
    // SELECT or SELECT DISTINCT
    let select = if s.distinct {
        format!("{} {}", Keyword::Select.format(opt, indent), Keyword::Distinct.format(opt, 0))
    } else {
        Keyword::Select.format(opt, indent)
    };

    let items = s.projection.iter()
        .map(|item| match item {
            SelectItem::ExprWithAlias { expr, alias } => {
                let cleaned_alias = match opt.alias_quotes {
                    Some(alias_quotes) => {
                        let a = alias
                            .to_string()
                            .trim_matches('"')
                            .to_lowercase();
                        
                        if alias_quotes {
                            format!("\"{}\"", a)
                        } else {
                            a
                        }
                    },
                    None => alias.to_string(),
                };
                (expr.to_string(), Some(cleaned_alias))
            },
            _ => (item.to_string(), None)
        })
        .collect::<Vec<(String, Option<String>)>>();

    let short_items = items.iter()
        .map(|(expr, alias)| match alias {
            None => expr.to_string(),
            Some(alias) => format!("{} {} {}", expr, Keyword::As.format(opt, 0), alias),
        })
        .collect::<Vec<String>>();

    let short_projection = opt.short_comma_delimiter.join(&short_items);
    let short_select = format!("{} {}", select, short_projection);

    if opt.adheres_to_line_limit(&short_select) {
        // SELECT column_1, column_2, ...
        return short_select
    }

    let long_items = items.iter()
        .map(|(expr, alias)| match alias {
            None => expr.to_string(),
            Some(alias) => {
                let spaces = if expr.len() >= opt.alias_position {
                    format!("\n{}", (0..opt.alias_position).map(|_| " ").collect::<String>())
                } else {
                    (0..(opt.alias_position - expr.len())).map(|_| " ").collect::<String>()
                };
                format!("{}{}{} {}", expr, spaces, Keyword::As.format(opt, 0), alias)
            }
        })
        .collect::<Vec<String>>();

    let long_projection = opt.long_comma_delimiter.join(&long_items);

    let after_select_whitespace = opt.long_comma_delimiter.as_whitespace().to_string();

    let long_select = format!(
        "{}{}",
        select,
        opt.indent.apply(&(after_select_whitespace + &long_projection), indent + 1)
    );
    long_select
}

impl Format for Select {
    fn format(&self, opt: &Options, indent: usize) -> String {
        let mut components = Vec::new();

        let select_projection = format_select_projection(self, opt, indent);
        components.push(select_projection);

        if !self.from.is_empty() {
            let from = self.from.format(opt, indent);
            components.push(from);
        }
        opt.body_delimiter.join(&components)
    }
}

impl Format for Vec<TableWithJoins> {
    fn format(&self, opt: &Options, indent: usize) -> String {
        opt.join_with_commas(
            &format!("{} ", Keyword::From.format(opt, indent)),
            &self.iter()
                .enumerate()
                .map(|(i, t)| {
                    if i == 0 {
                        t.format(opt, 0)
                    } else {
                        t.format(opt, 1)
                    }
                })
                .collect::<Vec<String>>(),
            "",
            indent + 1
        )
    }
}

impl Format for TableWithJoins {
    fn format(&self, opt: &Options, indent: usize) -> String {
        format!("{}", self)
    }
}
