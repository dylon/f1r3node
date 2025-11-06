use itertools::Itertools;
use models::rhoapi::Par;

use crate::rust::interpreter::matcher::par_count::ParCount;
use super::subset_iterator::SubsetIterator;

/// Creates an iterator over all possible (subset, complement) Par pairs.
///
/// This is the lazy version of the original `sub_pars` function. Instead of
/// eagerly generating all combinations upfront, it uses itertools' cartesian_product
/// to create a lazy 7-way cartesian product of SubsetIterators.
///
/// # Performance
///
/// Memory: O(1) for iterator state (vs O(2^n) for eager version)
/// Time per result: O(1) amortized (vs O(2^n) upfront cost)
/// Early termination benefit: Massive - only generates what's consumed
///
/// # Examples
///
/// ```ignore
/// let par = /* some Par object */;
/// let min = ParCount::default();
/// let max = ParCount { sends: 2, receives: 2, ...};
///
/// // Only generates combinations as needed
/// for (subset_par, complement_par) in SubParsIterator::new(&par, &min, &max, &min, &max) {
///     // Process each pair...
///     // Can break early without generating remaining combinations!
/// }
/// ```
pub struct SubParsIterator<'a> {
    inner: Box<dyn Iterator<Item = (Par, Par)> + 'a>,
}

impl<'a> SubParsIterator<'a> {
    pub fn new(
        par: &'a Par,
        min: &ParCount,
        max: &ParCount,
        min_prune: &ParCount,
        max_prune: &ParCount,
    ) -> Self {
        // Calculate min/max for each field (same logic as original)
        let send_max = std::cmp::min(
            max.sends as isize,
            par.sends.len() as isize - min_prune.sends as isize,
        );
        let receive_max = std::cmp::min(
            max.receives as isize,
            par.receives.len() as isize - min_prune.receives as isize,
        );
        let new_max = std::cmp::min(
            max.news as isize,
            par.news.len() as isize - min_prune.news as isize,
        );
        let expr_max = std::cmp::min(
            max.exprs as isize,
            par.exprs.len() as isize - min_prune.exprs as isize,
        );
        let match_max = std::cmp::min(
            max.matches as isize,
            par.matches.len() as isize - min_prune.matches as isize,
        );
        let unf_max = std::cmp::min(
            max.unforgeables as isize,
            par.unforgeables.len() as isize - min_prune.unforgeables as isize,
        );
        let bundle_max = std::cmp::min(
            max.bundles as isize,
            par.bundles.len() as isize - min_prune.bundles as isize,
        );

        let send_min = std::cmp::max(
            min.sends as isize,
            par.sends.len() as isize - max_prune.sends as isize,
        );
        let receive_min = std::cmp::max(
            min.receives as isize,
            par.receives.len() as isize - max_prune.receives as isize,
        );
        let new_min = std::cmp::max(
            min.news as isize,
            par.news.len() as isize - max_prune.news as isize,
        );
        let expr_min = std::cmp::max(
            min.exprs as isize,
            par.exprs.len() as isize - max_prune.exprs as isize,
        );
        let match_min = std::cmp::max(
            min.matches as isize,
            par.matches.len() as isize - max_prune.matches as isize,
        );
        let unf_min = std::cmp::max(
            min.unforgeables as isize,
            par.unforgeables.len() as isize - max_prune.unforgeables as isize,
        );
        let bundle_min = std::cmp::max(
            min.bundles as isize,
            par.bundles.len() as isize - max_prune.bundles as isize,
        );

        // Create 7-way cartesian product of lazy SubsetIterators
        // This is the key optimization - no upfront computation!
        let inner = SubsetIterator::new(&par.sends, send_min, send_max)
            .cartesian_product(SubsetIterator::new(&par.receives, receive_min, receive_max))
            .cartesian_product(SubsetIterator::new(&par.news, new_min, new_max))
            .cartesian_product(SubsetIterator::new(&par.exprs, expr_min, expr_max))
            .cartesian_product(SubsetIterator::new(&par.matches, match_min, match_max))
            .cartesian_product(SubsetIterator::new(&par.unforgeables, unf_min, unf_max))
            .cartesian_product(SubsetIterator::new(&par.bundles, bundle_min, bundle_max))
            .map(
                |(
                    (((((sub_sends, sub_receives), sub_news), sub_exprs), sub_matches), sub_unfs),
                    sub_bundles,
                )| {
                    // Build the two Par objects from subsets and complements
                    (
                        Par {
                            sends: sub_sends.0,
                            receives: sub_receives.0,
                            news: sub_news.0,
                            exprs: sub_exprs.0,
                            matches: sub_matches.0,
                            unforgeables: sub_unfs.0,
                            bundles: sub_bundles.0,
                            connectives: Vec::default(),
                            locally_free: Vec::default(),
                            connective_used: false,
                        },
                        Par {
                            sends: sub_sends.1,
                            receives: sub_receives.1,
                            news: sub_news.1,
                            exprs: sub_exprs.1,
                            matches: sub_matches.1,
                            unforgeables: sub_unfs.1,
                            bundles: sub_bundles.1,
                            connectives: Vec::default(),
                            locally_free: Vec::default(),
                            connective_used: false,
                        },
                    )
                },
            );

        SubParsIterator {
            inner: Box::new(inner),
        }
    }
}

impl<'a> Iterator for SubParsIterator<'a> {
    type Item = (Par, Par);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
