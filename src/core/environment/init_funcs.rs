//! Provides [`Environment::init_funcs`] method that initializes all the func-types.

use crate::ast::{Value, ValueKind};
use crate::core::{Environment, Func, Type};
use crate::error::Error;
use crate::funcs::{
    concat, hash, lower, normalize, to_camel_case, to_expr, to_ident, to_int, to_pascal_case,
    to_path, to_snake_case, to_str, to_type, upper,
};
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use syn::{LitInt, LitStr};

macro_rules! arg_type_err {
    ($func:expr, $arg_type:expr) => {
        panic!(
            "Func {}(...) received a value of incompatible type: {:?}",
            $func.signature(),
            $arg_type,
        )
    };
}

/// Generates func-types ([`Func`] instances) for string manipulation functions.
macro_rules! make_str_funcs {
    ($name:expr, $func:expr) => {
        vec![
            Rc::new(Func::new(
                ($name.to_string()),
                vec![Type::LitStr],
                Type::LitStr,
                move |func, _, _, values| {
                    let kind = values.iter().map(|item| item.kind()).collect::<Vec<_>>();
                    let [ValueKind::LitStr(lit_str)] = kind.as_slice() else {
                        arg_type_err!(func, values);
                    };
                    let string = $func(lit_str.value().as_str());
                    let result = LitStr::new(string.as_str(), lit_str.span());
                    Ok(Value::from_lit_str(result))
                },
            )),
            Rc::new(Func::new(
                ($name.to_string()),
                vec![Type::Ident],
                Type::Ident,
                move |func, _, _, values| {
                    let kind = values.iter().map(|item| item.kind()).collect::<Vec<_>>();
                    let [ValueKind::Ident(ident)] = kind.as_slice() else {
                        arg_type_err!(func, values);
                    };
                    let ident =
                        Ident::new($func(ident.to_string().as_str()).as_str(), ident.span());
                    Ok(Value::from_ident(ident))
                },
            )),
        ]
    };
}

