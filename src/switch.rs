use dioxus::prelude::*;
use crate::{
    utils::{class::XZClass, share::ZShare}, zab, zclass, zs
};


#[derive(Props)]
pub struct ZSwitchProps<'a> {
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
pub fn ZSwitch<'a>(cx: Scope<'a , ZSwitchProps<'a>>) -> Element {
    let zshare = use_shared_state::<ZShare>(cx).unwrap();
    cx.render(rsx! {
        div {
            class: zclass!(
                "rounded-[".to_owned()+zshare.read().radius.as_str()+"] w-12 h-7 border hover:ring-inset transition flex justify-start items-center overflow-hidden flex-none p-1",
                zab!( "cursor-pointer",!cx.props.disabled),
                zab!( "",cx.props.disabled,zab!("hover:ring-[99999px] hover:ring-black/5",cx.props.checked,zclass!(
                    zab!( "bg-white hover:ring-[99999px] hover:ring-black/5 hover:border-".to_owned()+zshare.read().color.as_str()+"-500",cx.props.color==""),
                    zab!( "bg-white hover:ring-[99999px] hover:ring-black/5 hover:border-".to_owned()+cx.props.color.as_str()+"-500",cx.props.color!="")
                ))),
                zab!(zclass!(
                    zab!( "bg-".to_owned()+zshare.read().color.as_str()+"-500 border-"+zshare.read().color.as_str()+"-500",cx.props.color==""),
                    zab!( "bg-".to_owned()+cx.props.color.as_str()+"-500 border-"+cx.props.color.as_str()+"-500",cx.props.color!="")
                ),cx.props.checked,"border-zinc-300"),
                cx.props.class
            ),
            onclick:move |evt| {
                if !cx.props.disabled {
                    cx.props.onclick.call(evt)
                }
            },
            div {
                class:zclass!(
                    "rounded-[".to_owned()+zshare.read().radius.as_str()+"] w-5 h-5 transition-[background] transition-[margin] flex justify-center items-center overflow-hidden flex-none",
                    zab!("ml-5 bg-white",cx.props.checked,"bg-zinc-300")
                )
            }
        }
    })
}
