/// Cross-product iterator for dynamically-defined number of iterators
///
/// This iterator computes the Cartesian product of N iterators where N
/// is determined at runtime. All iterators must yield the same type T.
#[derive(Debug)]
pub struct CrossProduct<T>
where
    T: Clone,
{
    /// Original values from all iterators, stored as Vec<Vec<T>>
    sources: Vec<Vec<T>>,
    /// Current indices pointing into each source Vec
    indices: Vec<usize>,
    /// Flag indicating if we've generated all combinations
    is_exhausted: bool,
}

impl<T> CrossProduct<T>
where
    T: Clone,
{
    /// Creates a new cross-product iterator from iterators over values.
    #[allow(dead_code)]
    pub fn from_iters<I>(iterators: Vec<I>) -> Self
    where
        I: Iterator<Item = T>,
    {
        let sources: Vec<Vec<T>> = iterators.into_iter().map(|iter| iter.collect()).collect();

        Self::from_vecs(sources)
    }

    /// Creates a new cross-product iterator from vectors of values.
    pub fn from_vecs(sources: Vec<Vec<T>>) -> Self {
        let exhausted = sources.is_empty() || sources.iter().any(|v| v.is_empty());
        let indices = vec![0; sources.len()];

        Self {
            sources,
            indices,
            is_exhausted: exhausted,
        }
    }

    /// Returns the current combination based on current indices.
    fn current_combination(&self) -> Vec<T> {
        self.indices
            .iter()
            .zip(&self.sources)
            .map(|(&idx, source)| source[idx].clone())
            .collect()
    }

    /// Advances indices to the next combination.
    ///
    /// Returns false when all combinations have been generated.
    fn advance_indices(&mut self) -> bool {
        // Edge case of no iterators
        if self.indices.is_empty() {
            return false;
        }

        // Attempting to increment from the rightmost index
        for i in (0..self.indices.len()).rev() {
            self.indices[i] += 1;

            if self.indices[i] < self.sources[i].len() {
                return true;
            }

            // This index overflowed, resetting it
            self.indices[i] = 0;
        }

        // All indices have wrapped around - the iterator has exhausted
        false
    }

    /// Returns the total number of combinations or `None` on overflow.
    pub fn total_combinations(&self) -> Option<usize> {
        if self.sources.is_empty() {
            return Some(0);
        }

        self.sources
            .iter()
            .map(|v| v.len())
            .try_fold(1usize, |acc, len| acc.checked_mul(len))
    }

    /// Returns the number of source iterators.
    #[allow(dead_code)]
    pub fn num_iterators(&self) -> usize {
        self.sources.len()
    }

    /// Checks if any source iterator is empty.
    fn has_empty_source(&self) -> bool {
        self.sources.iter().any(|v| v.is_empty())
    }

    /// Resets the iterator to its initial state.
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.indices.fill(0);
        self.is_exhausted = self.has_empty_source();
    }
}

impl<T> Iterator for CrossProduct<T>
where
    T: Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_exhausted {
            return None;
        }

        let result = self.current_combination();

        if !self.advance_indices() {
            self.is_exhausted = true;
        }

        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.is_exhausted {
            return (0, Some(0));
        }

        if let Some(total) = self.total_combinations() {
            let mut generated = 0;
            let mut multiplier = 1;

            for i in (0..self.indices.len()).rev() {
                generated += self.indices[i] * multiplier;
                multiplier *= self.sources[i].len();
            }

            let remaining = total.saturating_sub(generated);
            (remaining, Some(remaining))
        } else {
            (0, None)
        }
    }
}

