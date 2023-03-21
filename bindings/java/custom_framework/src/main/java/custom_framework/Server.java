package custom_framework;

import java.util.HashMap;
import java.util.Map;

public class Server {
    ModuleWrapper frameworkModule;
    private Map<Integer, RouteHandler> routeHandlers = new HashMap<>();

    public void start() {
        this.frameworkModule = new ModuleWrapper(routeHandlers);
        String stringifiedControllers = transformRoutesToString();
        this.frameworkModule.start(stringifiedControllers);
    }

    public void addRoute(RouteHandler routeHandler) {
        int routeIndex = this.routeHandlers.size();
        this.routeHandlers.put(routeIndex, routeHandler);
    }

    private String transformRoutesToString() {
        return this.routeHandlers.entrySet()
                .stream()
                .map(entry -> String.format("%d:%s", entry.getKey(), entry.getValue().getPath()))
                .reduce((a, b) -> String.format("%s,%s", a, b))
                .orElse("");
    }
}
