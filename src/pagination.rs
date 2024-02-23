use dioxus::prelude::*;

use crate::{{button::ZButton, icon::ZIcon}, utils::{class::XZClass, share::ZShare}, zab, zclass, zi18n, zs};


#[derive(Props)]
pub struct ZPaginationProps<'a> {
    #[props(default)]
    class: String,
    #[props(default=0)]
    total: i64,
    #[props(default=20)]
    page_count: i64,
    #[props(default=0)]
    page: i64,
    #[props(default)]
    onchange: EventHandler<'a, i64>,
}

// 获取页面总数
fn total_page(total:i64,page_count:i64)->i64 {
    // 取余数
    let mut ye = total/page_count;
    let yu = total%page_count;
    if yu>0 {
        ye += 1;
    }
    
    if ye == 0 {
        ye =1;
    }
    ye
}

fn vec_page(page:i64,total:i64,page_count:i64)-> Vec<i64> {
    let mut rvec = vec![];
    let tpage = total_page(total,page_count);
   

    if tpage>=7 {
        if page<=4 {
            (1..8).into_iter().for_each(|each|{
                rvec.push(each);
            });
        
        } else if  (tpage+1)-page<=4 {
            ((tpage-6)..(tpage+1)).into_iter().for_each(|each|{
                rvec.push(each);
            });
        } else {
            ((page-3)..(page+4)).into_iter().for_each(|each|{
                rvec.push(each);
            });
        }
    } else {
        (1..(tpage+1)).into_iter().for_each(|each|{
            rvec.push(each);
        });
    }
    
    rvec
}

#[component(no_case_check)]
pub fn ZPagination<'a>(cx: Scope<'a,ZPaginationProps<'a>>) -> Element {
    let zshare = use_shared_state::<ZShare>(cx).unwrap();
    cx.render(rsx! {
        div {
            
            class: zclass!(
                "flex gap-1",
                cx.props.class
            ),
            ZButton {
                class:"text-sm h-9 p-0",
                mode:"line",
                color:zshare.read().color.clone(),
                zi18n!(zshare,"c_pagination_total1").to_owned()+zs!(cx.props.total).as_str()+zi18n!(zshare,"c_pagination_total2")
            }
            // ZButton {
            //     class:zs!("text-sm h-9 p-0"),
            //     mode:zs!("line"),
            //     color:zshare.read().color.clone(),
            //     zi18n!(zshare,"c_pagination_total1").to_owned()+zs!(total_page(cx.props.total,cx.props.page_count)).as_str()+zi18n!(zshare,"c_pagination_total2")
            // }
            ZButton {
                class:"text-sm w-10 h-9",
                mode:"fill",
                color:zshare.read().color.clone(),
                disabled:cx.props.page==1,
                onclick:move |_| cx.props.onchange.call(cx.props.page-1),
                ZIcon {
                    icon:"arrow-left"
                }
            }
            vec_page(cx.props.page,cx.props.total,cx.props.page_count).iter().map(|i|{
                let i = i.clone();
                cx.render(rsx!{
                    ZButton {
                        class:"text-sm w-10 h-9",
                        mode:zab!("fill",cx.props.page==i,"half"),
                        color:zshare.read().color.clone(),
                        onclick:move |_| cx.props.onchange.call(i),
                        "{i}"
                    }
                })
            })
            ZButton {
                class:"text-sm w-10 h-9",
                mode:"fill",
                color:zshare.read().color.clone(),
                disabled:cx.props.page==total_page(cx.props.total,cx.props.page_count),
                onclick:move |_| cx.props.onchange.call(cx.props.page+1),
                ZIcon {
                    icon:"arrow-right"
                }
            }
        }
    })
}
