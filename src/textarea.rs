use dioxus::prelude::*;

use crate::{
    utils::{class::XZClass, share::ZShare}, zab, zclass, zs
};

#[derive(Props)]
pub struct ZTextareaProps<'a> {
    //fill line half gray transparent
    #[props(into,default)]
    class: String,
    // 输入框补充样式
    #[props(into,default)]
    class_input: String,
    // 始终显示颜色
    #[props(default)]
    always: bool,
    #[props(into,default = zs!(""))]
    color: String,
    #[props(default = 3)]
    rows: i64,
    #[props(default = 999999)]
    maxlength: i64,
    #[props(default)]
    disabled: bool,
    #[props(into,default)]
    value: String,
    #[props(default)]
    onchange: EventHandler<'a, String>,
    #[props(default)]
    onblur: EventHandler<'a, FocusEvent>,
    #[props(default)]
    onfocus: EventHandler<'a, FocusEvent>,
    // 前缀
    #[props(default)]
    prefix: Element<'a>,
    // 后缀
    #[props(default)]
    suffix: Element<'a>,
}

#[component(no_case_check)]
pub fn ZTextarea<'a>(cx: Scope<'a, ZTextareaProps<'a>>) -> Element {
    let focus = use_state(cx, || false);
    let zshare = use_shared_state::<ZShare>(cx).unwrap();
    cx.render(rsx! {
        div {
            class: zclass!(
                "rounded-[".to_owned()+zshare.read().radius.as_str()+"] border hover:ring-inset transition flex items-center overflow-hidden flex-none",
                zab!( "cursor-text",!cx.props.disabled),
                zab!( "cursor-not-allowed text-zinc-500 bg-zinc-100 border-zinc-100",cx.props.disabled,zab!(
                    zclass!(
                        zab!( "hover:ring-[99999px] hover:ring-black/5 bg-".to_owned()+zshare.read().color.as_str()+"-50 text-"+zshare.read().color.as_str()+"-500 border-"+zshare.read().color.as_str()+"-500",cx.props.color==""),
                        zab!( "hover:ring-[99999px] hover:ring-black/5 bg-".to_owned()+cx.props.color.as_str()+"-50 text-"+cx.props.color.as_str()+"-500 border-"+cx.props.color.as_str()+"-500",cx.props.color!="")
                    ),focus.get().clone()||cx.props.always,zclass!(
                        zab!( "hover:ring-[99999px] text-zinc-400 border-zinc-300 hover:ring-black/5 hover:border-".to_owned()+zshare.read().color.as_str()+"-500 hover:text-"+zshare.read().color.as_str()+"-500",cx.props.color==""),
                        zab!( "hover:ring-[99999px] text-zinc-400 border-zinc-300 hover:ring-black/5 hover:border-".to_owned()+cx.props.color.as_str()+"-500 hover:text-"+cx.props.color.as_str()+"-500",cx.props.color!="")
                    )
                )),
                cx.props.class
            ),

            
            &cx.props.prefix
            textarea {
                class:zclass!(
                    "px-3 py-2 w-full bg-transparent text-black z-20",
                    zab!( "cursor-not-allowed",cx.props.disabled),
                    cx.props.class_input
                ),
                rows:cx.props.rows,
                maxlength:cx.props.maxlength,
                onclick:move |_| {
                    focus.set(true);
                },
                onblur:move |evt| {
                    focus.set(false);
                    cx.props.onblur.call(evt);
                },
                onfocus:move |evt| {
                    focus.set(true);
                    cx.props.onfocus.call(evt);
                },
                oninput:move |evt| cx.props.onchange.call(evt.data.value.clone()),
                disabled:cx.props.disabled,
                value:cx.props.value.as_str(),
            },

            
            &cx.props.suffix
        }
    })
}
