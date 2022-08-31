use core::transform::transform;

use shared::swc::config::Options;
use transform::types::Extensions;

#[test]
fn test() {
  let code = "const a = {};
  const b = { ...a }";
  let config = transform::types::TransformConfig {
    swc: Options {
      config: shared::serde_json::from_str(
        r#"{
        "jsc": {
          "externalHelpers": true
        }
      }"#,
      )
      .unwrap(),
      ..Default::default()
    },
    extensions: Extensions {
      plugin_import: None,
    },
  };

  let res = transform(config, code).unwrap();
  insta::assert_snapshot!("plugin-import", res.code);
}
