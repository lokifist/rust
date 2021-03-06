// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(repr_simd, platform_intrinsics, rustc_attrs, core_intrinsics)]
#![allow(warnings)]

// Bad monomorphizations could previously cause LLVM asserts even though the
// error was caught in the compiler.

extern "platform-intrinsic" {
    fn simd_add<T>(x: T, y: T) -> T;
}

use std::intrinsics;

#[derive(Copy, Clone)]
struct Foo(i64);

#[rustc_no_mir] // FIXME #27840 MIR doesn't provide precise spans for calls.
unsafe fn test_cttz(v: Foo) -> Foo {
    intrinsics::cttz(v)
    //~^ ERROR `cttz` intrinsic: expected basic integer type, found `Foo`
}

#[rustc_no_mir] // FIXME #27840 MIR doesn't provide precise spans for calls.
unsafe fn test_fadd_fast(a: Foo, b: Foo) -> Foo {
    intrinsics::fadd_fast(a, b)
    //~^ ERROR `fadd_fast` intrinsic: expected basic float type, found `Foo`
}

#[rustc_no_mir] // FIXME #27840 MIR doesn't provide precise spans for calls.
unsafe fn test_simd_add(a: Foo, b: Foo) -> Foo {
    simd_add(a, b)
    //~^ ERROR `simd_add` intrinsic: expected SIMD input type, found non-SIMD `Foo`
}

fn main() {}
