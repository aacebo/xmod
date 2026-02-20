use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::ParseStream;

use super::expr::parse_expr;
use super::s;

pub(super) fn parse(input: ParseStream) -> syn::Result<TokenStream> {
    let content;
    syn::parenthesized!(content in input);
    let name_expr = parse_expr(&content)?;

    if !content.is_empty() {
        return Err(content.error("unexpected tokens in @include"));
    }

    let span = s();
    Ok(quote! {
        ::xtera::ast::Node::Include(::xtera::ast::IncludeNode {
            name: #name_expr,
            span: #span,
        })
    })
}
