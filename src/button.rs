use dioxus::prelude::*;

use crate::{
    icon::ZIcon, utils::{class::XZClass, share::ZShare}, zab, zclass, zs
};

#[derive(Props)]
pub struct ZButtonProps<'a> {
    //fill line half gray transparent
    #[props(into,default=zs!("fill"))]
    mode: String,
    #[props(into,default)]
    class: String,
    #[props(into,default=zs!("sky"))]
    color: String,
    #[props(default)]
    disabled: bool,
    #[props(default)]
    loading: bool,
    #[props(default)]
    onclick: EventHandler<'a, MouseEvent>,
    #[props(default)]
    children: Element<'a>,
}

#[component(no_case_check)]
pub fn ZButton<'a>(cx: Scope<'a, ZButtonProps<'a>>) -> Element {
    let zshare = use_shared_state::<ZShare>(cx).unwrap();
    cx.render(rsx! {
        button {
            onclick:move |evt| cx.props.onclick.call(evt),
            disabled:cx.props.disabled||cx.props.loading,
            class: zclass!(
                "px-3 py-2 rounded-[".to_owned()+zshare.read().radius.as_str()+"] border hover:ring-inset transition flex gap-1 justify-center items-center overflow-hidden flex-none",
                zab!( "cursor-wait",cx.props.loading),
                zab!( "cursor-not-allowed text-zinc-500 bg-zinc-100 border-zinc-100",cx.props.disabled,zclass!(
                    zab!( "hover:ring-[99999px] hover:ring-black/5 text-white bg-".to_owned()+cx.props.color.as_str()+"-500 border-"+cx.props.color.as_str()+"-500",cx.props.mode=="fill"),
                    zab!( "hover:ring-[99999px] hover:ring-black/5 text-".to_owned()+cx.props.color.as_str()+"-500 bg-"+cx.props.color.as_str()+"-100 border-"+cx.props.color.as_str()+"-100",cx.props.mode=="half"),
                    zab!( "hover:ring-[99999px] hover:ring-black/5 text-".to_owned()+cx.props.color.as_str()+"-500 bg-"+cx.props.color.as_str()+"-100 border-"+cx.props.color.as_str()+"-500",cx.props.mode=="half_line"),
                    zab!( "hover:ring-[99999px] hover:ring-black/5 hover:border-black/5 text-".to_owned()+cx.props.color.as_str()+"-500 border-transparent",cx.props.mode=="transparent"),
                    // 灰度背景按钮
                    zab!( "hover:ring-[99999px] hover:ring-black/5 text-".to_owned()+cx.props.color.as_str()+"-500 bg-zinc-100 border-zinc-100",cx.props.mode=="gray"),
                    zab!( "hover:ring-[99999px] hover:ring-black/5 text-".to_owned()+cx.props.color.as_str()+"-500 border-"+cx.props.color.as_str()+"-500",cx.props.mode=="line")
                )),
                cx.props.class
            ),
            if cx.props.loading {
                rsx!{
                    div{
                        class:"animate-spin",
                        ZIcon{
                            icon: "loader-2",
                        }
                    }
                }
            }
            &cx.props.children
        }
    })
}
