#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
pub mod bundle;
pub mod common;
pub mod order;
pub mod validator;

use crate::validator::ValidationClient;

pub fn init_validation() -> ValidationClient {
    todo!()
}
