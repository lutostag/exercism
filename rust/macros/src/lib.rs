#[macro_export]
macro_rules! hashmap {
    ( $( $x:expr => $y:expr ), * $(,)? ) => {
        {
            let mut _temp = std::collections::HashMap::new();
            $( _temp.insert($x, $y); )*
            _temp
        }
    };
}
