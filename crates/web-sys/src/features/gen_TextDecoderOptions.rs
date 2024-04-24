#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TextDecoderOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextDecoderOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoderOptions`*"]
    pub type TextDecoderOptions;
    #[wasm_bindgen(method, getter = "fatal")]
    fn fatal_shim(this: &TextDecoderOptions) -> bool;
    #[wasm_bindgen(method, setter = "fatal")]
    fn set_fatal_shim(this: &TextDecoderOptions, val: bool);
}
#[doc = "The trait to access properties on the `TextDecoderOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `TextDecoderOptions`*"]
pub trait TextDecoderOptionsGetters {
    #[doc = "Get the `fatal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoderOptions`*"]
    fn fatal(&self) -> bool;
}
impl TextDecoderOptionsGetters for TextDecoderOptions {
    fn fatal(&self) -> bool {
        self.fatal_shim()
    }
}
impl TextDecoderOptions {
    #[doc = "Construct a new `TextDecoderOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoderOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `fatal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoderOptions`*"]
    pub fn fatal(&mut self, val: bool) -> &mut Self {
        self.set_fatal_shim(val);
        self
    }
}
impl Default for TextDecoderOptions {
    fn default() -> Self {
        Self::new()
    }
}
