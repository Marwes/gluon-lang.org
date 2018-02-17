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
    let thread = gluon::new_vm();
    let mut compiler = gluon::Compiler::new();
    match compiler.run_expr::<OpaqueValue<RootedThread, Hole>>(&thread, "", expr) {
        Ok((v, t)) => {
            let env = thread.global_env().get_env();
            format!(
                "{}: {}",
                ValuePrinter::new(&*env, &t, unsafe { v.get_value() })
                    .width(80)
                    .max_level(5)
                    .to_string(),
                t
            )
        }
        Err(err) => err.to_string(),
    }
}
