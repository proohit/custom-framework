package custom_framework;

import java.util.List;

import org.wasmedge.FunctionInstanceContext;
import org.wasmedge.FunctionTypeContext;
import org.wasmedge.ModuleInstanceContext;
import org.wasmedge.enums.ValueType;

public class EnvModuleWrapper {
    private static final String ENV_MODULE_NAME = "env";
    private ModuleInstanceContext envModule;

    public EnvModuleWrapper(HandleRequestExternal handleRequestExternal) {
        this.envModule = new ModuleInstanceContext(ENV_MODULE_NAME);

        FunctionTypeContext functionTypes = new FunctionTypeContext(
                List.of(ValueType.i32, ValueType.i32, ValueType.i32),
                List.of(ValueType.i32));
        FunctionInstanceContext hostHandleRequestExternal = new FunctionInstanceContext(
                functionTypes,
                handleRequestExternal, null, 0);
        envModule.addFunction("handle_request_external",
                hostHandleRequestExternal);
    }

    public ModuleInstanceContext getEnvModule() {
        return this.envModule;
    }
}
