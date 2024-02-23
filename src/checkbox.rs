use dioxus::prelude::*;
use crate::{
    icon::ZIcon, utils::{class::XZClass, share::ZShare}, zab, zclass, zs
};


#[derive(Props)]
pub struct ZCheckboxProps<'a> {
    #[props(into,default)]
    class: String,
    #[props(into,default = zs!(""))]
    color: String,
    #[props(default)]
    disabled:bool,
    #[props(default)]
    onclick: EventHandler<'a, MouseEvent>,
    #[props(default)]
    checked:bool,
}

#[component(no_case_check)]
pub fn ZCheckbox<'a>(cx: Scope<'a , ZCheckboxProps<'a>>) -> Element {
    let zshare = use_shared_state::<ZShare>(cx).unwrap();
    cx.render(rsx! {
        div {
            class: zclass!(
                "rounded-[".to_owned()+zshare.read().radius.as_str()+"] w-7 h-7 border hover:ring-inset transition flex justify-center items-center overflow-hidden flex-none",
                zab!( "cursor-pointer",!cx.props.disabled),
                zab!( "",cx.props.disabled,zab!("hover:ring-[99999px] hover:ring-black/5",cx.props.checked,zclass!(
                    zab!( "bg-white hover:ring-[99999px] hover:ring-black/5 hover:border-".to_owned()+zshare.read().color.as_str()+"-500 hover:text-"+zshare.read().color.as_str()+"-500",cx.props.color==""),
                    zab!( "bg-white hover:ring-[99999px] hover:ring-black/5 hover:border-".to_owned()+cx.props.color.as_str()+"-500 hover:text-"+cx.props.color.as_str()+"-500",cx.props.color!="")
                ))),
                zab!(zclass!(
                    zab!( "bg-".to_owned()+zshare.read().color.as_str()+"-500 text-white border-"+zshare.read().color.as_str()+"-500",cx.props.color==""),
                    zab!( "bg-".to_owned()+cx.props.color.as_str()+"-500 text-white border-"+cx.props.color.as_str()+"-500",cx.props.color!="")
                ),cx.props.checked,"text-transparent border-zinc-300"),
                cx.props.class
            ),
            onclick:move |evt| {
                if !cx.props.disabled {
                    cx.props.onclick.call(evt)
                }
            },
            ZIcon {
                icon:"check"
            }
        }
    })
}
