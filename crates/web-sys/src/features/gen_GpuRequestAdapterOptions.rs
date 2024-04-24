#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPURequestAdapterOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuRequestAdapterOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRequestAdapterOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuRequestAdapterOptions;
    #[wasm_bindgen(method, getter = "forceFallbackAdapter")]
    fn force_fallback_adapter_shim(this: &GpuRequestAdapterOptions) -> bool;
    #[wasm_bindgen(method, setter = "forceFallbackAdapter")]
    fn set_force_fallback_adapter_shim(this: &GpuRequestAdapterOptions, val: bool);
    #[cfg(feature = "GpuPowerPreference")]
    #[wasm_bindgen(method, getter = "powerPreference")]
    fn power_preference_shim(this: &GpuRequestAdapterOptions) -> GpuPowerPreference;
    #[cfg(feature = "GpuPowerPreference")]
    #[wasm_bindgen(method, setter = "powerPreference")]
    fn set_power_preference_shim(this: &GpuRequestAdapterOptions, val: GpuPowerPreference);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuRequestAdapterOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuRequestAdapterOptions`*"]
pub trait GpuRequestAdapterOptionsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `forceFallbackAdapter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRequestAdapterOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn force_fallback_adapter(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPowerPreference")]
    #[doc = "Get the `powerPreference` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPowerPreference`, `GpuRequestAdapterOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn power_preference(&self) -> GpuPowerPreference;
}
#[cfg(web_sys_unstable_apis)]
impl GpuRequestAdapterOptionsGetters for GpuRequestAdapterOptions {
    #[cfg(web_sys_unstable_apis)]
    fn force_fallback_adapter(&self) -> bool {
        self.force_fallback_adapter_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPowerPreference")]
    fn power_preference(&self) -> GpuPowerPreference {
        self.power_preference_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuRequestAdapterOptions {
    #[doc = "Construct a new `GpuRequestAdapterOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRequestAdapterOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `forceFallbackAdapter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRequestAdapterOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn force_fallback_adapter(&mut self, val: bool) -> &mut Self {
        self.set_force_fallback_adapter_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPowerPreference")]
    #[doc = "Change the `powerPreference` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPowerPreference`, `GpuRequestAdapterOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn power_preference(&mut self, val: GpuPowerPreference) -> &mut Self {
        self.set_power_preference_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for GpuRequestAdapterOptions {
    fn default() -> Self {
        Self::new()
    }
}