/// Creates a cross-product iterator from vectors
pub fn cross_product<T>(items: Vec<Vec<T>>) -> CrossProduct<T>
where
    T: Clone,
{
    CrossProduct::from_vecs(items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::fmt::Debug;

    /// Test constructors with different input types and edge cases
    #[rstest]
    #[case::basic_multi_int(
        vec![vec![1, 2], vec![10, 20], vec![100]],
        vec![vec![1, 10, 100], vec![1, 20, 100], vec![2, 10, 100], vec![2, 20, 100]],
    )]
    #[case::basic_multi_char(
        vec![vec!['a', 'b'], vec!['x', 'y', 'z']],
        vec![vec!['a', 'x'], vec!['a', 'y'], vec!['a', 'z'], vec!['b', 'x'], vec!['b', 'y'], vec!['b', 'z']]
    )]
    #[case::single_iterator(
        vec![vec![1, 2, 3]],
        vec![vec![1], vec![2], vec![3]]
    )]
    #[case::simple_two_iterators(
        vec![vec!['a'], vec!['x', 'y']],
        vec![vec!['a', 'x'], vec!['a', 'y']]
    )]
    #[case::empty_source_in_middle(
        vec![vec![1, 2], vec![], vec![3, 4]],
        vec![] as Vec<Vec<i32>>
    )]
    #[case::no_iterators(
        vec![] as Vec<Vec<i32>>,
        vec![] as Vec<Vec<i32>>
    )]
    fn constructors<T: Clone + PartialEq + Debug>(
        #[case] input: Vec<Vec<T>>,
        #[case] expected: Vec<Vec<T>>,
    ) {
        let result_from_vecs: Vec<_> = CrossProduct::from_vecs(input.clone()).collect();
        assert_eq!(result_from_vecs, expected);

        let result_from_iterators: Vec<_> = cross_product(input).collect();
        assert_eq!(result_from_iterators, expected);
    }

    /// Test calculation of total number of combinations for various input sizes
    #[rstest]
    #[case::multiple_sizes(
        vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]],
        Some(24)
    )]
    #[case::single_elements(
        vec![vec![1], vec![2]],
        Some(1)
    )]
    #[case::empty_source(
        vec![vec![1, 2], vec![]],
        Some(0)
    )]
    fn total_combinations(#[case] input: Vec<Vec<i32>>, #[case] expected: Option<usize>) {
        let cp = CrossProduct::from_vecs(input);
        assert_eq!(cp.total_combinations(), expected);
    }

    /// Test that size_hint correctly tracks remaining combinations as iterator progresses
    #[rstest]
    fn size_hint() {
        let mut cp = CrossProduct::from_vecs(vec![vec![1, 2], vec![3, 4, 5]]);

        assert_eq!(cp.size_hint(), (6, Some(6)));

        cp.next();
        assert_eq!(cp.size_hint(), (5, Some(5)));

        cp.next();
        cp.next();
        assert_eq!(cp.size_hint(), (3, Some(3)));
    }

    /// Test that reset functionality returns iterator to initial state
    #[rstest]
    fn reset() {
        let mut cp = CrossProduct::from_vecs(vec![vec![1, 2], vec![3, 4]]);

        cp.next();
        cp.next();

        cp.reset();

        assert_eq!(cp.next(), Some(vec![1, 3]));
    }

    /// Test that the actual number of generated combinations matches expectations
    #[rstest]
    #[case::two_by_two(
        vec![vec![1, 2], vec![10, 20]],
        4
    )]
    #[case::one_by_three(
        vec![vec!['a'], vec!['x', 'y', 'z']],
        3
    )]
    #[case::three_by_one_by_two(
        vec![vec![1, 2, 3], vec![10], vec![100, 200]],
        6
    )]
    fn correct_number_of_combinations<T: Clone + Debug>(
        #[case] input: Vec<Vec<T>>,
        #[case] expected_count: usize,
    ) {
        let cp = CrossProduct::from_vecs(input);
        let result: Vec<_> = cp.collect();
        assert_eq!(result.len(), expected_count);
    }

    /// Test detection of empty sources in various configurations
    #[rstest]
    #[case::no_sources(
        vec![],
        false
    )]
    #[case::non_empty_source(
        vec![vec![1, 2]],
        false
    )]
    #[case::has_empty_source(
        vec![vec![1, 2], vec![]],
        true,
    )]
    fn has_empty_source(#[case] input: Vec<Vec<i32>>, #[case] expected: bool) {
        let cp = CrossProduct::from_vecs(input);
        assert_eq!(cp.has_empty_source(), expected);
    }
}
