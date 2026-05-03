import { createApp } from "vue";
import { definePreset } from "@primeuix/themes";
import App from "./App.vue";
import PrimeVue from "primevue/config";
import Aura from "@primeuix/themes/aura";
import colors from "tailwindcss/colors";

const surface = { 0: "#ffffff", ...colors.stone };

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
                surface,
                navigation: {
                    item: {
                        focus: {
                            background: "{accenthover}",
                            color: "{highlighttext}",
                        },
                        active: {
                            background: "{accenthover}",
                            color: "{highlighttext}",
                        },
                        icon: {
                            focus: { color: "{highlighttext}" },
                            active: { color: "{highlighttext}" },
                        },
                    },
                },
                reactive: {
                    "button-background": "{surface.200}",
                    "button-borderColor": "{surface.400}",
                    "button-hoverBorderColor": "{surface.500}",
                    "text-color": "{surface.900}",
                    "top-panel": "#FF0000",
                },
                panelheader: "{surface.200}",
                panelbody: "{surface.100}",
                panelfooter: "{surface.200}",
                textprimary: "{surface.900}",
                textsecondary: "{surface.600}",
                bordercolor: "{surface.300}",
                taskbg: "{surface.100}",
                highlighttext: "{chartreuse.800}",
                accenthover: "{chartreuse.100}",
            },
            dark: {
                surface,
                navigation: {
                    item: {
                        focus: {
                            background: "{accenthover}",
                            color: "{highlighttext}",
                        },
                        active: {
                            background: "{accenthover}",
                            color: "{highlighttext}",
                        },
                        icon: {
                            focus: { color: "{highlighttext}" },
                            active: { color: "{highlighttext}" },
                        },
                    },
                },
                reactive: {
                    "button-background": "{surface.800}",
                    "button-borderColor": "{surface.600}",
                    "button-hoverBorderColor": "{surface.950}",
                    "text-color": "{surface.200}",
                    "top-panel": "#FF0000",
                },
                panelheader: "{surface.900}",
                panelbody: "{surface.800}",
                panelfooter: "{surface.900}",
                textprimary: "{surface.200}",
                textsecondary: "{surface.400}",
                bordercolor: "{surface.700}",
                taskbg: "{surface.900}",
                highlighttext: "color-mix(in srgb, {chartreuse.500} 50%, white)",
                accenthover: "color-mix(in srgb, {chartreuse.900} 50%, transparent)",
            },
        },
    },

    components: {
        button: {
            colorScheme: {
                light: {
                    root: {
                        secondary: {
                            background: "{surface.0}",
                            borderColor: "{surface.400}",
                            color: "{surface.700}",
                            hoverBackground: "{accenthover}",
                            hoverBorderColor: "{surface.500}",
                            hoverColor: "{surface.700}",
                            activeBackground: "{surface.100}",
                            activeBorderColor: "{surface.500}",
                            activeColor: "{surface.700}",
                        },
                    },
                },
                dark: {
                    root: {
                        secondary: {
                            background: "{surface.700}",
                            borderColor: "{surface.700}",
                            color: "{surface.200}",
                            hoverBackground: "{surface.600}",
                            hoverBorderColor: "{surface.600}",
                            hoverColor: "{surface.50}",
                            activeBackground: "{surface.500}",
                            activeBorderColor: "{surface.500}",
                            activeColor: "{surface.50}",
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
                        background: "{surface.0}",
                        borderColor: "{surface.400}",
                        color: "{surface.700}",
                        hoverBorderColor: "{surface.500}",
                    },
                    option: {
                        focusBackground: "{accenthover}",
                        focusColor: "{highlighttext}",
                    },
                },
                dark: {
                    root: {
                        background: "{surface.700}",
                        borderColor: "{surface.700}",
                        color: "{surface.50}",
                        hoverBorderColor: "{surface.600}",
                    },
                    option: {
                        focusBackground: "{accenthover}",
                        focusColor: "{highlighttext}",
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
