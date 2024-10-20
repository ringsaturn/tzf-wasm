# `tzf-wasm`

```html
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>tzf-web Example</title>
        <script type="module">
            import init, {
                WasmFinder,
            } from "https://www.unpkg.com/tzf-wasm@v0.1.4/tzf_wasm.js";

            let finder;

            async function loadWasm() {
                await init();
                finder = new WasmFinder();
                const lng = -74.0060;
                const lat = 40.7128;
                const timezone = finder.get_tz(lng, lat);
                console.log(`Timezone for (${lat}, ${lng}): ${timezone}`);
            }

            loadWasm();
        </script>
    </head>
    <body></body>
</html>
```

For a simple web page, see <https://github.com/ringsaturn/tzf-web>.
