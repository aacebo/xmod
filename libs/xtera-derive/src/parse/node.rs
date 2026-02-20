use proc_macro2::TokenStream;
use quote::quote;
use syn::Token;
use syn::parse::ParseStream;

use super::expr::parse_expr;
use super::s;

pub(super) fn parse_nodes(input: ParseStream) -> syn::Result<Vec<TokenStream>> {
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
        super::if_node::parse(input)
    } else if lookahead.peek(Token![for]) {
        input.parse::<Token![for]>()?;
        super::for_node::parse(input)
    } else if lookahead.peek(Token![match]) {
        input.parse::<Token![match]>()?;
        super::match_node::parse(input)
    } else if lookahead.peek(syn::Ident) {
        let ident: syn::Ident = input.parse()?;
        if ident == "include" {
            super::include_node::parse(input)
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

pub(super) fn parse_block_body(input: ParseStream) -> syn::Result<TokenStream> {
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
