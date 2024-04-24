#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = EncodedAudioChunkMetadata)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EncodedAudioChunkMetadata` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type EncodedAudioChunkMetadata;
    #[cfg(feature = "AudioDecoderConfig")]
    #[wasm_bindgen(method, getter = "decoderConfig")]
    fn decoder_config_shim(this: &EncodedAudioChunkMetadata) -> &AudioDecoderConfig;
    #[cfg(feature = "AudioDecoderConfig")]
    #[wasm_bindgen(method, setter = "decoderConfig")]
    fn set_decoder_config_shim(this: &EncodedAudioChunkMetadata, val: &AudioDecoderConfig);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `EncodedAudioChunkMetadata` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunkMetadata`*"]
pub trait EncodedAudioChunkMetadataGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDecoderConfig")]
    #[doc = "Get the `decoderConfig` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderConfig`, `EncodedAudioChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn decoder_config(&self) -> &AudioDecoderConfig;
}
#[cfg(web_sys_unstable_apis)]
impl EncodedAudioChunkMetadataGetters for EncodedAudioChunkMetadata {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDecoderConfig")]
    fn decoder_config(&self) -> &AudioDecoderConfig {
        self.decoder_config_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl EncodedAudioChunkMetadata {
    #[doc = "Construct a new `EncodedAudioChunkMetadata`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDecoderConfig")]
    #[doc = "Change the `decoderConfig` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderConfig`, `EncodedAudioChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn decoder_config(&mut self, val: &AudioDecoderConfig) -> &mut Self {
        self.set_decoder_config_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for EncodedAudioChunkMetadata {
    fn default() -> Self {
        Self::new()
    }
}
