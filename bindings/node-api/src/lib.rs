#![deny(clippy::all)]

//TODO: remove unwraps
//TODO: maybe rework error handling? use custom error type to prevent .map_err calls

mod threadsafe_function;
mod server;
mod target;

#[macro_use]
extern crate napi_derive;
