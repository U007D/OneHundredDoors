#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![deny(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts,
unsafe_code, unused_import_braces, unused_qualifications)]
#![allow(non_snake_case)]
#![feature(inclusive_range_syntax)]

fn main() {
    let doors = vec![false; 100].iter_mut().enumerate()
                                .map(|(door, door_state)| (1...100).into_iter()
                                                                   .filter(|pass| door % pass == 0)
                                                                   .map(|_| { *door_state = !*door_state; *door_state })
                                                                   .last().unwrap()).collect::<Vec<_>>();

    println!("{:?}", doors);
}
