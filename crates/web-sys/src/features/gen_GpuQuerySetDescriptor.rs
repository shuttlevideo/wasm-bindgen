#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUQuerySetDescriptor)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuQuerySetDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQuerySetDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuQuerySetDescriptor;
    #[wasm_bindgen(method, getter = "label")]
    fn label_shim(this: &GpuQuerySetDescriptor) -> &str;
    #[wasm_bindgen(method, setter = "label")]
    fn set_label_shim(this: &GpuQuerySetDescriptor, val: &str);
    #[wasm_bindgen(method, getter = "count")]
    fn count_shim(this: &GpuQuerySetDescriptor) -> u32;
    #[wasm_bindgen(method, setter = "count")]
    fn set_count_shim(this: &GpuQuerySetDescriptor, val: u32);
    #[cfg(feature = "GpuQueryType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &GpuQuerySetDescriptor) -> GpuQueryType;
    #[cfg(feature = "GpuQueryType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &GpuQuerySetDescriptor, val: GpuQueryType);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuQuerySetDescriptor` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuQuerySetDescriptor`*"]
pub trait GpuQuerySetDescriptorGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQuerySetDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn label(&self) -> &str;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `count` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQuerySetDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn count(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuQueryType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQuerySetDescriptor`, `GpuQueryType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn type_(&self) -> GpuQueryType;
}
#[cfg(web_sys_unstable_apis)]
impl GpuQuerySetDescriptorGetters for GpuQuerySetDescriptor {
    #[cfg(web_sys_unstable_apis)]
    fn label(&self) -> &str {
        self.label_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn count(&self) -> u32 {
        self.count_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuQueryType")]
    fn type_(&self) -> GpuQueryType {
        self.type__shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuQuerySetDescriptor {
    #[cfg(feature = "GpuQueryType")]
    #[doc = "Construct a new `GpuQuerySetDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQuerySetDescriptor`, `GpuQueryType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(count: u32, type_: GpuQueryType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.count(count);
        ret.type_(type_);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQuerySetDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `count` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQuerySetDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn count(&mut self, val: u32) -> &mut Self {
        self.set_count_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuQueryType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQuerySetDescriptor`, `GpuQueryType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn type_(&mut self, val: GpuQueryType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
}
