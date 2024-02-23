use bson::Document;
use dioxus::prelude::*;
use crate::{
    utils::{bson::{bson_f64, bson_str}, class::XZClass, share::ZShare}, zab, zclass, zs
};


#[derive(PartialEq,Props)]
pub struct ZTableProps {
    #[props(default)]
    class: String,
    #[props(default = vec![])]
    head: Vec<ZTableHeadProps>,
    #[props(default = vec![])]
    body:Vec<Vec<String>>,
    #[props(default = vec![])]
    body2:Vec<Document>,
}

#[derive(Default,PartialEq,Props)]
pub struct ZTableHeadProps {
    #[props(into,default)]
    pub name: String,
    #[props(into,default)]
    pub field: String,
    #[props(into,default)]
    pub width: String,
    #[props(into,default)]
    pub r#type: String,
    #[props(into,default)]
    pub float: String,
}

#[component(no_case_check)]
pub fn ZTable(cx: Scope<ZTableProps>) -> Element {
    let zshare = use_shared_state::<ZShare>(cx).unwrap();
    cx.render(rsx! {
        div {
            class:zclass!("overflow-auto border border-zinc-300 rounded-[".to_owned()+zshare.read().radius.as_str()+"]",cx.props.class),
            table {
                class:"border-collapse border-none",
                thead {
                    tr {
                        class:"h-full",
                        cx.props.head.iter().map(|head| rsx!(
                            th {
                                class:zclass!("sticky min-w-[".to_owned()+head.width.as_str()+"] max-w-["+head.width.as_str()+"] p-0 top-0 first:left-0 first:z-20 last:right-0 last:z-20 z-table-diff"),
                                div {
                                    class:"z-table border-b border-l px-2 py-1 border-zinc-300 h-full flex justify-center items-center",
                                    // &head.custom_head
                                    "{head.name}"
                                }
                            }
                        ))
                    }
                }
                tbody {
                    cx.props.body2.iter().enumerate().map(|(ib,body)| rsx!(
                        tr {
                            class:"h-full",
                            cx.props.head.iter().enumerate().map(|(_,head)| rsx!(
                                td {
                                    class:"p-0 first:sticky first:bg-white last:bg-white last:sticky first:left-0 first:z-10 last:right-0 last:z-10 overflow-auto z-table-diff",
                                    
                                    div {
                                        class:zclass!("z-table border-b border-l border-zinc-300 h-full px-2 flex items-center",
                                        zab!("justify-center",head.float=="center"),
                                        zab!("justify-end",head.float=="right"),
                                        zab!("justify-left",head.float=="left")
                                    ),
                                        match head.r#type.as_str() {
                                            "idx"=>zs!(ib+1),
                                            "string"=>bson_str(&body, head.field.as_str()),
                                            "f64"=> format!("{:.2}", bson_f64(&body, head.field.as_str())),
                                            _=>bson_str(&body, head.field.as_str())
                                        }
                                        
                                    }
                                }
                            ))
                        }
                    ))
                    
                }
            }
        }
    })
}
