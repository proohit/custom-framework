use std::{collections::HashMap, env};

use once_cell::sync::Lazy;
use serde_json::Value;
use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    error::HostFuncError,
    CallingFrame, FuncType, ImportObjectBuilder, ValType, Vm, WasmValue,
};
use wasmedge_sys::Engine;

type RequestHandler = fn(Value) -> String;

static mut ROUTES: Lazy<HashMap<i32, RequestHandler>> = Lazy::new(|| HashMap::new());

pub struct Server {
    vm: Vm,
}

fn handle_request_external(
    calling_frame: CallingFrame,
    params: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, HostFuncError> {
    let request_handler_index = params[0].to_i32();
    let request_pointer = params[1].to_i32();
    let request_length = params[2].to_i32();

    let mem = calling_frame.memory_mut(0).unwrap();
    let request_body_string = get_string_from_pointer(
        request_pointer.try_into().unwrap(),
        request_length.try_into().unwrap(),
        &mem,
    );

    let response_body_string = unsafe {
        let request_handler = ROUTES.get(&request_handler_index).unwrap();
        request_handler(serde_json::from_str(&request_body_string).unwrap())
    };

    let response_body_pointer = get_string_pointer_2(&response_body_string, &calling_frame);
    Ok(vec![WasmValue::from_i32(response_body_pointer)])
}

fn get_string_pointer(data: &str, vm: &Vm) -> i32 {
    let data_len = data.len();
    let allocate_result = vm
        .run_func(
            Some("custom_framework"),
            "allocate",
            Some(WasmValue::from_i32((data_len + 1).try_into().unwrap())),
        )
        .unwrap();
    let input_pointer = allocate_result[0].to_i32();

    let mut mem = vm
        .named_module("custom_framework")
        .unwrap()
        .memory("memory")
        .unwrap();
    mem.write(data, input_pointer.try_into().unwrap()).unwrap();
    input_pointer
}

fn get_string_pointer_2(data: &str, calling_frame: &CallingFrame) -> i32 {
    let data_len = data.len();
    let executor = calling_frame.executor_mut().unwrap();
    let instance = calling_frame.module_instance().unwrap();
    let allocate_fn = instance.get_func("allocate").unwrap();
    let allocate_result = executor
        .run_func(
            &allocate_fn,
            Some(WasmValue::from_i32((data_len + 1).try_into().unwrap())),
        )
        .unwrap();
    let input_pointer = allocate_result[0].to_i32();

    let mut mem = calling_frame.memory_mut(0).unwrap();
    mem.set_data(data, input_pointer.try_into().unwrap())
        .unwrap();

    // terminate with 0
    mem.set_data(&[0], (input_pointer + data_len as i32).try_into().unwrap())
        .unwrap();
    input_pointer
}

fn get_string_from_pointer(ptr: u32, len: u32, mem: &wasmedge_sys::Memory) -> String {
    let mem_data = mem.get_data(ptr, len).unwrap();
    String::from_utf8(mem_data).unwrap()
}

impl Server {
    pub fn new() -> Server {
        let config = ConfigBuilder::new(CommonConfigOptions::default())
            .with_host_registration_config(
                //
                HostRegistrationConfigOptions::default().wasi(true),
            )
            .build()
            .unwrap();

        let wasm_file =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("lib/custom_framework_wasm.wasm");

        let mut env_vars: Vec<String> = Vec::new();
        for (k, v) in env::vars() {
            let var = format!("{}={}", k, v);
            env_vars.push(var);
        }

        let module = ImportObjectBuilder::new()
            .with_func_by_type(
                "handle_request_external",
                FuncType::new(
                    Some(vec![ValType::I32, ValType::I32, ValType::I32]),
                    Some(vec![ValType::I32]),
                ),
                handle_request_external,
            )
            .unwrap()
            .build("env")
            .unwrap();

        // create a vm
        let mut vm = Vm::new(Some(config))
            .unwrap()
            .register_import_module(module)
            .unwrap()
            .register_module_from_file("custom_framework", wasm_file)
            .unwrap();

        let mut wasi_module = vm.wasi_module().unwrap();
        wasi_module.initialize(None, Some(env_vars.iter().map(|s| &**s).collect()), None);

        Self::add_route(0, |value| serde_json::to_string(&value).unwrap());
        Self::add_route(1, |_value| "Test".to_string());

        Server { vm }
    }

    pub fn start(&self) {
        // print registerd modules
        println!("Registered modules: {:?}", self.vm.instance_names());
        let _ = &self
            .vm
            .run_func(
                Some("custom_framework"),
                "start",
                Some(WasmValue::from_i32(get_string_pointer(
                    "0:/,1:/test",
                    &self.vm,
                ))),
            )
            .unwrap();
    }

    fn add_route(route: i32, handler: RequestHandler) {
        unsafe {
            ROUTES.insert(route, handler);
        }
    }
}
