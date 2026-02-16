use logos::Logos;

use crate::ast::Span;

use super::error::{ParseError, Result};
use super::token::Token;

/// A positioned token.
#[derive(Debug, Clone, PartialEq)]
pub struct Spanned {
    pub token: LexToken,
    pub span: Span,
}

/// Extended token type that includes template-level tokens
/// not handled by logos.
#[derive(Debug, Clone, PartialEq)]
pub enum LexToken {
    /// Raw text content.
    Text(String),
    /// Start of interp `{{`.
    InterpStart,
    /// End of interp `}}`.
    InterpEnd,
    /// `@if`
    AtIf,
    /// `@else`
    AtElse,
    /// `@for`
    AtFor,
    /// `@switch`
    AtSwitch,
    /// `@case`
    AtCase,
    /// `@default`
    AtDefault,
    /// Closing brace `}` that ends a block body.
    CloseBrace,
    /// An expression-mode token produced by logos.
    Expr(Token),
    /// End of input.
    Eof,
}

const AT_KEYWORDS: &[(&str, LexToken)] = &[
    ("if", LexToken::AtIf),
    ("else", LexToken::AtElse),
    ("for", LexToken::AtFor),
    ("switch", LexToken::AtSwitch),
    ("case", LexToken::AtCase),
    ("default", LexToken::AtDefault),
];

