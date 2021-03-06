// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:issue_38715.rs
// aux-build:issue_38715-modern.rs

// Test that `#[macro_export] macro_rules!` shadow earlier `#[macro_export] macro_rules!`

#[macro_use]
extern crate issue_38715;
#[macro_use]
extern crate issue_38715_modern;

fn main() {
    foo!();
    foo_modern!();
}
