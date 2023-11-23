#![allow(clippy::module_inception)]

#[global_allocator]
static ALLOC: rpmalloc::RpMalloc = rpmalloc::RpMalloc;

mod board;
mod utils;
mod moves;
mod version;

fn main() {
    println!("protium {}.{}.{} Copyright (C) 2023 datawater", version::MAJOR, version::MINOR, version::PATCH);
    println!("The project is currently under development. It doesn't do anything (yet)");
}