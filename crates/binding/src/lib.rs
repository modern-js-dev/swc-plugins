mod binding_types;
mod thread_safe_function;
use binding_types::{IntoRawConfig, TransformConfigNapi};
use napi::{bindgen_prelude::AsyncTask, Env, JsObject, Result, Status, Task};

use napi_derive::napi;
use shared::serde_json;
use swc_core::{
  base::{
    config::{JsMinifyOptions, TerserSourceMapOption},
    Compiler as SwcCompiler, TransformOutput,
  },
  common::{
    collections::AHashMap,
    sync::{Lazy, RwLock},
    SourceMap,
  },
};

use std::{
  cell::RefCell,
  sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
  },
};
use swc_plugins_core::types::TransformConfig;

mod exit;
pub use exit::terminate_process;

// ===== Internal Rust struct under the hood =====
pub struct Compiler {
  pub id: u32,
  pub config: TransformConfig,
  pub swc_compiler: Arc<SwcCompiler>,
}

// js id -> Rust Compiler
pub static COMPILERS: Lazy<Arc<RwLock<AHashMap<u32, Compiler>>>> =
  Lazy::new(|| Arc::new(RwLock::new(AHashMap::default())));

static ID: AtomicU32 = AtomicU32::new(0);

thread_local! {
  pub static IS_SYNC: RefCell<bool> = RefCell::new(false)
}

// ========== API exposed to js ==========
// this id maps to a real struct in COMPILERS

#[napi(js_name = "Compiler")]
pub struct JsCompiler {
  pub id: u32,
}

#[napi]
impl JsCompiler {
  #[napi(constructor)]
  pub fn new(env: Env, config: TransformConfigNapi) -> Self {
    let id = ID.fetch_add(1, Ordering::Relaxed);

    let config = IntoRawConfig::into_raw_config(config, env).unwrap();

    let mut compilers = COMPILERS.write();
    compilers.insert(
      id,
      Compiler {
        id,
        config,
        swc_compiler: Arc::new(SwcCompiler::new(Arc::new(SourceMap::default()))),
      },
    );

    Self { id }
  }

  #[napi]
  pub fn transform(
    &self,
    filename: String,
    code: String,
    map: Option<String>,
  ) -> AsyncTask<TransformTask> {
    AsyncTask::new(TransformTask {
      code,
      filename,
      map,
      compiler_id: self.id,
    })
  }

  #[napi]
  pub fn transform_sync(
    &self,
    filename: String,
    code: String,
    map: Option<String>,
  ) -> Result<Output> {
    // This hack is for distinguish if transform is async or not, if yes, using threadsafe function, else using sync JS call
    IS_SYNC.with(|is_sync| {
      is_sync.replace(true);

      TransformTask {
        code,
        filename,
        map,
        compiler_id: self.id,
      }
      .transform()
      .map(|output| {
        let TransformOutput { code, map } = output;
        Output { code, map }
      })
    })
  }

  #[napi]
  pub fn release(&self) {
    let mut compilers = COMPILERS.write();
    compilers.remove(&self.id);
  }
}

#[napi]
pub fn minify(
  config: String,
  filename: String,
  code: String,
  map: Option<String>,
) -> AsyncTask<Minifier> {
  AsyncTask::new(Minifier::new(config, filename, code, map))
}

#[napi]
pub fn minify_sync(
  config: String,
  filename: String,
  code: String,
  map: Option<String>,
) -> Result<Output> {
  Minifier::new(config, filename, code, map)
    .minify()
    .map(|output| {
      let TransformOutput { code, map } = output;
      Output { code, map }
    })
}

// ======= Napi boiler plate code =======
#[napi(object)]
pub struct Output {
  pub code: String,
  pub map: Option<String>,
}

impl From<TransformOutput> for Output {
  fn from(o: TransformOutput) -> Self {
    Self {
      code: o.code,
      map: o.map,
    }
  }
}

pub struct TransformTask {
  pub compiler_id: u32,
  pub code: String,
  pub filename: String,
  pub map: Option<String>,
}

impl TransformTask {
  fn transform(&mut self) -> Result<TransformOutput> {
    let compilers = COMPILERS.read();

    let compiler = compilers
      .get(&self.compiler_id)
      .expect("Compiler is released, maybe you are using compiler after call release()");

    swc_plugins_core::transform(
      compiler.swc_compiler.clone(),
      &compiler.config,
      &self.filename,
      &self.code,
      self.map.clone(),
      Some(compiler.id.to_string()),
    )
    .map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))
  }
}

impl Task for TransformTask {
  type Output = TransformOutput;
  type JsValue = JsObject;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    IS_SYNC.with(|is_sync| {
      is_sync.replace(false);
      self.transform()
    })
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    let mut obj = env.create_object()?;
    obj.set_named_property("code", env.create_string(&output.code)?)?;
    if let Some(map) = output.map {
      obj.set_named_property("map", env.create_string(&map)?)?;
    }
    Ok(obj)
  }
}

pub struct Minifier {
  code: String,
  filename: String,
  config: JsMinifyOptions,
}

impl Minifier {
  pub fn new(config: String, filename: String, code: String, map: Option<String>) -> Self {
    let mut js_minify_options: JsMinifyOptions =
      serde_json::from_slice(&<String as Into<Vec<_>>>::into(config)).unwrap();

    if let Some(m) = map {
      let m: TerserSourceMapOption = serde_json::from_str(&m).expect("Invalid SourceMap string");
      js_minify_options.source_map = m.into();
    }

    Self {
      code,
      filename,
      config: js_minify_options,
    }
  }

  fn minify(&self) -> Result<TransformOutput> {
    swc_plugins_core::minify(&self.config, self.filename.clone(), &self.code)
      .map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))
  }
}

impl Task for Minifier {
  type Output = TransformOutput;
  type JsValue = JsObject;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    self.minify()
  }

  fn resolve(&mut self, env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    let mut obj = env.create_object()?;
    obj.set_named_property("code", env.create_string(&output.code)?)?;
    if let Some(map) = output.map {
      obj.set_named_property("map", env.create_string(&map)?)?;
    }

    Ok(obj)
  }
}
