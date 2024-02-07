use itertools::Itertools;
use psh_lexer::TokenKind;
use std::fmt;
use text_size::{TextRange, TextSize};

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub(super) expected: Vec<TokenKind>,
    pub(super) kind: ParseErrorKind,
    pub(super) context: ParseErrorContext,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ParseErrorKind {
    Missing { offset: TextSize },
    Unexpected { found: TokenKind, range: TextRange },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ParseErrorContext {
    FunctionCallArgExpr,
    PrefixExprExpr,
    ParenExprExpr,
    ParenExprRightParen,
    IfThenElseIfExpr,
    IfThenElseThenKw,
    IfThenElseThenExpr,
    IfThenElseElseKw,
    IfThenElseElseExpr,
    VariableDefIdent,
    VariableDefEquals,
    VariableDefExpr,
    TopLevelExpr,
}

impl ParseErrorContext {
    #[must_use]
    fn context_name<'a>(self) -> &'a str {
        match self {
            ParseErrorContext::FunctionCallArgExpr => "a function call argument",
            ParseErrorContext::PrefixExprExpr => "an expression after a prefix operator",
            ParseErrorContext::ParenExprExpr => "an expression inside parentheses",
            ParseErrorContext::ParenExprRightParen => "a close parenthesis after an expression",
            ParseErrorContext::IfThenElseIfExpr => {
                "the conditional expression in an if-then-else expression"
            }
            ParseErrorContext::IfThenElseThenKw => {
                "the `then` keyword in an if-then-else expression"
            }
            ParseErrorContext::IfThenElseThenExpr => {
                "the `then` expression in an if-then-else expression"
            }
            ParseErrorContext::IfThenElseElseKw => {
                "the `else` keyword in an if-then-else expression"
            }
            ParseErrorContext::IfThenElseElseExpr => {
                "the `else` expression in an if-then-else expression"
            }
            ParseErrorContext::VariableDefIdent => "the name in a variable definition",
            ParseErrorContext::VariableDefEquals => "the ‘=’ in a variable definition",
            ParseErrorContext::VariableDefExpr => "the expression in a variable definition",
            ParseErrorContext::TopLevelExpr => "a top level expression",
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let context_name = self.context.context_name();

        match self.kind {
            ParseErrorKind::Missing { offset } => {
                write!(f, "error at position {offset:?}")?;
                write!(f, " while parsing {context_name}. ")?;
                f.write_str("Missing expected ")?;
            }
            ParseErrorKind::Unexpected { found, range } => {
                write!(
                    f,
                    "error in range {}..{}",
                    u32::from(range.start()),
                    u32::from(range.end()),
                )?;
                write!(f, " while parsing {context_name}. ")?;
                write!(f, "Found {found}, but expected ")?;
            }
        }

        //
        // Expected
        //

        let vec = self.expected.iter().unique().collect::<Vec<_>>();
        let num_expected = vec.len();
        let is_first = |idx| idx == 0;
        let is_last = |idx| idx == num_expected - 1;

        for (idx, expected_kind) in vec.iter().enumerate() {
            if is_first(idx) {
                write!(f, "{expected_kind}")?;
            } else if is_last(idx) {
                write!(f, " or {expected_kind}")?;
            } else {
                write!(f, ", {expected_kind}")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[track_caller]
    fn check(expected: Vec<TokenKind>, kind: ParseErrorKind, output: &str) {
        let error = ParseError {
            expected,
            kind,
            context: ParseErrorContext::ParenExprExpr,
        };

        assert_eq!(format!("{error}"), output);
    }

    #[test]
    fn one_expected_did_find() {
        check(
            vec![TokenKind::Equals],
            ParseErrorKind::Missing {
                offset: TextSize::from(20),
            },
            "error at position 20 while parsing an expression inside parentheses. Missing expected ‘=’",
        );
    }

    #[test]
    fn duplicate_expected_did_find() {
        check(
            vec![TokenKind::Equals, TokenKind::Equals],
            ParseErrorKind::Missing {
                offset: TextSize::from(20),
            },
            "error at position 20 while parsing an expression inside parentheses. Missing expected ‘=’",
        );
    }

    #[test]
    fn one_expected_did_not_find() {
        check(
            vec![TokenKind::RParen],
            ParseErrorKind::Missing {
                offset: TextSize::from(6),
            },
            "error at position 6 while parsing an expression inside parentheses. Missing expected ‘)’",
        );
    }

    #[test]
    fn multiple_expected_did_find() {
        check(
            vec![
                TokenKind::Integer,
                TokenKind::Ident,
                TokenKind::Minus,
                TokenKind::LParen,
            ],
            ParseErrorKind::Unexpected {
                found: TokenKind::LetKw,
                range: TextRange::new(100.into(), 105.into()),
            },
            "error in range 100..105 while parsing an expression inside parentheses. Found ‘let’, but expected integer, identifier, ‘-’ or ‘(’",
        );
    }

    #[test]
    fn multiple_expected_did_not_find() {
        check(
            vec![TokenKind::Plus, TokenKind::Minus],
            ParseErrorKind::Missing {
                offset: TextSize::from(1),
            },
            "error at position 1 while parsing an expression inside parentheses. Missing expected ‘+’ or ‘-’",
        );
    }
}
