#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPURenderPipelineDescriptor)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuRenderPipelineDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuRenderPipelineDescriptor;
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuRenderPipelineDescriptor` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
pub trait GpuRenderPipelineDescriptorGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn label(&self) -> &str;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn layout(&self) -> &::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuDepthStencilState")]
    #[doc = "Get the `depthStencil` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn depth_stencil(&self) -> &GpuDepthStencilState;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFragmentState")]
    #[doc = "Get the `fragment` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn fragment(&self) -> &GpuFragmentState;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuMultisampleState")]
    #[doc = "Get the `multisample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuMultisampleState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn multisample(&self) -> &GpuMultisampleState;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPrimitiveState")]
    #[doc = "Get the `primitive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPrimitiveState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn primitive(&self) -> &GpuPrimitiveState;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexState")]
    #[doc = "Get the `vertex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`, `GpuVertexState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn vertex(&self) -> &GpuVertexState;
}
#[cfg(web_sys_unstable_apis)]
impl GpuRenderPipelineDescriptorGetters for GpuRenderPipelineDescriptor {
    #[cfg(web_sys_unstable_apis)]
    fn label(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("label"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(web_sys_unstable_apis)]
    fn layout(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("layout"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuDepthStencilState")]
    fn depth_stencil(&self) -> &GpuDepthStencilState {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("depthStencil"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFragmentState")]
    fn fragment(&self) -> &GpuFragmentState {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("fragment"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuMultisampleState")]
    fn multisample(&self) -> &GpuMultisampleState {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("multisample"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPrimitiveState")]
    fn primitive(&self) -> &GpuPrimitiveState {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("primitive"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexState")]
    fn vertex(&self) -> &GpuVertexState {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("vertex"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuRenderPipelineDescriptor {
    #[cfg(feature = "GpuVertexState")]
    #[doc = "Construct a new `GpuRenderPipelineDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`, `GpuVertexState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(layout: &::wasm_bindgen::JsValue, vertex: &GpuVertexState) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.layout(layout);
        ret.vertex(vertex);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("label"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn layout(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("layout"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuDepthStencilState")]
    #[doc = "Change the `depthStencil` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_stencil(&mut self, val: &GpuDepthStencilState) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("depthStencil"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFragmentState")]
    #[doc = "Change the `fragment` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn fragment(&mut self, val: &GpuFragmentState) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("fragment"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuMultisampleState")]
    #[doc = "Change the `multisample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuMultisampleState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn multisample(&mut self, val: &GpuMultisampleState) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("multisample"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPrimitiveState")]
    #[doc = "Change the `primitive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPrimitiveState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn primitive(&mut self, val: &GpuPrimitiveState) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("primitive"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexState")]
    #[doc = "Change the `vertex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`, `GpuVertexState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn vertex(&mut self, val: &GpuVertexState) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("vertex"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
