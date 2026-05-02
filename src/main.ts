import { createApp } from "vue";
import { definePreset } from "@primeuix/themes";
import App from "./App.vue";
import PrimeVue from "primevue/config";
import Aura from "@primeuix/themes/aura";

const fileraTheme = definePreset(Aura, {
    primitive: {
        chartreuse: {
            50: "#f2fee8",
            100: "#e1fcc8",
            200: "#c3f99a",
            300: "#96f260",
            400: "#67dd2e",
            500: "#40b51b",
            600: "#319114",
            700: "#267312",
            800: "#215b13",
            900: "#1e4d13",
            950: "#0b2a05",
        },
        olive: {
            50: "#fbfbf9",
            100: "#f4f4f0",
            200: "#e8e8e3",
            300: "#d8d8d0",
            400: "#abab9c",
            500: "#7c7c67",
            600: "#5b5b4b",
            700: "#474739",
            800: "#2b2b22",
            900: "#1d1d16",
            950: "#0c0c09",
        },
    },

    semantic: {
        primary: {
            50: "{chartreuse.50}",
            100: "{chartreuse.100}",
            200: "{chartreuse.200}",
            300: "{chartreuse.300}",
            400: "{chartreuse.400}",
            500: "{chartreuse.500}",
            600: "{chartreuse.600}",
            700: "{chartreuse.700}",
            800: "{chartreuse.800}",
            900: "{chartreuse.900}",
            950: "{chartreuse.950}",
        },
        colorScheme: {
            light: {
                surface: {
                    0: "#ffffff",
                    50: "{olive.50}",
                    100: "{olive.100}",
                    200: "{olive.200}",
                    300: "{olive.300}",
                    400: "{olive.400}",
                    500: "{olive.500}",
                    600: "{olive.600}",
                    700: "{olive.700}",
                    800: "{olive.800}",
                    900: "{olive.900}",
                    950: "{olive.950}",
                },
                navigation: {
                    item: {
                        focus: {
                            background: "{primary.100}",
                            color: "{primary.700}",
                        },
                        active: {
                            background: "{primary.100}",
                            color: "{primary.700}",
                        },
                        icon: {
                            focus: { color: "{primary.700}" },
                            active: { color: "{primary.700}" },
                        },
                    },
                },
                reactive: {
                    "button-background": "{olive.200}",
                    "button-borderColor": "{olive.400}",
                    "button-hoverBorderColor": "{olive.500}",
                    "text-color": "{olive.900}",
                    "top-panel": "#FF0000",
                },
                panelheader: "{olive.200}",
                panelbody: "{olive.100}",
                panelfooter: "{olive.200}",
                textprimary: "{olive.900}",
                textsecondary: "{olive.600}",
                bordercolor: "{olive.300}",
                taskbg: "{olive.100}",
            },
            dark: {
                surface: {
                    0: "#ffffff",
                    50: "{olive.50}",
                    100: "{olive.100}",
                    200: "{olive.200}",
                    300: "{olive.300}",
                    400: "{olive.400}",
                    500: "{olive.500}",
                    600: "{olive.600}",
                    700: "{olive.700}",
                    800: "{olive.800}",
                    900: "{olive.900}",
                    950: "{olive.950}",
                },
                navigation: {
                    item: {
                        focus: {
                            background: "{primary.900}",
                            color: "{primary.300}",
                        },
                        active: {
                            background: "{primary.900}",
                            color: "{primary.300}",
                        },
                        icon: {
                            focus: { color: "{primary.300}" },
                            active: { color: "{primary.300}" },
                        },
                    },
                },
                reactive: {
                    "button-background": "{olive.800}",
                    "button-borderColor": "{olive.600}",
                    "button-hoverBorderColor": "{olive.950}",
                    "text-color": "{olive.200}",
                    "top-panel": "#FF0000",
                },
                panelheader: "{olive.900}",
                panelbody: "{olive.800}",
                panelfooter: "{olive.900}",
                textprimary: "{olive.200}",
                textsecondary: "{olive.400}",
                bordercolor: "{olive.600}",
                taskbg: "{olive.900}",
            },
        },
    },

    components: {
        button: {
            colorScheme: {
                light: {
                    root: {
                        secondary: {
                            background: "{olive.400}",
                            borderColor: "{olive.400}",
                            color: "{olive.950}",
                            hoverBackground: "{olive.500}",
                            hoverBorderColor: "{olive.500}",
                            hoverColor: "{olive.950}",
                            activeBackground: "{olive.600}",
                            activeBorderColor: "{olive.600}",
                            activeColor: "{olive.50}",
                        },
                    },
                },
                dark: {
                    root: {
                        secondary: {
                            background: "{olive.700}",
                            borderColor: "{olive.700}",
                            color: "{olive.100}",
                            hoverBackground: "{olive.600}",
                            hoverBorderColor: "{olive.600}",
                            hoverColor: "{olive.50}",
                            activeBackground: "{olive.500}",
                            activeBorderColor: "{olive.500}",
                            activeColor: "{olive.50}",
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
                light: {
                    root: {
                        background: "{olive.400}",
                        borderColor: "{olive.400}",
                        color: "{olive.50}",
                        hoverBorderColor: "{olive.500}",
                    },
                },
                dark: {
                    root: {
                        background: "{olive.700}",
                        borderColor: "{olive.700}",
                        color: "{olive.50}",
                        hoverBorderColor: "{olive.600}",
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
        inputnumber: {
            colorScheme: {
                dark: {
                    button: {
                        background: "{reactive.button-background}",
                    },
                },
                light: {
                    button: {
                        background: "{primary.100}",
                        hoverBackground: "{primary.200}",
                        activeBackground: "{primary.300}",
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
