#[allow(clippy::wildcard_imports)]
use super::*;

pub(super) fn parse_path(
    p: &mut Parser,
    context: ParseErrorContext,
    recovery_set: TokenSet,
    kind: SyntaxKind,
) -> CompletedMarker {
    let parent_m = p.start();
    let path_m = p.start();
    ident::parse_ident(p, context, recovery_set);

    loop {
        if should_stop(p) {
            break;
        }
        p.expect_with_recovery(TokenKind::DoubleColon, context, recovery_set);
        ident::parse_ident(p, context, recovery_set);
    }

    path_m.complete(p, SyntaxKind::Path);
    return parent_m.complete(p, kind);

    fn should_stop(p: &mut Parser) -> bool {
        !p.at_set(ts![TokenKind::DoubleColon, TokenKind::Colon]) || p.at_top_level_token()
    }
}
