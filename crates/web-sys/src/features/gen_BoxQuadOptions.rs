#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BoxQuadOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BoxQuadOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`*"]
    pub type BoxQuadOptions;
}
#[doc = "The trait to access properties on the `BoxQuadOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`*"]
pub trait BoxQuadOptionsGetters {
    #[cfg(feature = "CssBoxType")]
    #[doc = "Get the `box` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`, `CssBoxType`*"]
    fn box_(&self) -> CssBoxType;
    #[doc = "Get the `relativeTo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`*"]
    fn relative_to(&self) -> &::js_sys::Object;
}
impl BoxQuadOptionsGetters for BoxQuadOptions {
    #[cfg(feature = "CssBoxType")]
    fn box_(&self) -> CssBoxType {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("box"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn relative_to(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("relativeTo"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl BoxQuadOptions {
    #[doc = "Construct a new `BoxQuadOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "CssBoxType")]
    #[doc = "Change the `box` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`, `CssBoxType`*"]
    pub fn box_(&mut self, val: CssBoxType) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("box"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `relativeTo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`*"]
    pub fn relative_to(&mut self, val: &::js_sys::Object) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("relativeTo"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for BoxQuadOptions {
    fn default() -> Self {
        Self::new()
    }
}
