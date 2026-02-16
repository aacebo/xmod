use logos::Logos;

fn unescape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('r') => out.push('\r'),
                Some('\\') => out.push('\\'),
                Some('\'') => out.push('\''),
                Some('"') => out.push('"'),
                Some('0') => out.push('\0'),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// Tokens produced inside expression contexts (within `{{ }}`,
/// `@keyword(...)` conditions, etc.).
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n]+")]
pub enum Token {
    // --- Literals ---
    #[regex(r"[0-9]+\.[0-9]+([eE][+-]?[0-9]+)?", |lex| lex.slice().parse::<f64>().ok())]
    Float(f64),

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().ok(), priority = 2)]
    Int(i64),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        Some(unescape(&s[1..s.len() - 1]))
    })]
    #[regex(r#"'([^'\\]|\\.)*'"#, |lex| {
        let s = lex.slice();
        Some(unescape(&s[1..s.len() - 1]))
    })]
    String(String),

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("null")]
    Null,

    // --- Keywords ---
    #[token("of")]
    Of,

    #[token("track")]
    Track,

    // --- Identifiers ---
    #[regex(r"[a-zA-Z_$][a-zA-Z0-9_$]*", |lex| lex.slice().to_string(), priority = 1)]
    Ident(String),

    // --- Operators ---
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("%")]
    Percent,

    #[token("==")]
    EqEq,

    #[token("!=")]
    BangEq,

    #[token("<")]
    Lt,

    #[token("<=")]
    Le,

    #[token(">")]
    Gt,

    #[token(">=")]
    Ge,

    #[token("&&")]
    AmpAmp,

    #[token("||")]
    PipePipe,

    #[token("!")]
    Bang,

    // --- Punctuation ---
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token(".")]
    Dot,

    #[token(",")]
    Comma,

    #[token("|")]
    Pipe,

    #[token(":")]
    Colon,

    #[token(";")]
    Semi,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_int() {
        let mut lex = Token::lexer("42");
        assert_eq!(lex.next(), Some(Ok(Token::Int(42))));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn lex_float() {
        let mut lex = Token::lexer("3.14");
        assert_eq!(lex.next(), Some(Ok(Token::Float(3.14))));
    }

    #[test]
    fn lex_string_double() {
        let mut lex = Token::lexer(r#""hello""#);
        assert_eq!(lex.next(), Some(Ok(Token::String("hello".to_string()))));
    }

    #[test]
    fn lex_string_single() {
        let mut lex = Token::lexer("'world'");
        assert_eq!(lex.next(), Some(Ok(Token::String("world".to_string()))));
    }

    #[test]
    fn lex_bool() {
        let mut lex = Token::lexer("true false");
        assert_eq!(lex.next(), Some(Ok(Token::True)));
        assert_eq!(lex.next(), Some(Ok(Token::False)));
    }

    #[test]
    fn lex_ident() {
        let mut lex = Token::lexer("foo _bar $baz");
        assert_eq!(lex.next(), Some(Ok(Token::Ident("foo".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("_bar".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::Ident("$baz".to_string()))));
    }

    #[test]
    fn lex_operators() {
        let mut lex = Token::lexer("+ - * / % == != < <= > >= && || !");
        assert_eq!(lex.next(), Some(Ok(Token::Plus)));
        assert_eq!(lex.next(), Some(Ok(Token::Minus)));
        assert_eq!(lex.next(), Some(Ok(Token::Star)));
        assert_eq!(lex.next(), Some(Ok(Token::Slash)));
        assert_eq!(lex.next(), Some(Ok(Token::Percent)));
        assert_eq!(lex.next(), Some(Ok(Token::EqEq)));
        assert_eq!(lex.next(), Some(Ok(Token::BangEq)));
        assert_eq!(lex.next(), Some(Ok(Token::Lt)));
        assert_eq!(lex.next(), Some(Ok(Token::Le)));
        assert_eq!(lex.next(), Some(Ok(Token::Gt)));
        assert_eq!(lex.next(), Some(Ok(Token::Ge)));
        assert_eq!(lex.next(), Some(Ok(Token::AmpAmp)));
        assert_eq!(lex.next(), Some(Ok(Token::PipePipe)));
        assert_eq!(lex.next(), Some(Ok(Token::Bang)));
    }

    #[test]
    fn lex_punctuation() {
        let mut lex = Token::lexer("( ) [ ] { } . , | : ;");
        assert_eq!(lex.next(), Some(Ok(Token::LParen)));
        assert_eq!(lex.next(), Some(Ok(Token::RParen)));
        assert_eq!(lex.next(), Some(Ok(Token::LBracket)));
        assert_eq!(lex.next(), Some(Ok(Token::RBracket)));
        assert_eq!(lex.next(), Some(Ok(Token::LBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::RBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::Comma)));
        assert_eq!(lex.next(), Some(Ok(Token::Pipe)));
        assert_eq!(lex.next(), Some(Ok(Token::Colon)));
        assert_eq!(lex.next(), Some(Ok(Token::Semi)));
    }

    #[test]
    fn lex_keywords() {
        let mut lex = Token::lexer("of track");
        assert_eq!(lex.next(), Some(Ok(Token::Of)));
        assert_eq!(lex.next(), Some(Ok(Token::Track)));
    }

    #[test]
    fn skips_whitespace() {
        let mut lex = Token::lexer("  42  ");
        assert_eq!(lex.next(), Some(Ok(Token::Int(42))));
        assert_eq!(lex.next(), None);
    }
}
