use std::iter::from_fn;

use logos::{Lexer, Logos};

/// Represents a token parsed by the lexer.
#[derive(Clone)]
pub struct Token {
    pub length: usize,
    pub token_type: TokenType,
}

fn integer_kilo(lex: &mut Lexer<TokenType>) -> Option<i64> {
    let slice = lex.slice();
    let n: i64 = slice[..slice.len() - 1].parse().ok()?; // skip 'k'
    Some(n * 1_000)
}

fn integer_mega(lex: &mut Lexer<TokenType>) -> Option<i64> {
    let slice = lex.slice();
    let n: i64 = slice[..slice.len() - 1].parse().ok()?; // skip 'm'
    Some(n * 1_000_000)
}

/// An enum of possible token types.
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum TokenType {
    #[regex(r"//.+\n")]
    LineComment,

    #[regex(r"/*.+*/")]
    BlockComment,

    /// An identifier token.
    #[regex(r"[a-zA-Z_]+", |lex| lex.slice().parse())]
    Ident(String),

    /// Represents the 'let' token.
    #[token("let")]
    Declaration,

    /// Represents a constant initiator.
    #[token("const")]
    ConstantDeclaration,

    /// Represents a function initiator.
    #[token("fn")]
    FuncInitiator,

    /// Represents a function return identifier.
    #[token("->")]
    FuncReturn,

    #[token("(")]
    ParenthesesOpen,

    #[token(")")]
    ParenthesesClose,

    #[token("{")]
    BracesOpen,

    #[token("}")]
    BracesClose,

    /// Represents an assignment operator.
    #[token("=")]
    Assign,

    /// Represents an addition operator.
    #[token("+")]
    Plus,   

    /// Represents an increment operator.
    #[token("+=")]
    PlusEq,

    /// Represents an equality operator.
    #[token("==")]
    Eq,

    /// Represents a less than operator.
    #[token("<")]
    Lt,

    /// Represents a greater than operator.
    #[token(">")]
    Gt,

    /// Represents a less than or equal operator.
    #[token("<=")]
    Le,

    /// Represents a greater than or equal to operator.
    #[token(">=")]  
    Ge,

    /// Represents a logical or operator.
    #[token("||")]
    Or,

    /// Represents a logical and operator.
    #[token("&&")]
    And,

    /// Represents an integer literal.
    #[regex("-?[0-9]+", |lex| lex.slice().parse())]
    #[regex("-?[0-9]+k", integer_kilo)]
    #[regex("-?[0-9]+m", integer_mega)]
    Integer(i64),

    /// Represents a float literal.
    #[regex(r"-?[0-9]+\.[0-9]+", |lex| lex.slice().parse())]
    #[regex(r"-?[0-9]+\.[0-9]+e-?[0-9]+", |lex| lex.slice().parse())]
    Float(f64),

    /// Represents a character literal.
    #[regex(r#"'[a-zA-Z]'"#, |lex| lex.slice().parse())]
    Char(char),

    /// Represents a string literal.
    #[regex(r#"".+""#, |lex| lex.slice().parse())]
    String(String),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut lex = TokenType::lexer(input);
    from_fn(move || {
        let token = Token {
            token_type: lex.next().unwrap(),
            length: lex.slice().len(),
        };
        Some(token)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ident() {
        let mut lex = TokenType::lexer("hello_world\ni_love_foxes");
        assert_eq!(lex.next(), Some(TokenType::Ident("hello_world".into())));
        assert_eq!(lex.next(), Some(TokenType::Ident("i_love_foxes".into())));
    }

    #[test]
    fn test_integer() {
        let mut lex = TokenType::lexer("1\n-30\n2k\n3m");
        assert_eq!(lex.next(), Some(TokenType::Integer(1)));
        assert_eq!(lex.next(), Some(TokenType::Integer(-30)));
        assert_eq!(lex.next(), Some(TokenType::Integer(2_000)));
        assert_eq!(lex.next(), Some(TokenType::Integer(3_000_000)));
    }

    #[test]
    fn test_float() {
        let mut lex = TokenType::lexer("1.0\n2.0e2\n3.1e3\n-46.2e-3");
        assert_eq!(lex.next(), Some(TokenType::Float(1.0)));
        assert_eq!(lex.next(), Some(TokenType::Float(2e2)));
        assert_eq!(lex.next(), Some(TokenType::Float(3.1e3)));
        assert_eq!(lex.next(), Some(TokenType::Float(-46.2e-3)));
    }

    #[test]
    fn test_tokenize() {
        let mut iter = tokenize("input\n123");
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        assert_eq!(first.length, "input".len());
        assert_eq!(first.token_type, TokenType::Ident("input".into()));
        assert_eq!(second.length, "123".len());
        assert_eq!(second.token_type, TokenType::Integer(123));
    }

    #[test]
    fn test_binary_expr() {
        let mut lex = TokenType::lexer("x <= 3");
        assert_eq!(lex.next(), Some(TokenType::Ident("x".into())));
        assert_eq!(lex.next(), Some(TokenType::Le));
        assert_eq!(lex.next(), Some(TokenType::Integer(3)));

        let mut lex = TokenType::lexer("x += 2");
        assert_eq!(lex.next(), Some(TokenType::Ident("x".into())));
        assert_eq!(lex.next(), Some(TokenType::PlusEq));
        assert_eq!(lex.next(), Some(TokenType::Integer(2)));
    }
    #[test]
    fn test_char() {
        let mut lex = TokenType::lexer("'u'");
        assert_eq!(lex.next(), Some(TokenType::Char('u')));
    }
}
