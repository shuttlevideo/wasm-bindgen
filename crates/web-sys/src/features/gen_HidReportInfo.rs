#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HIDReportInfo)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HidReportInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type HidReportInfo;
    #[wasm_bindgen(method, getter = "items")]
    fn items_shim(this: &HidReportInfo) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "items")]
    fn set_items_shim(this: &HidReportInfo, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "reportId")]
    fn report_id_shim(this: &HidReportInfo) -> u8;
    #[wasm_bindgen(method, setter = "reportId")]
    fn set_report_id_shim(this: &HidReportInfo, val: u8);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `HidReportInfo` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HidReportInfo`*"]
pub trait HidReportInfoGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `items` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn items(&self) -> &::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `reportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn report_id(&self) -> u8;
}
#[cfg(web_sys_unstable_apis)]
impl HidReportInfoGetters for HidReportInfo {
    #[cfg(web_sys_unstable_apis)]
    fn items(&self) -> &::wasm_bindgen::JsValue {
        self.items_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn report_id(&self) -> u8 {
        self.report_id_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl HidReportInfo {
    #[doc = "Construct a new `HidReportInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `items` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn items(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_items_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `reportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn report_id(&mut self, val: u8) -> &mut Self {
        self.set_report_id_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for HidReportInfo {
    fn default() -> Self {
        Self::new()
    }
}
