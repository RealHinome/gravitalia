import {
    defineConfig,
    presetAttributify,
    presetTypography,
    presetUno,
    presetMini
} from "unocss";

export default defineConfig({
    presets: [
        presetAttributify(),
        presetUno(),
        presetTypography(),
        presetMini({
            dark: "class"
        })
    ]
});