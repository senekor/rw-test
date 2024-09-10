//! Clippy has a few things to say about this code!
//!
//! First, ensure you are getting the warnings in your editor.
//!
//! Be sure to read the warnings and maybe even follow the links
//! to the documentation. You might be surprised by how informative it is!
//!
//! Lastly, fix the warnings. Some are even auto-fixable!
//! See if your editor let's you apply these fixes.

pub fn get_first_two_elems(v: &Vec<i32>) -> (i32, i32) {
    return (v[0], v[1]);
}

pub fn make_unsigned(x: i32) -> u32 {
    x.abs() as u32
}

pub fn floats_are_similar(a: f32, b: f32) -> bool {
    (a - b) < f32::EPSILON
}

pub fn decrement(x: i32) -> i32 {
    --x
}

pub fn read_int_from_str(s: &str) -> i32 {
    i32::from_str_radix(s, 10).unwrap()
}

pub fn get_one_quintillion() -> u64 {
    1_000_000_000_0000_000
}
