use proc_macro2::TokenStream;
use quote::quote;
use syn::Token;
use syn::parse::ParseStream;

use super::expr::parse_expr;
use super::node::parse_block_body;
use super::s;

pub(super) fn parse(input: ParseStream) -> syn::Result<TokenStream> {
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
