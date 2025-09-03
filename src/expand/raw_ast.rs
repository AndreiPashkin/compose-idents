use crate::ast::{
    Alias, AliasSpec, AliasSpecItem, AliasValue, Ast, Expr, LoopAlias, LoopSourceValue, RawAST,
    Tuple,
};
use crate::ast::{BlockRewrite, ExpandedAST};
use crate::error::Error;
use crate::expand::Expand;
use crate::util::cross_product::cross_product;
use crate::util::unique_id::next_unique_id;
use std::rc::Rc;

/// Validates that alias tuple and value tuple have compatible shapes for destructuring.
fn match_tuples(alias_tuple: &Tuple<Alias>, expr_tuple: &Tuple<Expr>) -> Result<(), Error> {
    use crate::ast::TupleValueKind;
    if alias_tuple.iter_destructuring().count() != expr_tuple.iter_destructuring().count() {
        return Err(Error::TypeError(
            "Mismatched number of elements in the tuple".to_string(),
            expr_tuple.span(),
        ));
    }
    for (alias_kind, expr_kind) in alias_tuple
        .iter_destructuring()
        .zip(expr_tuple.iter_destructuring())
    {
        match (alias_kind, expr_kind) {
            (TupleValueKind::Value(_), TupleValueKind::Value(_)) => {}
            (TupleValueKind::Tuple(_), TupleValueKind::Tuple(_)) => {}
            _ => {
                return Err(Error::TypeError(
                    "Shape of the value tuple doesn't match the shape of the alias tuple"
                        .to_string(),
                    expr_tuple.span(),
                ));
            }
        }
    }
    Ok(())
}

/// Builds a sequence of [`AliasSpecItem`] loop-aliases and loop-values.
fn make_spec_items(
    alias: &LoopAlias,
    source_value: &LoopSourceValue,
) -> Result<Vec<Rc<AliasSpecItem>>, Error> {
    let items = match (alias, source_value) {
        (LoopAlias::Simple(alias), LoopSourceValue::Value(expr)) => {
            let val = Rc::new(AliasValue::new(next_unique_id(), expr.clone(), expr.span()));
            vec![Rc::new(AliasSpecItem::new(
                next_unique_id(),
                alias.clone(),
                val,
            ))]
        }
        (LoopAlias::Tuple(alias_tuple), LoopSourceValue::Tuple(expr_tuple)) => {
            match_tuples(alias_tuple, expr_tuple)?;
            alias_tuple
                .iter_recursive()
                .zip(expr_tuple.iter_recursive())
                .map(|(a, e)| {
                    let value = Rc::new(AliasValue::new(next_unique_id(), e.clone(), e.span()));
                    Rc::new(AliasSpecItem::new(next_unique_id(), a.clone(), value))
                })
                .collect()
        }
        _ => {
            return Err(Error::TypeError(
                "Mismatched alias and value types".to_string(),
                source_value.span(),
            ))
        }
    };
    Ok(items)
}

impl Expand for RawAST {
    type Expanded = ExpandedAST;

    fn expand(&self) -> Result<Self::Expanded, Error> {
        let loops = match self.loops() {
            // No loops
            None => {
                let spec = self
                    .spec()
                    .unwrap_or_else(|| Rc::new(AliasSpec::new(next_unique_id(), vec![], None)));
                let block_rewrite = BlockRewrite::new(spec, self.block().clone());
                return Ok(ExpandedAST::new(next_unique_id(), vec![block_rewrite]));
            }
            Some(loops) => loops,
        };

        // Gather per-loop lists as owned clones to use the cross-product utility
        let per_loop_values: Vec<Vec<LoopSourceValue>> = loops
            .loops()
            .iter()
            .map(|item| item.list().values().to_vec())
            .collect();

        let mut block_rewrite_items: Vec<BlockRewrite> = Vec::new();

        for loop_values in cross_product(per_loop_values) {
            let mut spec_items: Vec<Rc<AliasSpecItem>> = Vec::new();
            for (item, value) in loops.loops().iter().zip(loop_values.iter()) {
                let alias = item.alias();
                let mut new_spec_items = make_spec_items(alias.as_ref(), value)?;
                spec_items.append(&mut new_spec_items);
            }

            if let Some(spec) = self.spec() {
                spec_items.extend(spec.items().iter().cloned());
                let spec = Rc::new(AliasSpec::new(
                    next_unique_id(),
                    spec_items,
                    spec.is_comma_used(),
                ));
                block_rewrite_items.push(BlockRewrite::new(spec, self.block().clone()));
            } else {
                let spec = Rc::new(AliasSpec::new(next_unique_id(), spec_items, None));
                block_rewrite_items.push(BlockRewrite::new(spec, self.block().clone()));
            }
        }

        Ok(ExpandedAST::new(next_unique_id(), block_rewrite_items))
    }
}
