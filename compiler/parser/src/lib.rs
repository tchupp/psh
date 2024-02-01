use rowan::GreenNode;
use std::fmt::Write as _;

use psh_lexer::Lexer;
use psh_syntax::SyntaxNode;

pub use crate::parser::ParseError;
use crate::parser::Parser;
use crate::sink::Sink;
use crate::source::Source;

mod event;
mod grammar;
mod parser;
mod sink;
mod source;
mod token_set;

#[cfg(test)]
mod tests;

#[must_use]
pub fn parse_repl_line(input: &str) -> Parse {
    let tokens: Vec<_> = Lexer::new(input).collect();
    let source = Source::new(&tokens);
    let parser = Parser::new(source);
    let events = parser.parse_repl_line();
    let sink = Sink::new(&tokens, events);

    sink.finish()
}

pub struct Parse {
    green_node: GreenNode,
    errors: Vec<ParseError>,
}

impl Parse {
    #[must_use]
    pub fn debug_tree(&self) -> String {
        let mut s = String::new();

        let tree = format!("{:#?}", self.syntax());

        // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
        s.push_str(&tree[0..tree.len() - 1]);

        for error in &self.errors {
            let _ = write!(s, "\n{error}");
        }

        s
    }

    #[must_use]
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }

    #[must_use]
    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }
}
