//! A generic token-level visitor that allows to recursively traverse a [`TokenStream`], and
//! replace arbitrary tokens with arbitrary sequences of tokens by invoking methods of a
//! user-provided [`StreamVisitor`].
//!
//! # Notes
//!
//! - The main feature of the visitor is that it can replace a single token with a sequence of
//!   multiple tokens.
//! - And also is able to do so recursively.
//! - This module exists for the sole purpose of serving as an internal component for
//!   [`AliasSubstitutionVisitor`]. The decision has been made to factor out the mechanism of
//!   recursive [`TokenStream`] traversal into a separate module to avoid having complex
//!   and ad-hoc stack-management interleaved with the logic of alias substitution.

use crate::error::Error;
use crate::util::log::debug;
use crate::util::token_stream::{TokenStreamExt, TokenVecExt};
use proc_macro2::{Group, Ident, Literal, Punct, TokenStream, TokenTree};
use std::collections::VecDeque;

/// Drives the traversal process, invokes hooks defined by user-implemented [`StreamVisitor`]
/// instance.
///
/// Operates recursively on token-level.
pub struct StreamWalker<'a, V: StreamVisitor> {
    visitor: &'a mut V,
}

impl<'a, V: StreamVisitor> StreamWalker<'a, V> {
    pub fn new(visitor: &'a mut V) -> Self {
        Self { visitor }
    }

