// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! ```rust
//!
//! use comprez::Unsigned;
//!
//! fn main() {
//!     let num = 1234u32;
//!
//!     let compressed = Unsigned::compress_61_439(num).unwrap();
//!     let decompressed = Unsigned::decompress_61_439(compressed.to_vec()).unwrap();
//!
//!     println!("size before compression: {}", num.to_be_bytes().len()); //size before compression: 4
//!
//!     println!("size after compression: {}", compressed.len()); //size after compression: 2
//!
//!     println!("{}", decompressed == num); //true
//! }
//! ```

mod error;
mod unsigned;
mod signed;


///unsigned only compression
pub struct Unsigned;


///signed intergers compression
pub struct  Signed;









