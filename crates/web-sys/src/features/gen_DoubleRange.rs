#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DoubleRange)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DoubleRange` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type DoubleRange;
    #[wasm_bindgen(method, getter = "max")]
    fn max_shim(this: &DoubleRange) -> f64;
    #[wasm_bindgen(method, setter = "max")]
    fn set_max_shim(this: &DoubleRange, val: f64);
    #[wasm_bindgen(method, getter = "min")]
    fn min_shim(this: &DoubleRange) -> f64;
    #[wasm_bindgen(method, setter = "min")]
    fn set_min_shim(this: &DoubleRange, val: f64);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `DoubleRange` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DoubleRange`*"]
pub trait DoubleRangeGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn max(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn min(&self) -> f64;
}
#[cfg(web_sys_unstable_apis)]
impl DoubleRangeGetters for DoubleRange {
    #[cfg(web_sys_unstable_apis)]
    fn max(&self) -> f64 {
        self.max_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn min(&self) -> f64 {
        self.min_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl DoubleRange {
    #[doc = "Construct a new `DoubleRange`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max(&mut self, val: f64) -> &mut Self {
        self.set_max_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn min(&mut self, val: f64) -> &mut Self {
        self.set_min_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for DoubleRange {
    fn default() -> Self {
        Self::new()
    }
}
