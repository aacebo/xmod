mod parse;

use proc_macro::TokenStream;

/// Compile-time template macro that produces an `xtera::Template`.
///
/// # Syntax
///
/// ```ignore
/// render! {
///     "Hello, " {{ name | upper }} "!"
///     @if (x > 5) {
///         "Big: " {{ x }}
///     } @else {
///         "Small"
///     }
///     @for (item of items; track item) {
///         "- " {{ item }} "\n"
///     }
///     @match (color) {
///         "red" => { "R" },
///         "blue" => { "B" },
///         _ => { "?" }
///     }
///     @include("header")
/// }
/// ```
#[proc_macro]
pub fn render(input: TokenStream) -> TokenStream {
    let input2: proc_macro2::TokenStream = input.into();

    match parse::parse(input2) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
