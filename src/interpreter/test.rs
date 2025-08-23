//! Provides useful utilities for testing [`Interpreter`].

/// Defines a multi-case test for [`Interpreter`].
macro_rules! make_interpreter_test {
    (
        $name:ident,
        $((
            $case: ident,
            { $($alias_spec: tt)* },
            $block: tt,
            $expected: tt,
            $expected_err_type: expr$(,)?
        )),+$(,)?
    ) => {
        #[rstest::rstest]
        $(#[case::$case(
            syn::parse_quote!($($alias_spec)*),
            syn::parse_quote!($block),
            syn::parse_quote!($expected),
            $expected_err_type,
        )])*
        fn $name(
            #[case] alias_spec_tokens: proc_macro2::TokenStream,
            #[case] block_tokens: proc_macro2::TokenStream,
            #[case] expected_tokens: proc_macro2::TokenStream,
            #[case] expected_err_type: Option<$crate::error::ErrorType>,
        ) -> syn::Result<()> {
            use std::rc::Rc;
            use $crate::interpreter::Interpreter;
            use $crate::util::deprecation::DeprecationService;
            use $crate::ast::ComposeIdentsArgs;
            use $crate::ast::AliasSpec;
            use $crate::core::Environment;

            let environment = Rc::new(Environment::new_initialized(1));
            Environment::maybe_set_global(environment.clone());

            let deprecation_service = DeprecationService::scoped();
            let interpreter = Interpreter::new(environment.clone(), deprecation_service);

            let alias_spec = syn::parse2::<AliasSpec>(alias_spec_tokens)?;
            let block = syn::parse2::<syn::Block>(block_tokens)?;
            let expected = syn::parse2::<syn::Block>(expected_tokens)?;

            let args = ComposeIdentsArgs::new(
                $crate::util::unique_id::next_unique_id(),
                std::rc::Rc::new(alias_spec),
                block,
            );

            let result = interpreter.execute(args);

            match (&result, expected_err_type) {
                (Err(err), Some(err_type)) if err.type_() == err_type => {
                    return Ok(());
                },
                _ => {}
            }

            assert!(result.is_ok(), "Interpreter execution failed: {:?}", result);

            let actual = result?;
            let expected_stmts = expected.stmts;
            let expected = quote::quote! { #(#expected_stmts)* };

            assert_eq!(actual.to_string(), expected.to_string());
            Ok(())
        }
    }
}

pub(crate) use make_interpreter_test;
