#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PopupBlockedEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PopupBlockedEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    pub type PopupBlockedEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &PopupBlockedEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &PopupBlockedEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &PopupBlockedEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &PopupBlockedEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &PopupBlockedEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &PopupBlockedEventInit, val: bool);
    #[wasm_bindgen(method, getter = "popupWindowFeatures")]
    fn popup_window_features_shim(this: &PopupBlockedEventInit) -> String;
    #[wasm_bindgen(method, setter = "popupWindowFeatures")]
    fn set_popup_window_features_shim(this: &PopupBlockedEventInit, val: &str);
    #[wasm_bindgen(method, getter = "popupWindowName")]
    fn popup_window_name_shim(this: &PopupBlockedEventInit) -> String;
    #[wasm_bindgen(method, setter = "popupWindowName")]
    fn set_popup_window_name_shim(this: &PopupBlockedEventInit, val: &str);
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, getter = "requestingWindow")]
    fn requesting_window_shim(this: &PopupBlockedEventInit) -> Option<Window>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, setter = "requestingWindow")]
    fn set_requesting_window_shim(this: &PopupBlockedEventInit, val: Option<&Window>);
}
#[doc = "The trait to access properties on the `PopupBlockedEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
pub trait PopupBlockedEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `popupWindowFeatures` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    fn popup_window_features(&self) -> String;
    #[doc = "Get the `popupWindowName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    fn popup_window_name(&self) -> String;
    #[cfg(feature = "Window")]
    #[doc = "Get the `requestingWindow` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`, `Window`*"]
    fn requesting_window(&self) -> Option<Window>;
}
impl PopupBlockedEventInitGetters for PopupBlockedEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn popup_window_features(&self) -> String {
        self.popup_window_features_shim()
    }
    fn popup_window_name(&self) -> String {
        self.popup_window_name_shim()
    }
    #[cfg(feature = "Window")]
    fn requesting_window(&self) -> Option<Window> {
        self.requesting_window_shim()
    }
}
impl PopupBlockedEventInit {
    #[doc = "Construct a new `PopupBlockedEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `popupWindowFeatures` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    pub fn popup_window_features(&mut self, val: &str) -> &mut Self {
        self.set_popup_window_features_shim(val);
        self
    }
    #[doc = "Change the `popupWindowName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    pub fn popup_window_name(&mut self, val: &str) -> &mut Self {
        self.set_popup_window_name_shim(val);
        self
    }
    #[cfg(feature = "Window")]
    #[doc = "Change the `requestingWindow` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`, `Window`*"]
    pub fn requesting_window(&mut self, val: Option<&Window>) -> &mut Self {
        self.set_requesting_window_shim(val);
        self
    }
}
impl Default for PopupBlockedEventInit {
    fn default() -> Self {
        Self::new()
    }
}
