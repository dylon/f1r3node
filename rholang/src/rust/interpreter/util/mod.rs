use models::{
    rhoapi::{Bundle, Connective, Expr, Match, New, Par, Receive, Send},
    rust::utils::union,
};

use super::matcher::has_locally_free::HasLocallyFree;

pub mod address_tools;
pub mod base58;
pub mod rev_address;

// Helper enum. This is 'GeneratedMessage' in Scala
#[derive(Clone, Debug)]
pub enum GeneratedMessage {
    Send(Send),
    Receive(Receive),
    New(New),
    Match(Match),
    Bundle(Bundle),
    Expr(Expr),
}

// These two functions need to be under 'rholang' dir because of HasLocallyFree Trait.
// This trait should, I think, be moved to models

// See models/src/main/scala/coop/rchain/models/rholang/implicits.scala - prepend
pub fn prepend_connective(mut p: Par, c: Connective, depth: i32) -> Par {
    let mut new_connectives = Vec::with_capacity(p.connectives.len() + 1);
    new_connectives.push(c.clone());
    new_connectives.append(&mut p.connectives);

    let c_clone = c.clone();
    let locally_free = c.locally_free(c_clone.clone(), depth);
    let connective_used = p.connective_used || c_clone.connective_used(c);

    Par {
        connectives: new_connectives,
        locally_free,
        connective_used,
        ..p
    }
}

pub fn prepend_expr(mut p: Par, e: Expr, depth: i32) -> Par {
    let mut new_exprs = Vec::with_capacity(p.exprs.len() + 1);
    new_exprs.push(e.clone());
    new_exprs.append(&mut p.exprs);

    let e_clone = e.clone();
    let locally_free = union(p.locally_free.clone(), e.locally_free(e_clone.clone(), depth));
    let connective_used = p.connective_used || e_clone.connective_used(e);

    Par {
        exprs: new_exprs,
        locally_free,
        connective_used,
        ..p
    }
}

pub fn prepend_new(mut p: Par, n: New) -> Par {
    let mut new_news = Vec::with_capacity(p.news.len() + 1);
    new_news.push(n.clone());
    new_news.append(&mut p.news);

    let n_clone = n.clone();
    let n_locally_free = n_clone.locally_free.clone();
    let locally_free = union(p.locally_free.clone(), n_locally_free);
    let connective_used = p.connective_used || n_clone.connective_used(n);

    Par {
        news: new_news,
        locally_free,
        connective_used,
        ..p
    }
}

pub fn prepend_bundle(mut p: Par, b: Bundle) -> Par {
    let mut new_bundles = Vec::with_capacity(p.bundles.len() + 1);
    new_bundles.push(b.clone());
    new_bundles.append(&mut p.bundles);

    Par {
        bundles: new_bundles,
        locally_free: union(p.locally_free.clone(), b.body.unwrap().locally_free),
        ..p
    }
}

// for locally_free parameter, in case when we have (bodyResult.par.locallyFree.from(boundCount).map(x => x - boundCount))
pub(crate) fn filter_and_adjust_bitset(bitset: Vec<u8>, bound_count: usize) -> Vec<u8> {
    bitset
        .into_iter()
        .enumerate()
        .filter_map(|(i, _)| {
            if i >= bound_count {
                Some(i as u8 - bound_count as u8)
            } else {
                None
            }
        })
        .collect()
}