pub struct Lexer<'src> {
    source: &'src str,
    pos: usize,
    peeked_expr: Option<Spanned>,
    brace_depth: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            pos: 0,
            peeked_expr: None,
            brace_depth: 0,
        }
    }

    pub fn open_brace(&mut self) {
        self.brace_depth += 1;
    }

    pub fn close_brace(&mut self) {
        self.brace_depth = self.brace_depth.saturating_sub(1);
    }

    /// Check if remaining source (after whitespace) starts with the
    /// given `@keyword`. Does not consume anything.
    pub fn starts_with_at_keyword(&self, keyword: &str) -> bool {
        let remaining = self.remaining().trim_start();

        if let Some(after_at) = remaining.strip_prefix('@') {
            if let Some(rest) = after_at.strip_prefix(keyword) {
                rest.is_empty()
                    || (!rest.as_bytes()[0].is_ascii_alphanumeric() && rest.as_bytes()[0] != b'_')
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Skip ASCII whitespace characters and advance position.
    pub fn skip_whitespace(&mut self) {
        while self.pos < self.source.len() && self.source.as_bytes()[self.pos].is_ascii_whitespace()
        {
            self.pos += 1;
        }
    }

    fn remaining(&self) -> &'src str {
        &self.source[self.pos..]
    }

    fn here(&self) -> Span {
        Span::new(self.pos, self.pos)
    }

    /// Scan the next token in text mode. Stops at `{{`, `@keyword`,
    /// `}` (when brace_depth > 0), or EOF.
    pub fn next_text(&mut self) -> Result<Spanned> {
        let start = self.pos;
        let bytes = self.source.as_bytes();
        let len = bytes.len();

        if self.pos >= len {
            return Ok(Spanned {
                token: LexToken::Eof,
                span: self.here(),
            });
        }

        let mut text_end = self.pos;

        while text_end < len {
            // Check for `{{`
            if text_end + 1 < len && bytes[text_end] == b'{' && bytes[text_end + 1] == b'{' {
                if text_end > start {
                    self.pos = text_end;
                    return Ok(Spanned {
                        token: LexToken::Text(self.source[start..text_end].to_string()),
                        span: Span::new(start, text_end),
                    });
                }

                self.pos = text_end + 2;
                return Ok(Spanned {
                    token: LexToken::InterpStart,
                    span: Span::new(text_end, text_end + 2),
                });
            }

            // Check for `}` when inside a block body
            if self.brace_depth > 0 && bytes[text_end] == b'}' {
                if text_end > start {
                    self.pos = text_end;
                    return Ok(Spanned {
                        token: LexToken::Text(self.source[start..text_end].to_string()),
                        span: Span::new(start, text_end),
                    });
                }

                self.pos = text_end + 1;
                return Ok(Spanned {
                    token: LexToken::CloseBrace,
                    span: Span::new(text_end, text_end + 1),
                });
            }

            // Check for `@keyword`
            if bytes[text_end] == b'@' {
                if let Some((kw_token, kw_len)) = self.try_at_keyword(text_end) {
                    if text_end > start {
                        self.pos = text_end;
                        return Ok(Spanned {
                            token: LexToken::Text(self.source[start..text_end].to_string()),
                            span: Span::new(start, text_end),
                        });
                    }

                    let span = Span::new(text_end, text_end + kw_len);
                    self.pos = text_end + kw_len;
                    return Ok(Spanned {
                        token: kw_token,
                        span,
                    });
                }
            }

            text_end += 1;
        }

        // Reached EOF — emit remaining text
        if text_end > start {
            self.pos = text_end;
            return Ok(Spanned {
                token: LexToken::Text(self.source[start..text_end].to_string()),
                span: Span::new(start, text_end),
            });
        }

        self.pos = text_end;
        Ok(Spanned {
            token: LexToken::Eof,
            span: self.here(),
        })
    }

    /// Scan the next token in expression mode. Checks for `}}`
    /// before delegating to logos.
    pub fn next_expr(&mut self) -> Result<Spanned> {
        if let Some(sp) = self.peeked_expr.take() {
            return Ok(sp);
        }

        self.scan_expr()
    }

    /// Peek at the next expression-mode token without consuming it.
    pub fn peek_expr(&mut self) -> Result<&Spanned> {
        if self.peeked_expr.is_none() {
            self.peeked_expr = Some(self.scan_expr()?);
        }

        Ok(self.peeked_expr.as_ref().unwrap())
    }

    fn scan_expr(&mut self) -> Result<Spanned> {
        let rem = self.remaining();

        if rem.is_empty() {
            return Ok(Spanned {
                token: LexToken::Eof,
                span: self.here(),
            });
        }

        // Skip leading whitespace
        let trimmed = rem.trim_start();
        let ws_len = rem.len() - trimmed.len();
        self.pos += ws_len;
        let rem = trimmed;

        if rem.is_empty() {
            return Ok(Spanned {
                token: LexToken::Eof,
                span: self.here(),
            });
        }

        // Check for `}}`
        if rem.starts_with("}}") {
            let span = Span::new(self.pos, self.pos + 2);
            self.pos += 2;
            return Ok(Spanned {
                token: LexToken::InterpEnd,
                span,
            });
        }

        let mut lex = Token::lexer(rem);
        match lex.next() {
            Some(Ok(tok)) => {
                let logo_span = lex.span();
                let span = Span::new(self.pos + logo_span.start, self.pos + logo_span.end);
                self.pos += logo_span.end;
                Ok(Spanned {
                    token: LexToken::Expr(tok),
                    span,
                })
            }
            Some(Err(())) => Err(ParseError::new(
                "unexpected character",
                Span::new(self.pos, self.pos + 1),
            )),
            None => Ok(Spanned {
                token: LexToken::Eof,
                span: self.here(),
            }),
        }
    }

    /// Check if source at `pos` starts with `@keyword` where keyword is
    /// one of the recognized template keywords. Returns the token and
    /// total byte length consumed (including `@`).
    fn try_at_keyword(&self, pos: usize) -> Option<(LexToken, usize)> {
        let after_at = &self.source[pos + 1..];

        for (kw, token) in AT_KEYWORDS {
            if after_at.starts_with(kw) {
                let end = pos + 1 + kw.len();
                // Ensure the keyword is not part of a longer identifier
                if end >= self.source.len()
                    || !self.source.as_bytes()[end].is_ascii_alphanumeric()
                        && self.source.as_bytes()[end] != b'_'
                {
                    return Some((token.clone(), 1 + kw.len()));
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_only() {
        let mut lex = Lexer::new("hello world");
        let sp = lex.next_text().unwrap();
        assert_eq!(sp.token, LexToken::Text("hello world".to_string()));
        let sp = lex.next_text().unwrap();
        assert_eq!(sp.token, LexToken::Eof);
    }

    #[test]
    fn interp() {
        let mut lex = Lexer::new("hello {{ name }}");
        let sp = lex.next_text().unwrap();
        assert_eq!(sp.token, LexToken::Text("hello ".to_string()));

        let sp = lex.next_text().unwrap();
        assert_eq!(sp.token, LexToken::InterpStart);

        let sp = lex.next_expr().unwrap();
        assert_eq!(sp.token, LexToken::Expr(Token::Ident("name".to_string())));

        let sp = lex.next_expr().unwrap();
        assert_eq!(sp.token, LexToken::InterpEnd);
    }

    #[test]
    fn at_keyword() {
        let mut lex = Lexer::new("@if (x) { hi }");
        let sp = lex.next_text().unwrap();
        assert_eq!(sp.token, LexToken::AtIf);

        let sp = lex.next_expr().unwrap();
        assert_eq!(sp.token, LexToken::Expr(Token::LParen));
    }

    #[test]
    fn at_not_keyword() {
        let mut lex = Lexer::new("email@test.com");
        let sp = lex.next_text().unwrap();
        assert_eq!(sp.token, LexToken::Text("email@test.com".to_string()));
    }

    #[test]
    fn brace_depth() {
        let mut lex = Lexer::new("@if (x) { hello }");

        let sp = lex.next_text().unwrap();
        assert_eq!(sp.token, LexToken::AtIf);

        let sp = lex.next_expr().unwrap();
        assert_eq!(sp.token, LexToken::Expr(Token::LParen));

        let sp = lex.next_expr().unwrap();
        assert_eq!(sp.token, LexToken::Expr(Token::Ident("x".to_string())));

        let sp = lex.next_expr().unwrap();
        assert_eq!(sp.token, LexToken::Expr(Token::RParen));

        let sp = lex.next_expr().unwrap();
        assert_eq!(sp.token, LexToken::Expr(Token::LBrace));

        lex.open_brace();

        let sp = lex.next_text().unwrap();
        assert_eq!(sp.token, LexToken::Text(" hello ".to_string()));

        let sp = lex.next_text().unwrap();
        assert_eq!(sp.token, LexToken::CloseBrace);

        lex.close_brace();
    }

    #[test]
    fn peek_expr() {
        let mut lex = Lexer::new("{{ a }}");

        let _ = lex.next_text().unwrap(); // InterpStart

        let peeked = lex.peek_expr().unwrap().clone();
        assert_eq!(peeked.token, LexToken::Expr(Token::Ident("a".to_string())));

        let next = lex.next_expr().unwrap();
        assert_eq!(next.token, peeked.token);
    }
}
