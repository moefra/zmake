use crate::make_builtin;
use ahash::AHashMap;

make_builtin! {
    self => {
        "moe.kawayi:zmake@1.0.0" => {
            crate::id::IdType::ToolType =>
            {
                "compiler" => "cpp.compiler",
                "preprocessor" => "cpp.preprocessor"
            },
            crate::id::IdType::ToolName =>{
                "gcc" => "cpp.gcc",
                "clang" => "cpp.clang",
                "msvc" => "cpp.msvc"
            }
        }
    },
}
