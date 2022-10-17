use modern_swc_core::types::Extensions;
use std::collections::HashMap;

use super::plugin_dynamic_import_node::DynImportNodeConfigNapi;
use super::plugin_emotion::EmotionOptionsNapi;
use super::plugin_lock_corejs_version::LockCoreJsVersionNapi;
use super::plugin_modularize_imports::PackageConfigNapi;
use super::plugin_react_utils;
use super::plugin_styled_components::StyledComponentsConfigNapi;
use super::{plugin_import, IntoRawConfig};

use napi_derive::napi;
/**
 * Internal plugin
 */
#[derive(Default)]
#[napi(object)]
pub struct ExtensionsNapi {
  pub modularize_imports: Option<HashMap<String, PackageConfigNapi>>,
  pub plugin_import: Option<Vec<plugin_import::PluginImportConfigNapi>>,
  pub react_utils: Option<plugin_react_utils::ReactUtilsConfigNapi>,
  pub lock_corejs_version: Option<LockCoreJsVersionNapi>,

  pub emotion: Option<EmotionOptionsNapi>,
  pub styled_components: Option<StyledComponentsConfigNapi>,
  pub styled_jsx: Option<bool>,

  pub dyn_import_node: Option<DynImportNodeConfigNapi>
}

impl IntoRawConfig<Extensions> for ExtensionsNapi {
  fn into_raw_config(self, env: napi::Env) -> napi::Result<Extensions> {
    let Self {
      modularize_imports,
      plugin_import,
      react_utils,
      lock_corejs_version,
      emotion,
      styled_components,
      styled_jsx,
      dyn_import_node
    } = self;

    Ok(Extensions {
      modularize_imports: modularize_imports.into_raw_config(env)?,
      plugin_import: plugin_import.into_raw_config(env)?,
      react_utils: react_utils.into_raw_config(env)?,
      lock_corejs_version: lock_corejs_version.into_raw_config(env)?,
      emotion: emotion.into_raw_config(env)?,
      styled_components: styled_components.into_raw_config(env)?,
      styled_jsx,
      dyn_import_node: dyn_import_node.into_raw_config(env)?
    })
  }
}
