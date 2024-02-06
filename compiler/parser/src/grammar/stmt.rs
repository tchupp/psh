#[allow(clippy::wildcard_imports)]
use super::*;

pub(super) fn stmt(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(TokenKind::LetKw) {
        Some(variable_def::parse_variable_def(p))
    } else {
        expr::parse_expr(p, ParseErrorContext::TopLevelExpr)
    }
}
