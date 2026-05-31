import { definePreset } from "@primeuix/themes";
import Aura from "@primeuix/themes/aura";
import colors from "tailwindcss/colors";

// === Filera base theme: stone surface + green/chartreuse accent ===
// This is the base preset; other themes (rosePineTheme below) are deltas
// layered on top of it via definePreset(fileraTheme, {...}).
const surface = { 0: "#ffffff", ...colors.stone };

export const fileraTheme = definePreset(Aura, {
    primitive: {
        accent: {
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
                highlighttext: "{accent.800}",
                accenthover: "{accent.100}",
                // File-row selection (left panel). Defaults to the accent tokens
                // so themes follow their own accent unless they override these.
                rowselectbg: "{accenthover}",
                rowselecttext: "{highlighttext}",
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
                bordercolor: "{surface.700}",
                taskbg: "{surface.700}",
                highlighttext: "color-mix(in srgb, {accent.500} 50%, white)",
                accenthover: "color-mix(in srgb, {accent.900} 50%, transparent)",
                rowselectbg: "{accenthover}",
                rowselecttext: "{highlighttext}",
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
                            borderColor: "color-mix(in srgb, {accent.300} 50%, white)",
                            hoverBorderColor: "color-mix(in srgb, {accent.200} 50%, white)",
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

// === Rose Pine theme: delta layered on top of the Filera base ===
// Overrides only what diverges from the base (rose accent, rose-pine surface,
// and the few tokens that differ). panelbody / textsecondary / taskbg are
// inherited from fileraTheme. This theme still needs polish.
const rosePineSurface = {
    0: "#ffffff",
    50: "#faf8ff",
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

export const rosePineTheme = definePreset(fileraTheme, {
    primitive: {
        accent: {
            50: "#fff0ef",
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
        colorScheme: {
            light: {
                surface: rosePineSurface,
                highlighttext: "{accent.700}",
                accenthover: "{accent.100}",
            },
            dark: {
                surface: rosePineSurface,
                bordercolor: "{surface.500}",
                highlighttext: "{accent.400}",
                accenthover: "color-mix(in srgb, {accent.400} 15%, transparent)",
            },
        },
    },
});

// === Catppuccin theme: Latte (light) -> Mocha (dark) surface, Mauve accent ===
const catppuccinSurface = {
    0: "#ffffff",
    50: "#eff1f5", // Latte base
    100: "#e6e9ef", // Latte mantle
    200: "#dce0e8", // Latte crust
    300: "#acb0be", // Latte surface2
    400: "#8c8fa1", // Latte overlay1
    500: "#6c7086", // Mocha overlay0
    600: "#585b70", // Mocha surface2
    700: "#45475a", // Mocha surface1
    800: "#313244", // Mocha surface0
    900: "#1e1e2e", // Mocha base
    950: "#11111b", // Mocha crust
};

export const catppuccinTheme = definePreset(fileraTheme, {
    primitive: {
        accent: {
            50: "#f6edfe",
            100: "#ecdcfd",
            200: "#dcc1fa",
            300: "#cba6f7", // Mocha mauve
            400: "#b27ff2",
            500: "#9a5cec",
            600: "#8839ef", // Latte mauve
            700: "#7320d6",
            800: "#5f1bb0",
            900: "#4d1890",
            950: "#2f0e5c",
        },
    },

    semantic: {
        colorScheme: {
            light: {
                surface: catppuccinSurface,
                highlighttext: "{accent.700}",
                accenthover: "{accent.100}",
            },
            dark: {
                surface: catppuccinSurface,
                bordercolor: "{surface.500}",
                highlighttext: "{accent.400}",
                accenthover: "color-mix(in srgb, {accent.400} 15%, transparent)",
            },
        },
    },
});

// === Tokyo Night theme: Day (light) -> Night (dark) surface, Blue accent ===
const tokyoNightSurface = {
    0: "#ffffff",
    50: "#e1e2e7", // Day bg
    100: "#d5d8e0",
    200: "#c4c8da",
    300: "#9aa0c0",
    400: "#737a9c",
    500: "#565f89", // Night comment
    600: "#414868", // Night terminal black
    700: "#292e42", // Night bg highlight
    800: "#24283b", // Night surface
    900: "#1a1b26", // Night bg
    950: "#16161e", // Night bg dark
};

export const tokyoNightTheme = definePreset(fileraTheme, {
    primitive: {
        accent: {
            50: "#eef3fe",
            100: "#dbe6fd",
            200: "#bccffb",
            300: "#9bb8f9",
            400: "#7aa2f7", // Tokyo Night blue
            500: "#5a82e0",
            600: "#4267c4",
            700: "#34509b",
            800: "#2b3f78",
            900: "#243460",
            950: "#16203d",
        },
        // Secondary Tokyo Night hues used for per-component accents below.
        purple: {
            300: "#cdb4fb",
            400: "#bb9af7", // Tokyo Night purple/magenta
            500: "#9d7cd8",
            600: "#7c5cbf",
            700: "#634a9c",
        },
        orange: {
            300: "#ffba8a",
            400: "#ff9e64", // Tokyo Night orange
            500: "#e8853f",
            600: "#c96a28",
            700: "#9e5220",
        },
        cyan: {
            300: "#a8deff",
            400: "#7dcfff", // Tokyo Night cyan
            500: "#5ab0e6",
            600: "#3d8fc4",
            700: "#2f6f99",
        },
    },

    semantic: {
        colorScheme: {
            light: {
                surface: tokyoNightSurface,
                highlighttext: "{accent.700}",
                accenthover: "{accent.100}",
                rowselectbg: "color-mix(in srgb, {cyan.400} 16%, transparent)",
                rowselecttext: "{cyan.700}",
            },
            dark: {
                surface: tokyoNightSurface,
                bordercolor: "{surface.500}",
                highlighttext: "{accent.400}",
                accenthover: "color-mix(in srgb, {accent.400} 15%, transparent)",
                rowselectbg: "color-mix(in srgb, {cyan.400} 16%, transparent)",
                rowselecttext: "{cyan.400}",
            },
        },
    },

    components: {
        // Batch Rename (the lone primary button) -> Tokyo Night purple.
        button: {
            colorScheme: {
                light: {
                    root: {
                        primary: {
                            background: "{purple.600}",
                            hoverBackground: "{purple.700}",
                            activeBackground: "{purple.700}",
                            borderColor: "{purple.600}",
                            hoverBorderColor: "{purple.700}",
                            activeBorderColor: "{purple.700}",
                            color: "#ffffff",
                            hoverColor: "#ffffff",
                            activeColor: "#ffffff",
                        },
                    },
                },
                dark: {
                    root: {
                        primary: {
                            background: "{purple.400}",
                            hoverBackground: "{purple.300}",
                            activeBackground: "{purple.500}",
                            borderColor: "{purple.400}",
                            hoverBorderColor: "{purple.300}",
                            activeBorderColor: "{purple.500}",
                            color: "{surface.950}",
                            hoverColor: "{surface.950}",
                            activeColor: "{surface.950}",
                        },
                    },
                },
            },
        },
        // Inclusive/Exclusive toggles -> orange checked state.
        togglebutton: {
            colorScheme: {
                light: {
                    content: { checkedBackground: "{orange.400}" },
                    root: { checkedColor: "{surface.900}", checkedBorderColor: "{orange.400}" },
                    icon: { checkedColor: "{surface.900}" },
                },
                dark: {
                    content: { checkedBackground: "{orange.400}" },
                    root: { checkedColor: "{surface.950}" },
                    icon: { checkedColor: "{surface.950}" },
                },
            },
        },
    },
});
