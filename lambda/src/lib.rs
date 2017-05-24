#[macro_use(lambda)]
extern crate crowbar;
#[macro_use]
extern crate cpython;

extern crate try_gluon;

lambda!(|event, context| {
    println!("hi cloudwatch logs, this is {}", context.function_name());
    let vm = try_gluon::make_eval_vm();
    let expr = match event {
        crowbar::Value::String(expr) => expr,
        _ => return Err("Unexpected input".into()),
    };
    let msg = try_gluon::eval(&vm, &expr);
    // return the event without doing anything with it
    Ok(crowbar::Value::String(msg))
});
