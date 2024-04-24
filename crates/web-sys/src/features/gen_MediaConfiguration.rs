#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaConfiguration)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaConfiguration`*"]
    pub type MediaConfiguration;
    #[cfg(feature = "AudioConfiguration")]
    #[wasm_bindgen(method, getter = "audio")]
    fn audio_shim(this: &MediaConfiguration) -> &AudioConfiguration;
    #[cfg(feature = "AudioConfiguration")]
    #[wasm_bindgen(method, setter = "audio")]
    fn set_audio_shim(this: &MediaConfiguration, val: &AudioConfiguration);
    #[cfg(feature = "VideoConfiguration")]
    #[wasm_bindgen(method, getter = "video")]
    fn video_shim(this: &MediaConfiguration) -> &VideoConfiguration;
    #[cfg(feature = "VideoConfiguration")]
    #[wasm_bindgen(method, setter = "video")]
    fn set_video_shim(this: &MediaConfiguration, val: &VideoConfiguration);
}
#[doc = "The trait to access properties on the `MediaConfiguration` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaConfiguration`*"]
pub trait MediaConfigurationGetters {
    #[cfg(feature = "AudioConfiguration")]
    #[doc = "Get the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`, `MediaConfiguration`*"]
    fn audio(&self) -> &AudioConfiguration;
    #[cfg(feature = "VideoConfiguration")]
    #[doc = "Get the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaConfiguration`, `VideoConfiguration`*"]
    fn video(&self) -> &VideoConfiguration;
}
impl MediaConfigurationGetters for MediaConfiguration {
    #[cfg(feature = "AudioConfiguration")]
    fn audio(&self) -> &AudioConfiguration {
        self.audio_shim()
    }
    #[cfg(feature = "VideoConfiguration")]
    fn video(&self) -> &VideoConfiguration {
        self.video_shim()
    }
}
impl MediaConfiguration {
    #[doc = "Construct a new `MediaConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaConfiguration`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "AudioConfiguration")]
    #[doc = "Change the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`, `MediaConfiguration`*"]
    pub fn audio(&mut self, val: &AudioConfiguration) -> &mut Self {
        self.set_audio_shim(val);
        self
    }
    #[cfg(feature = "VideoConfiguration")]
    #[doc = "Change the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaConfiguration`, `VideoConfiguration`*"]
    pub fn video(&mut self, val: &VideoConfiguration) -> &mut Self {
        self.set_video_shim(val);
        self
    }
}
impl Default for MediaConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
