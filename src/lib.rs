// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unordered containers, implemented as hash-tables

#![no_std]
#![feature(alloc, dropck_eyepatch, generic_param_attrs, allocator_api, shared, unique,
           sip_hash_13, fused, placement_new_protocol)]

extern crate alloc;

mod table;
pub mod map;
pub mod set;
pub mod fnv;

trait Recover<Q: ?Sized> {
    type Key;

    fn get(&self, key: &Q) -> Option<&Self::Key>;
    fn take(&mut self, key: &Q) -> Option<Self::Key>;
    fn replace(&mut self, key: Self::Key) -> Option<Self::Key>;
}

pub use map::HashMap;
pub use set::HashSet;
pub use fnv::FnvHashMap;
pub use fnv::FnvHashSet;
