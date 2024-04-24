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
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("fatal"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
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
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("fatal"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for TextDecoderOptions {
    fn default() -> Self {
        Self::new()
    }
}
