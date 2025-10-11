# `tzf-wasm` [![NPM Version](https://img.shields.io/npm/v/tzf-wasm)](https://www.npmjs.com/package/tzf-wasm)

A fast library to find the timezone of a given location using WebAssembly for
JavaScript, powered by [`tzf-rs`](https://github.com/ringsaturn/tzf-rs).

## Preview

The latest build from the `main` branch is published automatically to GitHub Pages:
[https://ringsaturn.github.io/tzf-wasm/](https://ringsaturn.github.io/tzf-wasm/)

## Use in HTML

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
                const timezone = finder.get_tz_name(lng, lat);
                console.log(`Timezone for (${lat}, ${lng}): ${timezone}`);
            }

            loadWasm();
        </script>
    </head>
    <body></body>
</html>
```

For a simple web page, see <https://github.com/ringsaturn/tzf-web>.

## LICENSE

This project is licensed under the [MIT license](./LICENSE). The data is
licensed under the
[ODbL license](https://github.com/ringsaturn/tzf-rel/blob/main/LICENSE), same as
[`evansiroky/timezone-boundary-builder`](https://github.com/evansiroky/timezone-boundary-builder)
