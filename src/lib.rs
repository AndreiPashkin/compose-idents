use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input,
    visit_mut::VisitMut,
    Block, Ident, LitStr, Token,
};

struct IdentSpecItem {
    alias: Ident,
    parts: Vec<String>,
}

impl IdentSpecItem {
    fn replacement(&self) -> Ident {
        let ident = self
            .parts
            .iter()
            .fold("".to_string(), |acc, item| format!("{}{}", acc, item));
        format_ident!("{}", ident)
    }
}

struct IdentSpec {
    items: Vec<IdentSpecItem>,
}

impl Parse for IdentSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        loop {
            let alias: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let content;
            bracketed!(content in input);
            let mut parts = Vec::new();
            loop {
                if let Ok(part) = content.parse::<Ident>() {
                    parts.push(part.to_string());
                } else if content.parse::<Token![_]>().is_ok() {
                    parts.push("_".to_string());
                } else if let Ok(part) = content.parse::<LitStr>() {
                    parts.push(part.value());
                } else {
                    return Err(content.error("Expected identifier or _"));
                }
                if content.is_empty() {
                    break;
                }
                content.parse::<Token![,]>()?;
            }
            items.push(IdentSpecItem { alias, parts });
            input.parse::<Token![;]>()?;
            if !input.peek(Ident) {
                break;
            }
        }
        Ok(IdentSpec { items })
    }
}

impl IdentSpec {
    fn replacements(&self) -> HashMap<Ident, Ident> {
        self.items
            .iter()
            .map(|item| (item.alias.clone(), item.replacement()))
            .collect()
    }
}

struct ComposeIdentsArgs {
    spec: IdentSpec,
    block: Block,
}

impl Parse for ComposeIdentsArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let spec: IdentSpec = input.parse()?;
        let block: Block = input.parse()?;
        Ok(ComposeIdentsArgs { spec, block })
    }
}

struct ComposeIdentsVisitor {
    replacements: HashMap<Ident, Ident>,
}

impl VisitMut for ComposeIdentsVisitor {
    fn visit_ident_mut(&mut self, ident: &mut Ident) {
        if let Some(replacement) = self.replacements.get(ident) {
            *ident = replacement.clone();
        }
    }
}

/// Compose identifiers from the provided parts and replace their aliases in the code block.
///
/// # Example
///
/// ```rust
/// use compose_idents::compose_idents;
///
/// compose_idents!(my_fn_1 = [foo, _, "baz"]; my_fn_2 = [spam, _, eggs]; {
///     fn my_fn_1() -> u32 {
///         111
///     }
///
///     fn my_fn_2() -> u32 {
///         999
///     }
/// });
///
/// assert_eq!(foo_baz(), 111);
/// assert_eq!(spam_eggs(), 999);
/// ```
#[proc_macro]
pub fn compose_idents(input: TokenStream) -> TokenStream {
    // Parse the input into our `Assignments` structure
    let args = parse_macro_input!(input as ComposeIdentsArgs);
    let mut visitor = ComposeIdentsVisitor {
        replacements: args.spec.replacements(),
    };
    let mut block = args.block;
    visitor.visit_block_mut(&mut block);

    let block_content = block.stmts;

    let expanded = quote! { #(#block_content)* };
    TokenStream::from(expanded)
}
