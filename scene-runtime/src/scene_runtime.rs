use deno_core::v8;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;

use crate::interface;

pub struct SceneRuntime {
  js: JsRuntime
}

impl SceneRuntime {
  fn new() -> SceneRuntime {
    let mut runtime = JsRuntime::new(RuntimeOptions {
      extensions: interface::ops(),
    ..Default::default()
    });

    let js_interface = include_str!("interface.js");

    Self::eval(&mut runtime, js_interface).expect("Interface Eval Failed");

    //TODO: Inject DecentralandInterface with Ops and glue code or 
      // globalThis.MagicSDK = {
      //   foo(x, y) {
      //     return Deno.core.opSync("op_sdk_foo", x, y);
      //   }
        
      //   async bar(x) {
      //     const y = await Deno.core.opAsync("op_sdk_bar", x);
      //     return y + 3.141;
      //   }
      // }
      // Only requirement is that args are (serde) deserializable and returns serializable
      // serde_v8 bijects Rust & v8 values
      // But for example if you're consuming some event stream originating in rust (even a TCP socket) essentially your opcalls can be polls that resolve to a promise on success


    SceneRuntime {
      js: runtime
    }
  }

  fn run<T>(&mut self, code: &T) -> dcl_common::Result<String>
  where T: AsRef<str> {
    let output: serde_json::Value = Self::eval(&mut self.js, code).expect("Eval failed");
    Ok("".to_string())
  }

  fn eval<T>(context: &mut JsRuntime, code: &T) -> Result<serde_json::Value, String>
  where T: AsRef<str> + ?Sized {
    let res = context.execute_script("SceneRuntime", code.as_ref());
    match res {
      Ok(global) => {
        let scope = &mut context.handle_scope();
        let local = v8::Local::new(scope, global);
        // Deserialize a `v8` object into a Rust type using `serde_v8`,
        // in this case deserialize to a JSON `Value`.
        let deserialized_value =
          serde_v8::from_v8::<serde_json::Value>(scope, local);

        match deserialized_value {
          Ok(value) => Ok(value),
          Err(err) => Err(format!("Cannot deserialize value: {:?}", err)),
        }
      }
      Err(err) => Err(format!("Evaling error: {:?}", err)),
    }
  }
}

#[cfg(test)]
mod test {
  use crate::*;
  use std::fs;

  #[test]
  fn it_setups_the_context_for_dcl_scenes()
  {
    let mut runtime = SceneRuntime::new();

    let scene_code = fs::read_to_string("../fixtures/old_scene.js").unwrap();
    let result = runtime.run(&scene_code).unwrap();
    
    assert!(true)
  }
}
