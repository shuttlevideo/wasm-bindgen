#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ObserverCallback)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ObserverCallback` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ObserverCallback`*"]
    pub type ObserverCallback;
}
#[doc = "The trait to access properties on the `ObserverCallback` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ObserverCallback`*"]
pub trait ObserverCallbackGetters {
    #[doc = "Get the `handleEvent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ObserverCallback`*"]
    fn handle_event(&self) -> &::js_sys::Function;
}
impl ObserverCallbackGetters for ObserverCallback {
    fn handle_event(&self) -> &::js_sys::Function {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("handleEvent"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl ObserverCallback {
    #[doc = "Construct a new `ObserverCallback`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ObserverCallback`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `handleEvent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ObserverCallback`*"]
    pub fn handle_event(&mut self, val: &::js_sys::Function) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("handleEvent"),
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
impl Default for ObserverCallback {
    fn default() -> Self {
        Self::new()
    }
}
