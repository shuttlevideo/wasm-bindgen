#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStats`*"]
    pub type RtcStats;
}
#[doc = "The trait to access properties on the `RtcStats` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcStats`*"]
pub trait RtcStatsGetters {
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStats`*"]
    fn id(&self) -> &str;
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStats`*"]
    fn timestamp(&self) -> f64;
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStats`, `RtcStatsType`*"]
    fn type_(&self) -> RtcStatsType;
}
impl RtcStatsGetters for RtcStats {
    fn id(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("id"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn timestamp(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("timestamp"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "RtcStatsType")]
    fn type_(&self) -> RtcStatsType {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("type"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl RtcStats {
    #[doc = "Construct a new `RtcStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStats`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStats`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("id"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("timestamp"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("type"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for RtcStats {
    fn default() -> Self {
        Self::new()
    }
}
