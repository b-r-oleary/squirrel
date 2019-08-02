use sqlparser::ast::Statement;
use sqlparser::ast::{Query, Fetch, Expr, Cte, SetExpr, Select, OrderByExpr, TableAlias, Ident, SelectItem};

use crate::options::{Delimiter, Options};
use crate::format::base::Format;
use crate::format::keywords::Keyword;


impl Format for Vec<Statement> {
    fn format(&self, opt: &Options, indent: usize) -> String {
        let statements = self.iter()
            .map(|s| s.format(opt, indent))
            .collect::<Vec<String>>();

        opt.query_delimiter.join(&statements)
    }
}

impl Format for Statement {
    fn format(&self, opt: &Options, indent: usize) -> String {
        match self {
            Statement::Query(query) => query.format(opt, indent),
            _ => unimplemented!(),
        }
    }
}

impl Format for Query {
    fn format(&self, opt: &Options, indent: usize) -> String {
        let mut components: Vec<String> = Vec::new();
        if !self.ctes.is_empty() {
            components.push(self.ctes.format(opt, indent));
        }
        components.push(self.body.format(opt, indent));
        if !self.order_by.is_empty() {
            components.push(self.order_by.format(opt, indent));
        }
        if let Some(ref limit) = self.limit {
            components.push(limit.format(opt, indent));
        }
        if let Some(ref offset) = self.offset {
            components.push(offset.format(opt, indent));
        }
        if let Some(ref fetch) = self.fetch {
            components.push(fetch.format(opt, indent));
        }
        opt.body_delimiter.join(&components)
    }
}

impl Format for Vec<Cte> {
    fn format(&self, opt: &Options, indent: usize) -> String {
        let ctes = self.iter()
            .map(|cte| cte.format(opt, indent))
            .collect::<Vec<String>>()
            .join(", ");

        format!("{} {}", Keyword::With.format(opt, indent), ctes)
    }
}

impl Format for Cte {
    fn format(&self, opt: &Options, indent: usize) -> String {
        opt.subquery_delimiter.join(&[
            format!(
                "{} {} (",
                self.alias.format(opt, 0),
                Keyword::As.format(opt, 0),
            ),
            self.query.format(opt, indent + 1),
            ")".to_string()
        ])
    }
}

impl Format for TableAlias {
    fn format(&self, opt: &Options, indent: usize) -> String {
        let alias = self.name.format(opt, indent);
        if self.columns.is_empty() {
            alias
        } else {
            format!("{} ({})", alias, self.columns.format(opt, indent))
        }
    }
}

impl Format for SetExpr {
    fn format(&self, opt: &Options, indent: usize) -> String {
        match self {
            SetExpr::Select(select) => select.format(opt, indent),
            _ => unimplemented!()
        }
    }
}

impl Format for Vec<OrderByExpr> {
    fn format(&self, opt: &Options, indent: usize) -> String {
        let exprs = self.iter().map(|expr| expr.format(opt, 0)).collect::<Vec<String>>();
        opt.join_with_commas(
            &format!("{} ", Keyword::OrderBy.format(opt, 0)),
            &exprs,
            "",
            indent + 1
        )
    }
}

impl Format for OrderByExpr {
    fn format(&self, opt: &Options, indent: usize) -> String {
        let expr = self.expr.format(opt, indent);
        match self.asc {
            Some(true) => format!("{} {}", Keyword::Asc.format(opt, 0), expr),
            Some(false) => format!("{} {}", Keyword::Desc.format(opt, 0), expr),
            None => expr
        }
    }
}

impl Format for Expr {
    fn format(&self, opt: &Options, indent: usize) -> String {
        unimplemented!()
    }
}

impl Format for Fetch {
    fn format(&self, opt: &Options, indent: usize) -> String {
        unimplemented!()
    }
}

impl Format for Vec<Ident> {
    fn format(&self, opt: &Options, indent: usize) -> String {
        let idents = self.iter().map(|i| i.format(opt, 0)).collect::<Vec<String>>();
        opt.join_with_commas("", &idents, "", indent)
    }
}

impl Format for Ident {
    fn format(&self, opt: &Options, indent: usize) -> String {
        opt.indent.apply(&self.to_string(), indent)
    }
}
