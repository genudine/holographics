#[macro_export]
macro_rules! query {
    () => {
        indexmap::IndexMap::new()
    };
    ($field:expr, $value:expr) => {
        indexmap::IndexMap::from([($field, $value)])
    };
}
