use crate::ast::{Ast, Call, Expr, ExprKind, ValueKind};
use crate::core::{Environment, Func, Type};
use crate::error::{internal_error, Error};
use crate::resolve::{Resolve, Scope};
use crate::util::log::debug;
use std::rc::Rc;

fn resolve_arg(
    arg: Rc<Expr>,
    coercion_cost_multiplier: u32,
    resolved_coercion_cost: &mut u32,
    resolved_args: &mut Vec<Rc<Expr>>,
    environment: &Environment,
    scope: &mut Scope,
    expected_type: Option<&Type>,
) -> Result<(), Error> {
    debug!(
        "Resolving argument \"{:?}\" with expected type \"{:?}\"...",
        arg, expected_type
    );

    arg.resolve(environment, scope, expected_type)?;
    let metadata = scope.metadata_mut();

    let arg_coercion_cost = match arg.kind() {
        ExprKind::ValueExpr(value) => {
            let Some(value_metadata) = metadata.get_value_metadata(value.id()) else {
                return Err(internal_error!(
                    "Value metadata is expected be set after resolve call"
                ));
            };
            value_metadata.coercion_cost
        }
        ExprKind::FuncCallExpr(call) => {
            let Some(call_metadata) = metadata.get_call_metadata(call.id()) else {
                return Err(internal_error!(
                    "Call metadata is expected be set after resolve call"
                ));
            };
            call_metadata.coercion_cost
        }
    };
    *resolved_coercion_cost += arg_coercion_cost * coercion_cost_multiplier;
    resolved_args.push(arg.clone());

    debug!("Successfully resolved argument.");
    Ok(())
}

/// Attempts to resolve a call for a particular function type.
///
/// Call resolutions involves:
///
///   - Checking call argument and function argument types for compatibility.
///   - Assigning coercion cost to the call.
///
/// Upon success returns a tuple containing the coercion cost and a vector of resolved arguments.
fn resolve_call_for_func(
    func: &Func,
    call: &Call,
    environment: &Environment,
    scope: &mut Scope,
    expected_type: Option<&Type>,
) -> Result<(u32, Vec<Rc<Expr>>), Error> {
    debug!(
        "Resolving call \"{:?}\" for function \"{:?}\" and expected type \"{:?}\"...",
        call, func, expected_type
    );

    let mut coercion_cost = 0;

    // Account for coercing the return value.
    if let Some(expected_type) = expected_type {
        match Type::coercion_cost(func.out_type(), expected_type) {
            Some(cost) => {
                coercion_cost += cost;
            }
            None => return Err(Error::make_coercion_error(func.out_type(), expected_type)),
        }
    }

    let raw_args = match (
        func.arg_types(),
        func.is_variadic(),
        call.raw_args(),
        call.raw_tokens(),
    ) {
        // Case of a function that takes raw tokens as the input. A single argument is always
        // expected.
        ([arg_type], _, _, Some(raw_arg)) if *arg_type == Type::Raw => {
            debug_assert!(matches!(
                raw_arg.kind(),
                ExprKind::ValueExpr(value) if matches!(value.kind(), ValueKind::Raw(_)),
            ));

            raw_arg.resolve(environment, scope, Some(&Type::Raw))?;
            return Ok((coercion_cost, vec![raw_arg.clone()]));
        }
        // Case of a non-variadic function - compare by the number of arguments.
        (arg_types, false, args, _) if arg_types.len() == args.len() => args,
        // Case of a variadic function - compare by the number of arguments, but allow for
        // extra arguments.
        (arg_types, true, args, _) if arg_types.len() <= args.len() => args,
        _ => return Err(Error::make_sig_error(func, call)),
    };

    let mut args: Vec<Rc<Expr>> = vec![];

    for (expected_type, arg) in func.non_variadic_arg_types().iter().zip(raw_args.iter()) {
        resolve_arg(
            arg.clone(),
            1,
            &mut coercion_cost,
            &mut args,
            environment,
            scope,
            Some(expected_type),
        )?;
    }

    if let Some(variadic_arg_type) = func.variadic_arg_type() {
        for arg in raw_args.iter().skip(func.non_variadic_arg_types().len()) {
            resolve_arg(
                arg.clone(),
                2,
                &mut coercion_cost,
                &mut args,
                environment,
                scope,
                Some(&variadic_arg_type),
            )?;
        }
    }

    Ok((coercion_cost, args))
}

type FuncResolutionMetadata = (u32, Scope, Rc<Func>, Vec<Rc<Expr>>);

impl Resolve for Call {
    /// Resolves a function call by resolving its arguments and binding the call to a built-in
    /// function.
    fn resolve(
        &self,
        environment: &Environment,
        scope: &mut Scope,
        expected_type: Option<&Type>,
    ) -> Result<(), Error> {
        let name = self.name().to_string();
        debug!("Resolving a call for function: {:?}...", self);

        let func_candidates = match environment.get_func_variants(name.as_str()) {
            Some(funcs) => funcs,
            None => {
                return Err(Error::UndefinedFunctionError(name.clone(), self.span()));
            }
        };

        debug!("Found {} candidate func-types...", func_candidates.len());
        let mut funcs: Vec<FuncResolutionMetadata> = vec![];

        for func in func_candidates {
            let mut scope_candidate = scope.deep_clone();
            match resolve_call_for_func(
                func,
                self,
                environment,
                &mut scope_candidate,
                expected_type,
            ) {
                Ok((coercion_cost, args)) => {
                    funcs.push((coercion_cost, scope_candidate, func.clone(), args))
                }
                Err(err @ Error::InternalError(_)) => return Err(err),
                Err(_) => continue,
            };
        }
        // Selecting the function with the lowest coercion cost using additional criteria for
        // tie-breaking.
        funcs.sort_by_key(|x| (x.0, x.2.is_variadic(), x.2.num_args(), x.2.id()));

        match funcs.into_iter().next() {
            Some((coercion_cost, scope_candidate, func, args)) => {
                scope_candidate.metadata_mut().set_call_metadata(
                    self.id(),
                    args,
                    func.clone(),
                    expected_type.unwrap_or(func.out_type()).clone(),
                    coercion_cost,
                );
                *scope = scope_candidate;
            }
            None => {
                let pretty_sig = environment
                    .make_pretty_func_sig(&name)
                    .unwrap_or_else(|| "unknown function".to_string());
                return Err(Error::SignatureError(
                    pretty_sig,
                    self.to_string(),
                    self.span(),
                ));
            }
        }

        Ok(())
    }
}
