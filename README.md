# `tzf-wasm`

```html
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>tzf-wasm demo</title>
</head>
<script src="https://www.unpkg.com/tzf-wasm@0.1.1/tzf_wasm.js" type="module"></script>

<body>
    <script type="module">
        import * as tzf_wasm from 'https://www.unpkg.com/tzf-wasm@0.1.1/tzf_wasm.js';
        tzf_wasm.default().then(() => {
            console.log(tzf_wasm.getTz(116.3883, 39.9289));
        });
    </script>
</body>

</html>
```
