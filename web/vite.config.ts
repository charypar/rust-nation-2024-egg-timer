import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import commonjs from "vite-plugin-commonjs";

// https://vitejs.dev/config/
export default defineConfig({
    assetsInclude: ["**/*.wasm"], // tell Vite WASM is an asset
    optimizeDeps: {
        exclude: ["shared"], // prevent Vite from prebundling it
    },
    plugins: [react(), commonjs()],
    server: {
        fs: {
            allow: [".."],
        },
    },
});
