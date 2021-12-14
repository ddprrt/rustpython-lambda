use lambda_runtime::{handler_fn, Context};
use rustpython::rustpython_vm as vm;
use serde_json::{json, Value};
use vm::PyResult;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: Value, _: Context) -> Result<Value, lambda_runtime::Error> {
    let source = event["source"].as_str().unwrap();

    let result: PyResult = vm::Interpreter::default().enter(|vm| {
        let scope = vm.new_scope_with_builtins();

        let code_obj = vm
            .compile(
                r#"print("Hello World!")"#,
                vm::compile::Mode::Exec,
                "<embedded>".to_owned(),
            )
            .map_err(|err| vm.new_syntax_error(&err))?;

        vm.run_code_obj(code_obj, scope)
    });

    Ok(Value::String("ok".to_string()))
}
