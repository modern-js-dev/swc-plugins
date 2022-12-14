use shared::{
  serde::Deserialize,
};
use swc_core::{
  common::DUMMY_SP,
  ecma::{
    ast::{CallExpr, Callee, ImportDecl, Lit, Str},
    atoms::JsWord,
    visit::{as_folder, Fold, VisitMut},
  },
};

static COREJS: &str = "core-js";
static SWC_HELPERS: &str = "@swc/helpers";

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "shared::serde", rename_all = "camelCase")]
pub struct LockCoreJsVersion {
  pub swc_helpers: String,
  pub corejs: String,
}

impl LockCoreJsVersion {
  fn need_replace(&self, value: impl AsRef<str>) -> Option<String> {
    if value.as_ref().starts_with(COREJS) {
      Some(value.as_ref().replace(COREJS, &self.corejs))
    } else if value.as_ref().starts_with(SWC_HELPERS) {
      Some(value.as_ref().replace(SWC_HELPERS, &self.swc_helpers))
    } else {
      None
    }
  }
}

impl VisitMut for LockCoreJsVersion {
  fn visit_mut_import_decl(&mut self, decl: &mut ImportDecl) {
    if let Some(value) = self.need_replace(&decl.src.value) {
      decl.src = Box::new(Str {
        span: DUMMY_SP,
        value: JsWord::from(value.as_str()),
        raw: None,
      });
    }
  }

  fn visit_mut_call_expr(&mut self, call_expr: &mut CallExpr) {
    match &call_expr.callee {
      Callee::Import(_) => {
        // import('core-js')
        if let Some(Lit::Str(specifier)) = call_expr.args[0].expr.as_mut_lit() {
          if let Some(replaced) = self.need_replace(&specifier.value) {
            *specifier = Str {
              span: DUMMY_SP,
              value: JsWord::from(replaced.as_str()),
              raw: None,
            };
          }
        }
      }

      Callee::Expr(expr) => {
        if let Some(id) = expr.as_ident() {
          if &id.sym != "require" {
            return;
          }

          // require('core-js')
          if let Some(Lit::Str(specifier)) = call_expr.args[0].expr.as_mut_lit() {
            if let Some(value) = self.need_replace(&specifier.value) {
              specifier.span = DUMMY_SP;
              specifier.value = JsWord::from(value);
            }
          }
        }
      }
      _ => {}
    }
  }
}

pub fn lock_corejs_version(corejs: String, swc_helpers: String) -> impl Fold {
  as_folder(LockCoreJsVersion {
    corejs,
    swc_helpers,
  })
}
