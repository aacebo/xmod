use crate::{
    Template,
    ast::{
        ArrayExpr, BinaryExpr, BinaryOp, CallExpr, Expr, ForNode, IdentExpr, IfBranch, IfNode,
        IndexExpr, InterpNode, MemberExpr, Node, ObjectExpr, PipeExpr, Span, SwitchCase,
        SwitchNode, TextNode, UnaryExpr, UnaryOp, ValueExpr,
    },
};

use super::error::{ParseError, Result};
use super::lexer::{LexToken, Lexer, Spanned};
use super::token::Token;

pub struct Parser<'src> {
    lexer: Lexer<'src>,
    source_len: usize,
}

impl<'src> Parser<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            lexer: Lexer::new(source),
            source_len: source.len(),
        }
    }

    /// Parse the entire template.
    pub fn parse(&mut self) -> Result<Template> {
        let nodes = self.parse_nodes()?;
        let span = if nodes.is_empty() {
            Span::new(0, 0)
        } else {
            nodes
                .first()
                .unwrap()
                .span()
                .merge(nodes.last().unwrap().span())
        };

        Ok(Template::new(nodes, span))
    }

    // ── Template-level parsing ──────────────────────────────────────

    fn parse_nodes(&mut self) -> Result<Vec<Node>> {
        let mut nodes = Vec::new();

        loop {
            match self.parse_node()? {
                Some(node) => nodes.push(node),
                None => break,
            }
        }

        Ok(nodes)
    }

    fn parse_node(&mut self) -> Result<Option<Node>> {
        let sp = self.lexer.next_text()?;

        match sp.token {
            LexToken::Text(s) => Ok(Some(Node::Text(TextNode {
                text: s,
                span: sp.span,
            }))),
            LexToken::InterpStart => {
                let expr = self.parse_expr()?;
                let end = self.expect_interp_end()?;
                let span = sp.span.merge(end.span);
                Ok(Some(Node::Interp(InterpNode { expr, span })))
            }
            LexToken::AtIf => {
                let (branches, else_body) = self.parse_if(sp.span)?;
                let span = sp.span.merge(self.last_span());
                Ok(Some(Node::If(IfNode {
                    branches,
                    else_body,
                    span,
                })))
            }
            LexToken::AtFor => {
                let for_node = self.parse_for()?;
                let span = sp.span.merge(self.last_span());
                Ok(Some(Node::For(ForNode { span, ..for_node })))
            }
            LexToken::AtSwitch => {
                let switch_node = self.parse_switch()?;
                let span = sp.span.merge(self.last_span());
                Ok(Some(Node::Switch(SwitchNode {
                    span,
                    ..switch_node
                })))
            }
            LexToken::CloseBrace | LexToken::Eof => Ok(None),
            other => Err(ParseError::new(
                format!("unexpected token: {other:?}"),
                sp.span,
            )),
        }
    }

    fn parse_if(&mut self, start_span: Span) -> Result<(Vec<IfBranch>, Option<Vec<Node>>)> {
        let mut branches = Vec::new();

        // First branch
        self.expect_expr_token(&Token::LParen)?;
        let condition = self.parse_expr()?;
        self.expect_expr_token(&Token::RParen)?;
        let body = self.parse_block_body()?;
        branches.push(IfBranch {
            condition,
            body,
            span: start_span.merge(self.last_span()),
        });

        let mut else_body = None;

        loop {
            if !self.lexer.starts_with_at_keyword("else") {
                break;
            }

            self.lexer.skip_whitespace();
            let sp = self.lexer.next_text()?;
            if sp.token != LexToken::AtElse {
                return Err(ParseError::new(
                    format!("expected @else, got {:?}", sp.token),
                    sp.span,
                ));
            }

            if self.lexer.starts_with_at_keyword("if") {
                self.lexer.skip_whitespace();
                let _ = self.lexer.next_text()?;

                self.expect_expr_token(&Token::LParen)?;
                let condition = self.parse_expr()?;
                self.expect_expr_token(&Token::RParen)?;
                let body = self.parse_block_body()?;
                branches.push(IfBranch {
                    condition,
                    body,
                    span: sp.span.merge(self.last_span()),
                });

                continue;
            }

            else_body = Some(self.parse_block_body()?);
            break;
        }

        Ok((branches, else_body))
    }

    fn parse_for(&mut self) -> Result<ForNode> {
        self.expect_expr_token(&Token::LParen)?;

        let binding_sp = self.lexer.next_expr()?;
        let binding = match binding_sp.token {
            LexToken::Expr(Token::Ident(s)) => s,
            other => {
                return Err(ParseError::new(
                    format!("expected identifier, got {other:?}"),
                    binding_sp.span,
                ));
            }
        };

        let of_sp = self.lexer.next_expr()?;
        if of_sp.token != LexToken::Expr(Token::Of) {
            return Err(ParseError::new(
                format!("expected 'of', got {:?}", of_sp.token),
                of_sp.span,
            ));
        }

        let iterable = self.parse_expr()?;

        self.expect_expr_token(&Token::Semi)?;

        let track_sp = self.lexer.next_expr()?;
        if track_sp.token != LexToken::Expr(Token::Track) {
            return Err(ParseError::new(
                format!("expected 'track', got {:?}", track_sp.token),
                track_sp.span,
            ));
        }

        let track = self.parse_expr()?;

        self.expect_expr_token(&Token::RParen)?;

        let body = self.parse_block_body()?;

        Ok(ForNode {
            binding,
            iterable,
            track,
            body,
            span: Span::new(0, 0), // will be overwritten by caller
        })
    }

    fn parse_switch(&mut self) -> Result<SwitchNode> {
        self.expect_expr_token(&Token::LParen)?;
        let expr = self.parse_expr()?;
        self.expect_expr_token(&Token::RParen)?;
        self.expect_expr_token(&Token::LBrace)?;
        self.lexer.open_brace();

        let mut cases = Vec::new();
        let mut default = None;

        loop {
            let sp = self.lexer.next_text()?;

            let sp = if matches!(&sp.token, LexToken::Text(s) if s.trim().is_empty()) {
                self.lexer.next_text()?
            } else {
                sp
            };

            match sp.token {
                LexToken::AtCase => {
                    self.expect_expr_token(&Token::LParen)?;
                    let value = self.parse_expr()?;
                    self.expect_expr_token(&Token::RParen)?;
                    let body = self.parse_block_body()?;
                    cases.push(SwitchCase {
                        value,
                        body,
                        span: sp.span.merge(self.last_span()),
                    });
                }
                LexToken::AtDefault => {
                    let body = self.parse_block_body()?;
                    default = Some(body);
                }
                LexToken::CloseBrace => {
                    self.lexer.close_brace();
                    break;
                }
                LexToken::Eof => {
                    return Err(ParseError::new(
                        "unexpected end of input in @switch block",
                        sp.span,
                    ));
                }
                other => {
                    return Err(ParseError::new(
                        format!("expected @case, @default, or '}}', got {other:?}"),
                        sp.span,
                    ));
                }
            }
        }

        Ok(SwitchNode {
            expr,
            cases,
            default,
            span: Span::new(0, 0), // will be overwritten by caller
        })
    }

    fn parse_block_body(&mut self) -> Result<Vec<Node>> {
        self.expect_expr_token(&Token::LBrace)?;
        self.lexer.open_brace();

        let nodes = self.parse_nodes()?;

        self.lexer.close_brace();

        Ok(nodes)
    }

    // ── Expression parsing ──────────────────────────────────────────

    fn parse_expr(&mut self) -> Result<Expr> {
        self.parse_pipe_expr()
    }

    fn parse_pipe_expr(&mut self) -> Result<Expr> {
        let mut expr = self.parse_binary(0)?;

        loop {
            let peeked = self.lexer.peek_expr()?;
            if peeked.token != LexToken::Expr(Token::Pipe) {
                break;
            }

            self.lexer.next_expr()?;

            let name_sp = self.lexer.next_expr()?;
            let name = match name_sp.token {
                LexToken::Expr(Token::Ident(s)) => s,
                other => {
                    return Err(ParseError::new(
                        format!("expected pipe name, got {other:?}"),
                        name_sp.span,
                    ));
                }
            };

            let mut args = Vec::new();
            loop {
                let peeked = self.lexer.peek_expr()?;
                if peeked.token != LexToken::Expr(Token::Colon) {
                    break;
                }

                self.lexer.next_expr()?;
                args.push(self.parse_binary(0)?);
            }

            let span = expr.span().merge(name_sp.span);
            expr = Expr::Pipe(PipeExpr {
                value: Box::new(expr),
                name,
                args,
                span,
            });
        }

        Ok(expr)
    }

    fn parse_binary(&mut self, min_bp: u8) -> Result<Expr> {
        let mut left = self.parse_unary()?;

        loop {
            let peeked = self.lexer.peek_expr()?;
            let op = match &peeked.token {
                LexToken::Expr(tok) => Self::token_to_binary_op(tok),
                _ => None,
            };

            let op = match op {
                Some(op) => op,
                None => break,
            };

            let (l_bp, r_bp) = op.precedence();
            if l_bp < min_bp {
                break;
            }

            self.lexer.next_expr()?;

            let right = self.parse_binary(r_bp)?;
            let span = left.span().merge(right.span());
            left = Expr::Binary(BinaryExpr {
                left: Box::new(left),
                op,
                right: Box::new(right),
                span,
            });
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr> {
        let peeked = self.lexer.peek_expr()?;

        match &peeked.token {
            LexToken::Expr(Token::Bang) => {
                let sp = self.lexer.next_expr()?;
                let operand = self.parse_unary()?;
                let span = sp.span.merge(operand.span());
                Ok(Expr::Unary(UnaryExpr {
                    op: UnaryOp::Not,
                    operand: Box::new(operand),
                    span,
                }))
            }
            LexToken::Expr(Token::Minus) => {
                let sp = self.lexer.next_expr()?;
                let operand = self.parse_unary()?;
                let span = sp.span.merge(operand.span());
                Ok(Expr::Unary(UnaryExpr {
                    op: UnaryOp::Neg,
                    operand: Box::new(operand),
                    span,
                }))
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr> {
        let mut expr = self.parse_atom()?;

        loop {
            let peeked = self.lexer.peek_expr()?;

            match &peeked.token {
                LexToken::Expr(Token::Dot) => {
                    self.lexer.next_expr()?;
                    let field_sp = self.lexer.next_expr()?;
                    let field = match field_sp.token {
                        LexToken::Expr(Token::Ident(s)) => s,
                        LexToken::Expr(Token::Of) => "of".to_string(),
                        LexToken::Expr(Token::Track) => "track".to_string(),
                        other => {
                            return Err(ParseError::new(
                                format!("expected field name, got {other:?}"),
                                field_sp.span,
                            ));
                        }
                    };

                    let span = expr.span().merge(field_sp.span);
                    expr = Expr::Member(MemberExpr {
                        object: Box::new(expr),
                        field,
                        span,
                    });
                }
                LexToken::Expr(Token::LBracket) => {
                    self.lexer.next_expr()?;
                    let index = self.parse_expr()?;
                    let end = self.expect_expr_token(&Token::RBracket)?;
                    let span = expr.span().merge(end.span);
                    expr = Expr::Index(IndexExpr {
                        object: Box::new(expr),
                        index: Box::new(index),
                        span,
                    });
                }
                LexToken::Expr(Token::LParen) => {
                    self.lexer.next_expr()?;
                    let args = self.parse_args()?;
                    let end = self.expect_expr_token(&Token::RParen)?;
                    let span = expr.span().merge(end.span);
                    expr = Expr::Call(CallExpr {
                        callee: Box::new(expr),
                        args,
                        span,
                    });
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_atom(&mut self) -> Result<Expr> {
        let sp = self.lexer.next_expr()?;

        match sp.token {
            LexToken::Expr(Token::Int(n)) => Ok(Expr::Value(ValueExpr {
                value: xval::Value::from_i64(n),
                span: sp.span,
            })),
            LexToken::Expr(Token::Float(n)) => Ok(Expr::Value(ValueExpr {
                value: xval::Value::from_f64(n),
                span: sp.span,
            })),
            LexToken::Expr(Token::String(s)) => Ok(Expr::Value(ValueExpr {
                value: xval::Value::from_string(s),
                span: sp.span,
            })),
            LexToken::Expr(Token::True) => Ok(Expr::Value(ValueExpr {
                value: xval::Value::from_bool(true),
                span: sp.span,
            })),
            LexToken::Expr(Token::False) => Ok(Expr::Value(ValueExpr {
                value: xval::Value::from_bool(false),
                span: sp.span,
            })),
            LexToken::Expr(Token::Null) => Ok(Expr::Value(ValueExpr {
                value: xval::Value::Null,
                span: sp.span,
            })),
            LexToken::Expr(Token::Ident(s)) => Ok(Expr::Ident(IdentExpr {
                name: s,
                span: sp.span,
            })),
            LexToken::Expr(Token::Of) => Ok(Expr::Ident(IdentExpr {
                name: "of".to_string(),
                span: sp.span,
            })),
            LexToken::Expr(Token::Track) => Ok(Expr::Ident(IdentExpr {
                name: "track".to_string(),
                span: sp.span,
            })),
            LexToken::Expr(Token::LParen) => {
                let inner = self.parse_expr()?;
                let end = self.expect_expr_token(&Token::RParen)?;
                let span = sp.span.merge(end.span);
                // Re-wrap with the parenthesized span
                Ok(Self::with_span(inner, span))
            }
            LexToken::Expr(Token::LBracket) => {
                let mut elements = Vec::new();
                let peeked = self.lexer.peek_expr()?;
                if peeked.token != LexToken::Expr(Token::RBracket) {
                    elements.push(self.parse_expr()?);
                    loop {
                        let peeked = self.lexer.peek_expr()?;
                        if peeked.token != LexToken::Expr(Token::Comma) {
                            break;
                        }
                        self.lexer.next_expr()?;
                        elements.push(self.parse_expr()?);
                    }
                }

                let end = self.expect_expr_token(&Token::RBracket)?;
                let span = sp.span.merge(end.span);
                Ok(Expr::Array(ArrayExpr { elements, span }))
            }
            LexToken::Expr(Token::LBrace) => {
                let mut entries = Vec::new();
                let peeked = self.lexer.peek_expr()?;
                if peeked.token != LexToken::Expr(Token::RBrace) {
                    entries.push(self.parse_object_entry()?);
                    loop {
                        let peeked = self.lexer.peek_expr()?;
                        if peeked.token != LexToken::Expr(Token::Comma) {
                            break;
                        }
                        self.lexer.next_expr()?;
                        entries.push(self.parse_object_entry()?);
                    }
                }

                let end = self.expect_expr_token(&Token::RBrace)?;
                let span = sp.span.merge(end.span);
                Ok(Expr::Object(ObjectExpr { entries, span }))
            }
            other => Err(ParseError::new(
                format!("expected expression, got {other:?}"),
                sp.span,
            )),
        }
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>> {
        let mut args = Vec::new();

        let peeked = self.lexer.peek_expr()?;
        if peeked.token == LexToken::Expr(Token::RParen) {
            return Ok(args);
        }

        args.push(self.parse_expr()?);

        loop {
            let peeked = self.lexer.peek_expr()?;
            if peeked.token != LexToken::Expr(Token::Comma) {
                break;
            }

            self.lexer.next_expr()?;
            args.push(self.parse_expr()?);
        }

        Ok(args)
    }

    fn parse_object_entry(&mut self) -> Result<(String, Expr)> {
        let key_sp = self.lexer.next_expr()?;
        let key = match key_sp.token {
            LexToken::Expr(Token::Ident(s)) => s,
            LexToken::Expr(Token::String(s)) => s,
            other => {
                return Err(ParseError::new(
                    format!("expected object key, got {other:?}"),
                    key_sp.span,
                ));
            }
        };

        self.expect_expr_token(&Token::Colon)?;
        let value = self.parse_expr()?;
        Ok((key, value))
    }

    // ── Helpers ─────────────────────────────────────────────────────

    fn expect_expr_token(&mut self, expected: &Token) -> Result<Spanned> {
        let sp = self.lexer.next_expr()?;

        if sp.token == LexToken::Expr(expected.clone()) {
            Ok(sp)
        } else {
            Err(ParseError::new(
                format!("expected {expected:?}, got {:?}", sp.token),
                sp.span,
            ))
        }
    }

    fn expect_interp_end(&mut self) -> Result<Spanned> {
        let sp = self.lexer.next_expr()?;

        if sp.token == LexToken::InterpEnd {
            Ok(sp)
        } else {
            Err(ParseError::new(
                format!("expected '}}}}', got {:?}", sp.token),
                sp.span,
            ))
        }
    }

    fn last_span(&self) -> Span {
        Span::new(self.source_len, self.source_len)
    }

    /// Re-create an Expr with a new span (for parenthesized expressions).
    fn with_span(expr: Expr, span: Span) -> Expr {
        match expr {
            Expr::Value(mut e) => {
                e.span = span;
                Expr::Value(e)
            }
            Expr::Ident(mut e) => {
                e.span = span;
                Expr::Ident(e)
            }
            Expr::Member(mut e) => {
                e.span = span;
                Expr::Member(e)
            }
            Expr::Index(mut e) => {
                e.span = span;
                Expr::Index(e)
            }
            Expr::Call(mut e) => {
                e.span = span;
                Expr::Call(e)
            }
            Expr::Pipe(mut e) => {
                e.span = span;
                Expr::Pipe(e)
            }
            Expr::Binary(mut e) => {
                e.span = span;
                Expr::Binary(e)
            }
            Expr::Unary(mut e) => {
                e.span = span;
                Expr::Unary(e)
            }
            Expr::Array(mut e) => {
                e.span = span;
                Expr::Array(e)
            }
            Expr::Object(mut e) => {
                e.span = span;
                Expr::Object(e)
            }
        }
    }

    fn token_to_binary_op(token: &Token) -> Option<BinaryOp> {
        match token {
            Token::Plus => Some(BinaryOp::Add),
            Token::Minus => Some(BinaryOp::Sub),
            Token::Star => Some(BinaryOp::Mul),
            Token::Slash => Some(BinaryOp::Div),
            Token::Percent => Some(BinaryOp::Mod),
            Token::EqEq => Some(BinaryOp::Eq),
            Token::BangEq => Some(BinaryOp::Ne),
            Token::Lt => Some(BinaryOp::Lt),
            Token::Le => Some(BinaryOp::Le),
            Token::Gt => Some(BinaryOp::Gt),
            Token::Ge => Some(BinaryOp::Ge),
            Token::AmpAmp => Some(BinaryOp::And),
            Token::PipePipe => Some(BinaryOp::Or),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::*;
    use crate::parse::parse;

    #[test]
    fn plain_text() {
        let tpl = parse("hello world").unwrap();
        assert_eq!(tpl.nodes().len(), 1);
        assert!(
            matches!(&tpl.nodes()[0], Node::Text(TextNode { text, .. }) if text == "hello world")
        );
    }

    #[test]
    fn interp() {
        let tpl = parse("hello {{ name }}").unwrap();
        assert_eq!(tpl.nodes().len(), 2);
        assert!(matches!(&tpl.nodes()[0], Node::Text(TextNode { text, .. }) if text == "hello "));
        assert!(
            matches!(&tpl.nodes()[1], Node::Interp(InterpNode { expr: Expr::Ident(IdentExpr { name, .. }), .. }) if name == "name")
        );
    }

    #[test]
    fn pipe() {
        let tpl = parse("{{ value | uppercase }}").unwrap();
        assert_eq!(tpl.nodes().len(), 1);
        match &tpl.nodes()[0] {
            Node::Interp(InterpNode {
                expr: Expr::Pipe(PipeExpr { name, args, .. }),
                ..
            }) => {
                assert_eq!(name, "uppercase");
                assert!(args.is_empty());
            }
            other => panic!("expected pipe, got {other:?}"),
        }
    }

    #[test]
    fn pipe_with_args() {
        let tpl = parse("{{ value | slice:0:5 }}").unwrap();
        match &tpl.nodes()[0] {
            Node::Interp(InterpNode {
                expr: Expr::Pipe(PipeExpr { name, args, .. }),
                ..
            }) => {
                assert_eq!(name, "slice");
                assert_eq!(args.len(), 2);
            }
            other => panic!("expected pipe, got {other:?}"),
        }
    }

    #[test]
    fn binary_precedence() {
        let tpl = parse("{{ a + b * c }}").unwrap();
        match &tpl.nodes()[0] {
            Node::Interp(InterpNode {
                expr: Expr::Binary(BinaryExpr { op, right, .. }),
                ..
            }) => {
                assert_eq!(*op, BinaryOp::Add);
                assert!(matches!(
                    &**right,
                    Expr::Binary(BinaryExpr {
                        op: BinaryOp::Mul,
                        ..
                    })
                ));
            }
            other => panic!("expected binary, got {other:?}"),
        }
    }

    #[test]
    fn unary() {
        let tpl = parse("{{ !done }}").unwrap();
        match &tpl.nodes()[0] {
            Node::Interp(InterpNode {
                expr: Expr::Unary(UnaryExpr { op, .. }),
                ..
            }) => {
                assert_eq!(*op, UnaryOp::Not);
            }
            other => panic!("expected unary, got {other:?}"),
        }
    }

    #[test]
    fn member_access() {
        let tpl = parse("{{ obj.field }}").unwrap();
        match &tpl.nodes()[0] {
            Node::Interp(InterpNode {
                expr: Expr::Member(MemberExpr { field, .. }),
                ..
            }) => {
                assert_eq!(field, "field");
            }
            other => panic!("expected member, got {other:?}"),
        }
    }

    #[test]
    fn index_access() {
        let tpl = parse("{{ arr[0] }}").unwrap();
        assert!(matches!(
            &tpl.nodes()[0],
            Node::Interp(InterpNode {
                expr: Expr::Index(_),
                ..
            })
        ));
    }

    #[test]
    fn function_call() {
        let tpl = parse("{{ greet('world') }}").unwrap();
        match &tpl.nodes()[0] {
            Node::Interp(InterpNode {
                expr: Expr::Call(CallExpr { args, .. }),
                ..
            }) => {
                assert_eq!(args.len(), 1);
            }
            other => panic!("expected call, got {other:?}"),
        }
    }

    #[test]
    fn method_call() {
        let tpl = parse("{{ obj.method(1, 2) }}").unwrap();
        match &tpl.nodes()[0] {
            Node::Interp(InterpNode {
                expr: Expr::Call(CallExpr { callee, args, .. }),
                ..
            }) => {
                assert!(
                    matches!(&**callee, Expr::Member(MemberExpr { field, .. }) if field == "method")
                );
                assert_eq!(args.len(), 2);
            }
            other => panic!("expected call, got {other:?}"),
        }
    }

    #[test]
    fn if_block() {
        let tpl = parse("@if (show) { visible }").unwrap();
        assert_eq!(tpl.nodes().len(), 1);
        match &tpl.nodes()[0] {
            Node::If(IfNode {
                branches,
                else_body,
                ..
            }) => {
                assert_eq!(branches.len(), 1);
                assert!(else_body.is_none());
            }
            other => panic!("expected if, got {other:?}"),
        }
    }

    #[test]
    fn if_else_block() {
        let tpl = parse("@if (show) { visible } @else { hidden }").unwrap();
        match &tpl.nodes()[0] {
            Node::If(IfNode {
                branches,
                else_body,
                ..
            }) => {
                assert_eq!(branches.len(), 1);
                assert!(else_body.is_some());
            }
            other => panic!("expected if, got {other:?}"),
        }
    }

    #[test]
    fn if_else_if_else() {
        let tpl = parse("@if (a) { one } @else @if (b) { two } @else { three }").unwrap();
        match &tpl.nodes()[0] {
            Node::If(IfNode {
                branches,
                else_body,
                ..
            }) => {
                assert_eq!(branches.len(), 2);
                assert!(else_body.is_some());
            }
            other => panic!("expected if, got {other:?}"),
        }
    }

    #[test]
    fn for_block() {
        let tpl = parse("@for (item of items; track item.id) { {{ item.name }} }").unwrap();
        assert_eq!(tpl.nodes().len(), 1);
        match &tpl.nodes()[0] {
            Node::For(ForNode { binding, body, .. }) => {
                assert_eq!(binding, "item");
                assert!(!body.is_empty());
            }
            other => panic!("expected for, got {other:?}"),
        }
    }

    #[test]
    fn switch_block() {
        let tpl = parse("@switch (color) { @case ('red') { Red! } @case ('blue') { Blue! } @default { Other } }").unwrap();
        match &tpl.nodes()[0] {
            Node::Switch(SwitchNode { cases, default, .. }) => {
                assert_eq!(cases.len(), 2);
                assert!(default.is_some());
            }
            other => panic!("expected switch, got {other:?}"),
        }
    }

    #[test]
    fn nested_if_in_for() {
        let tpl =
            parse("@for (item of items; track item.id) { @if (item.visible) { {{ item.name }} } }")
                .unwrap();
        assert_eq!(tpl.nodes().len(), 1);
        match &tpl.nodes()[0] {
            Node::For(ForNode { body, .. }) => {
                let has_if = body.iter().any(|n| matches!(n, Node::If(_)));
                assert!(has_if);
            }
            other => panic!("expected for, got {other:?}"),
        }
    }

    #[test]
    fn literals() {
        let tpl = parse("{{ null }}").unwrap();
        assert!(matches!(
            &tpl.nodes()[0],
            Node::Interp(InterpNode {
                expr: Expr::Value(ValueExpr {
                    value: xval::Value::Null,
                    ..
                }),
                ..
            })
        ));

        let tpl = parse("{{ true }}").unwrap();
        assert!(matches!(
            &tpl.nodes()[0],
            Node::Interp(InterpNode {
                expr: Expr::Value(ValueExpr {
                    value: xval::Value::Bool(_),
                    ..
                }),
                ..
            })
        ));

        let tpl = parse("{{ 42 }}").unwrap();
        assert!(matches!(
            &tpl.nodes()[0],
            Node::Interp(InterpNode {
                expr: Expr::Value(ValueExpr {
                    value: xval::Value::Number(_),
                    ..
                }),
                ..
            })
        ));

        let tpl = parse("{{ 3.14 }}").unwrap();
        assert!(matches!(
            &tpl.nodes()[0],
            Node::Interp(InterpNode {
                expr: Expr::Value(ValueExpr {
                    value: xval::Value::Number(_),
                    ..
                }),
                ..
            })
        ));
    }

    #[test]
    fn grouped_expression() {
        let tpl = parse("{{ (a + b) * c }}").unwrap();
        match &tpl.nodes()[0] {
            Node::Interp(InterpNode {
                expr: Expr::Binary(BinaryExpr { op, left, .. }),
                ..
            }) => {
                assert_eq!(*op, BinaryOp::Mul);
                assert!(matches!(
                    &**left,
                    Expr::Binary(BinaryExpr {
                        op: BinaryOp::Add,
                        ..
                    })
                ));
            }
            other => panic!("expected binary, got {other:?}"),
        }
    }

    #[test]
    fn empty_template() {
        let tpl = parse("").unwrap();
        assert!(tpl.nodes().is_empty());
    }

    #[test]
    fn array_literal() {
        let tpl = parse("{{ [1, 2, 3] }}").unwrap();
        match &tpl.nodes()[0] {
            Node::Interp(InterpNode {
                expr: Expr::Array(ArrayExpr { elements, .. }),
                ..
            }) => {
                assert_eq!(elements.len(), 3);
            }
            other => panic!("expected array, got {other:?}"),
        }
    }

    #[test]
    fn array_empty() {
        let tpl = parse("{{ [] }}").unwrap();
        match &tpl.nodes()[0] {
            Node::Interp(InterpNode {
                expr: Expr::Array(ArrayExpr { elements, .. }),
                ..
            }) => {
                assert!(elements.is_empty());
            }
            other => panic!("expected array, got {other:?}"),
        }
    }

    #[test]
    fn object_literal() {
        let tpl = parse("{{ { a: 1, b: '2' } }}").unwrap();
        match &tpl.nodes()[0] {
            Node::Interp(InterpNode {
                expr: Expr::Object(ObjectExpr { entries, .. }),
                ..
            }) => {
                assert_eq!(entries.len(), 2);
                assert_eq!(entries[0].0, "a");
                assert_eq!(entries[1].0, "b");
            }
            other => panic!("expected object, got {other:?}"),
        }
    }

    #[test]
    fn object_empty() {
        let tpl = parse("{{ {} }}").unwrap();
        match &tpl.nodes()[0] {
            Node::Interp(InterpNode {
                expr: Expr::Object(ObjectExpr { entries, .. }),
                ..
            }) => {
                assert!(entries.is_empty());
            }
            other => panic!("expected object, got {other:?}"),
        }
    }

    #[test]
    fn nested_object_in_array() {
        let tpl = parse("{{ [{ a: 1 }, { b: 2 }] }}").unwrap();
        match &tpl.nodes()[0] {
            Node::Interp(InterpNode {
                expr: Expr::Array(ArrayExpr { elements, .. }),
                ..
            }) => {
                assert_eq!(elements.len(), 2);
                assert!(matches!(&elements[0], Expr::Object(_)));
                assert!(matches!(&elements[1], Expr::Object(_)));
            }
            other => panic!("expected array, got {other:?}"),
        }
    }
}
