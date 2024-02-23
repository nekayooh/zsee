use std::collections::HashMap;

use crate::toast::ZToastProps;



pub struct ZShare{
    pub color:String,
    pub radius:String,
    pub i18n:String,
    pub word:HashMap<String,String>,
    pub toast:ZToastProps,
}