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
                reactive: {
                    "button-background": "{primary.900}",
                    "button-borderColor": "{primary.400}",
                    "button-hoverBorderColor": "{primary.950}",
                    "text-color": "{primary.400}",
                    "top-panel": "#FF0000",
                },
                panelheader: "{primary.300}",
                panelbody: "{primary.200}",
                panelfooter: "{primary.300}",
                textprimary: "{primary.700}",
                textsecondary: "{primary.500}",
                bordercolor: "{primary.400}",
                taskbg: "{primary.300}",
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
                reactive: {
                    "button-background": "{primary.800}",
                    "button-borderColor": "{primary.600}",
                    "button-hoverBorderColor": "{primary.950}",
                    "text-color": "{primary.400}",
                    "top-panel": "#FF0000",
                },
                panelheader: "{primary.900}",
                panelbody: "{primary.800}",
                panelfooter: "{primary.900}",
                textprimary: "{primary.400}",
                textsecondary: "{primary.600}",
                bordercolor: "{primary.600}",
                taskbg: "{primary.900}",
            },
        },
    },

    components: {
        button: {
            colorScheme: {
                dark: {
                    root: {
                        secondary: {
                            background: "{reactive.button-background}",
                            borderColor: "{reactive.button-borderColor}",
                            color: "{reactive.text-color}",
                            hoverBorderColor: "{reactive.button-borderColor}",
                        },
                    },
                },
            },
        },
        inputtext: {
            colorScheme: {
                dark: {
                    root: {
                        background: "{reactive.button-background}",
                        borderColor: "{reactive.button-borderColor}",
                        color: "{reactive.text-color}",
                        hoverBorderColor: "{reactive.button-borderColor}",
                    },
                },
            },
        },
        select: {
            colorScheme: {
                dark: {
                    root: {
                        background: "{reactive.button-background}",
                        borderColor: "{reactive.button-borderColor}",
                        color: "{reactive.text-color}",
                        hoverBorderColor: "{reactive.button-borderColor}",
                    },
                },
            },
        },
        togglebutton: {
            colorScheme: {
                dark: {
                    root: {
                        background: "{reactive.button-background}",
                        borderColor: "{reactive.button-borderColor}",
                        color: "{reactive.text-color}",
                        checkedBackground: "{reactive.button-background}",
                        checkedBorderColor: "{reactive.button-borderColor}",
                        checkedColor: "{reactive.text-color}",
                    },
                },
            },
        },
        floatlabel: {
            colorScheme: {
                dark: {
                    on: {
                        active: {
                            background: "{panelbody}",
                        },
                    },
                },
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
            cssLayer: {
                name: "primevue",
                order: "theme, base, PrimeVue",
            },
        },
    },
});

app.mount("#app");
