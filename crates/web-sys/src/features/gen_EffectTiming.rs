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
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("delay"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "PlaybackDirection")]
    fn direction(&self) -> PlaybackDirection {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("direction"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn duration(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("duration"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn easing(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("easing"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn end_delay(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("endDelay"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "FillMode")]
    fn fill(&self) -> FillMode {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("fill"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn iteration_start(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("iterationStart"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn iterations(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("iterations"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
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
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("delay"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "PlaybackDirection")]
    #[doc = "Change the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`, `PlaybackDirection`*"]
    pub fn direction(&mut self, val: PlaybackDirection) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("direction"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    pub fn duration(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("duration"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    pub fn easing(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("easing"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `endDelay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    pub fn end_delay(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("endDelay"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "FillMode")]
    #[doc = "Change the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`, `FillMode`*"]
    pub fn fill(&mut self, val: FillMode) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("fill"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `iterationStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    pub fn iteration_start(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("iterationStart"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    pub fn iterations(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("iterations"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for EffectTiming {
    fn default() -> Self {
        Self::new()
    }
}
