import { createApp } from "vue";
import App from "./App.vue";
import PrimeVue from "primevue/config";
import { fileraTheme } from "./themes";

const userAgentData = (
    navigator as Navigator & { userAgentData?: { platform?: string } }
).userAgentData;
const platform = userAgentData?.platform ?? navigator.platform ?? navigator.userAgent;

if (/linux/i.test(platform) && !/android/i.test(navigator.userAgent)) {
    document.documentElement.classList.add("platform-linux");
}

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
