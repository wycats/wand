use std::time::Instant;

use wasmtime::{Config, Engine, Func, Instance, IntoFunc, Linker, Memory, Module, Store};

use crate::traits::GetMemory;

pub struct WasmEngine {
    engine: Engine,
    start: Instant,
}

impl WasmEngine {
    pub fn new() -> anyhow::Result<WasmEngine> {
        tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_ansi(true)
            .init();

        let mut config = Config::default();
        let config = config.cache_config_load_default()?;

        let engine = Engine::new(&config);
        // let engine = Engine::default();

        // let wasi = Wasi::new(
        //     &store,
        //     WasiCtxBuilder::new()
        //         .inherit_stdio()
        //         .inherit_args()
        //         .build()?,
        // );
        // wasi.add_to_linker(&mut linker)?;

        Ok(WasmEngine {
            engine,
            start: Instant::now(),
        })
    }

    pub fn linker(&self) -> WasmLinker {
        let store = Store::new(&self.engine);

        WasmLinker {
            engine: self.engine.clone(),
            start: self.start,
            linker: Linker::new(&store),
        }
    }
}

pub struct WasmLinker {
    engine: Engine,
    #[allow(unused)]
    start: Instant,
    linker: Linker,
}

impl WasmLinker {
    pub fn func<Params, Args>(
        &mut self,
        module: &str,
        name: &str,
        func: impl IntoFunc<Params, Args>,
    ) -> anyhow::Result<&mut Self> {
        self.linker.func(module, name, func)?;
        Ok(self)
    }

    pub fn instantiate_module<'a>(&'a self, bytes: &'a [u8]) -> anyhow::Result<ModuleInstance> {
        let module = self.module(bytes)?;

        self.instantiate(module)
    }

    pub fn module(&self, bytes: &[u8]) -> anyhow::Result<Module> {
        Module::validate(&self.engine, bytes)?;

        Module::new(&self.engine, bytes)
    }

    pub fn instantiate<'a>(&'a self, module: Module) -> anyhow::Result<ModuleInstance> {
        let instance = self.linker.instantiate(&module)?;
        ModuleInstance::instantiate(module, instance)
    }
}

pub struct ModuleInstance {
    #[allow(unused)]
    module: Module,
    instance: Instance,
}

impl GetMemory for ModuleInstance {
    fn memory(&self) -> Memory {
        self.instance
            .get_export("memory")
            .expect("expected a 'memory' export")
            .into_memory()
            .expect("expected 'memory' export to be memory")
    }
}

impl ModuleInstance {
    pub fn instantiate(module: Module, instance: Instance) -> anyhow::Result<ModuleInstance> {
        Ok(ModuleInstance { module, instance })
    }

    pub fn get_func(&self, name: &str) -> Func {
        self.instance
            .get_func(name)
            .unwrap_or_else(|| panic!("unexpectedly missing function {}", name))
    }

    pub fn call0<R>(&self, name: &str) -> Result<R, wasmtime::Trap>
    where
        R: wasmtime::WasmTy,
    {
        let func = self.get_func(name).get0::<R>()?;
        func()
    }

    pub fn call1<A, R>(&self, name: &str, arg: A) -> Result<R, wasmtime::Trap>
    where
        A: wasmtime::WasmTy,
        R: wasmtime::WasmTy,
    {
        let func = self.get_func(name).get1::<A, R>()?;
        func(arg)
    }

    pub fn call2<A1, A2, R>(&self, name: &str, arg1: A1, arg2: A2) -> Result<R, wasmtime::Trap>
    where
        A1: wasmtime::WasmTy,
        A2: wasmtime::WasmTy,
        R: wasmtime::WasmTy,
    {
        let func = self.get_func(name).get2::<A1, A2, R>()?;
        func(arg1, arg2)
    }
}
