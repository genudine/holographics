/// query! templates out a HashMap (actually an IndexMap) for you. These will be made into query parameters.
/// - Simple K,V:  
///   `query!(key, value)` => { key: value }
/// - Multiple K,V:  
///   `query!((key, value), (key2, value2))` => { key: value, key2: value2 }
/// - Default:
///   `query!()` => {}
#[macro_export]
macro_rules! query {
    () => {
        indexmap::IndexMap::<&'static str, String>::new()
    };
    ($field:expr, $value:expr) => {
        indexmap::IndexMap::<&'static str, String>::from([($field, $value)])
    };
    ($field:expr, $($rest:tt)*) => {
        {
            let mut map = indexmap::IndexMap::<&'static str, String>::from([$field, $($rest)*]);
            map.extend(query!($($rest)*));
            map
        }
    };
}
