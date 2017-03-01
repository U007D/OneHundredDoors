//#![cfg_attr(feature="clippy", feature(plugin))]
//#![cfg_attr(feature="clippy", plugin(clippy))]
//#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts,
//unsafe_code, unstable_features, unused_import_braces, unused_qualifications)]
//#![allow(non_snake_case)]
#![feature(inclusive_range_syntax)]

extern crate itertools;
use itertools::Itertools;

fn main() {
    let mut doors = vec![false; 100];

    let _ = (1...100).into_iter()
             .map(|pass| doors.iter_mut().step(pass)
                              .map(|door| *door = !*door ).last()).last();

    println!("{:?}", doors);
}
