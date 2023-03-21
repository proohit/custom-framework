pub enum Constants {
    CustomFrameworkModulePath,
    CustomFrameworkModuleName,
    MemoryName,
    FunctionStart,
    FunctionAllocate,
    EnvModuleName,
    HandleRequestExternalFunctionName,
}

impl Constants {
    pub fn as_str(&self) -> &str {
        match self {
            Constants::CustomFrameworkModulePath => "lib/custom_framework_wasm.wasm",
            Constants::CustomFrameworkModuleName => "custom_framework",
            Constants::MemoryName => "memory",
            Constants::FunctionStart => "start",
            Constants::FunctionAllocate => "allocate",
            Constants::EnvModuleName => "env",
            Constants::HandleRequestExternalFunctionName => "handle_request_external",
        }
    }
}
