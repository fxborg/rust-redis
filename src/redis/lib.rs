#[crate_id = "redis#0.1"];
#[desc = "A Rust client library for Redis"];
#[license = "MIT"];
#[crate_type = "lib"];

pub use redis::{Result,Redis,Nil,Int,Data,List,Error,Status};
pub mod redis;