#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BasicCardRequest)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BasicCardRequest` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
    pub type BasicCardRequest;
    #[wasm_bindgen(method, getter = "supportedNetworks")]
    fn supported_networks_shim(this: &BasicCardRequest) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "supportedNetworks")]
    fn set_supported_networks_shim(this: &BasicCardRequest, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "supportedTypes")]
    fn supported_types_shim(this: &BasicCardRequest) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "supportedTypes")]
    fn set_supported_types_shim(this: &BasicCardRequest, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `BasicCardRequest` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
pub trait BasicCardRequestGetters {
    #[doc = "Get the `supportedNetworks` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
    fn supported_networks(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `supportedTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
    fn supported_types(&self) -> &::wasm_bindgen::JsValue;
}
impl BasicCardRequestGetters for BasicCardRequest {
    fn supported_networks(&self) -> &::wasm_bindgen::JsValue {
        self.supported_networks_shim()
    }
    fn supported_types(&self) -> &::wasm_bindgen::JsValue {
        self.supported_types_shim()
    }
}
impl BasicCardRequest {
    #[doc = "Construct a new `BasicCardRequest`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `supportedNetworks` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
    pub fn supported_networks(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_supported_networks_shim(val);
        self
    }
    #[doc = "Change the `supportedTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
    pub fn supported_types(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_supported_types_shim(val);
        self
    }
}
impl Default for BasicCardRequest {
    fn default() -> Self {
        Self::new()
    }
}
