use proc_macro2::TokenStream;
use quote::quote;
use syn::Token;
use syn::parse::ParseStream;

/// Local mirror of binary operator types — needed for Pratt precedence
/// climbing without depending on xtera at build time.
#[derive(Clone, Copy)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

impl BinOp {
    fn precedence(self) -> (u8, u8) {
        match self {
            Self::Or => (1, 2),
            Self::And => (3, 4),
            Self::Eq | Self::Ne => (5, 6),
            Self::Lt | Self::Le | Self::Gt | Self::Ge => (7, 8),
            Self::Add | Self::Sub => (9, 10),
            Self::Mul | Self::Div | Self::Mod => (11, 12),
        }
    }

    fn to_tokens(self) -> TokenStream {
        match self {
            Self::Add => quote! { ::xtera::ast::BinaryOp::Add },
            Self::Sub => quote! { ::xtera::ast::BinaryOp::Sub },
            Self::Mul => quote! { ::xtera::ast::BinaryOp::Mul },
            Self::Div => quote! { ::xtera::ast::BinaryOp::Div },
            Self::Mod => quote! { ::xtera::ast::BinaryOp::Mod },
            Self::Eq => quote! { ::xtera::ast::BinaryOp::Eq },
            Self::Ne => quote! { ::xtera::ast::BinaryOp::Ne },
            Self::Lt => quote! { ::xtera::ast::BinaryOp::Lt },
            Self::Le => quote! { ::xtera::ast::BinaryOp::Le },
            Self::Gt => quote! { ::xtera::ast::BinaryOp::Gt },
            Self::Ge => quote! { ::xtera::ast::BinaryOp::Ge },
            Self::And => quote! { ::xtera::ast::BinaryOp::And },
            Self::Or => quote! { ::xtera::ast::BinaryOp::Or },
        }
    }
}

/// Shorthand for the zero span used in generated code.
/// Relies on `src` being in scope in the generated code block.
fn s() -> TokenStream {
    quote! { ::xtera::ast::Span::new(0, 0, src.clone()) }
}

/// Parse the macro input and produce a `TokenStream` that constructs
/// an `xtera::Template`.
pub fn parse(input: TokenStream) -> syn::Result<TokenStream> {
    let source_str = input.to_string();
    let nodes = syn::parse2::<Nodes>(input)?;
    let node_tokens = nodes.0;
    let span = s();
    Ok(quote! {
        {
            let src: ::std::sync::Arc<str> = ::std::sync::Arc::from(#source_str);
            ::xtera::Template::new(
                src.clone(),
                ::xtera::ast::BlockNode {
                    nodes: vec![#(#node_tokens),*],
                    span: #span,
                },
            )
        }
    })
}

/// Wrapper for parsing a list of nodes from a TokenStream.
struct Nodes(Vec<TokenStream>);

impl syn::parse::Parse for Nodes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Nodes(parse_nodes(input)?))
    }
}

// ── Node parsing ────────────────────────────────────────────────────

fn parse_nodes(input: ParseStream) -> syn::Result<Vec<TokenStream>> {
    let mut nodes = Vec::new();
    while !input.is_empty() {
        nodes.push(parse_node(input)?);
    }
    Ok(nodes)
}

fn parse_node(input: ParseStream) -> syn::Result<TokenStream> {
    if input.peek(syn::LitStr) {
        parse_text_node(input)
    } else if input.peek(syn::token::Brace) {
        parse_interp_node(input)
    } else if input.peek(Token![@]) {
        parse_directive(input)
    } else {
        Err(input.error("expected string literal, {{ expr }}, or @directive"))
    }
}

fn parse_text_node(input: ParseStream) -> syn::Result<TokenStream> {
    let lit: syn::LitStr = input.parse()?;
    let text = lit.value();
    let span = s();
    Ok(quote! {
        ::xtera::ast::Node::Text(::xtera::ast::TextNode {
            text: #text.into(),
            span: #span,
        })
    })
}

fn parse_interp_node(input: ParseStream) -> syn::Result<TokenStream> {
    let outer_content;
    syn::braced!(outer_content in input);

    if !outer_content.peek(syn::token::Brace) {
        return Err(outer_content.error("expected {{ expr }}, not bare { }"));
    }

    let inner_content;
    syn::braced!(inner_content in outer_content);
    let expr = parse_expr(&inner_content)?;

    if !inner_content.is_empty() {
        return Err(inner_content.error("unexpected tokens in interpolation"));
    }
    if !outer_content.is_empty() {
        return Err(outer_content.error("unexpected tokens after {{ expr }}"));
    }

    let span = s();
    Ok(quote! {
        ::xtera::ast::Node::Interp(::xtera::ast::InterpNode {
            expr: #expr,
            span: #span,
        })
    })
}

