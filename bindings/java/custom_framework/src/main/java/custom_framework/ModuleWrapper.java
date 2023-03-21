package custom_framework;

import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;

import org.wasmedge.AstModuleContext;
import org.wasmedge.ConfigureContext;
import org.wasmedge.ExecutorContext;
import org.wasmedge.FunctionInstanceContext;
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

public class ModuleWrapper {
    private ExecutorContext executor;
    private ModuleInstanceContext frameworkModule;
    private EnvModuleWrapper envModuleWrapper;

    public ModuleWrapper(Map<Integer, RouteHandler> routes) {
        try (InputStream in = getClass().getResourceAsStream(Constants.CustomFrameworkPath.getValue())) {
            byte[] bytesArray = in.readAllBytes();
            WasmEdge.init();
            ConfigureContext config = new ConfigureContext();
            config.addHostRegistration(
                    HostRegistration.WasmEdge_HostRegistration_Wasi);
            StoreContext store = new StoreContext();
            new WasmEdgeVm(config, store);

            this.envModuleWrapper = new EnvModuleWrapper(new HandleRequestExternal(this, routes));
            ModuleInstanceContext envModule = this.envModuleWrapper.getEnvModule();

            LoaderContext loader = new LoaderContext(config);
            AstModuleContext astFrameworkModule = loader.parseFromBuffer(bytesArray, bytesArray.length);
            ValidatorContext validator = new ValidatorContext(config);
            validator.validate(astFrameworkModule);
            this.executor = new ExecutorContext(config, null);
            executor.registerImport(store, envModule);
            this.frameworkModule = executor.register(store, astFrameworkModule,
                    Constants.CustomFrameworkModuleName.getValue());
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public void start(String controllers) {
        List<Value> params = List.of(new I32Value(getStringPointer(controllers)));
        FunctionInstanceContext start = frameworkModule.findFunction(Constants.FunctionStart.getValue());
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
        FunctionInstanceContext allocate = frameworkModule.findFunction(Constants.FunctionAllocate.getValue());
        executor.invoke(allocate, params, returns);

        MemoryInstanceContext mem = frameworkModule.findMemory(Constants.MemoryName.getValue());

        byte[] rawData = data.getBytes(StandardCharsets.UTF_8);
        mem.setData(rawData, ((I32Value) returns.get(0)).getValue(), rawData.length);
        return ((I32Value) returns.get(0)).getValue();
    }

    public MemoryInstanceContext getMemory() {
        return frameworkModule.findMemory(Constants.MemoryName.getValue());
    }

}
