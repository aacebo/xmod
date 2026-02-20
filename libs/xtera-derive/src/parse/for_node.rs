use proc_macro2::TokenStream;
use quote::quote;
use syn::Token;
use syn::parse::ParseStream;

use super::expr::parse_expr;
use super::node::parse_block_body;
use super::s;

pub(super) fn parse(input: ParseStream) -> syn::Result<TokenStream> {
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

    if !header.is_empty() {
        return Err(header.error("unexpected tokens after track expression"));
    }

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
