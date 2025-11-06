use std::fmt::Debug;

/// Lazily generates all subsets of a slice within size constraints.
///
/// Uses a bitmask-based approach to generate subsets on-demand. Each bit in the mask
/// determines whether the corresponding element is included in the subset (1) or
/// complement (0).
///
/// This approach trades off result ordering for simplicity and true lazy evaluation.
/// The original recursive implementation generated results in a specific order, but
/// for the spatial matcher's use case (early termination on first match), order
/// doesn't matter - we just need to generate all valid (subset, complement) pairs.
///
/// # Performance
///
/// Time complexity: O(2^n) worst case (same as original)
/// Space complexity: O(1) for iterator state + O(n) per result
/// Memory improvement: No recursive calls, no intermediate vectors
///
/// # Examples
///
/// ```ignore
/// let items = vec![1, 2, 3];
/// let mut iter = SubsetIterator::new(&items, 1, 2);
///
/// // Generates all subsets of size 1 or 2
/// while let Some((subset, complement)) = iter.next() {
///     assert!(subset.len() >= 1 && subset.len() <= 2);
///     assert_eq!(subset.len() + complement.len(), 3);
/// }
/// ```
#[derive(Clone)]
pub struct SubsetIterator<'a, T> {
    items: &'a [T],
    min_size: usize,
    max_size: usize,
    /// Current bitmask being evaluated (counts up from 0)
    current_mask: usize,
    /// Maximum mask value (2^n)
    max_mask: usize,
}

impl<'a, T: Clone + Debug> SubsetIterator<'a, T> {
    pub fn new(items: &'a [T], min_size: isize, max_size: isize) -> Self {
        let len = items.len();

        // Handle edge cases
        let (min_size, max_size) = if max_size < 0 || min_size > max_size {
            // No valid subsets
            (0, 0)
        } else {
            let min_size = min_size.max(0) as usize;
            let max_size = max_size.min(len as isize) as usize;
            (min_size, max_size)
        };

        // Calculate 2^n, but cap at reasonable limit to avoid overflow
        // For more than 63 elements, we'd need BigInt which is overkill
        let max_mask = if len < 64 {
            1usize << len
        } else {
            // For very large inputs, we can't enumerate all subsets anyway
            // This will effectively make the iterator empty for len >= 64
            0
        };

        SubsetIterator {
            items,
            min_size,
            max_size,
            current_mask: 0,
            max_mask,
        }
    }

    /// Count number of 1 bits in the mask (population count)
    #[inline]
    fn popcount(mask: usize) -> usize {
        mask.count_ones() as usize
    }

    /// Generate (subset, complement) for given bitmask
    fn mask_to_subsets(&self, mask: usize) -> (Vec<T>, Vec<T>) {
        let mut subset = Vec::new();
        let mut complement = Vec::new();

        for (i, item) in self.items.iter().enumerate() {
            if (mask & (1 << i)) != 0 {
                subset.push(item.clone());
            } else {
                complement.push(item.clone());
            }
        }

        (subset, complement)
    }
}

impl<'a, T: Clone + Debug> Iterator for SubsetIterator<'a, T> {
    type Item = (Vec<T>, Vec<T>);

    fn next(&mut self) -> Option<Self::Item> {
        // Iterate through all possible bitmasks
        while self.current_mask < self.max_mask {
            let mask = self.current_mask;
            self.current_mask += 1;

            // Check if this mask represents a valid subset size
            let subset_size = Self::popcount(mask);

            if subset_size >= self.min_size && subset_size <= self.max_size {
                return Some(self.mask_to_subsets(mask));
            }
        }

        None
    }
}