    /// Walks recursively through the provided [`TokenStream`] and invokes methods of the
    /// user-defined visitor.
    pub fn walk(&mut self, stream: TokenStream) -> Result<TokenStream, Error> {
        debug!("Walking a stream: \"{}\"", stream);

        let mut stack = VecDeque::<(usize, Vec<TokenTree>)>::new();
        stack.push_back((0, stream.into_vec()));

        let mut ctx = VisitorCtx::new(stack);

        loop {
            match (
                ctx.stack().len(),
                ctx.current_group_len(),
                ctx.is_current_group_exhausted(),
                ctx.current_token(),
            ) {
                // Stack is supposed to be non-empty when main loop iterates
                (0, _, _, _) => {
                    unreachable!()
                }
                // All tokens have been processed
                (1, _, Some(true), _) => {
                    return match self
                        .visitor
                        .exit_group_mut(&ctx, ctx.current_group().unwrap())?
                    {
                        StreamVisitorAction::Continue => {
                            Ok(ctx.current_group().unwrap().to_owned().into_token_stream())
                        }
                        StreamVisitorAction::Skip => {
                            ctx.pop_remove();
                            Ok(TokenStream::new())
                        }
                        StreamVisitorAction::Replace(new_stream) => {
                            debug!(
                                "Replacing a group: \"{}\", with \"{}\"",
                                ctx.current_group().unwrap().to_token_stream(),
                                new_stream,
                            );
                            ctx.replace_current_group(new_stream);
                            self.visitor.after_replace_mut(&ctx)?;
                            Ok(ctx.current_group().unwrap().to_token_stream())
                        }
                    };
                }
                // Current group is exhausted
                (_, _, Some(true), _) => {
                    match self
                        .visitor
                        .exit_group_mut(&ctx, ctx.current_group().unwrap())?
                    {
                        StreamVisitorAction::Continue => {
                            ctx.pop_fold();
                            ctx.advance_current_group();
                        }
                        StreamVisitorAction::Skip => {
                            ctx.pop_remove();
                        }
                        StreamVisitorAction::Replace(new_stream) => {
                            debug!(
                                "Replacing a group: \"{}\", with \"{}\"",
                                ctx.current_group().unwrap().to_token_stream(),
                                new_stream,
                            );
                            ctx.replace_current_group(new_stream);
                            ctx.pop_fold();
                            ctx.advance_current_group();
                            self.visitor.after_replace_mut(&ctx)?;
                        }
                    };
                }
                // A new group encountered
                (_, _, _, Some(TokenTree::Group(group))) => {
                    match self.visitor.visit_group_mut(&ctx, group)? {
                        StreamVisitorAction::Continue => {
                            ctx.push_group(group.stream().to_vec());

                            match self
                                .visitor
                                .enter_group_mut(&ctx, ctx.current_group().unwrap())?
                            {
                                StreamVisitorAction::Continue => {}
                                StreamVisitorAction::Skip => {
                                    ctx.pop_remove();
                                }
                                StreamVisitorAction::Replace(new_stream) => {
                                    debug!(
                                        "Replacing a group: {}, with {}",
                                        ctx.current_group().unwrap().to_token_stream(),
                                        new_stream,
                                    );
                                    ctx.replace_current_group(new_stream);
                                    self.visitor.after_replace_mut(&ctx)?;
                                }
                            }
                        }
                        StreamVisitorAction::Skip => {
                            ctx.remove_current_token();
                        }
                        StreamVisitorAction::Replace(new_stream) if !new_stream.is_empty() => {
                            debug!(
                                "Replacing a token: \"{}\", with \"{}\", in \"{}\"",
                                ctx.current_token().unwrap(),
                                new_stream,
                                ctx.current_group().unwrap().to_token_stream(),
                            );
                            ctx.replace_current_token(new_stream);
                            ctx.advance_current_group();
                            self.visitor.after_replace_mut(&ctx)?;
                        }
                        StreamVisitorAction::Replace(new_stream) if new_stream.is_empty() => {
                            debug!(
                                "Replacing a token: \"{}\", with \"{}\", in \"{}\"",
                                ctx.current_token().unwrap(),
                                new_stream,
                                ctx.current_group().unwrap().to_token_stream(),
                            );
                            ctx.replace_current_token(new_stream);
                            self.visitor.after_replace_mut(&ctx)?;
                        }
                        _ => unreachable!(),
                    }
                }
                // A token encountered
                (_, _, _, Some(token)) => {
                    let action = match token {
                        TokenTree::Ident(ident) => self.visitor.visit_ident_mut(&ctx, ident)?,
                        TokenTree::Punct(punct) => self.visitor.visit_punct_mut(&ctx, punct)?,
                        TokenTree::Literal(literal) => {
                            self.visitor.visit_literal_mut(&ctx, literal)?
                        }
                        _ => unreachable!(),
                    };
                    match action {
                        StreamVisitorAction::Continue => {
                            ctx.advance_current_group();
                        }
                        StreamVisitorAction::Skip => {
                            ctx.remove_current_token();
                        }
                        StreamVisitorAction::Replace(new_stream) if !new_stream.is_empty() => {
                            debug!(
                                "Replacing a token: \"{}\", with \"{}\", in \"{}\"",
                                ctx.current_token().unwrap(),
                                new_stream,
                                ctx.current_group().unwrap().to_token_stream(),
                            );
                            ctx.replace_current_token(new_stream);
                            ctx.advance_current_group();
                            self.visitor.after_replace_mut(&ctx)?;
                        }
                        StreamVisitorAction::Replace(new_stream) if new_stream.is_empty() => {
                            debug!(
                                "Replacing a token: \"{}\", with \"{}\", in \"{}\"",
                                ctx.current_token().unwrap(),
                                new_stream,
                                ctx.current_group().unwrap().to_token_stream(),
                            );
                            ctx.replace_current_token(new_stream);
                            self.visitor.after_replace_mut(&ctx)?;
                        }
                        _ => unreachable!(),
                    };
                }
                _ => unreachable!(),
            };
        }
    }
}

/// Context that reflects the current state of traversal and provides useful helper-methods
/// to the visitor.
pub struct VisitorCtx {
    stack: VecDeque<(usize, Vec<TokenTree>)>,
}

impl VisitorCtx {
    pub fn new(stack: VecDeque<(usize, Vec<TokenTree>)>) -> Self {
        Self { stack }
    }

