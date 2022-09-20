use crate::types::TransformConfig;
use either::Either;
use napi::Env;
use react_utils::react_utils;
use shared::{swc_common::chain, swc_ecma_transforms_base::pass::noop, swc_ecma_visit::Fold};

use modularize_imports::{modularize_imports, Config as ModularizedConfig};
use plugin_import::plugin_import;

pub fn internal_transform_pass<'a>(env: Option<Env>, config: &'a TransformConfig) -> impl Fold + 'a {
  let extensions = &config.extensions;

  let modularize_imports = extensions
    .modularize_imports
    .as_ref()
    .map(|config| {
      Either::Left(modularize_imports(ModularizedConfig {
        packages: config.clone(),
      }))
    })
    .unwrap_or(Either::Right(noop()));

  let plugin_import = extensions
    .plugin_import
    .as_ref()
    .map(|config| Either::Left(plugin_import(config, env)))
    .unwrap_or(Either::Right(noop()));

  let react_utils = if let Some(c) = &extensions.react_utils {
    Either::Left(react_utils(&c))
  } else {
    Either::Right(noop())
  };

  chain!(modularize_imports, plugin_import, react_utils)
}
