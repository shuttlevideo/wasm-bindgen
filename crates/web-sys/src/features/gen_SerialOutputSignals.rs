#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SerialOutputSignals)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SerialOutputSignals` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOutputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type SerialOutputSignals;
    #[wasm_bindgen(method, getter = "break")]
    fn break__shim(this: &SerialOutputSignals) -> bool;
    #[wasm_bindgen(method, setter = "break")]
    fn set_break__shim(this: &SerialOutputSignals, val: bool);
    #[wasm_bindgen(method, getter = "dataTerminalReady")]
    fn data_terminal_ready_shim(this: &SerialOutputSignals) -> bool;
    #[wasm_bindgen(method, setter = "dataTerminalReady")]
    fn set_data_terminal_ready_shim(this: &SerialOutputSignals, val: bool);
    #[wasm_bindgen(method, getter = "requestToSend")]
    fn request_to_send_shim(this: &SerialOutputSignals) -> bool;
    #[wasm_bindgen(method, setter = "requestToSend")]
    fn set_request_to_send_shim(this: &SerialOutputSignals, val: bool);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `SerialOutputSignals` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SerialOutputSignals`*"]
pub trait SerialOutputSignalsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `break` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOutputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn break_(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `dataTerminalReady` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOutputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn data_terminal_ready(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `requestToSend` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOutputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn request_to_send(&self) -> bool;
}
#[cfg(web_sys_unstable_apis)]
impl SerialOutputSignalsGetters for SerialOutputSignals {
    #[cfg(web_sys_unstable_apis)]
    fn break_(&self) -> bool {
        self.break__shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn data_terminal_ready(&self) -> bool {
        self.data_terminal_ready_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn request_to_send(&self) -> bool {
        self.request_to_send_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl SerialOutputSignals {
    #[doc = "Construct a new `SerialOutputSignals`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOutputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `break` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOutputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn break_(&mut self, val: bool) -> &mut Self {
        self.set_break__shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `dataTerminalReady` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOutputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn data_terminal_ready(&mut self, val: bool) -> &mut Self {
        self.set_data_terminal_ready_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `requestToSend` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOutputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_to_send(&mut self, val: bool) -> &mut Self {
        self.set_request_to_send_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for SerialOutputSignals {
    fn default() -> Self {
        Self::new()
    }
}
