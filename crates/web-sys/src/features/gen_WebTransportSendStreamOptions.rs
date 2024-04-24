#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportSendStreamOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportSendStreamOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportSendStreamOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type WebTransportSendStreamOptions;
    #[wasm_bindgen(method, getter = "sendOrder")]
    fn send_order_shim(this: &WebTransportSendStreamOptions) -> Option<f64>;
    #[wasm_bindgen(method, setter = "sendOrder")]
    fn set_send_order_shim(this: &WebTransportSendStreamOptions, val: Option<f64>);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `WebTransportSendStreamOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WebTransportSendStreamOptions`*"]
pub trait WebTransportSendStreamOptionsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `sendOrder` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportSendStreamOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn send_order(&self) -> Option<f64>;
}
#[cfg(web_sys_unstable_apis)]
impl WebTransportSendStreamOptionsGetters for WebTransportSendStreamOptions {
    #[cfg(web_sys_unstable_apis)]
    fn send_order(&self) -> Option<f64> {
        self.send_order_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl WebTransportSendStreamOptions {
    #[doc = "Construct a new `WebTransportSendStreamOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportSendStreamOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `sendOrder` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportSendStreamOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn send_order(&mut self, val: Option<f64>) -> &mut Self {
        self.set_send_order_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for WebTransportSendStreamOptions {
    fn default() -> Self {
        Self::new()
    }
}
