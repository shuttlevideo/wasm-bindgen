#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCOfferOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcOfferOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
    pub type RtcOfferOptions;
}
#[doc = "The trait to access properties on the `RtcOfferOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
pub trait RtcOfferOptionsGetters {
    #[doc = "Get the `iceRestart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
    fn ice_restart(&self) -> bool;
    #[doc = "Get the `offerToReceiveAudio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
    fn offer_to_receive_audio(&self) -> bool;
    #[doc = "Get the `offerToReceiveVideo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
    fn offer_to_receive_video(&self) -> bool;
}
impl RtcOfferOptionsGetters for RtcOfferOptions {
    fn ice_restart(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("iceRestart"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn offer_to_receive_audio(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("offerToReceiveAudio"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn offer_to_receive_video(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("offerToReceiveVideo"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl RtcOfferOptions {
    #[doc = "Construct a new `RtcOfferOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `iceRestart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
    pub fn ice_restart(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("iceRestart"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `offerToReceiveAudio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
    pub fn offer_to_receive_audio(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("offerToReceiveAudio"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `offerToReceiveVideo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
    pub fn offer_to_receive_video(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("offerToReceiveVideo"),
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
impl Default for RtcOfferOptions {
    fn default() -> Self {
        Self::new()
    }
}
