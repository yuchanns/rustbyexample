import { defineConfig } from "vite"
import ViteRsw from "vite-plugin-rsw"

export default defineConfig({
    plugins: [
        ViteRsw({
            mode: process.env.NODE_ENV == "production" ? "release" : "dev",
            crates: ["yew-test"]
        })
    ],
});