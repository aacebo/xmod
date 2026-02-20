mod expr;
mod for_node;
mod if_node;
mod include_node;
mod match_node;
mod node;

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::ParseStream;

#[derive(Clone, Copy)]
pub(crate) enum BinOp {
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
    pub fn precedence(self) -> (u8, u8) {
        match self {
            Self::Or => (1, 2),
            Self::And => (3, 4),
            Self::Eq | Self::Ne => (5, 6),
            Self::Lt | Self::Le | Self::Gt | Self::Ge => (7, 8),
            Self::Add | Self::Sub => (9, 10),
            Self::Mul | Self::Div | Self::Mod => (11, 12),
        }
    }

    pub fn to_tokens(self) -> TokenStream {
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

pub(crate) fn s() -> TokenStream {
    quote! { ::xtera::ast::Span::new(0, 0, src.clone()) }
}

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

struct Nodes(Vec<TokenStream>);

impl syn::parse::Parse for Nodes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Nodes(node::parse_nodes(input)?))
    }
}
