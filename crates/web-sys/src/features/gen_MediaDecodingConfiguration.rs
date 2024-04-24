#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaDecodingConfiguration)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaDecodingConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDecodingConfiguration`*"]
    pub type MediaDecodingConfiguration;
    #[cfg(feature = "AudioConfiguration")]
    #[wasm_bindgen(method, getter = "audio")]
    fn audio_shim(this: &MediaDecodingConfiguration) -> &AudioConfiguration;
    #[cfg(feature = "AudioConfiguration")]
    #[wasm_bindgen(method, setter = "audio")]
    fn set_audio_shim(this: &MediaDecodingConfiguration, val: &AudioConfiguration);
    #[cfg(feature = "VideoConfiguration")]
    #[wasm_bindgen(method, getter = "video")]
    fn video_shim(this: &MediaDecodingConfiguration) -> &VideoConfiguration;
    #[cfg(feature = "VideoConfiguration")]
    #[wasm_bindgen(method, setter = "video")]
    fn set_video_shim(this: &MediaDecodingConfiguration, val: &VideoConfiguration);
    #[cfg(feature = "MediaDecodingType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &MediaDecodingConfiguration) -> MediaDecodingType;
    #[cfg(feature = "MediaDecodingType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &MediaDecodingConfiguration, val: MediaDecodingType);
}
#[doc = "The trait to access properties on the `MediaDecodingConfiguration` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaDecodingConfiguration`*"]
pub trait MediaDecodingConfigurationGetters {
    #[cfg(feature = "AudioConfiguration")]
    #[doc = "Get the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`, `MediaDecodingConfiguration`*"]
    fn audio(&self) -> &AudioConfiguration;
    #[cfg(feature = "VideoConfiguration")]
    #[doc = "Get the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDecodingConfiguration`, `VideoConfiguration`*"]
    fn video(&self) -> &VideoConfiguration;
    #[cfg(feature = "MediaDecodingType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDecodingConfiguration`, `MediaDecodingType`*"]
    fn type_(&self) -> MediaDecodingType;
}
impl MediaDecodingConfigurationGetters for MediaDecodingConfiguration {
    #[cfg(feature = "AudioConfiguration")]
    fn audio(&self) -> &AudioConfiguration {
        self.audio_shim()
    }
    #[cfg(feature = "VideoConfiguration")]
    fn video(&self) -> &VideoConfiguration {
        self.video_shim()
    }
    #[cfg(feature = "MediaDecodingType")]
    fn type_(&self) -> MediaDecodingType {
        self.type__shim()
    }
}
impl MediaDecodingConfiguration {
    #[cfg(feature = "MediaDecodingType")]
    #[doc = "Construct a new `MediaDecodingConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDecodingConfiguration`, `MediaDecodingType`*"]
    pub fn new(type_: MediaDecodingType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.type_(type_);
        ret
    }
    #[cfg(feature = "AudioConfiguration")]
    #[doc = "Change the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`, `MediaDecodingConfiguration`*"]
    pub fn audio(&mut self, val: &AudioConfiguration) -> &mut Self {
        self.set_audio_shim(val);
        self
    }
    #[cfg(feature = "VideoConfiguration")]
    #[doc = "Change the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDecodingConfiguration`, `VideoConfiguration`*"]
    pub fn video(&mut self, val: &VideoConfiguration) -> &mut Self {
        self.set_video_shim(val);
        self
    }
    #[cfg(feature = "MediaDecodingType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDecodingConfiguration`, `MediaDecodingType`*"]
    pub fn type_(&mut self, val: MediaDecodingType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
}
