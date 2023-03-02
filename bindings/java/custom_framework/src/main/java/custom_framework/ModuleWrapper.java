package custom_framework;

import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.function.Function;

import org.wasmedge.AstModuleContext;
import org.wasmedge.ConfigureContext;
import org.wasmedge.ExecutorContext;
import org.wasmedge.FunctionInstanceContext;
import org.wasmedge.FunctionTypeContext;
import org.wasmedge.I32Value;
import org.wasmedge.LoaderContext;
import org.wasmedge.MemoryInstanceContext;
import org.wasmedge.ModuleInstanceContext;
import org.wasmedge.StoreContext;
import org.wasmedge.ValidatorContext;
import org.wasmedge.Value;
import org.wasmedge.WasmEdge;
import org.wasmedge.WasmEdgeVm;
import org.wasmedge.enums.HostRegistration;
import org.wasmedge.enums.ValueType;

public class ModuleWrapper {
    private static final String ENV_MODULE_NAME = "env";
    private static final String FRAMEWORK_MODULE_NAME = "custom_framework";
    private static final String WASM_BINARY_PATH = "/custom_framework_wasm.wasm";

    private ExecutorContext executor;
    private ModuleInstanceContext frameworkModule;
    private Map<Integer, Function<String, String>> ROUTES = new HashMap<>();

    public ModuleWrapper() {

        try (InputStream in = getClass().getResourceAsStream(WASM_BINARY_PATH)) {

            byte[] bytesArray = in.readAllBytes();
            WasmEdge.init();
            ConfigureContext config = new ConfigureContext();
            config.addHostRegistration(
                    HostRegistration.WasmEdge_HostRegistration_Wasi);
            StoreContext store = new StoreContext();
            new WasmEdgeVm(config, store);

            ModuleInstanceContext envModule = this.getEnvModule();

            LoaderContext loader = new LoaderContext(config);
            AstModuleContext astFrameworkModule = loader.parseFromBuffer(bytesArray, bytesArray.length);
            ValidatorContext validator = new ValidatorContext(config);
            validator.validate(astFrameworkModule);
            this.executor = new ExecutorContext(config, null);
            executor.registerImport(store, envModule);
            this.frameworkModule = executor.register(store, astFrameworkModule, FRAMEWORK_MODULE_NAME);

            ROUTES.put(0, (String body) -> {
                return "Hello World!";
            });
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public void start() {
        List<Value> params = List.of(new I32Value(getStringPointer("0:/")));
        FunctionInstanceContext start = frameworkModule.findFunction("start");
        executor.invoke(start, params, List.of());
    }

    public String getStringFromPointer(int pointer, int length, MemoryInstanceContext memory) {
        byte[] data = memory.getData(pointer, length);
        return new String(data, StandardCharsets.UTF_8);
    }

    public int getStringPointer(String data) {
        int dataLength = data.length();
        List<Value> params = List.of(new I32Value(dataLength + 1));
        List<Value> returns = new ArrayList<>();
        FunctionInstanceContext allocate = frameworkModule.findFunction("allocate");
        executor.invoke(allocate, params, returns);

        MemoryInstanceContext mem = frameworkModule.findMemory("memory");

        byte[] rawData = data.getBytes(StandardCharsets.UTF_8);
        mem.setData(rawData, ((I32Value) returns.get(0)).getValue(), rawData.length);
        return ((I32Value) returns.get(0)).getValue();
    }

    public MemoryInstanceContext getMemory() {
        return frameworkModule.findMemory("memory");
    }

    public Map<Integer, Function<String, String>> getROUTES() {
        return ROUTES;
    }

    private ModuleInstanceContext getEnvModule() {
        ModuleInstanceContext envModule = new ModuleInstanceContext(ENV_MODULE_NAME);

        FunctionTypeContext functionTypes = new FunctionTypeContext(
                List.of(ValueType.i32, ValueType.i32, ValueType.i32),
                List.of(ValueType.i32));
        HandleRequestExternal handlerFunction = new HandleRequestExternal(this);
        FunctionInstanceContext hostHandleRequestExternal = new FunctionInstanceContext(
                functionTypes,
                handlerFunction, null, 0);
        envModule.addFunction("handle_request_external",
                hostHandleRequestExternal);
        return envModule;
    }
}
