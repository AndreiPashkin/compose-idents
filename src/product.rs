use std::mem::MaybeUninit;

/// Iterator over a cartesian product of outputs of the input iterators.
///
/// In other words each iteration of the iterator yields values of all the iterators produced at
/// each iteration of the "innermost" of them.
///
/// # Notes
///
/// The core implementation idea is similar to an odometer:
///
///   - The "innermost" iterator ticks fastest - at each tick the iterator yields.
///   - When it cycles, the next outer iterator ticks once.
///   - When the outermost iterator cycles, the whole iteration ends.
pub struct ProductIterator<I, V>
where
    I: Iterator<Item = V> + Clone,
    V: Clone,
{
    /// Source iterators used to reset inner iterators when they exhaust.
    source: Vec<I>,
    /// Current iterators at their respective positions.
    iters: Vec<I>,
    /// Temporarily stored values of the current combination.
    values: Vec<MaybeUninit<V>>,
    /// Index of the iterator currently being advanced.
    i: usize,
}

impl<I, V> ProductIterator<I, V>
where
    I: Iterator<Item = V> + Clone,
    V: Clone,
{
    /// Creates a new [`ProductIterator`] from the provided iterators.
    pub fn new(iters: &[I]) -> Self {
        let mut values: Vec<MaybeUninit<V>> = Vec::with_capacity(iters.len());
        unsafe {
            values.set_len(iters.len());
        }
        Self {
            iters: iters.to_vec(),
            source: iters.to_vec(),
            values,
            i: 0,
        }
    }
}

impl<I, V> Iterator for ProductIterator<I, V>
where
    I: Iterator<Item = V> + Clone,
    V: Clone,
{
    type Item = Box<[V]>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.iters.len();
        if n == 0 {
            return None;
        }

        loop {
            let value = self.iters[self.i].next();
            let is_innermost = self.i == n - 1;
            let is_outermost = self.i == 0;

            match (value, is_outermost, is_innermost) {
                (Some(value), _, false) => {
                    self.values[self.i] = MaybeUninit::new(value);
                    self.i += 1;
                    debug_assert!(self.i < n);
                }
                (Some(value), _, true) => {
                    self.values[self.i] = MaybeUninit::new(value);
                    let result: Vec<V> = unsafe {
                        self.values
                            .iter()
                            .map(|v| v.assume_init_ref().clone())
                            .collect()
                    };
                    return Some(result.into_boxed_slice());
                }
                (None, false, _) => {
                    self.iters[self.i] = self.source[self.i].clone();
                    debug_assert!(self.i > 0);
                    self.i -= 1;
                }
                (None, true, _) => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ProductIterator;

    #[test]
    fn product_iterator_simple() {
        let iter1 = vec!['A', 'B'].into_iter();
        let iter2 = vec!['a', 'b'].into_iter();
        let iter3 = vec!['0', '1'].into_iter();
        let iters = [iter1, iter2, iter3];

        let product_iter = ProductIterator::new(&iters);
        let collected: Vec<Box<[char]>> = product_iter.collect();
        let expected: Vec<Box<[char]>> = vec![
            Box::new(['A', 'a', '0']),
            Box::new(['A', 'a', '1']),
            Box::new(['A', 'b', '0']),
            Box::new(['A', 'b', '1']),
            Box::new(['B', 'a', '0']),
            Box::new(['B', 'a', '1']),
            Box::new(['B', 'b', '0']),
            Box::new(['B', 'b', '1']),
        ];

        assert_eq!(collected, expected,);
    }
}

pub fn product<I, V>(iters: &[I]) -> ProductIterator<I, V>
where
    I: Iterator<Item = V> + Clone,
    V: Clone,
{
    ProductIterator::new(iters)
}
