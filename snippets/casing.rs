use compose_idents::compose;

compose!(
    MY_SNAKE_CASE_STATIC = snake_case(snakeCase),
    MY_CAMEL_CASE_STATIC = camel_case(camel_case),
    MY_PASCAL_CASE_STATIC = pascal_case(concat(pascal, _, case)),
    {
        static MY_SNAKE_CASE_STATIC: u32 = 1;
        static MY_CAMEL_CASE_STATIC: u32 = 2;
        static MY_PASCAL_CASE_STATIC: u32 = 3;
    },
);

assert_eq!(snake_case, 1);
assert_eq!(camelCase, 2);
assert_eq!(PascalCase, 3);
