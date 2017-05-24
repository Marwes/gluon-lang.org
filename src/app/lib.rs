extern crate futures;
extern crate gluon;
#[macro_use]
extern crate log;

use std::time::Instant;

use futures::Async;

use gluon::base::symbol::{Symbol, SymbolRef};
use gluon::base::kind::{ArcKind, KindEnv};
use gluon::base::types::{Alias, ArcType, TypeEnv};
use gluon::vm::thread::{RootedThread, Thread, ThreadInternal};
use gluon::vm::Error;
use gluon::vm::internal::ValuePrinter;
use gluon::vm::api::{Hole, OpaqueValue};
use gluon::Compiler;
use gluon::import::{DefaultImporter, Import};

pub struct EmptyEnv;

impl KindEnv for EmptyEnv {
    fn find_kind(&self, _type_name: &SymbolRef) -> Option<ArcKind> {
        None
    }
}

impl TypeEnv for EmptyEnv {
    fn find_type(&self, _id: &SymbolRef) -> Option<&ArcType> {
        None
    }
    fn find_type_info(&self, _id: &SymbolRef) -> Option<&Alias<Symbol, ArcType>> {
        None
    }
    fn find_record(&self, _fields: &[Symbol]) -> Option<(ArcType, ArcType)> {
        None
    }
}

pub fn make_eval_vm() -> RootedThread {
    let vm = RootedThread::new();

    // Ensure the import macro cannot be abused to to open files
    {
        // Ensure the lock to `paths` are released
        let import = Import::new(DefaultImporter);
        import.paths.write().unwrap().clear();
        vm.get_macros().insert(String::from("import"), import);
    }

    // Initialize the basic types such as `Bool` and `Option` so they are available when loading
    // other modules
    Compiler::new()
        .implicit_prelude(false)
        .run_expr::<OpaqueValue<&Thread, Hole>>(&vm, "", r#" import! "std/types.glu" "#)
        .unwrap();

    gluon::vm::primitives::load(&vm).expect("Loaded primitives library");
    // Load the io library so the prelude can be loaded (`IO` actions won't actually execute however)
    gluon::io::load(&vm).expect("Loaded IO library");

    vm
}

pub fn eval(vm: &Thread, body: &str) -> String {
    info!("Eval: `{}`", body);

    // Prevent a single thread from allocating to much memory
    vm.set_memory_limit(2_000_000);

    {
        let mut context = vm.context();

        // Prevent the stack from consuming to much memory
        context.set_max_stack_size(10000);

        // Prevent infinite loops from running forever
        let start = Instant::now();
        context.set_hook(Some(Box::new(move |_, _| if start.elapsed().as_secs() < 10 {
            Ok(Async::Ready(()))
        } else {
            Err(Error::Message("Thread has exceeded the allowed exection time".into()))
        })));
    }

    let (value, typ) =
        match Compiler::new().run_expr::<OpaqueValue<&Thread, Hole>>(&vm, "<top>", &body) {
            Ok(value) => value,
            Err(err) => return format!("{}", err),
        };

    unsafe {
        format!("{} : {}",
                ValuePrinter::new(&EmptyEnv, &typ, value.get_value()).max_level(6),
                typ)
    }
}
