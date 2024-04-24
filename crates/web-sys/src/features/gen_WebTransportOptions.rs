#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type WebTransportOptions;
    #[wasm_bindgen(method, getter = "allowPooling")]
    fn allow_pooling_shim(this: &WebTransportOptions) -> bool;
    #[wasm_bindgen(method, setter = "allowPooling")]
    fn set_allow_pooling_shim(this: &WebTransportOptions, val: bool);
    #[cfg(feature = "WebTransportCongestionControl")]
    #[wasm_bindgen(method, getter = "congestionControl")]
    fn congestion_control_shim(this: &WebTransportOptions) -> WebTransportCongestionControl;
    #[cfg(feature = "WebTransportCongestionControl")]
    #[wasm_bindgen(method, setter = "congestionControl")]
    fn set_congestion_control_shim(this: &WebTransportOptions, val: WebTransportCongestionControl);
    #[wasm_bindgen(method, getter = "requireUnreliable")]
    fn require_unreliable_shim(this: &WebTransportOptions) -> bool;
    #[wasm_bindgen(method, setter = "requireUnreliable")]
    fn set_require_unreliable_shim(this: &WebTransportOptions, val: bool);
    #[wasm_bindgen(method, getter = "serverCertificateHashes")]
    fn server_certificate_hashes_shim(this: &WebTransportOptions) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "serverCertificateHashes")]
    fn set_server_certificate_hashes_shim(
        this: &WebTransportOptions,
        val: &::wasm_bindgen::JsValue,
    );
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `WebTransportOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WebTransportOptions`*"]
pub trait WebTransportOptionsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `allowPooling` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn allow_pooling(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportCongestionControl")]
    #[doc = "Get the `congestionControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportCongestionControl`, `WebTransportOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn congestion_control(&self) -> WebTransportCongestionControl;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `requireUnreliable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn require_unreliable(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `serverCertificateHashes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn server_certificate_hashes(&self) -> &::wasm_bindgen::JsValue;
}
#[cfg(web_sys_unstable_apis)]
impl WebTransportOptionsGetters for WebTransportOptions {
    #[cfg(web_sys_unstable_apis)]
    fn allow_pooling(&self) -> bool {
        self.allow_pooling_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportCongestionControl")]
    fn congestion_control(&self) -> WebTransportCongestionControl {
        self.congestion_control_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn require_unreliable(&self) -> bool {
        self.require_unreliable_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn server_certificate_hashes(&self) -> &::wasm_bindgen::JsValue {
        self.server_certificate_hashes_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl WebTransportOptions {
    #[doc = "Construct a new `WebTransportOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `allowPooling` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn allow_pooling(&mut self, val: bool) -> &mut Self {
        self.set_allow_pooling_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportCongestionControl")]
    #[doc = "Change the `congestionControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportCongestionControl`, `WebTransportOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn congestion_control(&mut self, val: WebTransportCongestionControl) -> &mut Self {
        self.set_congestion_control_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `requireUnreliable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn require_unreliable(&mut self, val: bool) -> &mut Self {
        self.set_require_unreliable_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `serverCertificateHashes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn server_certificate_hashes(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_server_certificate_hashes_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for WebTransportOptions {
    fn default() -> Self {
        Self::new()
    }
}