impl Environment {
    /// Initializes the function types.
    pub fn init_funcs() -> HashMap<String, Vec<Rc<Func>>> {
        let mut funcs = HashMap::new();
        funcs.insert(
            "upper".to_string(),
            make_str_funcs!("upper".to_string(), upper),
        );
        funcs.insert(
            "lower".to_string(),
            make_str_funcs!("lower".to_string(), lower),
        );
        funcs.insert(
            "snake_case".to_string(),
            make_str_funcs!("snake_case".to_string(), to_snake_case),
        );
        funcs.insert(
            "camel_case".to_string(),
            make_str_funcs!("camel_case".to_string(), to_camel_case),
        );
        funcs.insert(
            "pascal_case".to_string(),
            make_str_funcs!("pascal_case".to_string(), to_pascal_case),
        );
        funcs.insert(
            "normalize".to_string(),
            vec![Rc::new(Func::new(
                "normalize".to_string(),
                vec![Type::Raw],
                Type::Ident,
                |func, _, span, values| {
                    let kind = values.iter().map(|item| item.kind()).collect::<Vec<_>>();
                    let [ValueKind::Raw(tokens)] = kind.as_slice() else {
                        arg_type_err!(func, values);
                    };
                    let ident = Ident::new(normalize(tokens.to_string().as_str()).as_str(), *span);
                    Ok(Value::from_ident(ident))
                },
            ))],
        );
        funcs.insert(
            "hash".to_string(),
            vec![
                // hash(str)
                Rc::new(Func::new(
                    "hash".to_string(),
                    vec![Type::LitStr],
                    Type::LitStr,
                    |func, state, span, values| {
                        let kind = values.iter().map(|item| item.kind()).collect::<Vec<_>>();
                        let [ValueKind::LitStr(lit_str)] = kind.as_slice() else {
                            arg_type_err!(func, values);
                        };
                        let result = hash(lit_str.value().as_str(), state);
                        let lit_str = LitStr::new(result.as_str(), *span);

                        Ok(Value::from_lit_str(lit_str))
                    },
                )),
                // hash(ident)
                Rc::new(Func::new(
                    "hash".to_string(),
                    vec![Type::Ident],
                    Type::Ident,
                    |func, state, span, values| {
                        let kind = values.iter().map(|item| item.kind()).collect::<Vec<_>>();
                        let [ValueKind::Ident(ident)] = kind.as_slice() else {
                            arg_type_err!(func, values);
                        };
                        let string = ident.to_string();
                        let mut result = hash(string.as_str(), state);
                        result.insert_str(0, "__");

                        Ok(Value::from_ident(Ident::new(result.as_str(), *span)))
                    },
                )),
                // hash(tokens)
                Rc::new(Func::new(
                    "hash".to_string(),
                    vec![Type::Tokens],
                    Type::Ident,
                    |func, state, span, values| {
                        let kind = values.iter().map(|item| item.kind()).collect::<Vec<_>>();
                        let [ValueKind::Tokens(stream)] = kind.as_slice() else {
                            arg_type_err!(func, values);
                        };
                        let string = stream.to_string();
                        let mut result = hash(string.as_str(), state);
                        result.insert_str(0, "__");

                        Ok(Value::from_ident(Ident::new(result.as_str(), *span)))
                    },
                )),
            ],
        );
        funcs.insert(
            "concat".to_string(),
            vec![
                // concat(ident1, ident2, ...)
                Rc::new(Func::new(
                    "concat".to_string(),
                    vec![Type::Variadic(Box::new(Type::Ident))],
                    Type::Ident,
                    |func, _, span, values| {
                        let mut strings = Vec::new();
                        for value in values {
                            let ValueKind::Ident(ident) = value.kind() else {
                                arg_type_err!(func, values);
                            };
                            strings.push(ident.to_string());
                        }
                        let strs = strings.iter().map(|s| s.as_str()).collect::<Vec<_>>();
                        let result = concat(strs.as_slice());
                        let ident = Ident::new(result.as_str(), *span);

                        Ok(Value::from_ident(ident))
                    },
                )),
                // concat(ident, tokens...)
                Rc::new(Func::new(
                    "concat".to_string(),
                    vec![Type::Ident, Type::Variadic(Box::new(Type::Tokens))],
                    Type::Ident,
                    |func, _, span, values| {
                        let mut strings = Vec::new();
                        let Some(first) = values.first() else {
                            arg_type_err!(func, values);
                        };
                        strings.push(first.to_token_stream().to_string());
                        for value in values.iter().skip(1) {
                            let ValueKind::Tokens(tokens) = value.kind() else {
                                arg_type_err!(func, values);
                            };
                            strings.push(tokens.to_string());
                        }
                        let strs = strings.iter().map(|s| s.as_str()).collect::<Vec<_>>();
                        let result = concat(strs.as_slice());

                        let Ok(ident) = syn::parse_str::<Ident>(result.as_str()) else {
                            return Err(Error::EvalError(
                                format!(
                                    "Failed to produce a valid identifier \
                                    from concatenated arguments: {}",
                                    values
                                        .iter()
                                        .map(|v| v.to_token_stream().to_string())
                                        .collect::<Vec<_>>()
                                        .join(", "),
                                ),
                                *span,
                            ));
                        };

                        Ok(Value::from_ident(ident))
                    },
                )),
                // concat(str...)
                Rc::new(Func::new(
                    "concat".to_string(),
                    vec![Type::Variadic(Box::new(Type::LitStr))],
                    Type::LitStr,
                    |func, _, span, values| {
                        let mut strings = Vec::new();
                        for value in values {
                            let ValueKind::LitStr(lit_str) = value.kind() else {
                                arg_type_err!(func, values);
                            };
                            strings.push(lit_str.value());
                        }
                        let strs = strings.iter().map(|s| s.as_str()).collect::<Vec<_>>();
                        let result = concat(strs.as_slice());
                        let lit_str = LitStr::new(result.as_str(), *span);

                        Ok(Value::from_lit_str(lit_str))
                    },
                )),
                // concat(int...)
                Rc::new(Func::new(
                    "concat".to_string(),
                    vec![Type::Variadic(Box::new(Type::LitInt))],
                    Type::LitInt,
                    |func, _, span, values| {
                        let mut digits = Vec::new();
                        for value in values {
                            let ValueKind::LitInt(lit_int) = value.kind() else {
                                arg_type_err!(func, values);
                            };
                            digits.push(lit_int.base10_digits());
                        }
                        let result = concat(digits.as_slice());
                        let lit_int = LitInt::new(result.as_str(), *span);

                        Ok(Value::from_lit_int(lit_int))
                    },
                )),
                // concat(tokens1, tokens2, ...)
                Rc::new(Func::new(
                    "concat".to_string(),
                    vec![Type::Variadic(Box::new(Type::Tokens))],
                    Type::Tokens,
                    |func, _, _, values| {
                        let mut tokens = TokenStream::new();
                        for value in values {
                            let ValueKind::Tokens(stream) = value.kind() else {
                                arg_type_err!(func, values);
                            };
                            tokens.extend(stream.clone().into_iter());
                        }

                        Ok(Value::from_tokens(tokens))
                    },
                )),
            ],
        );
        // Casting functions
        funcs.insert(
            "to_ident".to_string(),
            vec![Rc::new(Func::new(
                "to_ident".to_string(),
                vec![Type::Tokens],
                Type::Ident,
                |func, _, _, values| {
                    let kind = values.iter().map(|v| v.kind()).collect::<Vec<_>>();
                    let [ValueKind::Tokens(_tokens)] = kind.as_slice() else {
                        arg_type_err!(func, values);
                    };
                    to_ident(values[0].as_ref())
                },
            ))],
        );
        funcs.insert(
            "to_path".to_string(),
            vec![Rc::new(Func::new(
                "to_path".to_string(),
                vec![Type::Tokens],
                Type::Path,
                |func, _, _, values| {
                    let kind = values.iter().map(|v| v.kind()).collect::<Vec<_>>();
                    let [ValueKind::Tokens(_tokens)] = kind.as_slice() else {
                        arg_type_err!(func, values);
                    };
                    to_path(values[0].as_ref())
                },
            ))],
        );
        funcs.insert(
            "to_type".to_string(),
            vec![Rc::new(Func::new(
                "to_type".to_string(),
                vec![Type::Tokens],
                Type::Type,
                |func, _, _, values| {
                    let kind = values.iter().map(|v| v.kind()).collect::<Vec<_>>();
                    let [ValueKind::Tokens(_tokens)] = kind.as_slice() else {
                        arg_type_err!(func, values);
                    };
                    to_type(values[0].as_ref())
                },
            ))],
        );
        funcs.insert(
            "to_expr".to_string(),
            vec![Rc::new(Func::new(
                "to_expr".to_string(),
                vec![Type::Tokens],
                Type::Expr,
                |func, _, _, values| {
                    let kind = values.iter().map(|v| v.kind()).collect::<Vec<_>>();
                    let [ValueKind::Tokens(_tokens)] = kind.as_slice() else {
                        arg_type_err!(func, values);
                    };
                    to_expr(values[0].as_ref())
                },
            ))],
        );
        funcs.insert(
            "to_str".to_string(),
            vec![Rc::new(Func::new(
                "to_str".to_string(),
                vec![Type::Tokens],
                Type::LitStr,
                |func, _, _, values| {
                    let kind = values.iter().map(|v| v.kind()).collect::<Vec<_>>();
                    let [ValueKind::Tokens(_tokens)] = kind.as_slice() else {
                        arg_type_err!(func, values);
                    };
                    to_str(values[0].as_ref())
                },
            ))],
        );
        funcs.insert(
            "to_int".to_string(),
            vec![Rc::new(Func::new(
                "to_int".to_string(),
                vec![Type::Tokens],
                Type::LitInt,
                |func, _, _, values| {
                    let kind = values.iter().map(|v| v.kind()).collect::<Vec<_>>();
                    let [ValueKind::Tokens(_tokens)] = kind.as_slice() else {
                        arg_type_err!(func, values);
                    };
                    to_int(values[0].as_ref())
                },
            ))],
        );
        funcs.insert(
            "to_tokens".to_string(),
            vec![Rc::new(Func::new(
                "to_tokens".to_string(),
                vec![Type::Tokens],
                Type::Tokens,
                |func, _, _, values| {
                    let kind = values.iter().map(|v| v.kind()).collect::<Vec<_>>();
                    let [ValueKind::Tokens(_tokens)] = kind.as_slice() else {
                        arg_type_err!(func, values);
                    };
                    Ok(values[0].deref().clone())
                },
            ))],
        );
        funcs.insert(
            "raw".to_string(),
            vec![Rc::new(Func::new(
                "raw".to_string(),
                vec![Type::Raw],
                Type::Tokens,
                |func, _, _, values| {
                    let kind = values.iter().map(|v| v.kind()).collect::<Vec<_>>();
                    let [ValueKind::Raw(tokens)] = kind.as_slice() else {
                        arg_type_err!(func, values);
                    };
                    Ok(Value::from_tokens(tokens.clone()))
                },
            ))],
        );
        funcs
    }
}
