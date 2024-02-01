#[allow(clippy::wildcard_imports)]
use super::*;

pub(super) fn parse_ident(p: &mut Parser, context: ParseErrorContext, recovery_set: TokenSet) {
    p.expect_with_recovery(TokenKind::Ident, context, recovery_set);
}
