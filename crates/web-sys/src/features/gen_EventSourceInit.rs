#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = EventSourceInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EventSourceInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSourceInit`*"]
    pub type EventSourceInit;
    #[wasm_bindgen(method, getter = "withCredentials")]
    fn with_credentials_shim(this: &EventSourceInit) -> bool;
    #[wasm_bindgen(method, setter = "withCredentials")]
    fn set_with_credentials_shim(this: &EventSourceInit, val: bool);
}
#[doc = "The trait to access properties on the `EventSourceInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `EventSourceInit`*"]
pub trait EventSourceInitGetters {
    #[doc = "Get the `withCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSourceInit`*"]
    fn with_credentials(&self) -> bool;
}
impl EventSourceInitGetters for EventSourceInit {
    fn with_credentials(&self) -> bool {
        self.with_credentials_shim()
    }
}
impl EventSourceInit {
    #[doc = "Construct a new `EventSourceInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSourceInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `withCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSourceInit`*"]
    pub fn with_credentials(&mut self, val: bool) -> &mut Self {
        self.set_with_credentials_shim(val);
        self
    }
}
impl Default for EventSourceInit {
    fn default() -> Self {
        Self::new()
    }
}
