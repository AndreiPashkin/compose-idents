//! Provides [`Combined`] type and [`combine`] macro for combining AST types.
use crate::error::combine_errors;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};

/// Combines two syntactic elements into a single one and enables parsing them speculatively.
///
/// Useful for parsing multiple alternative kinds of terminators in one go.
pub enum Combined<A, B>
where
    A: Parse,
    B: Parse,
{
    A(A),
    B(B),
}

impl<A, B> Parse for Combined<A, B>
where
    A: Parse,
    B: Parse,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let mut errors = Vec::new();
        let fork = input.fork();
        match fork.parse::<A>() {
            Ok(a) => {
                input.advance_to(&fork);
                return Ok(Self::A(a));
            }
            Err(err) => errors.push(err),
        }

        let fork = input.fork();
        match fork.parse::<B>() {
            Ok(b) => {
                input.advance_to(&fork);
                return Ok(Self::B(b));
            }
            Err(err) => errors.push(err),
        }

        Err(combine_errors(
            "Unable to parse any of the combined tokens (see the errors below)",
            span,
            errors,
        ))
    }
}

/// Combines tokens into a single one that has a speculative [`Parse`] implemented for it.
macro_rules! combine {
    ($A:ty, $B:ty) => {
        $crate::util::combined::Combined::<$A, $B>
    };
    ($A:ty, $B:ty $(, $tail:ty)+) => {
        $crate::util::combined::Combined::<$A, combine!($B $(, $tail)+)>
    };
}

pub(crate) use combine;
