// run-rustfix

#![warn(clippy::manual_rem_euclid)]

fn main() {
    let value: i32 = 5;

    let _: i32 = ((value % 4) + 4) % 4;
    let _: i32 = (4 + (value % 4)) % 4;
    let _: i32 = (value % 4 + 4) % 4;
    let _: i32 = (4 + value % 4) % 4;
    let _: i32 = 1 + (4 + value % 4) % 4;

    let _: i32 = (3 + value % 4) % 4;
    let _: i32 = (-4 + value % -4) % -4;
    let _: i32 = ((5 % 4) + 4) % 4;

    // Make sure the lint does not trigger if it would cause an error, like with an ambiguous
    // integer type
    let not_annotated = 24;
    let _ = ((not_annotated % 4) + 4) % 4;
    let inferred: _ = 24;
    let _ = ((inferred % 4) + 4) % 4;

    // For lint to apply the constant must always be on the RHS of the previous value for %
    let _: i32 = 4 % ((value % 4) + 4);
    let _: i32 = ((4 % value) + 4) % 4;
}

// Should lint for params too
pub fn rem_euclid_4(num: i32) -> i32 {
    ((num % 4) + 4) % 4
}
