extern crate try_gluon;

extern crate futures;
#[macro_use]
extern crate iron;
extern crate persistent;
extern crate staticfile;
extern crate mount;
extern crate gluon;
extern crate serde_json;
extern crate log;
extern crate env_logger;

use std::fs::{File, read_dir};
use std::io::{self, Read};

use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use iron::typemap::Key;

use serde_json::Value;

use staticfile::Static;

use mount::Mount;

use gluon::vm::thread::RootedThread;

pub struct VMKey;

impl Key for VMKey {
    type Value = RootedThread;
}

fn eval(req: &mut Request) -> IronResult<Response> {
    let mut body = String::new();

    itry!(req.body.read_to_string(&mut body));

    let global_vm = req.get::<persistent::Read<VMKey>>().unwrap();
    let s = global_vm
        .new_thread()
        .map(|vm| try_gluon::eval(&vm, &body))
        .unwrap_or_else(|err| err.to_string());

    let mime: Mime = "text/plain".parse().unwrap();

    Ok(Response::with((status::Ok, mime, serde_json::to_string(&s).unwrap())))
}

pub struct Examples;

impl Key for Examples {
    type Value = String;
}

fn examples(req: &mut Request) -> IronResult<Response> {
    let s = req.get::<persistent::Read<Examples>>().unwrap();
    Ok(Response::with((status::Ok, (*s).clone())))
}

/// Load all examples into a JSON array `[{ name: .., value: .. }, ..]`
fn load_examples() -> Value {
    let vec = read_dir("public/examples")
        .unwrap()
        .map(|entry| {
            let path = try!(entry).path();
            let name = String::from(path.file_stem().unwrap().to_str().unwrap());
            let mut file = try!(File::open(path));
            let mut contents = String::new();

            try!(file.read_to_string(&mut contents));

            let value = vec![("name".into(), Value::String(name)),
                             ("value".into(), Value::String(contents))];

            Ok(Value::Object(value.into_iter().collect()))
        })
        .collect::<io::Result<_>>()
        .unwrap();

    Value::Array(vec)
}

fn main() {
    env_logger::init().unwrap();
    let mut mount = Mount::new();

    mount.mount("/", Static::new("dist"));

    {
        let mut middleware = Chain::new(eval);

        let vm = try_gluon::make_eval_vm();
        middleware.link(persistent::Read::<VMKey>::both(vm));

        mount.mount("/eval", middleware);
    }

    {
        let mut middleware = Chain::new(examples);
        let examples_string = serde_json::to_string(&load_examples()).unwrap();

        middleware.link(persistent::Read::<Examples>::both(examples_string));
        mount.mount("/examples", middleware);
    }

    let address = "0.0.0.0:8080";

    // Dropping `server` causes it to block so keep it alive until the end of scope
    let _server = Iron::new(mount).http(address).unwrap();

    println!("Server started at `{}`", address);
}
