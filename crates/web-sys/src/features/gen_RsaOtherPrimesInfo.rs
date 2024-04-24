#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RsaOtherPrimesInfo)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RsaOtherPrimesInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    pub type RsaOtherPrimesInfo;
    #[wasm_bindgen(method, getter = "d")]
    fn d_shim(this: &RsaOtherPrimesInfo) -> &str;
    #[wasm_bindgen(method, setter = "d")]
    fn set_d_shim(this: &RsaOtherPrimesInfo, val: &str);
    #[wasm_bindgen(method, getter = "r")]
    fn r_shim(this: &RsaOtherPrimesInfo) -> &str;
    #[wasm_bindgen(method, setter = "r")]
    fn set_r_shim(this: &RsaOtherPrimesInfo, val: &str);
    #[wasm_bindgen(method, getter = "t")]
    fn t_shim(this: &RsaOtherPrimesInfo) -> &str;
    #[wasm_bindgen(method, setter = "t")]
    fn set_t_shim(this: &RsaOtherPrimesInfo, val: &str);
}
#[doc = "The trait to access properties on the `RsaOtherPrimesInfo` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
pub trait RsaOtherPrimesInfoGetters {
    #[doc = "Get the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    fn d(&self) -> &str;
    #[doc = "Get the `r` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    fn r(&self) -> &str;
    #[doc = "Get the `t` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    fn t(&self) -> &str;
}
impl RsaOtherPrimesInfoGetters for RsaOtherPrimesInfo {
    fn d(&self) -> &str {
        self.d_shim()
    }
    fn r(&self) -> &str {
        self.r_shim()
    }
    fn t(&self) -> &str {
        self.t_shim()
    }
}
impl RsaOtherPrimesInfo {
    #[doc = "Construct a new `RsaOtherPrimesInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    pub fn new(d: &str, r: &str, t: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.d(d);
        ret.r(r);
        ret.t(t);
        ret
    }
    #[doc = "Change the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    pub fn d(&mut self, val: &str) -> &mut Self {
        self.set_d_shim(val);
        self
    }
    #[doc = "Change the `r` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    pub fn r(&mut self, val: &str) -> &mut Self {
        self.set_r_shim(val);
        self
    }
    #[doc = "Change the `t` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    pub fn t(&mut self, val: &str) -> &mut Self {
        self.set_t_shim(val);
        self
    }
}
