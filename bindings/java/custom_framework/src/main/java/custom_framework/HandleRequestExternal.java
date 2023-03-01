package custom_framework;

import java.util.List;

import org.wasmedge.HostFunction;
import org.wasmedge.I32Value;
import org.wasmedge.MemoryInstanceContext;
import org.wasmedge.Result;
import org.wasmedge.Value;

class HandleRequestExternal implements HostFunction {
    private final Server server;

    public HandleRequestExternal(Server server) {
        this.server = server;
    }

    @Override
    public Result apply(MemoryInstanceContext arg0, List<Value> arg1, List<Value> arg2) {
        int requestHandlerIndex = ((I32Value) arg1.get(0)).getValue();
        int requestPointer = ((I32Value) arg1.get(1)).getValue();
        int requestLength = ((I32Value) arg1.get(2)).getValue();

        MemoryInstanceContext mem = server.frameworkModule.findMemory("memory");
        String request = this.server.getStringFromPointer(requestPointer, requestLength, mem);
        String response = this.server.ROUTES.get(requestHandlerIndex).apply(request);
        int responsePointer = this.server.getStringPointer(response);
        arg2.add(new I32Value(responsePointer));

        return new Result();
    }
}