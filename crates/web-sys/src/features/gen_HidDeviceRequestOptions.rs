#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HIDDeviceRequestOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HidDeviceRequestOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDeviceRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type HidDeviceRequestOptions;
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `HidDeviceRequestOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HidDeviceRequestOptions`*"]
pub trait HidDeviceRequestOptionsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `filters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDeviceRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn filters(&self) -> &::wasm_bindgen::JsValue;
}
#[cfg(web_sys_unstable_apis)]
impl HidDeviceRequestOptionsGetters for HidDeviceRequestOptions {
    #[cfg(web_sys_unstable_apis)]
    fn filters(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("filters"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
#[cfg(web_sys_unstable_apis)]
impl HidDeviceRequestOptions {
    #[doc = "Construct a new `HidDeviceRequestOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDeviceRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(filters: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.filters(filters);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `filters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDeviceRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn filters(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("filters"),
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
