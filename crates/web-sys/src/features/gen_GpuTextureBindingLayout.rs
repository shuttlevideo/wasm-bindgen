#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUTextureBindingLayout)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuTextureBindingLayout` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuTextureBindingLayout;
    #[wasm_bindgen(method, getter = "multisampled")]
    fn multisampled_shim(this: &GpuTextureBindingLayout) -> bool;
    #[wasm_bindgen(method, setter = "multisampled")]
    fn set_multisampled_shim(this: &GpuTextureBindingLayout, val: bool);
    #[cfg(feature = "GpuTextureSampleType")]
    #[wasm_bindgen(method, getter = "sampleType")]
    fn sample_type_shim(this: &GpuTextureBindingLayout) -> GpuTextureSampleType;
    #[cfg(feature = "GpuTextureSampleType")]
    #[wasm_bindgen(method, setter = "sampleType")]
    fn set_sample_type_shim(this: &GpuTextureBindingLayout, val: GpuTextureSampleType);
    #[cfg(feature = "GpuTextureViewDimension")]
    #[wasm_bindgen(method, getter = "viewDimension")]
    fn view_dimension_shim(this: &GpuTextureBindingLayout) -> GpuTextureViewDimension;
    #[cfg(feature = "GpuTextureViewDimension")]
    #[wasm_bindgen(method, setter = "viewDimension")]
    fn set_view_dimension_shim(this: &GpuTextureBindingLayout, val: GpuTextureViewDimension);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuTextureBindingLayout` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuTextureBindingLayout`*"]
pub trait GpuTextureBindingLayoutGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `multisampled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn multisampled(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureSampleType")]
    #[doc = "Get the `sampleType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureBindingLayout`, `GpuTextureSampleType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn sample_type(&self) -> GpuTextureSampleType;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureViewDimension")]
    #[doc = "Get the `viewDimension` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureBindingLayout`, `GpuTextureViewDimension`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn view_dimension(&self) -> GpuTextureViewDimension;
}
#[cfg(web_sys_unstable_apis)]
impl GpuTextureBindingLayoutGetters for GpuTextureBindingLayout {
    #[cfg(web_sys_unstable_apis)]
    fn multisampled(&self) -> bool {
        self.multisampled_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureSampleType")]
    fn sample_type(&self) -> GpuTextureSampleType {
        self.sample_type_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureViewDimension")]
    fn view_dimension(&self) -> GpuTextureViewDimension {
        self.view_dimension_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuTextureBindingLayout {
    #[doc = "Construct a new `GpuTextureBindingLayout`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `multisampled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn multisampled(&mut self, val: bool) -> &mut Self {
        self.set_multisampled_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureSampleType")]
    #[doc = "Change the `sampleType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureBindingLayout`, `GpuTextureSampleType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn sample_type(&mut self, val: GpuTextureSampleType) -> &mut Self {
        self.set_sample_type_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureViewDimension")]
    #[doc = "Change the `viewDimension` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTextureBindingLayout`, `GpuTextureViewDimension`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn view_dimension(&mut self, val: GpuTextureViewDimension) -> &mut Self {
        self.set_view_dimension_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for GpuTextureBindingLayout {
    fn default() -> Self {
        Self::new()
    }
}
