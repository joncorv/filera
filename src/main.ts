import { createApp } from "vue";
import { definePreset } from "@primeuix/themes";
import App from "./App.vue";
import PrimeVue from "primevue/config";
import Aura from "@primeuix/themes/aura";

const fileraTheme = definePreset(Aura, {
    semantic: {
        primary: {
            50: "{stone.50}",
            100: "{stone.100}",
            200: "{stone.200}",
            300: "{stone.300}",
            400: "{stone.400}",
            500: "{stone.500}", // This becomes the new primary color
            600: "{stone.600}",
            700: "{stone.700}",
            800: "{stone.800}",
            900: "{stone.900}",
            950: "{stone.950}",
        },
        colorScheme: {
            light: {
                surface: {
                    0: "#ffffff",
                    50: "{stone.50}",
                    100: "{stone.100}",
                    200: "{stone.200}",
                    300: "{stone.300}",
                    400: "{stone.400}",
                    500: "{stone.500}",
                    600: "{stone.600}",
                    700: "{stone.700}",
                    800: "{stone.800}",
                    900: "{stone.900}",
                    950: "{stone.950}",
                },
                app: {
                    button_bg: "{stone.50}",
                },
            },
            dark: {
                surface: {
                    0: "#ffffff",
                    50: "{stone.50}",
                    100: "{stone.100}",
                    200: "{stone.200}",
                    300: "{stone.300}",
                    400: "{stone.400}",
                    500: "{stone.500}",
                    600: "{stone.600}",
                    700: "{stone.700}",
                    800: "{stone.800}",
                    900: "{stone.900}",
                    950: "{stone.950}",
                },
                app: {
                    button_bg: "{stone.900}",
                },
            },
        },
    },

    components: {
        inputtext: {
            root: {
                background: "{app:button_bg}",
            },
        },
    },
});

const app = createApp(App);

app.use(PrimeVue, {
    theme: {
        preset: fileraTheme,
        options: {
            darkModeSelector: true,
        },
    },
});

app.mount("#app");
