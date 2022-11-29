use std::sync::Arc;

// reexports some same version libs
pub use anyhow;
pub use serde_json;
pub use swc_core;
use swc_core::common::{comments::SingleThreadedComments, Mark, SourceMap};
pub use swc_core::ecma::transforms::testing as swc_ecma_transforms_testing;
pub use testing;
pub extern crate serde;
pub use ahash;
pub use dashmap;

pub struct PluginContext {
  pub cm: Arc<SourceMap>,
  pub top_level_mark: Mark,
  pub unresolved_mark: Mark,
  pub comments: SingleThreadedComments,
  pub config_hash: Option<String>, // This can be used by plugins to do caching

  // Use this to determine if we should remove __esModule mark in pure commonjs module
  // Remove this when SWC fix https://github.com/swc-project/swc/issues/6500
  pub is_source_esm: bool,
}
