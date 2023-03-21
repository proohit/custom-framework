pub(self) mod constants;
pub(self) mod env_module;

use self::{constants::Constants, env_module::get_env_module};
use once_cell::sync::Lazy;
use std::{env, sync::RwLock};
use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    Vm, WasmValue,
};

static mut VM: Lazy<RwLock<Option<Vm>>> = Lazy::new(|| RwLock::new(None));

pub(crate) fn init() {
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(
            //
            HostRegistrationConfigOptions::default().wasi(true),
        )
        .build()
        .unwrap();

    let wasm_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(Constants::CustomFrameworkModulePath.as_str());

    let mut env_vars: Vec<String> = Vec::new();
    for (k, v) in env::vars() {
        let var = format!("{}={}", k, v);
        env_vars.push(var);
    }

    let module = get_env_module();

    // create a vm
    let mut vm = Vm::new(Some(config))
        .unwrap()
        .register_import_module(module)
        .unwrap()
        .register_module_from_file(Constants::CustomFrameworkModuleName.as_str(), wasm_file)
        .unwrap();

    let mut wasi_module = vm.wasi_module().unwrap();
    wasi_module.initialize(None, Some(env_vars.iter().map(|s| &**s).collect()), None);

    unsafe {
        let mut NEW_VM = VM.write().unwrap();
        *NEW_VM = Some(vm);
    }
}

pub(crate) fn get_string_pointer(data: &str) -> i32 {
    let data_len = data.len();
    let allocate_result = unsafe {
        VM.read()
            .unwrap()
            .as_ref()
            .unwrap()
            .run_func(
                Some(Constants::CustomFrameworkModuleName.as_str()),
                Constants::FunctionAllocate.as_str(),
                Some(WasmValue::from_i32((data_len + 1).try_into().unwrap())),
            )
            .unwrap()
    };
    let input_pointer = allocate_result[0].to_i32();

    let mut mem = unsafe {
        VM.read()
            .unwrap()
            .as_ref()
            .unwrap()
            .named_module(Constants::CustomFrameworkModuleName.as_str())
            .unwrap()
            .memory(Constants::MemoryName.as_str())
            .unwrap()
    };
    mem.write(data, input_pointer.try_into().unwrap()).unwrap();
    input_pointer
}

pub(crate) fn get_string_from_pointer(ptr: u32, len: u32) -> String {
    let mem = unsafe {
        VM.read()
            .unwrap()
            .as_ref()
            .unwrap()
            .named_module(Constants::CustomFrameworkModuleName.as_str())
            .unwrap()
            .memory(Constants::MemoryName.as_str())
            .unwrap()
    };
    mem.read_string(ptr, len).unwrap()
}

pub(crate) fn start(raw_routes: &str) {
    unsafe {
        VM.read()
            .unwrap()
            .as_ref()
            .unwrap()
            .run_func(
                Some(Constants::CustomFrameworkModuleName.as_str()),
                Constants::FunctionStart.as_str(),
                Some(WasmValue::from_i32(get_string_pointer(raw_routes))),
            )
            .unwrap();
    }
}
