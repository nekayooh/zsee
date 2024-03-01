use bson::{doc, Document};

// 全部包含返回true
#[macro_export]
macro_rules! eq_check {
    ($doc:expr,$($key:expr),*) => {
        {
            let mut one_eq = true;
            $(
                if $doc!=$key {
                    one_eq = false;
                }
            )*
            one_eq
        }
    };
}
// 包含其中一个字段返回true
#[macro_export]
macro_rules! eq_check_one {
    ($doc:expr,$($key:expr),*) => {
        {
            let mut one_eq = false;
            $(
                if $doc==$key {
                    one_eq = true;
                }
            )*
            one_eq
        }
    };
}
// 查询校验字段
#[macro_export]
macro_rules! doc_check {
    ($doc:expr,$($key:expr),*) => {
        {
            let mut all_has = true;
            $(
                if !$doc.contains_key($key) {
                    all_has = false;
                }
            )*
            all_has
        }
    };
}
// 获取string
#[macro_export]
macro_rules! doc_get {
    ($doc:expr,$key:expr) => {
        {
            match $doc.get($key) {
                Some(one) => {
                    one.to_string()
                }
                _ => {
                    "".to_string()
                }
            }
        }
    };
}



pub fn bson_str(bs: &bson::Document, key: &str) -> String {
    match bs.clone().get_str(&key) {
        Ok(one) => {
            one.to_string()
        }
        _ => {
            "".to_string()
        }
    }
}

// pub fn bson_json(bs: &bson::Document, key: &str) -> String {
//     match bs.clone().get_document(&key) {
//         Ok(one) => {
//             match serde_json::to_string(one) {
//                 Ok(json_string) => {
//                     json_string
//                 }
//                 Err(_) => {
//                     "{}".to_string()
//                 }
//             }
//         }
//         _ => {
//             "{}".to_string()
//         }
//     }
// }

// pub fn bson_array_json(bs: &bson::Document, key: &str) -> String {
//     match bs.clone().get_array(&key) {
//         Ok(array) => {
//             match serde_json::to_string(array) {
//                 Ok(json_string) => {
//                     json_string
//                 }
//                 Err(_) => {
//                     "[]".to_string()
//                 }
//             }
//         }
//         _ => {
//             "[]".to_string()
//         }
//     }
// }
pub fn bson_vstr(bs: &bson::Document, key: &str) -> Vec<String> {
    match bs.clone().get_array(&key) {
        Ok(array) => {
            let mut rtn = vec![];
            for each in array.iter() {
                rtn.push(each.as_str().unwrap_or_default().to_string());
            }
            rtn.clone()
        }
        _ => {
           vec![]
        }
    }
}
pub fn bson_vdoc(bs: &bson::Document, key: &str) -> Vec<Document> {
    match bs.clone().get_array(&key) {
        Ok(array) => {
            let mut rtn = vec![];
            for each in array.iter() {
                rtn.push(each.as_document().unwrap_or(&doc!{}).clone());
            }
            rtn.clone()
        }
        _ => {
           vec![]
        }
    }
}

pub fn bson_doc(bs: &bson::Document, key: &str) -> Document {
    match bs.clone().get_document(&key) {
        Ok(ndoc) => {
            ndoc.clone()
        }
        _ => {
           doc! {}
        }
    }
}

pub fn bson_bool(bs: &bson::Document, key: &str) -> bool {
    match bs.clone().get_bool(&key) {
        Ok(one) => {
            one
        }
        _ => {
            false
        }
    }
}

pub fn bson_f64(bs: &bson::Document, key: &str) -> f64 {
    let mut one_f64 = 0.00;
    match bs.clone().get_f64(&key) {
        Ok(count) => {
            one_f64 = count;
        }
        _ => {
            match bs.clone().get_i32(key) {
                Ok(count) => {
                    one_f64 = count.into();
                }
                _ => {
                    match bs.clone().get_str(key) {
                        Ok(count) => {
                            one_f64 = count.parse::<f64>().unwrap_or(0.0).into();
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    one_f64
}

pub fn bson_i64(bs: &bson::Document, key: &str) -> i64 {
    let mut one_i64 = 0;
    match bs.clone().get_i64(&key) {
        Ok(count) => {
            one_i64 = count;
        }
        _ => {
            match bs.clone().get_i32(key) {
                Ok(count) => {
                    one_i64 = count.into();
                }
                _ => {
                    match bs.clone().get_str(key) {
                        Ok(count) => {
                            one_i64 = count.parse::<i64>().unwrap_or(0).into();
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    one_i64
}

// pub fn _bson_i16(bs: &bson::Document, key: &str) -> i16 {
//     let mut one_i16 = 0;
//     match bs.clone().get_i32(&key) {
//         Ok(count) => {
//             one_i16 = count as i16;
//         }
//         _ => {
//             match bs.clone().get_i64(key) {
//                 Ok(count) => {
//                     one_i16 = count as i16;
//                 }
//                 _ => {}
//             }
//         }
//     }
//     one_i16
// }

// pub fn _bson_time(bs: &bson::Document, key: &str) -> i64 {
//     let mut one_f64 = 0;
//     match bs.clone().get_datetime(&key) {
//         Ok(count) => {
//             one_f64 = count.timestamp_millis();
//         }
//         _ => {}
//     }
//     one_f64
// }