    fn stack(&self) -> &VecDeque<(usize, Vec<TokenTree>)> {
        &self.stack
    }

    fn current_group(&self) -> Option<&[TokenTree]> {
        self.stack.back().map(|(_, tokens)| tokens.as_slice())
    }
    fn current_group_len(&self) -> Option<usize> {
        self.stack.back().map(|(_, tokens)| tokens.len())
    }
    fn is_current_group_exhausted(&self) -> Option<bool> {
        self.stack.back().map(|(i, tokens)| *i >= tokens.len())
    }

    fn current_token(&self) -> Option<&TokenTree> {
        self.stack.back().and_then(|(i, tokens)| tokens.get(*i))
    }
    /// Replaces the current token with a sequence of tokens provided in form of a [`TokenStream`].
    ///
    /// # Notes
    ///
    /// - Places the cursor at the last token of the newly inserted sequence. If the developer
    ///   wants to advance past it - a dedicated method should be invoked.
    fn replace_current_token(&mut self, stream: TokenStream) {
        if let Some((i, tokens)) = self.stack.back_mut() {
            let stream_vec = stream.into_vec();
            let stream_len = stream_vec.len();
            tokens.splice(*i..=*i, stream_vec);
            *i = tokens.len().min(*i + (stream_len.saturating_sub(1)));
        }
    }
    /// Removes the current token from the current group
    fn remove_current_token(&mut self) {
        if let Some((i, tokens)) = self.stack.back_mut() {
            if *i < tokens.len() {
                tokens.remove(*i);
            }
        }
    }

    /// Replaces a group of tokens at index `i` in `parent_tokens` with the provided `group_tokens`.
    ///
    /// # Notes
    ///
    /// - Preserves the span of the original group token.
    fn fold_group(group_tokens: &[TokenTree], parent_tokens: &mut [TokenTree], i: usize) {
        let TokenTree::Group(original_group) = parent_tokens[i].clone() else {
            panic!(
                "Expected a group at index {}, found: {:?}",
                i, parent_tokens[i]
            );
        };
        let mut new_group = Group::new(
            original_group.delimiter(),
            group_tokens.iter().cloned().collect::<TokenStream>(),
        );
        new_group.set_span(original_group.span());
        parent_tokens[i] = TokenTree::Group(new_group);
    }
    /// Pops the current group from the stack and folds it into the parent group.
    ///
    /// # Notes
    ///
    /// - Does not advance the group's traversal. After this operation the cursor will be pointing
    ///   to the group token of the parent group.
    fn pop_fold(&mut self) {
        let group = self.stack.pop_back();
        if let Some((_, tokens)) = group {
            if let Some(parent) = self.stack.back_mut() {
                Self::fold_group(tokens.as_slice(), &mut parent.1, parent.0);
            }
        }
    }
    /// Pops the current group from the stack and removes it from the parent group.
    fn pop_remove(&mut self) {
        let group = self.stack.pop_back();
        if group.is_some() {
            if let Some(parent) = self.stack.back_mut() {
                parent.1.remove(parent.0);
            }
        }
    }
    /// Advances the current group's traversal to the next token.
    fn advance_current_group(&mut self) {
        if let Some((i, tokens)) = self.stack.back_mut() {
            *i = tokens.len().min(*i + 1);
        }
    }
    /// Replaces the entire contents of the current group.
    fn replace_current_group(&mut self, stream: TokenStream) {
        if let Some((_, tokens)) = self.stack.back_mut() {
            tokens.clear();
            tokens.extend(stream.into_vec());
        }
    }
    /// Pushes a new group.
    fn push_group(&mut self, group: Vec<TokenTree>) {
        self.stack.push_back((0, group));
    }

