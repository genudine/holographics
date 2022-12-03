#[macro_export]
macro_rules! query {
    () => {
        indexmap::IndexMap::new()
    };
    ($field:expr, $value:expr) => {
        indexmap::IndexMap::from([($field, $value)])
    };
    ($field:expr, $($rest:tt)*) => {
        {
            let mut map = indexmap::IndexMap::from([$field]);
            map.extend(query!($($rest)*));
            map
        }
    };
}
