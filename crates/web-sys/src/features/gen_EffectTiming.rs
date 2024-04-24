#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = EffectTiming)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EffectTiming` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    pub type EffectTiming;
    #[wasm_bindgen(method, getter = "delay")]
    fn delay_shim(this: &EffectTiming) -> f64;
    #[wasm_bindgen(method, setter = "delay")]
    fn set_delay_shim(this: &EffectTiming, val: f64);
    #[cfg(feature = "PlaybackDirection")]
    #[wasm_bindgen(method, getter = "direction")]
    fn direction_shim(this: &EffectTiming) -> PlaybackDirection;
    #[cfg(feature = "PlaybackDirection")]
    #[wasm_bindgen(method, setter = "direction")]
    fn set_direction_shim(this: &EffectTiming, val: PlaybackDirection);
    #[wasm_bindgen(method, getter = "duration")]
    fn duration_shim(this: &EffectTiming) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "duration")]
    fn set_duration_shim(this: &EffectTiming, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "easing")]
    fn easing_shim(this: &EffectTiming) -> &str;
    #[wasm_bindgen(method, setter = "easing")]
    fn set_easing_shim(this: &EffectTiming, val: &str);
    #[wasm_bindgen(method, getter = "endDelay")]
    fn end_delay_shim(this: &EffectTiming) -> f64;
    #[wasm_bindgen(method, setter = "endDelay")]
    fn set_end_delay_shim(this: &EffectTiming, val: f64);
    #[cfg(feature = "FillMode")]
    #[wasm_bindgen(method, getter = "fill")]
    fn fill_shim(this: &EffectTiming) -> FillMode;
    #[cfg(feature = "FillMode")]
    #[wasm_bindgen(method, setter = "fill")]
    fn set_fill_shim(this: &EffectTiming, val: FillMode);
    #[wasm_bindgen(method, getter = "iterationStart")]
    fn iteration_start_shim(this: &EffectTiming) -> f64;
    #[wasm_bindgen(method, setter = "iterationStart")]
    fn set_iteration_start_shim(this: &EffectTiming, val: f64);
    #[wasm_bindgen(method, getter = "iterations")]
    fn iterations_shim(this: &EffectTiming) -> f64;
    #[wasm_bindgen(method, setter = "iterations")]
    fn set_iterations_shim(this: &EffectTiming, val: f64);
}
#[doc = "The trait to access properties on the `EffectTiming` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
pub trait EffectTimingGetters {
    #[doc = "Get the `delay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    fn delay(&self) -> f64;
    #[cfg(feature = "PlaybackDirection")]
    #[doc = "Get the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`, `PlaybackDirection`*"]
    fn direction(&self) -> PlaybackDirection;
    #[doc = "Get the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    fn duration(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    fn easing(&self) -> &str;
    #[doc = "Get the `endDelay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    fn end_delay(&self) -> f64;
    #[cfg(feature = "FillMode")]
    #[doc = "Get the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`, `FillMode`*"]
    fn fill(&self) -> FillMode;
    #[doc = "Get the `iterationStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    fn iteration_start(&self) -> f64;
    #[doc = "Get the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    fn iterations(&self) -> f64;
}
impl EffectTimingGetters for EffectTiming {
    fn delay(&self) -> f64 {
        self.delay_shim()
    }
    #[cfg(feature = "PlaybackDirection")]
    fn direction(&self) -> PlaybackDirection {
        self.direction_shim()
    }
    fn duration(&self) -> &::wasm_bindgen::JsValue {
        self.duration_shim()
    }
    fn easing(&self) -> &str {
        self.easing_shim()
    }
    fn end_delay(&self) -> f64 {
        self.end_delay_shim()
    }
    #[cfg(feature = "FillMode")]
    fn fill(&self) -> FillMode {
        self.fill_shim()
    }
    fn iteration_start(&self) -> f64 {
        self.iteration_start_shim()
    }
    fn iterations(&self) -> f64 {
        self.iterations_shim()
    }
}
impl EffectTiming {
    #[doc = "Construct a new `EffectTiming`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `delay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    pub fn delay(&mut self, val: f64) -> &mut Self {
        self.set_delay_shim(val);
        self
    }
    #[cfg(feature = "PlaybackDirection")]
    #[doc = "Change the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`, `PlaybackDirection`*"]
    pub fn direction(&mut self, val: PlaybackDirection) -> &mut Self {
        self.set_direction_shim(val);
        self
    }
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    pub fn duration(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_duration_shim(val);
        self
    }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    pub fn easing(&mut self, val: &str) -> &mut Self {
        self.set_easing_shim(val);
        self
    }
    #[doc = "Change the `endDelay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    pub fn end_delay(&mut self, val: f64) -> &mut Self {
        self.set_end_delay_shim(val);
        self
    }
    #[cfg(feature = "FillMode")]
    #[doc = "Change the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`, `FillMode`*"]
    pub fn fill(&mut self, val: FillMode) -> &mut Self {
        self.set_fill_shim(val);
        self
    }
    #[doc = "Change the `iterationStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    pub fn iteration_start(&mut self, val: f64) -> &mut Self {
        self.set_iteration_start_shim(val);
        self
    }
    #[doc = "Change the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    pub fn iterations(&mut self, val: f64) -> &mut Self {
        self.set_iterations_shim(val);
        self
    }
}
impl Default for EffectTiming {
    fn default() -> Self {
        Self::new()
    }
}
