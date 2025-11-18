use deno_core::{JsRuntime, RuntimeOptions};
use deno_fs::{FsPermissions, RealFs};
use deno_permissions::PermissionsContainer;
use std::cell::Cell;
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::LazyLock;
use tracing::{trace, trace_span};

#[derive(Debug)]
pub struct EngineOptions {
    pub tokio_runtime: tokio::runtime::Runtime,
}

pub struct Engine {
    pub runtime: JsRuntime,
    pub tokio_runtime: tokio::runtime::Runtime,
}

impl Engine {
    pub fn new_resolve_engine(options: EngineOptions) -> eyre::Result<Self> {
        let mut fs =
            deno_fs::deno_fs::init::<deno_permissions::PermissionsContainer>(Rc::from(RealFs {}));

        fs.op_state_fn = Some(Box::from(|status: &mut deno_core::OpState| {
            status.put(12);
        }));

        deno_tty

        let runtime = JsRuntime::try_new(RuntimeOptions {
            extensions: vec![],
            ..Default::default()
        })?;

        //runtime.load_side_es_module(specifier)?;

        Ok(Engine {
            runtime,
            tokio_runtime: options.tokio_runtime,
        })
    }

    pub fn new(options: EngineOptions) -> eyre::Result<Self> {
        let fs =
            deno_fs::deno_fs::init::<deno_permissions::PermissionsContainer>(Rc::from(RealFs {}));

        let runtime = JsRuntime::try_new(RuntimeOptions {
            extensions: vec![],
            ..Default::default()
        })?;

        //runtime.load_side_es_module(specifier)?;

        Ok(Engine {
            runtime,
            tokio_runtime: options.tokio_runtime,
        })
    }

    pub fn execute_and_to_json(&mut self, code: &str, source_name: &str) -> std::string::String {
        "".to_string()
    }
}
