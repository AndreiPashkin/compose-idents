//! Tests for substitutions within different syntactic constructs.

use crate::interpreter::test::make_interpreter_test;

make_interpreter_test!(
    non_terminals,
    // Function name substitution.
    (
        function_name,
        { alias = foo },
        {
            fn alias() {}
        },
        {
            fn foo() {}
        },
        None,
    ),
    // Module name substitution.
    (
        module_name,
        { alias = foo },
        {
            mod alias {
                pub fn my_fn() {}
            }
        },
        {
            mod foo {
                pub fn my_fn() {}
            }
        },
        None,
    ),
    // Function name within a module.
    (
        function_in_module,
        { alias = foo },
        {
            mod my_mod {
                fn alias() {}
            }
        },
        {
            mod my_mod {
                fn foo() {}
            }
        },
        None,
    ),
    // Generic type parameter name and usage.
    (
        generic_param_type,
        { alias = Type },
        {
            fn my_fn<alias>(x: alias) {}
        },
        {
            fn my_fn<Type>(x: Type) {}
        },
        None,
    ),
    // Const generic parameter and usage in an array length.
    (
        const_generic_param,
        { alias = N },
        {
            fn my_array<const alias: usize>() {
                let _: [u8; alias];
            }
        },
        {
            fn my_array<const N: usize>() {
                let _: [u8; N];
            }
        },
        None,
    ),
    // Struct name substitution.
    (
        struct_name,
        { alias = Foo },
        {
            struct alias;
        },
        {
            struct Foo;
        },
        None,
    ),
    // Enum variant name substitution.
    (
        enum_variant_name,
        { alias = Variant },
        {
            enum E {
                alias,
            }
        },
        {
            enum E {
                Variant,
            }
        },
        None,
    ),
    // Trait name substitution.
    (
        trait_name,
        { alias = MyTrait },
        {
            trait alias {}
        },
        {
            trait MyTrait {}
        },
        None,
    ),
    // Module path segment substitution in a use path.
    (
        path_segment_in_use,
        { alias = Bar },
        {
            use foo::alias::baz;
        },
        {
            use foo::Bar::baz;
        },
        None,
    ),
    // Where-clause trait path substitution.
    (
        where_clause_trait,
        { alias = Iterator },
        {
            fn my_fn<T>()
            where
                T: alias<Item = u8>,
            {
            }
        },
        {
            fn my_fn<T>()
            where
                T: Iterator<Item = u8>,
            {
            }
        },
        None,
    ),
    // Lifetime parameter and reference substitution.
    (
        lifetime_param_and_ref,
        { alias = a },
        {
            fn my_fn<'alias>(x: &'alias u8) {}
        },
        {
            fn my_fn<'a>(x: &'a u8) {}
        },
        None,
    ),
    // Substitution within a generic const expression (array length).
    (
        const_generic_expr_in_type,
        { alias = N },
        {
            fn my_fn<const alias: usize>() {
                let _: [u8; alias + 1];
            }
        },
        {
            fn my_fn<const N: usize>() {
                let _: [u8; N + 1];
            }
        },
        None,
    ),
    // Associated type bound name substitution in where-clause.
    (
        where_clause_assoc_type_name,
        { alias = Item },
        {
            fn my_fn<T>()
            where
                T: Iterator<alias = u8>,
            {
            }
        },
        {
            fn my_fn<T>()
            where
                T: Iterator<Item = u8>,
            {
            }
        },
        None,
    ),
);
