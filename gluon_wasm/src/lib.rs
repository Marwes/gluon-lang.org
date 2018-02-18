#![feature(proc_macro)]

extern crate gluon;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use gluon::vm::api::{Hole, OpaqueValue};
use gluon::vm::thread::ThreadInternal;
use gluon::vm::internal::ValuePrinter;
use gluon::RootedThread;

#[wasm_bindgen]
#[no_mangle]
pub extern "C" fn run_expr(expr: &str) -> String {
    gluon::new_vm();
    expr.to_string()
}
