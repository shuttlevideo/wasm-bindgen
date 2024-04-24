#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HashChangeEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HashChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    pub type HashChangeEventInit;
}
#[doc = "The trait to access properties on the `HashChangeEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
pub trait HashChangeEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `newURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    fn new_url(&self) -> &str;
    #[doc = "Get the `oldURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    fn old_url(&self) -> &str;
}
impl HashChangeEventInitGetters for HashChangeEventInit {
    fn bubbles(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("bubbles"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn cancelable(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("cancelable"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn composed(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("composed"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn new_url(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("newURL"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn old_url(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("oldURL"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl HashChangeEventInit {
    #[doc = "Construct a new `HashChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("bubbles"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("cancelable"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("composed"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `newURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    pub fn new_url(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("newURL"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `oldURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
    pub fn old_url(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("oldURL"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for HashChangeEventInit {
    fn default() -> Self {
        Self::new()
    }
}
