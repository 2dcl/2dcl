use js_sandbox::{Script, AnyError};

fn main() -> Result<(), AnyError> {
    let mut script = Script::from_file("examples/main.ts")?;

    // // or, at compile time:
    // let code: &'static str = include_str!("script.js");
    // let mut script = Script::from_string(code).expect("Init succeeds");

    let result: String = script.call("get", &())?;
    println!("{}", result);
    Ok(())
}
