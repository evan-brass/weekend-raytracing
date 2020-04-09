# Building:

First, compile the renderer code by running
```
wasm-pack build --target web
```
from the renderer folder.  This will require having wasm-pack installed (<https://rustwasm.github.io/wasm-pack/>);

All the source is static files so any dev server should work.  I have a simple caddy file so if you have caddy 2.0 installed than from the root folder run:
```
caddy start
```
Then visit <https://localhost/> in your web browser.

I also intend to have a hosted version on github pages.
TODO: insert github pages link

# Source:
I'm following the tutorial here: <https://raytracing.github.io/books/RayTracingInOneWeekend.html>