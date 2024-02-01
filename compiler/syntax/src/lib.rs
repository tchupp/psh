use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use psh_lexer::TokenKind;

pub type SyntaxNode = rowan::SyntaxNode<PshLanguage>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<PshLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<PshLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<PshLanguage>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, FromPrimitive, ToPrimitive)]
pub enum SyntaxKind {
    Whitespace,
    LetKw,
    IfKw,
    ThenKw,
    ElseKw,
    Ident,
    Integer,
    Fraction,
    String,
    Colon,
    DoubleColon,
    Comma,
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LAngle,
    RAngle,
    Pipe,
    Comment,
    Error,

    SourceFile,
    Path,
    VariableRef,
    ValueDef,

    StringLiteral,
    IntLiteral,
    FractionLiteral,
    UnaryExpr,
    InfixExpr,

    IfExpr,
    ThenExpr,
    ElseExpr,
    IfThenElseExpr,

    Unit,
    ParenExpr,
    ParenPattern,
    TupleExpr,
    TuplePattern,
    TuplePatternArg,
}

impl From<TokenKind> for SyntaxKind {
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::Whitespace => Self::Whitespace,
            TokenKind::LetKw => Self::LetKw,
            TokenKind::IfKw => Self::IfKw,
            TokenKind::ThenKw => Self::ThenKw,
            TokenKind::ElseKw => Self::ElseKw,
            TokenKind::Ident => Self::Ident,
            TokenKind::Integer => Self::Integer,
            TokenKind::Fraction => Self::Fraction,
            TokenKind::String => Self::String,
            TokenKind::Colon => Self::Colon,
            TokenKind::DoubleColon => Self::DoubleColon,
            TokenKind::Comma => Self::Comma,
            TokenKind::Plus => Self::Plus,
            TokenKind::Minus => Self::Minus,
            TokenKind::Star => Self::Star,
            TokenKind::Slash => Self::Slash,
            TokenKind::Equals => Self::Equals,
            TokenKind::LParen => Self::LParen,
            TokenKind::RParen => Self::RParen,
            TokenKind::LBrace => Self::LBrace,
            TokenKind::RBrace => Self::RBrace,
            TokenKind::LBracket => Self::LBracket,
            TokenKind::RBracket => Self::RBracket,
            TokenKind::LAngle => Self::LAngle,
            TokenKind::RAngle => Self::RAngle,
            TokenKind::Pipe => Self::Pipe,
            TokenKind::Comment => Self::Comment,
            TokenKind::Error => Self::Error,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PshLanguage {}

impl rowan::Language for PshLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}
