// https://stackoverflow.com/questions/34953711/unwrap-inner-type-when-enum-variant-is-known
macro_rules! cast {
    ($target: expr, $pat: path) => {
        {
            if let $pat(a) = $target {
                a
            } else {
                panic!("mismatched variant when cast to {}", stringify!($pat));
            }
        }
    };
}

pub(crate) use cast;