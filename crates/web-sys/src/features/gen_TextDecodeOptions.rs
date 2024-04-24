#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TextDecodeOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextDecodeOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`*"]
    pub type TextDecodeOptions;
    #[wasm_bindgen(method, getter = "stream")]
    fn stream_shim(this: &TextDecodeOptions) -> bool;
    #[wasm_bindgen(method, setter = "stream")]
    fn set_stream_shim(this: &TextDecodeOptions, val: bool);
}
#[doc = "The trait to access properties on the `TextDecodeOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`*"]
pub trait TextDecodeOptionsGetters {
    #[doc = "Get the `stream` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`*"]
    fn stream(&self) -> bool;
}
impl TextDecodeOptionsGetters for TextDecodeOptions {
    fn stream(&self) -> bool {
        self.stream_shim()
    }
}
impl TextDecodeOptions {
    #[doc = "Construct a new `TextDecodeOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `stream` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`*"]
    pub fn stream(&mut self, val: bool) -> &mut Self {
        self.set_stream_shim(val);
        self
    }
}
impl Default for TextDecodeOptions {
    fn default() -> Self {
        Self::new()
    }
}
