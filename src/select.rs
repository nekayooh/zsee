use dioxus::prelude::*;

use crate::{
    icon::ZIcon, utils::{class::XZClass, share::ZShare}, zab, zclass, zs
};

#[derive(Props)]
pub struct ZSelectProps<'a> {
    //fill line half gray transparent
    #[props(into,default)]
    class: String,
    // 输入框补充样式
    #[props(into,default)]
    class_input: String,
    // 始终显示颜色
    #[props(default)]
    always: bool,
    // 是否开启搜索
    #[props(default)]
    filter: bool,
    #[props(into,default = zs!(""))]
    color: String,
    #[props(default)]
    disabled: bool,
    #[props(default)]
    value: String,
    // option
    #[props(default)]
    option: Vec<Vec<String>>,
    #[props(default)]
    onchange: EventHandler<'a, String>,
    // 前缀
    #[props(default)]
    prefix: Element<'a>,
    // 后缀
    #[props(default)]
    suffix: Element<'a>,
}

fn get_value_from_key(option:Vec<Vec<String>>,key:String)-> String{
    let one:Vec<Vec<String>> = option.iter().filter(|one|one[0].clone()==key).cloned().collect();
        if one.len()>0 {
            one[0][1].clone()
        }else {
            "".to_string()
        }
}

#[component(no_case_check)]
pub fn ZSelect<'a>(cx: Scope<'a, ZSelectProps<'a>>) -> Element {
    // 是否显示下拉列表
    let open = use_state(cx, || false);
    // 搜索显示值
    let search = use_state(cx, || "".to_string());
    let psearch = use_state(cx, || "".to_string());
    // 组件key值
    let key = use_state(cx, || "".to_string());
    let zshare = use_shared_state::<ZShare>(cx).unwrap();
    use_future(cx, (), |_|{
        // 初始化赋值
        search.set(get_value_from_key(cx.props.option.clone(),cx.props.value.clone()));
        psearch.set(get_value_from_key(cx.props.option.clone(),cx.props.value.clone()));
        key.set(cx.props.value.clone());
        async move {}
    });
    cx.render(rsx! {
        div {
            class:zclass!("flex","flex-col"),
            div {
                class: zclass!(
                    "rounded-[".to_owned()+zshare.read().radius.as_str()+"] border hover:ring-inset transition flex items-center overflow-hidden flex-none",
                    zab!( "cursor-text",!cx.props.disabled),
                    zab!( "cursor-not-allowed text-zinc-500 bg-zinc-100 border-zinc-100",cx.props.disabled,zab!(
                        zclass!(
                            zab!( "hover:ring-[99999px] hover:ring-black/5 bg-".to_owned()+zshare.read().color.as_str()+"-50 text-"+zshare.read().color.as_str()+"-500 border-"+zshare.read().color.as_str()+"-500",cx.props.color==""),
                            zab!( "hover:ring-[99999px] hover:ring-black/5 bg-".to_owned()+cx.props.color.as_str()+"-50 text-"+cx.props.color.as_str()+"-500 border-"+cx.props.color.as_str()+"-500",cx.props.color!="")
                        ),open.get().clone()||cx.props.always,zclass!(
                            zab!( "hover:ring-[99999px] text-zinc-400 border-zinc-300 hover:ring-black/5 hover:border-".to_owned()+zshare.read().color.as_str()+"-500 hover:text-"+zshare.read().color.as_str()+"-500",cx.props.color==""),
                            zab!( "hover:ring-[99999px] text-zinc-400 border-zinc-300 hover:ring-black/5 hover:border-".to_owned()+cx.props.color.as_str()+"-500 hover:text-"+cx.props.color.as_str()+"-500",cx.props.color!="")
                        )
                    )),
                    cx.props.class
                ),

                &cx.props.prefix
                input {
                    class:zclass!(
                        "px-3 py-2 w-full bg-transparent text-black z-20",
                        zab!( "cursor-not-allowed",cx.props.disabled,zab!( "cursor-text",cx.props.filter,"cursor-pointer")),
                        zab!( "z-[123]",open.get().clone(),"z-20"),
                        cx.props.class_input
                    ),
                    readonly:!cx.props.filter,
                    onclick:move |_| {
                        open.set(true);
                    },
                    onfocus:move |_| {
                        if !open.get().clone() {
                            psearch.set("".to_string());
                        }
                    },
                    oninput:move |evt| {
                        psearch.set(evt.data.value.clone());
                    },
                    disabled:cx.props.disabled,
                    value:"{psearch}",
                    placeholder:"{search}",
                    
                }
                &cx.props.suffix
            }
            div {
                class:"relative",
                div {
                    class:zclass!("fixed left-0 top-0 bg-tansparent",zab!("w-screen h-screen z-[121]",open.get().clone(),"w-0 h-0")),
                    onclick:move |_| {
                        // 关闭下拉选框，恢复原值
                        search.set(get_value_from_key(cx.props.option.clone(),cx.props.value.clone()));
                        psearch.set(get_value_from_key(cx.props.option.clone(),cx.props.value.clone()));
                        key.set(cx.props.value.clone());
                        open.set(false);
                    },
                }
                div {
                    class:zclass!(
                        "rounded-[".to_owned()+zshare.read().radius.as_str()+"] absolute border-zinc-300 bg-white flex flex-col items-center transition-[max-height,min-height,padding,border,opacity] overflow-auto z-[122] w-full",
                    zab!("max-h-[13rem] min-h-[13rem] border opacity-100",open.get().clone(),"max-h-[0px] min-h-[0px] opacity-0")),
                    cx.props.option.iter().filter(|each| each[1].contains(psearch.get())).map(|each|{
                        cx.render(rsx!(div{
                            class:zclass!("w-full px-3 py-2 cursor-pointer flex items-center gap-1",
                            zab!( "text-black hover:bg-".to_owned()+zshare.read().color.as_str()+"-100 hover:text-"+zshare.read().color.as_str()+"-500",cx.props.color==""),
                            zab!( "text-black hover:bg-".to_owned()+cx.props.color.as_str()+"-100 hover:text-"+cx.props.color.as_str()+"-500",cx.props.color!=""),
                            zab!( "bg-".to_owned()+zshare.read().color.as_str()+"-100 text-"+zshare.read().color.as_str()+"-500",cx.props.color==""&&key.get().clone()==each[0]),
                            zab!( "bg-".to_owned()+cx.props.color.as_str()+"-100 text-"+cx.props.color.as_str()+"-500",cx.props.color!=""&&key.get().clone()==each[0])),
                            onclick:move |_| {
                                // 赋值
                                key.set(each[0].to_string());
                                search.set(each[1].to_string());
                                psearch.set(each[1].to_string());
                                cx.props.onchange.call(each[0].to_string());
                                open.set(false);
                            },
                            if key.get().clone()==each[0] {
                                cx.render(rsx!(ZIcon {
                                    icon:"check",
                                }))
                            }
                            "{each[1]}"
                        }))
                    })
                    if  cx.props.option.iter().filter(|each| each[1].contains(psearch.get())).clone().collect::<Vec<_>>().len()==0{
                        cx.render(rsx!(div{
                            class:zclass!("w-full h-[13rem] flex justify-center items-center text-zinc-500 text-sm"),
                            "暂无结果"
                        }))
                    }
                }
            }
        }
    })
}
