#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = EncodedVideoChunkMetadata)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EncodedVideoChunkMetadata` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type EncodedVideoChunkMetadata;
    #[wasm_bindgen(method, getter = "alphaSideData")]
    fn alpha_side_data_shim(this: &EncodedVideoChunkMetadata) -> ::js_sys::Object;
    #[wasm_bindgen(method, setter = "alphaSideData")]
    fn set_alpha_side_data_shim(this: &EncodedVideoChunkMetadata, val: &::js_sys::Object);
    #[cfg(feature = "VideoDecoderConfig")]
    #[wasm_bindgen(method, getter = "decoderConfig")]
    fn decoder_config_shim(this: &EncodedVideoChunkMetadata) -> VideoDecoderConfig;
    #[cfg(feature = "VideoDecoderConfig")]
    #[wasm_bindgen(method, setter = "decoderConfig")]
    fn set_decoder_config_shim(this: &EncodedVideoChunkMetadata, val: &VideoDecoderConfig);
    #[cfg(feature = "SvcOutputMetadata")]
    #[wasm_bindgen(method, getter = "svc")]
    fn svc_shim(this: &EncodedVideoChunkMetadata) -> SvcOutputMetadata;
    #[cfg(feature = "SvcOutputMetadata")]
    #[wasm_bindgen(method, setter = "svc")]
    fn set_svc_shim(this: &EncodedVideoChunkMetadata, val: &SvcOutputMetadata);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `EncodedVideoChunkMetadata` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`*"]
pub trait EncodedVideoChunkMetadataGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `alphaSideData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn alpha_side_data(&self) -> ::js_sys::Object;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoDecoderConfig")]
    #[doc = "Get the `decoderConfig` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`, `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn decoder_config(&self) -> VideoDecoderConfig;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SvcOutputMetadata")]
    #[doc = "Get the `svc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`, `SvcOutputMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn svc(&self) -> SvcOutputMetadata;
}
#[cfg(web_sys_unstable_apis)]
impl EncodedVideoChunkMetadataGetters for EncodedVideoChunkMetadata {
    #[cfg(web_sys_unstable_apis)]
    fn alpha_side_data(&self) -> ::js_sys::Object {
        self.alpha_side_data_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoDecoderConfig")]
    fn decoder_config(&self) -> VideoDecoderConfig {
        self.decoder_config_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SvcOutputMetadata")]
    fn svc(&self) -> SvcOutputMetadata {
        self.svc_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl EncodedVideoChunkMetadata {
    #[doc = "Construct a new `EncodedVideoChunkMetadata`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `alphaSideData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn alpha_side_data(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_alpha_side_data_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoDecoderConfig")]
    #[doc = "Change the `decoderConfig` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`, `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn decoder_config(&mut self, val: &VideoDecoderConfig) -> &mut Self {
        self.set_decoder_config_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SvcOutputMetadata")]
    #[doc = "Change the `svc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`, `SvcOutputMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn svc(&mut self, val: &SvcOutputMetadata) -> &mut Self {
        self.set_svc_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for EncodedVideoChunkMetadata {
    fn default() -> Self {
        Self::new()
    }
}
