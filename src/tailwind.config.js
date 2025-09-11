/** @type {import('tailwindcss').Config} */
export default {
    content: ["./index.html", "./src/**/*.{vue,js,ts,jsx,tsx}"],
    theme: {
        extend: {
            colors: {
                test: "rgb(var(--p-test) / <alpha-value>)",
                primary: "rgb(var(--p-primary) / <alpha-value>)",
                surface: {
                    0: "rgb(var(--p-surface-0) / <alpha-value>)",
                    50: "rgb(var(--p-surface-50) / <alpha-value>)",
                    100: "rgb(var(--p-surface-100) / <alpha-value>)",
                    200: "rgb(var(--p-surface-200) / <alpha-value>)",
                    300: "rgb(var(--p-surface-300) / <alpha-value>)",
                    400: "rgb(var(--p-surface-400) / <alpha-value>)",
                    500: "rgb(var(--p-surface-500) / <alpha-value>)",
                    600: "rgb(var(--p-surface-600) / <alpha-value>)",
                    700: "rgb(var(--p-surface-700) / <alpha-value>)",
                    800: "rgb(var(--p-surface-800) / <alpha-value>)",
                    900: "rgb(var(--p-surface-900) / <alpha-value>)",
                    950: "rgb(var(--p-surface-950) / <alpha-value>)",
                },
            },
        },
    },
    darkMode: "class",
    plugins: [],
};
