#[macro_export]
macro_rules! zclass {
    ($($each:expr),*) => {
        {
            let mut vec_class:Vec<String> = Vec::new();
            $(
                vec_class.push($each.to_string());
            )*
            XZClass::from(vec_class)
        }
    };
}

#[macro_export]
macro_rules! zcs {
    ($($each:expr),*) => {
        {
            let mut vec_class:Vec<String> = Vec::new();
            $(
                vec_class.push($each.to_string());
            )*
            vec_class.join(" ")
        }
    };
}

// 格式化成string
#[macro_export]
macro_rules! zs {
    ($a:expr) => {
        $a.to_string()
    };
}

#[macro_export]
macro_rules! zab {
    ($a:expr,$bool:expr) => {
        if $bool {
            $a.to_string()
        } else {
            "".to_string()
        }
    };

    ($a:expr,$bool:expr,$b:expr) => {
        if $bool {
            $a.to_string()
        } else {
            $b.to_string()
        }
    };
}


#[macro_export]
macro_rules! zi18n {
    ($zshare:expr,$key:expr) => {
        ($zshare.read().word.get($key).unwrap_or(&("".to_string()))).as_str()
    };
}
