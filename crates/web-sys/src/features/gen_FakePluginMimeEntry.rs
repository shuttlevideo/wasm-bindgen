#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FakePluginMimeEntry)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FakePluginMimeEntry` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    pub type FakePluginMimeEntry;
    #[wasm_bindgen(method, getter = "description")]
    fn description_shim(this: &FakePluginMimeEntry) -> String;
    #[wasm_bindgen(method, setter = "description")]
    fn set_description_shim(this: &FakePluginMimeEntry, val: &str);
    #[wasm_bindgen(method, getter = "extension")]
    fn extension_shim(this: &FakePluginMimeEntry) -> String;
    #[wasm_bindgen(method, setter = "extension")]
    fn set_extension_shim(this: &FakePluginMimeEntry, val: &str);
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &FakePluginMimeEntry) -> String;
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &FakePluginMimeEntry, val: &str);
}
#[doc = "The trait to access properties on the `FakePluginMimeEntry` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
pub trait FakePluginMimeEntryGetters {
    #[doc = "Get the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    fn description(&self) -> String;
    #[doc = "Get the `extension` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    fn extension(&self) -> String;
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    fn type_(&self) -> String;
}
impl FakePluginMimeEntryGetters for FakePluginMimeEntry {
    fn description(&self) -> String {
        self.description_shim()
    }
    fn extension(&self) -> String {
        self.extension_shim()
    }
    fn type_(&self) -> String {
        self.type__shim()
    }
}
impl FakePluginMimeEntry {
    #[doc = "Construct a new `FakePluginMimeEntry`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    pub fn new(type_: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::type_(&mut ret, type_);
        ret
    }
    #[doc = "Change the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    pub fn description(&mut self, val: &str) -> &mut Self {
        self.set_description_shim(val);
        self
    }
    #[doc = "Change the `extension` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    pub fn extension(&mut self, val: &str) -> &mut Self {
        self.set_extension_shim(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.set_type__shim(val);
        self
    }
}
