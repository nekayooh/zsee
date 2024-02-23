use dioxus::prelude::*;

use crate::{utils::class::XZClass, zclass};


#[derive(PartialEq,Props)]
pub struct ZIconProps {
    #[props(into,default)]
    class: String,
    #[props(into,default)]
    icon: String,
}

#[component(no_case_check)]
pub fn ZIcon(cx: Scope<ZIconProps>) -> Element {
    cx.render(rsx! {
        i {
            class: zclass!(
                "ti ti-".to_owned()+cx.props.icon.as_str(),
                cx.props.class
            ),
        }
    })
}
