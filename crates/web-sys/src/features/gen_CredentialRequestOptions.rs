#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CredentialRequestOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CredentialRequestOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialRequestOptions`*"]
    pub type CredentialRequestOptions;
}
#[doc = "The trait to access properties on the `CredentialRequestOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `CredentialRequestOptions`*"]
pub trait CredentialRequestOptionsGetters {
    #[cfg(feature = "PublicKeyCredentialRequestOptions")]
    #[doc = "Get the `publicKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialRequestOptions`, `PublicKeyCredentialRequestOptions`*"]
    fn public_key(&self) -> &PublicKeyCredentialRequestOptions;
    #[cfg(feature = "AbortSignal")]
    #[doc = "Get the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `CredentialRequestOptions`*"]
    fn signal(&self) -> &AbortSignal;
}
impl CredentialRequestOptionsGetters for CredentialRequestOptions {
    #[cfg(feature = "PublicKeyCredentialRequestOptions")]
    fn public_key(&self) -> &PublicKeyCredentialRequestOptions {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("publicKey"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "AbortSignal")]
    fn signal(&self) -> &AbortSignal {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("signal"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl CredentialRequestOptions {
    #[doc = "Construct a new `CredentialRequestOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialRequestOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "PublicKeyCredentialRequestOptions")]
    #[doc = "Change the `publicKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialRequestOptions`, `PublicKeyCredentialRequestOptions`*"]
    pub fn public_key(&mut self, val: &PublicKeyCredentialRequestOptions) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("publicKey"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `CredentialRequestOptions`*"]
    pub fn signal(&mut self, val: &AbortSignal) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("signal"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for CredentialRequestOptions {
    fn default() -> Self {
        Self::new()
    }
}
