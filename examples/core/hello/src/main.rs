/* This example is in the public domain */

use ainakan::Ainakan;
use std::sync::LazyLock;

static AINAKAN: LazyLock<Ainakan> = LazyLock::new(|| unsafe { Ainakan::obtain() });

fn main() {
    println!("Hello, world!");
}
