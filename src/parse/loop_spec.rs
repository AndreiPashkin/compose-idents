use crate::ast::{LoopSpec, LoopSpecItem};
use crate::util::unique_id::next_unique_id;
use syn::parse::{Parse, ParseStream};
use syn::Token;

impl Parse for LoopSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut loops: Vec<LoopSpecItem> = Vec::new();

        while input.peek(Token![for]) {
            let loop_spec: LoopSpecItem = input.parse()?;
            loops.push(loop_spec);
        }

        if loops.is_empty() {
            return Err(input.error("Failed to parse any loops"));
        }

        Ok(LoopSpec::new(next_unique_id(), loops))
    }
}
