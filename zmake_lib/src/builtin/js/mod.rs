use crate::{make_builtin_js, module_specifier::ModuleSpecifier};

pub static RT_CODE: &'static str = concat!(std::env!("CARGO_MANIFEST_DIR"), "/../dist/rt.js");

#[::static_init::dynamic(lazy)]
pub static RT: ModuleSpecifier = ModuleSpecifier::Builtin("rt".to_string());

make_builtin_js! {
    pub fn log(scope, args, return_value) => {

        if args.length() <= 1 {
            return;
        }

        return_value.set_undefined();
    }
}
