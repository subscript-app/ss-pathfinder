#![allow(unused)]
// pathfinder/renderer/src/lib.rs
//
// Copyright © 2020 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Pathfinder's renderer and associated objects.

#![warn(missing_docs)]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;

pub mod concurrent;
pub mod gpu;
pub mod options;
pub mod paint;
pub mod scene;

mod allocator;
mod builder;
mod gpu_data;
mod tile_map;
mod tiler;
mod tiles;
