use std::fmt;

use logos::Logos;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Logos)]
#[logos(
    subpattern alpha_num_id = r"[A-Za-z][A-Za-z0-9]*",
)]
pub enum TokenKind {
    #[regex(r"[ \t\f\r\n]+")]
    Whitespace,

    // Keywords
    #[token("let")]
    LetKw,
    #[token("if")]
    IfKw,
    #[token("then")]
    ThenKw,
    #[token("else")]
    ElseKw,

    #[regex("_?(?&alpha_num_id)(_(?&alpha_num_id))+")]
    #[regex("_?(?&alpha_num_id)")]
    Ident,

    #[regex("[0-9]+")]
    Integer,

    #[regex(r"[0-9]+\.[0-9]+")]
    Fraction,

    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#)]
    #[regex(r#"'([^'\\]|\\t|\\u|\\n|\\")*'"#)]
    String,

    #[token(":")]
    Colon,
    #[token("::")]
    DoubleColon,

    #[token(",")]
    Comma,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("=")]
    Equals,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("<")]
    LAngle,

    #[token(">")]
    RAngle,

    #[token("|")]
    Pipe,

    #[regex("--[^\n]*")]
    Comment,

    Error,
}

impl TokenKind {
    #[must_use]
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Whitespace => "whitespace",
            Self::LetKw => "‘let’",
            Self::IfKw => "‘if‘",
            Self::ThenKw => "‘then‘",
            Self::ElseKw => "‘else‘",
            Self::Ident => "identifier",
            Self::Integer => "integer",
            Self::Fraction => "fraction",
            Self::String => "string",
            Self::Colon => "‘:’",
            Self::DoubleColon => "‘::’",
            Self::Comma => "‘,’",
            Self::Plus => "‘+’",
            Self::Minus => "‘-’",
            Self::Star => "‘*’",
            Self::Slash => "‘/’",
            Self::Equals => "‘=’",
            Self::LParen => "‘(’",
            Self::RParen => "‘)’",
            Self::LBrace => "‘{’",
            Self::RBrace => "‘}’",
            Self::LBracket => "‘[’",
            Self::RBracket => "‘]’",
            Self::LAngle => "‘<’",
            Self::RAngle => "‘>’",
            Self::Pipe => "‘|’",
            Self::Comment => "comment",
            Self::Error => "an unrecognized token",
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Lexer;
    use maplit::btreemap;
    use std::collections::BTreeMap;

    use super::*;

    #[track_caller]
    fn check(input: &str, kind: TokenKind) {
        let mut lexer = Lexer::new(input);

        let token = lexer.next().unwrap();
        assert_eq!((token.text, token.kind), (input, kind));
    }

    #[track_caller]
    fn check_multiple(input: &str, kinds: &[TokenKind]) {
        let (actual_text, actual_kinds) = Lexer::new(input)
            .map(|token| (token.text.to_owned(), vec![token.kind]))
            .reduce(|(mut acc_text, mut acc_kinds), (text, kind)| {
                acc_text.push_str(&text);
                acc_kinds.push(kind[0]);

                (acc_text, acc_kinds)
            })
            .expect("lexer should have completed non-empty");

        assert_eq!(
            (input.to_owned(), kinds),
            (actual_text, actual_kinds.as_slice())
        );
    }

    #[test]
    fn lex_spaces_and_tabs_and_newlines() {
        check("  \t  \r \n ", TokenKind::Whitespace);
    }

    #[test]
    fn lex_spaces() {
        check("   ", TokenKind::Whitespace);
    }

    #[test]
    fn test_keywords() {
        let source = btreemap! {
            "if" => TokenKind::IfKw,
            "then" => TokenKind::ThenKw,
            "else" => TokenKind::ElseKw,
        };

        for (source, expected) in source {
            check(source, expected);
        }
    }

    #[test]
    fn test_symbols() {
        let source = btreemap! {
            ":" => TokenKind::Colon,
            "::" => TokenKind::DoubleColon,
            "," => TokenKind::Comma,
            "+" => TokenKind::Plus,
            "-" => TokenKind::Minus,
            "*" => TokenKind::Star,
            "/" => TokenKind::Slash,
            "=" => TokenKind::Equals,
            "(" => TokenKind::LParen,
            ")" => TokenKind::RParen,
            "{" => TokenKind::LBrace,
            "}" => TokenKind::RBrace,
            "|" => TokenKind::Pipe,
        };

        for (source, expected) in source {
            check(source, expected);
        }
    }

    #[test]
    fn test_non_ident_symbols() {
        let source: BTreeMap<&str, Vec<TokenKind>> = btreemap! {
            "||" => vec![TokenKind::Pipe, TokenKind::Pipe],
            "<::>" => vec![TokenKind::LAngle, TokenKind::DoubleColon, TokenKind::RAngle],
        };

        for (source, expected) in source {
            check_multiple(source, &expected);
        }
    }

    #[test]
    fn character_identifiers() {
        let source = btreemap! {
            "asdf" => TokenKind::Ident,
            "_asdf" => TokenKind::Ident,
            "Asdf" => TokenKind::Ident,
            "_Asdf" => TokenKind::Ident,
            "abcd" => TokenKind::Ident,
            "ab123cde456" => TokenKind::Ident,
            "x" => TokenKind::Ident,
            "ABCdef" => TokenKind::Ident,
            "thingThing" => TokenKind::Ident,
            "thingThing" => TokenKind::Ident,
            "thingThingThingThing" => TokenKind::Ident,
        };

        for (source, expected) in source {
            check(source, expected);
        }
    }

    #[test]
    fn lex_integral() {
        check("123456", TokenKind::Integer);
    }

    #[test]
    fn lex_fraction() {
        check("123456.123456", TokenKind::Fraction);
    }

    #[test]
    fn lex_string() {
        check(r#""hello""#, TokenKind::String);
        check("'char'", TokenKind::String);
    }

    #[test]
    fn lex_comment() {
        check("-- foo", TokenKind::Comment);
    }
}
