use crate::{
    icon::ZIcon, utils::{
        class:: XZClass,
        share::ZShare,
    }, zab, zclass
};
use dioxus::prelude::*;


#[derive(PartialEq,Clone,Default)]
pub struct ZToastProps {
    pub show: bool,
    pub icon: String,
    pub color: String,
    pub text: String,
    pub time: u32,
}

impl ZToastProps {
    pub fn new()->Self{
        ZToastProps {
            time:2000,
            ..Default::default()
        }
    }
    pub fn show(color:String,icon:String,text:String,)->Self{
        ZToastProps {
            show:true,
            color,
            icon,
            text,
            time:2000,
        }
    }
    pub fn show_t(color:String,icon:String,text:String,time:u32)->Self{
        ZToastProps {
            show:true,
            color,
            icon,
            text,
            time,
        }
    }
}

#[component(no_case_check)]
pub fn ZToast(cx: Scope) -> Element {
    let zshare = use_shared_state::<ZShare>(cx).unwrap();
    use_future(cx, &(zshare.read().toast), |_| {
        let zshare = zshare.clone();
        async move {
            // gloo_console::console!(format!("{:?}",zshare.read().toast.time));
            let xtime = zshare.read().toast.time;
            gloo_timers::future::TimeoutFuture::new(xtime).await;

            zshare.write().toast.show = false
        }
    });
    cx.render(rsx! {
        div {
            class:"fixed left-[50vw] top-[50vh] flex justify-center items-center z-[999]",
            div {
                onclick:move|_|{
                    zshare.write().toast.show=false
                },
                class:zclass!(
                    "rounded-[".to_owned()+zshare.read().radius.as_str()+"] fixed border-zinc-300 bg-white flex items-center gap-2 transition-[max-width,max-height,padding,border,opacity] select-none overflow-hidden",
                zab!("max-w-[50vw] max-h-[50vh] px-5 py-3 border opacity-100",zshare.read().toast.show,"max-w-[0px] max-h-[0px] opacity-0")),
                    ZIcon {
                        class: "text-".to_owned()+zshare.read().toast.color.as_str()+"-500",
                        icon: zshare.read().toast.icon.clone()
                    }
                    "{zshare.read().toast.text}"
            }
        }
    })
}
