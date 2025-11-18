#[macro_export]
macro_rules! make_builtin {
    {
        self => { $($id:literal => { $($type:path => { $($key:literal => $value:literal),* }),* } ),* },
        $( $submodule:ident ),*
    } => {


        #[::static_init::dynamic(lazy)]
        pub static BUILTINS: AHashMap<String, crate::id::Id> = {
            let mut map = ::ahash::AHashMap::<::std::string::String, $crate::id::Id>::new();

            //a.into_iter().map(|(k, v)| b.insert(k, v));
            $(
                $submodule::BUILTINS.clone().into_iter().map(|(k, v)| map.insert(k, v));
            )*

            $(
                let id = $id;

                $(
                    let id_type: &'static str = $type.into();

                    $(
                        let value = format!("{}#{}::{}", id,id_type,$value);
                        let value = <$crate::id::Id as ::std::str::FromStr>::from_str(&value).unwrap();

                        map.insert($key.to_string(),value);
                    )*
                )*
            )*

            map
        };
    }
}
