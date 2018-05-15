// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct Foo<T: Clone>(T);

use std::ops::Add;

impl<T: Clone, Add> Add for Foo<T> {
//~^ ERROR expected trait, found type parameter
    type Output = usize;

    fn add(self, rhs: Self) -> Self::Output {
        //~^ ERROR ambiguous associated type
        unimplemented!();
    }
}

fn main() {}
