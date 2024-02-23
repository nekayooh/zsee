use dioxus::{core::{exports::bumpalo::Bump, AttributeValue}, prelude::*};



#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct XZClass(String);


impl XZClass {
    
    pub fn default() ->XZClass {
        XZClass("".to_string())
    }
    pub fn to_string(self) -> String {
        self.0
    }
}


impl From<XZClass> for String {
    fn from(v: XZClass) -> Self {
        v.0
    }
}

impl From<String> for XZClass {
    fn from(v: String) -> Self {
        Self(v)
    }
}

impl From<&str> for XZClass {
    fn from(v: &str) -> Self {
        Self(v.to_string())
    }
}
impl From<Vec<String>> for XZClass {
    fn from(v: Vec<String>) -> Self {
        Self(v.join(" "))
    }
}


impl<'a> IntoAttributeValue<'a> for XZClass {
    fn into_value(self, bump: &'a Bump) -> AttributeValue<'a> {
        let str_buf = dioxus::core::exports::bumpalo::collections::String::from_str_in(&self.0, bump);
        AttributeValue::Text(str_buf.into_bump_str())
    }
}

impl<'a> IntoAttributeValue<'a> for &XZClass {
    fn into_value(self, bump: &'a Bump) -> AttributeValue<'a> {
        let str_buf = dioxus::core::exports::bumpalo::collections::String::from_str_in(&self.0, bump);
        AttributeValue::Text(str_buf.into_bump_str())
    }
}