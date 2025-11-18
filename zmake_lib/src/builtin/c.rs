use crate::make_builtin;
use ahash::AHashMap;
use static_init::dynamic;

make_builtin! {
    self => {
        "moe.kawayi:zmake@1.0.0" => {
            crate::id::IdType::ToolType =>
            {
                "compiler" => "c.compiler",
                "preprocessor" => "c.preprocessor"
            },
            crate::id::IdType::ToolName =>{
                "gcc" => "c.gcc",
                "clang" => "c.clang",
                "msvc" => "c.msvc"
            }
        }
    },
}
