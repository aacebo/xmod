use proc_macro2::TokenStream;
use quote::quote;
use syn::Token;
use syn::parse::ParseStream;

use super::expr::parse_expr;
use super::node::parse_block_body;
use super::s;

pub(super) fn parse(input: ParseStream) -> syn::Result<TokenStream> {
    let expr_content;
    syn::parenthesized!(expr_content in input);
    let expr = parse_expr(&expr_content)?;

    let arms_content;
    syn::braced!(arms_content in input);

    let mut arms = Vec::new();
    let mut default_body = quote! { None };
    let mut has_default = false;
    let span = s();

    while !arms_content.is_empty() {
        if arms_content.peek(Token![_]) {
            if has_default {
                return Err(arms_content.error("duplicate default arm '_'"));
            }

            arms_content.parse::<Token![_]>()?;
            arms_content.parse::<Token![=>]>()?;
            let body = parse_block_body(&arms_content)?;
            default_body = quote! { Some(#body) };
            has_default = true;
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
