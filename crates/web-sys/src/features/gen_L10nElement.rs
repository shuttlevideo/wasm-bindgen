#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = L10nElement)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `L10nElement` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub type L10nElement;
}
#[doc = "The trait to access properties on the `L10nElement` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
pub trait L10nElementGetters {
    #[doc = "Get the `l10nArgs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    fn l10n_args(&self) -> Option<&::js_sys::Object>;
    #[doc = "Get the `l10nAttrs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    fn l10n_attrs(&self) -> Option<&str>;
    #[doc = "Get the `l10nId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    fn l10n_id(&self) -> &str;
    #[doc = "Get the `localName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    fn local_name(&self) -> &str;
    #[doc = "Get the `namespaceURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    fn namespace_uri(&self) -> &str;
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    fn type_(&self) -> Option<&str>;
}
impl L10nElementGetters for L10nElement {
    fn l10n_args(&self) -> Option<&::js_sys::Object> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("l10nArgs"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn l10n_attrs(&self) -> Option<&str> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("l10nAttrs"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn l10n_id(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("l10nId"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn local_name(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("localName"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn namespace_uri(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("namespaceURI"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn type_(&self) -> Option<&str> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("type"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl L10nElement {
    #[doc = "Construct a new `L10nElement`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn new(l10n_id: &str, local_name: &str, namespace_uri: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.l10n_id(l10n_id);
        ret.local_name(local_name);
        ret.namespace_uri(namespace_uri);
        ret
    }
    #[doc = "Change the `l10nArgs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn l10n_args(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("l10nArgs"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `l10nAttrs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn l10n_attrs(&mut self, val: Option<&str>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("l10nAttrs"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `l10nId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn l10n_id(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("l10nId"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `localName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn local_name(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("localName"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `namespaceURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn namespace_uri(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("namespaceURI"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn type_(&mut self, val: Option<&str>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("type"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
