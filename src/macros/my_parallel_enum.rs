use citationberg;
use wasm_bindgen::prelude::*;

macro_rules! my_parallel_enum {
  ($name:ident, $target:path, { $($variant:ident),* $(,)? }) => {
      #[wasm_bindgen]
      #[derive(Clone)]
      pub enum $name {
          $($variant),*
      }

      impl From<$name> for $target {
          fn from(v: $name) -> Self {
              match v {
                  $($name::$variant => <$target>::$variant),*
              }
          }
      }

      impl From<$target> for $name {
          fn from(v: $target) -> Self {
              match v {
                  $(<$target>::$variant => $name::$variant),*
              }
          }
      }
  };
}

pub(crate) use my_parallel_enum;
