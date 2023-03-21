package custom_framework;

public enum Constants {
    EnvModuleName("env"),
    HandleRequestExternalFunctionName("handle_request_external"),
    CustomFrameworkModuleName("custom_framework"),
    CustomFrameworkPath("/custom_framework_wasm.wasm"),
    FunctionAllocate("allocate"),
    FunctionStart("start"),
    MemoryName("memory");

    private String value;

    Constants(String value) {
        this.value = value;
    }

    public String getValue() {
        return value;
    }
}
