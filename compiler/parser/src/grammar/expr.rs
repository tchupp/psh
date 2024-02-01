#[allow(clippy::wildcard_imports)]
use super::*;

pub(crate) const EXPR_FIRSTS: TokenSet = ts![
    TokenKind::Integer,
    TokenKind::Fraction,
    TokenKind::String,
    TokenKind::Ident,
    TokenKind::Minus,
    TokenKind::LParen,
    TokenKind::IfKw,
];

enum BinaryOp {
    Pipe,
    Add,
    Sub,
    Mul,
    Div,
}

impl BinaryOp {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Pipe => (1, 2),
            Self::Add | Self::Sub => (3, 4),
            Self::Mul | Self::Div => (5, 6),
        }
    }
}

enum UnaryOp {
    Neg,
}

impl UnaryOp {
    fn binding_power(&self) -> ((), u8) {
        match self {
            Self::Neg => ((), 5),
        }
    }
}

pub(super) fn parse_expr(p: &mut Parser, context: ParseErrorContext) -> Option<CompletedMarker> {
    parse_expr_with_recovery(p, ts![], context)
}

fn parse_expr_with_recovery(
    p: &mut Parser,
    recovery_set: TokenSet,
    context: ParseErrorContext,
) -> Option<CompletedMarker> {
    parse_expr_with_binding_power(p, 0, recovery_set, context)
}

const SUPPORTED_OPERATORS: TokenSet = ts![
    TokenKind::Plus,
    TokenKind::Minus,
    TokenKind::Star,
    TokenKind::Slash,
];

fn parse_expr_with_binding_power(
    p: &mut Parser,
    minimum_binding_power: u8,
    recovery_set: TokenSet,
    context: ParseErrorContext,
) -> Option<CompletedMarker> {
    let mut lhs = parse_lhs(p, recovery_set, context)?;

    loop {
        if !p.at_set(SUPPORTED_OPERATORS) {
            // We’re not at an operator we recognize;
            // we don’t know what to do next, so we return and let caller decide.
            break;
        }

        let op = if p.at(TokenKind::Plus) {
            BinaryOp::Add
        } else if p.at(TokenKind::Minus) {
            BinaryOp::Sub
        } else if p.at(TokenKind::Star) {
            BinaryOp::Mul
        } else if p.at(TokenKind::Slash) {
            BinaryOp::Div
        } else if p.at(TokenKind::Pipe) {
            BinaryOp::Pipe
        } else {
            unreachable!("we should never end up here, since the list of known operators is checked beforehand")
        };

        let (left_binding_power, right_binding_power) = op.binding_power();

        if left_binding_power < minimum_binding_power {
            break;
        }

        // Eat the operator’s token.
        p.bump_any();

        let m = lhs.precede(p);
        let parsed_rhs =
            parse_expr_with_binding_power(p, right_binding_power, recovery_set, context).is_some();
        lhs = m.complete(p, SyntaxKind::InfixExpr);

        if !parsed_rhs {
            break;
        }
    }

    Some(lhs)
}

fn parse_lhs(
    p: &mut Parser,
    recovery_set: TokenSet,
    context: ParseErrorContext,
) -> Option<CompletedMarker> {
    let cm = if p.at(TokenKind::Integer) {
        parse_int_literal(p)
    } else if p.at(TokenKind::Fraction) {
        parse_fraction_literal(p)
    } else if p.at(TokenKind::String) {
        parse_string_literal(p)
    } else if p.at(TokenKind::Ident) {
        parse_variable_ref(p)
    } else if p.at(TokenKind::Minus) {
        parse_prefix_expr(p)
    } else if p.at(TokenKind::LParen) {
        parse_paren_expr(p)
    } else if p.at(TokenKind::IfKw) {
        parse_if_then_else_expr(p)
    } else {
        p.error_with_recovery(context, recovery_set);
        return None;
    };

    Some(cm)
}

pub(crate) fn parse_int_literal(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.bump(TokenKind::Integer);
    m.complete(p, SyntaxKind::IntLiteral)
}

pub(crate) fn parse_fraction_literal(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.bump(TokenKind::Fraction);
    m.complete(p, SyntaxKind::FractionLiteral)
}

pub(crate) fn parse_string_literal(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.bump(TokenKind::String);
    m.complete(p, SyntaxKind::StringLiteral)
}

pub(crate) fn parse_variable_ref(p: &mut Parser) -> CompletedMarker {
    path::parse_path(
        p,
        ParseErrorContext::VariableRef,
        ts![],
        SyntaxKind::VariableRef,
    )
}

fn parse_if_then_else_expr(p: &mut Parser) -> CompletedMarker {
    let if_then_else_m = p.start();
    p.bump(TokenKind::IfKw);

    let if_m = p.start();
    parse_expr_with_recovery(
        p,
        ts![TokenKind::ThenKw],
        ParseErrorContext::IfThenElseIfExpr,
    );
    if_m.complete(p, SyntaxKind::IfExpr);

    p.expect_with_recovery(
        TokenKind::ThenKw,
        ParseErrorContext::IfThenElseThenKw,
        EXPR_FIRSTS,
    );
    let then_m = p.start();
    parse_expr_with_recovery(
        p,
        ts![TokenKind::ElseKw],
        ParseErrorContext::IfThenElseThenExpr,
    );
    then_m.complete(p, SyntaxKind::ThenExpr);

    p.expect_with_recovery(
        TokenKind::ElseKw,
        ParseErrorContext::IfThenElseElseKw,
        EXPR_FIRSTS,
    );
    let else_m = p.start();
    parse_expr(p, ParseErrorContext::IfThenElseElseExpr);
    else_m.complete(p, SyntaxKind::ElseExpr);

    if_then_else_m.complete(p, SyntaxKind::IfThenElseExpr)
}

fn parse_prefix_expr(p: &mut Parser) -> CompletedMarker {
    let m = p.start();

    let op = UnaryOp::Neg;
    let ((), right_binding_power) = op.binding_power();

    // Eat the operator’s token.
    p.bump(TokenKind::Minus);

    parse_expr_with_binding_power(
        p,
        right_binding_power,
        TokenSet::default(),
        ParseErrorContext::PrefixExprExpr,
    );

    m.complete(p, SyntaxKind::UnaryExpr)
}

fn parse_paren_expr(p: &mut Parser) -> CompletedMarker {
    let paren_m = p.start();
    p.bump(TokenKind::LParen);

    let mut arg_len = 0;
    loop {
        if should_stop(p) {
            break;
        }

        parse_expr(p, ParseErrorContext::ParenExprExpr);
        arg_len += 1;

        if should_stop(p) {
            break;
        }

        p.expect_with_recovery(
            TokenKind::Comma,
            ParseErrorContext::ParenExprComma,
            EXPR_FIRSTS,
        );
    }

    p.expect(TokenKind::RParen, ParseErrorContext::ParenExprRightParen);

    let kind = match arg_len {
        0 => SyntaxKind::Unit,
        1 => SyntaxKind::ParenExpr,
        _ => SyntaxKind::TupleExpr,
    };
    return paren_m.complete(p, kind);

    fn should_stop(p: &mut Parser) -> bool {
        p.maybe_at(TokenKind::RParen) || p.at_eof()
    }
}