    /// Reconstructs the whole `TokenStream` from the nested groups at the current traversal state.
    pub fn current_stream(&self) -> TokenStream {
        match self.stack.len() {
            0 => return TokenStream::new(),
            1 => {
                let (_, tokens) = self.stack[0].clone();
                return tokens.into_iter().collect();
            }
            _ => {}
        };

        let mut iter = self.stack.iter().cloned().rev();

        let back = iter.next().unwrap().1;

        let folded = iter.fold(back, |acc, (i, mut token_vec)| {
            Self::fold_group(&acc, &mut token_vec, i);
            token_vec.into_iter().collect::<Vec<_>>()
        });

        folded.into_iter().collect()
    }
}

/// Action that can be taken by a visitor.
#[allow(dead_code)]
pub enum StreamVisitorAction {
    /// Continue as usual without doing anything.
    Continue,
    /// Drop the current token (or group) and continue traversal.
    Skip,
    /// Replace the current token (or group) with a new sequence of tokens.
    ///
    /// After this operation the cursor will be placed at the last token of the newly inserted
    /// sequence. If the sequence was empty, the cursor will be placed at the next token.
    Replace(TokenStream),
}

/// Defines user-specified part of visitor's behavior - allows to edit the recursive token structure
/// of [`TokenStream`] by optionally overriding specific methods that correspond to different
/// token types (such as [`StreamVisitor::visit_ident_mut`]) and events (such as
/// [`StreamVisitor::enter_group_mut`]).
#[allow(unused_variables)]
pub trait StreamVisitor {
    fn visit_ident_mut(
        &mut self,
        ctx: &VisitorCtx,
        ident: &Ident,
    ) -> Result<StreamVisitorAction, Error> {
        Ok(StreamVisitorAction::Continue)
    }
    fn visit_punct_mut(
        &mut self,
        ctx: &VisitorCtx,
        punct: &Punct,
    ) -> Result<StreamVisitorAction, Error> {
        Ok(StreamVisitorAction::Continue)
    }
    fn visit_literal_mut(
        &mut self,
        ctx: &VisitorCtx,
        literal: &Literal,
    ) -> Result<StreamVisitorAction, Error> {
        Ok(StreamVisitorAction::Continue)
    }
    fn visit_group_mut(
        &mut self,
        ctx: &VisitorCtx,
        group: &Group,
    ) -> Result<StreamVisitorAction, Error> {
        Ok(StreamVisitorAction::Continue)
    }
    fn enter_group_mut(
        &mut self,
        ctx: &VisitorCtx,
        group: &[TokenTree],
    ) -> Result<StreamVisitorAction, Error> {
        Ok(StreamVisitorAction::Continue)
    }
    fn exit_group_mut(
        &mut self,
        ctx: &VisitorCtx,
        group: &[TokenTree],
    ) -> Result<StreamVisitorAction, Error> {
        Ok(StreamVisitorAction::Continue)
    }
    /// Triggered whenever [`StreamVisitorAction::Replace`] is issued and a replacement operation is
    /// performed.
    fn after_replace_mut(&mut self, ctx: &VisitorCtx) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{StreamVisitor, StreamVisitorAction, StreamWalker};
    use crate::error::Error;
    use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, TokenStream, TokenTree};
    use quote::quote;
    use rstest::rstest;

    /// A record in the log of [`TestVisitor`] that describes an event that occurred during the
    /// traversal of the token stream.
    #[derive(Debug, PartialEq)]
    enum LogEvent {
        VisitIdent(String),
        VisitPunct(char),
        VisitLiteral(String),
        VisitGroup(Delimiter),
        EnterGroup,
        ExitGroup,
        AfterReplace,
    }

    /// A test visitor compatible with [`StreamWalker`] that logs events and allows to optionally
    /// pass callbacks for specific events.
    struct TestVisitor {
        log: Vec<LogEvent>,
        on_ident: Option<Box<dyn FnMut(&Ident) -> Result<StreamVisitorAction, Error>>>,
        on_punct: Option<Box<dyn FnMut(&Punct) -> Result<StreamVisitorAction, Error>>>,
        on_literal: Option<Box<dyn FnMut(&Literal) -> Result<StreamVisitorAction, Error>>>,
        on_visit_group: Option<Box<dyn FnMut(&Group) -> Result<StreamVisitorAction, Error>>>,
        on_enter_group: Option<Box<dyn FnMut(&[TokenTree]) -> Result<StreamVisitorAction, Error>>>,
        on_exit_group: Option<Box<dyn FnMut(&[TokenTree]) -> Result<StreamVisitorAction, Error>>>,
    }

    impl TestVisitor {
        fn new() -> Self {
            Self {
                log: Vec::new(),
                on_ident: None,
                on_punct: None,
                on_literal: None,
                on_visit_group: None,
                on_enter_group: None,
                on_exit_group: None,
            }
        }
        fn after_replace_count(&self) -> usize {
            self.log
                .iter()
                .filter(|e| matches!(e, LogEvent::AfterReplace))
                .count()
        }
    }

    impl StreamVisitor for TestVisitor {
        fn visit_ident_mut(
            &mut self,
            _ctx: &super::VisitorCtx,
            ident: &Ident,
        ) -> Result<StreamVisitorAction, Error> {
            self.log.push(LogEvent::VisitIdent(ident.to_string()));
            if let Some(cb) = self.on_ident.as_mut() {
                cb(ident)
            } else {
                Ok(StreamVisitorAction::Continue)
            }
        }
        fn visit_punct_mut(
            &mut self,
            _ctx: &super::VisitorCtx,
            punct: &Punct,
        ) -> Result<StreamVisitorAction, Error> {
            self.log.push(LogEvent::VisitPunct(punct.as_char()));
            if let Some(cb) = self.on_punct.as_mut() {
                cb(punct)
            } else {
                Ok(StreamVisitorAction::Continue)
            }
        }
        fn visit_literal_mut(
            &mut self,
            _ctx: &super::VisitorCtx,
            lit: &Literal,
        ) -> Result<StreamVisitorAction, Error> {
            self.log.push(LogEvent::VisitLiteral(lit.to_string()));
            if let Some(cb) = self.on_literal.as_mut() {
                cb(lit)
            } else {
                Ok(StreamVisitorAction::Continue)
            }
        }
        fn visit_group_mut(
            &mut self,
            _ctx: &super::VisitorCtx,
            group: &Group,
        ) -> Result<StreamVisitorAction, Error> {
            self.log.push(LogEvent::VisitGroup(group.delimiter()));
            if let Some(cb) = self.on_visit_group.as_mut() {
                cb(group)
            } else {
                Ok(StreamVisitorAction::Continue)
            }
        }
        fn enter_group_mut(
            &mut self,
            _ctx: &super::VisitorCtx,
            group: &[TokenTree],
        ) -> Result<StreamVisitorAction, Error> {
            let _ = group;
            self.log.push(LogEvent::EnterGroup);
            if let Some(cb) = self.on_enter_group.as_mut() {
                cb(group)
            } else {
                Ok(StreamVisitorAction::Continue)
            }
        }
        fn exit_group_mut(
            &mut self,
            _ctx: &super::VisitorCtx,
            group: &[TokenTree],
        ) -> Result<StreamVisitorAction, Error> {
            let _ = group;
            self.log.push(LogEvent::ExitGroup);
            if let Some(cb) = self.on_exit_group.as_mut() {
                cb(group)
            } else {
                Ok(StreamVisitorAction::Continue)
            }
        }
        fn after_replace_mut(&mut self, _ctx: &super::VisitorCtx) -> Result<(), Error> {
            self.log.push(LogEvent::AfterReplace);
            Ok(())
        }
    }

    #[rstest]
    fn visit_order_simple_group() {
        let input: TokenStream = quote!((a));
        let mut visitor = TestVisitor::new();

        let mut walker = StreamWalker::new(&mut visitor);
        let actual = walker.walk(input).unwrap();
        assert_eq!(actual.to_string(), "(a)");

        assert_eq!(
            visitor.log,
            vec![
                LogEvent::VisitGroup(Delimiter::Parenthesis),
                LogEvent::EnterGroup,
                LogEvent::VisitIdent("a".into()),
                LogEvent::ExitGroup,
                LogEvent::ExitGroup,
            ]
        );
    }

    #[rstest]
    fn replace_ident_with_multiple_tokens() {
        let input: TokenStream = quote!(a);
        let mut visitor = TestVisitor::new();
        visitor.on_ident = Some(Box::new(|id: &Ident| {
            if id == "a" {
                Ok(StreamVisitorAction::Replace(quote!(x y)))
            } else {
                Ok(StreamVisitorAction::Continue)
            }
        }));

        let mut walker = StreamWalker::new(&mut visitor);
        let actual = walker.walk(input).unwrap();
        assert_eq!(actual.to_string(), "x y");
        assert_eq!(visitor.after_replace_count(), 1);
    }

    #[rstest]
    fn replace_ident_with_empty_stream_triggers_after_replace() {
        let input: TokenStream = quote!(a);
        let mut visitor = TestVisitor::new();
        visitor.on_ident = Some(Box::new(|id: &Ident| {
            if id == "a" {
                Ok(StreamVisitorAction::Replace(quote!()))
            } else {
                Ok(StreamVisitorAction::Continue)
            }
        }));

        let mut walker = StreamWalker::new(&mut visitor);
        let actual = walker.walk(input).unwrap();
        assert_eq!(actual.to_string(), "");
        assert_eq!(visitor.after_replace_count(), 1);
    }

    #[rstest]
    fn remove_ident_by_skipping() {
        let input: TokenStream = quote!(a b);
        let mut visitor = TestVisitor::new();
        visitor.on_ident = Some(Box::new(|id: &Ident| {
            if id == "a" {
                Ok(StreamVisitorAction::Skip)
            } else {
                Ok(StreamVisitorAction::Continue)
            }
        }));

        let mut walker = StreamWalker::new(&mut visitor);
        let actual = walker.walk(input).unwrap();
        assert_eq!(actual.to_string(), "b");
    }

    #[rstest]
    fn replace_group_on_visit() {
        let input: TokenStream = quote!((a));
        let mut visitor = TestVisitor::new();
        visitor.on_visit_group = Some(Box::new(|g: &Group| {
            if g.delimiter() == Delimiter::Parenthesis {
                Ok(StreamVisitorAction::Replace(quote!(x + y)))
            } else {
                Ok(StreamVisitorAction::Continue)
            }
        }));

        let mut walker = StreamWalker::new(&mut visitor);
        let actual = walker.walk(input).unwrap();
        assert_eq!(actual.to_string(), "x + y");
        assert_eq!(visitor.after_replace_count(), 1);
    }

    #[rstest]
    fn skip_group_on_exit_removes_it() {
        let input: TokenStream = quote!((a) c);
        let mut visitor = TestVisitor::new();
        visitor.on_exit_group = Some(Box::new(|group: &[TokenTree]| {
            let ts: TokenStream = group.iter().cloned().collect();
            if ts.to_string() == "a" {
                Ok(StreamVisitorAction::Skip)
            } else {
                Ok(StreamVisitorAction::Continue)
            }
        }));

        let mut walker = StreamWalker::new(&mut visitor);
        let actual = walker.walk(input).unwrap();
        assert_eq!(actual.to_string(), "c");
    }

    #[rstest]
    fn error_is_propagated() {
        let input: TokenStream = quote!(boom);
        let mut visitor = TestVisitor::new();
        visitor.on_ident = Some(Box::new(|id: &Ident| {
            if id == "boom" {
                Err(Error::make_internal_error("boom".into()))
            } else {
                Ok(StreamVisitorAction::Continue)
            }
        }));

        let mut walker = StreamWalker::new(&mut visitor);
        let err = walker.walk(input).expect_err("expected error");
        assert!(matches!(err, Error::InternalError(_)));
    }
}
