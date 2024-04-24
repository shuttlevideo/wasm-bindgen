#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaEncodingConfiguration)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaEncodingConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncodingConfiguration`*"]
    pub type MediaEncodingConfiguration;
    #[cfg(feature = "AudioConfiguration")]
    #[wasm_bindgen(method, getter = "audio")]
    fn audio_shim(this: &MediaEncodingConfiguration) -> &AudioConfiguration;
    #[cfg(feature = "AudioConfiguration")]
    #[wasm_bindgen(method, setter = "audio")]
    fn set_audio_shim(this: &MediaEncodingConfiguration, val: &AudioConfiguration);
    #[cfg(feature = "VideoConfiguration")]
    #[wasm_bindgen(method, getter = "video")]
    fn video_shim(this: &MediaEncodingConfiguration) -> &VideoConfiguration;
    #[cfg(feature = "VideoConfiguration")]
    #[wasm_bindgen(method, setter = "video")]
    fn set_video_shim(this: &MediaEncodingConfiguration, val: &VideoConfiguration);
    #[cfg(feature = "MediaEncodingType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &MediaEncodingConfiguration) -> MediaEncodingType;
    #[cfg(feature = "MediaEncodingType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &MediaEncodingConfiguration, val: MediaEncodingType);
}
#[doc = "The trait to access properties on the `MediaEncodingConfiguration` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaEncodingConfiguration`*"]
pub trait MediaEncodingConfigurationGetters {
    #[cfg(feature = "AudioConfiguration")]
    #[doc = "Get the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`, `MediaEncodingConfiguration`*"]
    fn audio(&self) -> &AudioConfiguration;
    #[cfg(feature = "VideoConfiguration")]
    #[doc = "Get the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncodingConfiguration`, `VideoConfiguration`*"]
    fn video(&self) -> &VideoConfiguration;
    #[cfg(feature = "MediaEncodingType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncodingConfiguration`, `MediaEncodingType`*"]
    fn type_(&self) -> MediaEncodingType;
}
impl MediaEncodingConfigurationGetters for MediaEncodingConfiguration {
    #[cfg(feature = "AudioConfiguration")]
    fn audio(&self) -> &AudioConfiguration {
        self.audio_shim()
    }
    #[cfg(feature = "VideoConfiguration")]
    fn video(&self) -> &VideoConfiguration {
        self.video_shim()
    }
    #[cfg(feature = "MediaEncodingType")]
    fn type_(&self) -> MediaEncodingType {
        self.type__shim()
    }
}
impl MediaEncodingConfiguration {
    #[cfg(feature = "MediaEncodingType")]
    #[doc = "Construct a new `MediaEncodingConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncodingConfiguration`, `MediaEncodingType`*"]
    pub fn new(type_: MediaEncodingType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.type_(type_);
        ret
    }
    #[cfg(feature = "AudioConfiguration")]
    #[doc = "Change the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`, `MediaEncodingConfiguration`*"]
    pub fn audio(&mut self, val: &AudioConfiguration) -> &mut Self {
        self.set_audio_shim(val);
        self
    }
    #[cfg(feature = "VideoConfiguration")]
    #[doc = "Change the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncodingConfiguration`, `VideoConfiguration`*"]
    pub fn video(&mut self, val: &VideoConfiguration) -> &mut Self {
        self.set_video_shim(val);
        self
    }
    #[cfg(feature = "MediaEncodingType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncodingConfiguration`, `MediaEncodingType`*"]
    pub fn type_(&mut self, val: MediaEncodingType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
}
