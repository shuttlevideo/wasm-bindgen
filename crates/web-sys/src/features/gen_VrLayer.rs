#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = VRLayer)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrLayer` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrLayer`*"]
    pub type VrLayer;
}
#[doc = "The trait to access properties on the `VrLayer` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `VrLayer`*"]
pub trait VrLayerGetters {
    #[doc = "Get the `leftBounds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrLayer`*"]
    fn left_bounds(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `rightBounds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrLayer`*"]
    fn right_bounds(&self) -> &::wasm_bindgen::JsValue;
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `VrLayer`*"]
    fn source(&self) -> Option<&HtmlCanvasElement>;
}
impl VrLayerGetters for VrLayer {
    fn left_bounds(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("leftBounds"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn right_bounds(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("rightBounds"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "HtmlCanvasElement")]
    fn source(&self) -> Option<&HtmlCanvasElement> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("source"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl VrLayer {
    #[doc = "Construct a new `VrLayer`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrLayer`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `leftBounds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrLayer`*"]
    pub fn left_bounds(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("leftBounds"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `rightBounds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrLayer`*"]
    pub fn right_bounds(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("rightBounds"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `VrLayer`*"]
    pub fn source(&mut self, val: Option<&HtmlCanvasElement>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("source"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for VrLayer {
    fn default() -> Self {
        Self::new()
    }
}
