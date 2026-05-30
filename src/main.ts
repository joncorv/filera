import { createApp } from "vue";
import { definePreset } from "@primeuix/themes";
import App from "./App.vue";
import PrimeVue from "primevue/config";
import Aura from "@primeuix/themes/aura";
const userAgentData = (
    navigator as Navigator & { userAgentData?: { platform?: string } }
).userAgentData;
const platform = userAgentData?.platform ?? navigator.platform ?? navigator.userAgent;

if (/linux/i.test(platform) && !/android/i.test(navigator.userAgent)) {
    document.documentElement.classList.add("platform-linux");
}

const surface = {
    0:   "#ffffff",
    50:  "#faf8ff",
    100: "#f0effe",
    200: "#e0def4",
    300: "#908caa",
    400: "#6e6a86",
    500: "#56526e",
    600: "#44415a",
    700: "#393552",
    800: "#2a273f",
    900: "#232136",
    950: "#1a1826",
};

const fileraTheme = definePreset(Aura, {
    primitive: {
        deep: "#1f1d2e",
        iris: "#c4a7e7",
        love: "#eb6f92",
        rose: "#ea9a97",
        accent: {
            50:  "#fff0ef",
            100: "#ffe4e2",
            200: "#ffccc9",
            300: "#f5b0ad",
            400: "#ea9a97",
            500: "#d97f7c",
            600: "#bf6361",
            700: "#a04949",
            800: "#833435",
            900: "#672224",
            950: "#471516",
        },
    },

    semantic: {
        primary: {
            50: "{accent.50}",
            100: "{accent.100}",
            200: "{accent.200}",
            300: "{accent.300}",
            400: "{accent.400}",
            500: "{accent.500}",
            600: "{accent.600}",
            700: "{accent.700}",
            800: "{accent.800}",
            900: "{accent.900}",
            950: "{accent.950}",
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
                highlighttext: "{accent.700}",
                accenthover: "{accent.100}",
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
                panelbody: "{surface.900}",
                panelfooter: "{surface.900}",
                textprimary: "{surface.200}",
                textsecondary: "{surface.300}",
                bordercolor: "{surface.500}",
                taskbg: "{surface.700}",
                highlighttext: "{accent.400}",
                accenthover: "color-mix(in srgb, {accent.400} 15%, transparent)",
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
                            borderColor: "color-mix(in srgb, {surface.400} 50%, transparent)",
                            color: "{surface.200}",
                            hoverBackground: "{surface.600}",
                            hoverBorderColor: "color-mix(in srgb, {surface.300} 50%, transparent)",
                            hoverColor: "{surface.50}",
                            activeBackground: "{surface.500}",
                            activeBorderColor: "{surface.500}",
                            activeColor: "{surface.50}",
                        },
                        primary: {
                            borderColor: "color-mix(in srgb, {accent.300} 85%, white)",
                            hoverBorderColor: "color-mix(in srgb, {accent.200} 85%, white)",
                        },
                    },
                },
            },
        },
        inputtext: {
            colorScheme: {
                dark: {
                    root: {
                        background: "{surface.800}",
                        borderColor: "{surface.600}",
                        color: "{surface.200}",
                        hoverBorderColor: "{surface.600}",
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
                        borderColor: "color-mix(in srgb, {surface.400} 50%, transparent)",
                        color: "{surface.50}",
                        hoverBorderColor: "color-mix(in srgb, {surface.300} 50%, transparent)",
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
                        background: "{surface.800}",
                        borderColor: "{surface.600}",
                        color: "{surface.200}",
                        checkedBackground: "{surface.800}",
                        checkedBorderColor: "{surface.600}",
                        checkedColor: "{surface.200}",
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
                        background: "{surface.800}",
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
