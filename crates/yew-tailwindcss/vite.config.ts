import { defineConfig } from "vite"
import ViteRsw from "vite-plugin-rsw"

export default defineConfig({
    plugins: [
        ViteRsw({
            mode: "release",
            crates: ["yew-test"]
        })
    ],
});