fn parse_directive(input: ParseStream) -> syn::Result<TokenStream> {
    input.parse::<Token![@]>()?;

    let lookahead = input.lookahead1();
    if lookahead.peek(Token![if]) {
        input.parse::<Token![if]>()?;
        parse_if_node(input)
    } else if lookahead.peek(Token![for]) {
        input.parse::<Token![for]>()?;
        parse_for_node(input)
    } else if lookahead.peek(Token![match]) {
        input.parse::<Token![match]>()?;
        parse_match_node(input)
    } else if lookahead.peek(syn::Ident) {
        let ident: syn::Ident = input.parse()?;
        if ident == "include" {
            parse_include_node(input)
        } else {
            Err(syn::Error::new(
                ident.span(),
                format!("unknown directive: @{}", ident),
            ))
        }
    } else {
        Err(lookahead.error())
    }
}

fn parse_if_node(input: ParseStream) -> syn::Result<TokenStream> {
    let cond_content;
    syn::parenthesized!(cond_content in input);
    let condition = parse_expr(&cond_content)?;
    let body = parse_block_body(input)?;
    let span = s();

    let mut branches = vec![quote! {
        ::xtera::ast::IfBranch {
            condition: #condition,
            body: #body,
            span: #span,
        }
    }];
    let mut else_body = quote! { None };

    while input.peek(Token![@]) && input.peek2(Token![else]) {
        input.parse::<Token![@]>()?;
        input.parse::<Token![else]>()?;

        if input.peek(Token![if]) {
            input.parse::<Token![if]>()?;
            let cond_content;
            syn::parenthesized!(cond_content in input);
            let condition = parse_expr(&cond_content)?;
            let body = parse_block_body(input)?;
            branches.push(quote! {
                ::xtera::ast::IfBranch {
                    condition: #condition,
                    body: #body,
                    span: #span,
                }
            });
        } else {
            let body = parse_block_body(input)?;
            else_body = quote! { Some(#body) };
            break;
        }
    }

    Ok(quote! {
        ::xtera::ast::Node::If(::xtera::ast::IfNode {
            branches: vec![#(#branches),*],
            else_body: #else_body,
            span: #span,
        })
    })
}

fn parse_for_node(input: ParseStream) -> syn::Result<TokenStream> {
    let header;
    syn::parenthesized!(header in input);

    let binding: syn::Ident = header.parse()?;
    let binding_str = binding.to_string();

    let of_kw: syn::Ident = header.parse()?;
    if of_kw != "of" {
        return Err(syn::Error::new(of_kw.span(), "expected 'of'"));
    }

    let iterable = parse_expr(&header)?;
    header.parse::<Token![;]>()?;

    let track_kw: syn::Ident = header.parse()?;
    if track_kw != "track" {
        return Err(syn::Error::new(track_kw.span(), "expected 'track'"));
    }

    let track = parse_expr(&header)?;
    let body = parse_block_body(input)?;
    let span = s();

    Ok(quote! {
        ::xtera::ast::Node::For(::xtera::ast::ForNode {
            binding: #binding_str.into(),
            iterable: #iterable,
            track: #track,
            body: #body,
            span: #span,
        })
    })
}

fn parse_match_node(input: ParseStream) -> syn::Result<TokenStream> {
    let expr_content;
    syn::parenthesized!(expr_content in input);
    let expr = parse_expr(&expr_content)?;

    let arms_content;
    syn::braced!(arms_content in input);

    let mut arms = Vec::new();
    let mut default_body = quote! { None };
    let span = s();

    while !arms_content.is_empty() {
        if arms_content.peek(Token![_]) {
            arms_content.parse::<Token![_]>()?;
            arms_content.parse::<Token![=>]>()?;
            let body = parse_block_body(&arms_content)?;
            default_body = quote! { Some(#body) };
        } else {
            let pattern = parse_expr(&arms_content)?;
            arms_content.parse::<Token![=>]>()?;
            let body = parse_block_body(&arms_content)?;
            arms.push(quote! {
                ::xtera::ast::MatchNodeArm {
                    pattern: #pattern,
                    body: #body,
                    span: #span,
                }
            });
        }

        if arms_content.peek(Token![,]) {
            arms_content.parse::<Token![,]>()?;
        }
    }

    Ok(quote! {
        ::xtera::ast::Node::Match(::xtera::ast::MatchNode {
            expr: #expr,
            arms: vec![#(#arms),*],
            default: #default_body,
            span: #span,
        })
    })
}

fn parse_include_node(input: ParseStream) -> syn::Result<TokenStream> {
    let content;
    syn::parenthesized!(content in input);
    let name_expr = parse_expr(&content)?;
    let span = s();
    Ok(quote! {
        ::xtera::ast::Node::Include(::xtera::ast::IncludeNode {
            name: #name_expr,
            span: #span,
        })
    })
}

fn parse_block_body(input: ParseStream) -> syn::Result<TokenStream> {
    let content;
    syn::braced!(content in input);
    let nodes = parse_nodes(&content)?;
    let span = s();
    Ok(quote! {
        ::xtera::ast::BlockNode {
            nodes: vec![#(#nodes),*],
            span: #span,
        }
    })
}

// ── Expression parsing (Pratt precedence climbing) ──────────────────

fn parse_expr(input: ParseStream) -> syn::Result<TokenStream> {
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
        let op = peek_binary_op(input);
        let op = match op {
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
    // Compound operators first (order matters for disambiguation)
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
        parse_expr(&content)
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
