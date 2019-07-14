#![feature(async_await)]
#![feature(async_closure)]
#![feature(test)]
#![deny(nonstandard_style)]
#![deny(future_incompatible)]
#![deny(rust_2018_idioms)]
#![warn(unused)]

pub mod debugging;
pub mod loading;
pub mod logging;
// pub mod rhi;
pub mod settings;
pub mod shaderpack;

extern crate bitflags;

extern crate bitflags;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
