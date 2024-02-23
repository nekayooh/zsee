use crate::{
    {button::ZButton, icon::ZIcon}, utils::{class::XZClass, share::ZShare}, zab, zclass,  zs
};
use dioxus::prelude::*;

#[derive(Props)]
pub struct ZModalProps<'a> {
    #[props(default)]
    pub show: bool,
    #[props(into,default)]
    pub icon: String,
    #[props(into,default)]
    pub color: String,
    #[props(into,default)]
    pub title: String,
    #[props(into,default=zs!("80vw"))]
    pub width: String,
    #[props(into,default=zs!("80vh"))]
    pub height: String,
    // 自定义
    #[props(default=false)]
    pub custom: bool,
    #[props(default)]
    pub onclose: EventHandler<'a, MouseEvent>,
    #[props(default)]
    children: Element<'a>,
}

#[component(no_case_check)]
pub fn ZModal<'a>(cx: Scope<'a, ZModalProps<'a>>) -> Element {
    let zshare = use_shared_state::<ZShare>(cx).unwrap();
    // use_future(cx, &(zshare.read().toast), |_| {
    //     let zshare = zshare.clone();
    //     async move {
    //         // gloo_console::console!(format!("{:?}",zshare.read().toast.time));
    //         let xtime = zshare.read().toast.time;
    //         gloo_timers::future::TimeoutFuture::new(xtime).await;

    //         zshare.write().toast.show = false
    //     }
    // });
    cx.render(rsx! {
        div {
            class:"fixed left-[50vw] top-[50vh] flex justify-center items-center z-[999]",
            div {
                class:zclass!("absolute transition-[background,width,height,left,top]",zab!("left-[-50vw] top-[-50vh] w-screen h-screen bg-zinc-500/20 z-[1]",cx.props.show,"left-0 top-0 w-0 h-0 bg-transparent"))
            }
            div {
                class:zclass!(
                    "rounded-[".to_owned()+zshare.read().radius.as_str()+"] fixed border-zinc-300 bg-white flex flex-col items-center gap-2 transition-[max-width,max-height,min-width,min-height,padding,border,opacity] overflow-hidden z-10",
                zab!("max-w-[".to_owned()+cx.props.width.clone().as_str()+"] max-h-["+cx.props.height.clone().as_str()+"] min-w-["+cx.props.width.clone().as_str()+"] min-h-["+cx.props.height.clone().as_str()+"] px-5 py-3 border opacity-100",cx.props.show,"max-w-[0px] max-h-[0px] min-w-[0px] min-h-[0px] opacity-0".to_string())),
                
                if cx.props.custom {
                    cx.render(rsx!(&cx.props.children))
                } else {
                    cx.render(rsx!(
                        div {
                            class:"w-full flex-none flex items-center gap-2",
                            ZIcon {
                                class: "flex-none text-".to_owned()+zab!(zshare.read().color,cx.props.color=="",cx.props.color).as_str()+"-500",
                                icon: cx.props.icon.clone()
                            }
                            div {
                                class:"flex-grow text-sm",
                                "{cx.props.title}"
                            }
                            ZButton {
                                class:"flex-none rounded-full !p-0 w-5 h-5 !text-white",
                                mode:"fill",
                                color:"red",
                                onclick:move |evt|cx.props.onclose.call(evt),
                                ZIcon {
                                    class: "text-white text-sm",
                                    icon: "x"
                                }
                            }
                        }
                        &cx.props.children
                    ))
                }
            }
        }
    })
}
