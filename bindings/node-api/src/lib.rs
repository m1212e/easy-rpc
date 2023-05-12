#![deny(clippy::all)]

//TODO: remove unwraps
//TODO: maybe rework error handling? use custom error type to prevent .map_err calls

mod server;
mod target;
mod threadsafe_function;

#[macro_use]
extern crate napi_derive;

lazy_static::lazy_static! {
    /**
        This only exists to initialize the logger. It needs to be referenced somewhere in the code to prevent the compiler form
        optimizing it away
     */
    static ref INITIALIZED: bool = {
        simple_logger::init_with_level(log::Level::Warn).unwrap();
        true
    };
}
