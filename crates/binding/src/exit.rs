use napi::{bindgen_prelude::AsyncTask, JsUndefined, Task};
use napi_derive::napi;

#[napi]
/// terminate the process when node.js process.exit
pub fn terminate_process(code: i32) -> AsyncTask<Exit> {
  AsyncTask::new(Exit(code))
}

pub struct Exit(i32);

#[napi]
impl Task for Exit {
  type Output = ();

  type JsValue = JsUndefined;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    if self.0 == 0 {
      // It's mean that node.js exit normally, if the code equal to zero.
      return Ok(());
    }
    std::process::exit(self.0);
  }

  fn resolve(&mut self, env: napi::Env, _output: Self::Output) -> napi::Result<Self::JsValue> {
    env.get_undefined()
  }
}
