use psh_lexer::TokenKind;
use psh_syntax::SyntaxKind;

use crate::parser::marker::CompletedMarker;
use crate::parser::ParseErrorContext;
use crate::parser::Parser;
use crate::token_set::TokenSet;
use crate::ts;

mod expr;
mod ident;
mod path;
mod stmt;
mod variable_def;

pub(crate) fn repl_line(p: &mut Parser) -> CompletedMarker {
    let m = p.start();

    while !p.at_eof() {
        stmt::stmt(p);
    }

    m.complete(p, SyntaxKind::SourceFile)
}
