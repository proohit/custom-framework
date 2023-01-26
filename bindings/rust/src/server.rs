use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    Vm,
};

pub struct Server {
    vm: Vm,
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

        // create a vm
        let mut vm = Vm::new(Some(config))
            .unwrap()
            .register_module_from_file("custom_framework", wasm_file)
            .unwrap();

        let mut wasi_module = vm.wasi_module().unwrap();
        wasi_module.initialize(None, None, None);

        Server { vm }
    }

    pub fn start(&self) {
        let _ = &self
            .vm
            .run_func(Some("custom_framework"), "start", None)
            .unwrap();
    }
}
