use syn::buffer::Cursor;

/// Returns the number of tokens between `start` and `end`.
///
/// # Notes
///
/// - The function works recursively - counting tokens even inside token groups.
#[allow(dead_code)]
pub fn token_distance<'a>(start: &Cursor<'a>, end: &Cursor<'a>) -> usize {
    let mut start = *start;
    let mut end = *end;

    // Ensure `start` is always before `end`.
    if start > end {
        std::mem::swap(&mut start, &mut end);
    }

    fn inner<'b>(mut start: Cursor<'b>, end: Cursor<'b>) -> usize {
        let mut total = 0;
        while start < end {
            if let Some((inside, _delim, _span, after)) = start.any_group() {
                let sub_end = if end < after { end } else { after };
                total += inner(inside, sub_end);
                if end < after {
                    break;
                }
                start = after;
            } else {
                match start.token_tree() {
                    Some((_, next)) => {
                        total += 1;
                        start = next;
                    }
                    None => break,
                }
            }
        }
        total
    }

    inner(start, end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream;
    use syn::buffer::TokenBuffer;
    use syn::parse_str;

    /// Advances the cursor to the first occurrence of a token with the given string representation.
    fn advance_cursor<'a>(cursor: Cursor<'a>, target: &str) -> Cursor<'a> {
        fn search<'b>(cur: Cursor<'b>, target: &str) -> Option<Cursor<'b>> {
            let mut c = cur;
            loop {
                if let Some((inside, _, _, after)) = c.any_group() {
                    if let Some(found) = search(inside, target) {
                        return Some(found);
                    }
                    c = after;
                    continue;
                }

                match c.token_tree() {
                    Some((tt, next)) => {
                        if tt.to_string() == target {
                            return Some(c);
                        }
                        c = next;
                    }
                    None => return None,
                }
            }
        }

        search(cursor, target).unwrap_or(cursor)
    }

    #[test]
    fn token_distance_forward() {
        let stream: TokenStream = parse_str("a + b - c * d").unwrap();
        let buf = TokenBuffer::new2(stream);
        let start = buf.begin();
        let end = advance_cursor(start, "c");
        assert_eq!(token_distance(&start, &end), 4);
    }

    #[test]
    fn token_distance_backward() {
        let stream: TokenStream = parse_str("x * y / z").unwrap();
        let buf = TokenBuffer::new2(stream);
        let start = buf.begin();
        let end = advance_cursor(start, "/");
        assert_eq!(token_distance(&start, &end), token_distance(&end, &start));
    }

    #[test]
    fn token_distance_recursive() {
        let stream: TokenStream = parse_str("a + (b * c) - d").unwrap();
        let buf = TokenBuffer::new2(stream);
        let start = buf.begin();
        let end = advance_cursor(start, "c");
        assert_eq!(token_distance(&start, &end), 4);
    }
}
