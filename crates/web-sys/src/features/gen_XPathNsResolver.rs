#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = XPathNSResolver)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XPathNsResolver` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathNsResolver`*"]
    pub type XPathNsResolver;
    #[wasm_bindgen(method, getter = "lookupNamespaceURI")]
    fn lookup_namespace_uri_shim(this: &XPathNsResolver) -> &::js_sys::Function;
    #[wasm_bindgen(method, setter = "lookupNamespaceURI")]
    fn set_lookup_namespace_uri_shim(this: &XPathNsResolver, val: &::js_sys::Function);
}
#[doc = "The trait to access properties on the `XPathNsResolver` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `XPathNsResolver`*"]
pub trait XPathNsResolverGetters {
    #[doc = "Get the `lookupNamespaceURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathNsResolver`*"]
    fn lookup_namespace_uri(&self) -> &::js_sys::Function;
}
impl XPathNsResolverGetters for XPathNsResolver {
    fn lookup_namespace_uri(&self) -> &::js_sys::Function {
        self.lookup_namespace_uri_shim()
    }
}
impl XPathNsResolver {
    #[doc = "Construct a new `XPathNsResolver`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathNsResolver`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `lookupNamespaceURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathNsResolver`*"]
    pub fn lookup_namespace_uri(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_lookup_namespace_uri_shim(val);
        self
    }
}
impl Default for XPathNsResolver {
    fn default() -> Self {
        Self::new()
    }
}
