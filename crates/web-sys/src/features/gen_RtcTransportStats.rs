#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCTransportStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcTransportStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    pub type RtcTransportStats;
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &RtcTransportStats) -> String;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &RtcTransportStats, val: &str);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &RtcTransportStats) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &RtcTransportStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &RtcTransportStats) -> RtcStatsType;
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &RtcTransportStats, val: RtcStatsType);
    #[wasm_bindgen(method, getter = "bytesReceived")]
    fn bytes_received_shim(this: &RtcTransportStats) -> u32;
    #[wasm_bindgen(method, setter = "bytesReceived")]
    fn set_bytes_received_shim(this: &RtcTransportStats, val: u32);
    #[wasm_bindgen(method, getter = "bytesSent")]
    fn bytes_sent_shim(this: &RtcTransportStats) -> u32;
    #[wasm_bindgen(method, setter = "bytesSent")]
    fn set_bytes_sent_shim(this: &RtcTransportStats, val: u32);
}
#[doc = "The trait to access properties on the `RtcTransportStats` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
pub trait RtcTransportStatsGetters {
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    fn id(&self) -> String;
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    fn timestamp(&self) -> f64;
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsType`, `RtcTransportStats`*"]
    fn type_(&self) -> RtcStatsType;
    #[doc = "Get the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    fn bytes_received(&self) -> u32;
    #[doc = "Get the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    fn bytes_sent(&self) -> u32;
}
impl RtcTransportStatsGetters for RtcTransportStats {
    fn id(&self) -> String {
        self.id_shim()
    }
    fn timestamp(&self) -> f64 {
        self.timestamp_shim()
    }
    #[cfg(feature = "RtcStatsType")]
    fn type_(&self) -> RtcStatsType {
        self.type__shim()
    }
    fn bytes_received(&self) -> u32 {
        self.bytes_received_shim()
    }
    fn bytes_sent(&self) -> u32 {
        self.bytes_sent_shim()
    }
}
impl RtcTransportStats {
    #[doc = "Construct a new `RtcTransportStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsType`, `RtcTransportStats`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    pub fn bytes_received(&mut self, val: u32) -> &mut Self {
        self.set_bytes_received_shim(val);
        self
    }
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    pub fn bytes_sent(&mut self, val: u32) -> &mut Self {
        self.set_bytes_sent_shim(val);
        self
    }
}
impl Default for RtcTransportStats {
    fn default() -> Self {
        Self::new()
    }
}
