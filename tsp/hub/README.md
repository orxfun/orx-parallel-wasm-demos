# TSP demo hub

The hub is a static framework switcher. Its buttons navigate to the separately deployed vanilla, React, Yew, and Leptos demos; it does not build or bundle those apps.

The live hub is at [orx-parallel-wasm-demo-tsp.pages.dev](https://orx-parallel-wasm-demo-tsp.pages.dev/). The destinations are configured in `index.html`. Each README button links to that framework's source guide:

- [Vanilla](../vanilla/README.md)
- [React](../react/README.md)
- [Yew](../yew/README.md)
- [Leptos](../leptos/README.md)

## Run locally

From this directory, serve the static files over HTTP:

```bash
python3 -m http.server 8080
```

Open `http://localhost:8080`. Do not open `index.html` through `file://`.

The hub links to external demo origins, so its local server does not need the wasm thread headers. Each deployed demo must be served with `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp`. `hub/_headers` applies only to the hub's own Cloudflare Pages deployment; the demo sites need their own headers.
