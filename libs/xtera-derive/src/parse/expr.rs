use proc_macro2::TokenStream;
use quote::quote;
use syn::Token;
use syn::parse::ParseStream;

use super::BinOp;
use super::s;

pub(super) fn parse_expr(input: ParseStream) -> syn::Result<TokenStream> {
    parse_pipe(input)
}

fn parse_pipe(input: ParseStream) -> syn::Result<TokenStream> {
    let mut expr = parse_binary(input, 0)?;

    while input.peek(Token![|]) && !input.peek(Token![||]) {
        input.parse::<Token![|]>()?;
        let name: syn::Ident = input.parse()?;
        let name_str = name.to_string();

        let mut args = Vec::new();
        while input.peek(Token![:]) && !input.peek(Token![::]) {
            input.parse::<Token![:]>()?;
            args.push(parse_binary(input, 0)?);
        }

        let span = s();
        expr = quote! {
            ::xtera::ast::Expr::Pipe(::xtera::ast::PipeExpr {
                value: Box::new(#expr),
                name: #name_str.into(),
                args: vec![#(#args),*],
                span: #span,
            })
        };
    }

    Ok(expr)
}

fn parse_binary(input: ParseStream, min_bp: u8) -> syn::Result<TokenStream> {
    let mut left = parse_unary(input)?;

    loop {
        let op = match peek_binary_op(input) {
            Some(op) => op,
            None => break,
        };

        let (l_bp, r_bp) = op.precedence();
        if l_bp < min_bp {
            break;
        }

        consume_binary_op(input, op)?;

        let right = parse_binary(input, r_bp)?;
        let op_tokens = op.to_tokens();
        let span = s();
        left = quote! {
            ::xtera::ast::Expr::Binary(::xtera::ast::BinaryExpr {
                left: Box::new(#left),
                op: #op_tokens,
                right: Box::new(#right),
                span: #span,
            })
        };
    }

    Ok(left)
}

fn peek_binary_op(input: ParseStream) -> Option<BinOp> {
    if input.peek(Token![==]) {
        Some(BinOp::Eq)
    } else if input.peek(Token![!=]) {
        Some(BinOp::Ne)
    } else if input.peek(Token![<=]) {
        Some(BinOp::Le)
    } else if input.peek(Token![>=]) {
        Some(BinOp::Ge)
    } else if input.peek(Token![&&]) {
        Some(BinOp::And)
    } else if input.peek(Token![||]) {
        Some(BinOp::Or)
    } else if input.peek(Token![+]) {
        Some(BinOp::Add)
    } else if input.peek(Token![-]) {
        Some(BinOp::Sub)
    } else if input.peek(Token![*]) {
        Some(BinOp::Mul)
    } else if input.peek(Token![/]) {
        Some(BinOp::Div)
    } else if input.peek(Token![%]) {
        Some(BinOp::Mod)
    } else if input.peek(Token![<]) {
        Some(BinOp::Lt)
    } else if input.peek(Token![>]) {
        Some(BinOp::Gt)
    } else {
        None
    }
}

fn consume_binary_op(input: ParseStream, op: BinOp) -> syn::Result<()> {
    match op {
        BinOp::Add => {
            input.parse::<Token![+]>()?;
        }
        BinOp::Sub => {
            input.parse::<Token![-]>()?;
        }
        BinOp::Mul => {
            input.parse::<Token![*]>()?;
        }
        BinOp::Div => {
            input.parse::<Token![/]>()?;
        }
        BinOp::Mod => {
            input.parse::<Token![%]>()?;
        }
        BinOp::Eq => {
            input.parse::<Token![==]>()?;
        }
        BinOp::Ne => {
            input.parse::<Token![!=]>()?;
        }
        BinOp::Lt => {
            input.parse::<Token![<]>()?;
        }
        BinOp::Le => {
            input.parse::<Token![<=]>()?;
        }
        BinOp::Gt => {
            input.parse::<Token![>]>()?;
        }
        BinOp::Ge => {
            input.parse::<Token![>=]>()?;
        }
        BinOp::And => {
            input.parse::<Token![&&]>()?;
        }
        BinOp::Or => {
            input.parse::<Token![||]>()?;
        }
    }
    Ok(())
}

fn parse_unary(input: ParseStream) -> syn::Result<TokenStream> {
    if input.peek(Token![!]) {
        input.parse::<Token![!]>()?;
        let operand = parse_unary(input)?;
        let span = s();
        Ok(quote! {
            ::xtera::ast::Expr::Unary(::xtera::ast::UnaryExpr {
                op: ::xtera::ast::UnaryOp::Not,
                operand: Box::new(#operand),
                span: #span,
            })
        })
    } else if input.peek(Token![-]) {
        input.parse::<Token![-]>()?;
        let operand = parse_unary(input)?;
        let span = s();
        Ok(quote! {
            ::xtera::ast::Expr::Unary(::xtera::ast::UnaryExpr {
                op: ::xtera::ast::UnaryOp::Neg,
                operand: Box::new(#operand),
                span: #span,
            })
        })
    } else {
        parse_postfix(input)
    }
}

fn parse_postfix(input: ParseStream) -> syn::Result<TokenStream> {
    let mut expr = parse_atom(input)?;

    loop {
        if input.peek(Token![.]) {
            input.parse::<Token![.]>()?;
            let field: syn::Ident = input.parse()?;
            let field_str = field.to_string();
            let span = s();
            expr = quote! {
                ::xtera::ast::Expr::Member(::xtera::ast::MemberExpr {
                    object: Box::new(#expr),
                    field: #field_str.into(),
                    span: #span,
                })
            };
        } else if input.peek(syn::token::Bracket) {
            let content;
            syn::bracketed!(content in input);
            let index = parse_expr(&content)?;
            let span = s();
            expr = quote! {
                ::xtera::ast::Expr::Index(::xtera::ast::IndexExpr {
                    object: Box::new(#expr),
                    index: Box::new(#index),
                    span: #span,
                })
            };
        } else if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            let args = parse_comma_separated_exprs(&content)?;
            let span = s();
            expr = quote! {
                ::xtera::ast::Expr::Call(::xtera::ast::CallExpr {
                    callee: Box::new(#expr),
                    args: vec![#(#args),*],
                    span: #span,
                })
            };
        } else {
            break;
        }
    }

    Ok(expr)
}

fn parse_atom(input: ParseStream) -> syn::Result<TokenStream> {
    let span = s();

    if input.peek(syn::LitBool) {
        let lit: syn::LitBool = input.parse()?;
        let b = lit.value;
        Ok(quote! {
            ::xtera::ast::Expr::Value(::xtera::ast::ValueExpr {
                value: ::xval::Value::from_bool(#b),
                span: #span,
            })
        })
    } else if input.peek(syn::LitStr) {
        let lit: syn::LitStr = input.parse()?;
        let val = lit.value();
        Ok(quote! {
            ::xtera::ast::Expr::Value(::xtera::ast::ValueExpr {
                value: ::xval::Value::from_str(#val),
                span: #span,
            })
        })
    } else if input.peek(syn::LitFloat) {
        let lit: syn::LitFloat = input.parse()?;
        let n: f64 = lit.base10_parse()?;
        Ok(quote! {
            ::xtera::ast::Expr::Value(::xtera::ast::ValueExpr {
                value: ::xval::Value::from_f64(#n),
                span: #span,
            })
        })
    } else if input.peek(syn::LitInt) {
        let lit: syn::LitInt = input.parse()?;
        let n: i64 = lit.base10_parse()?;
        Ok(quote! {
            ::xtera::ast::Expr::Value(::xtera::ast::ValueExpr {
                value: ::xval::Value::from_i64(#n),
                span: #span,
            })
        })
    } else if input.peek(syn::Ident) {
        let ident: syn::Ident = input.parse()?;
        let name = ident.to_string();
        if name == "null" {
            Ok(quote! {
                ::xtera::ast::Expr::Value(::xtera::ast::ValueExpr {
                    value: ::xval::Value::Null,
                    span: #span,
                })
            })
        } else {
            Ok(quote! {
                ::xtera::ast::Expr::Ident(::xtera::ast::IdentExpr {
                    name: #name.into(),
                    span: #span,
                })
            })
        }
    } else if input.peek(syn::token::Paren) {
        let content;
        syn::parenthesized!(content in input);
        let expr = parse_expr(&content)?;
        if !content.is_empty() {
            return Err(content.error("unexpected tokens in parenthesized expression"));
        }
        Ok(expr)
    } else if input.peek(syn::token::Bracket) {
        let content;
        syn::bracketed!(content in input);
        let elements = parse_comma_separated_exprs(&content)?;
        Ok(quote! {
            ::xtera::ast::Expr::Array(::xtera::ast::ArrayExpr {
                elements: vec![#(#elements),*],
                span: #span,
            })
        })
    } else if input.peek(syn::token::Brace) {
        let content;
        syn::braced!(content in input);
        let mut keys = Vec::new();
        let mut values = Vec::new();
        while !content.is_empty() {
            let key = if content.peek(syn::Ident) {
                let ident: syn::Ident = content.parse()?;
                ident.to_string()
            } else if content.peek(syn::LitStr) {
                let lit: syn::LitStr = content.parse()?;
                lit.value()
            } else {
                return Err(content.error("expected object key (identifier or string)"));
            };
            content.parse::<Token![:]>()?;
            let value = parse_expr(&content)?;
            keys.push(key);
            values.push(value);
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            } else {
                break;
            }
        }
        Ok(quote! {
            ::xtera::ast::Expr::Object(::xtera::ast::ObjectExpr {
                entries: vec![#((#keys.into(), #values)),*],
                span: #span,
            })
        })
    } else {
        Err(input.error("expected expression"))
    }
}

fn parse_comma_separated_exprs(input: ParseStream) -> syn::Result<Vec<TokenStream>> {
    let mut exprs = Vec::new();
    while !input.is_empty() {
        exprs.push(parse_expr(input)?);
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        } else {
            break;
        }
    }
    Ok(exprs)
}
