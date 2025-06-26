import { createApp } from "vue";
import App from "./App.vue";
import PrimeVue from 'primevue/config';
import Aura from '@primeuix/themes/aura'
import Lara from '@primeuix/themes/lara'
import Material from '@primeuix/themes/material'


const app = createApp(App)

app.use(PrimeVue, {
    theme: {
        preset: Aura,
        options: {
            darkModeSelector: true,
        }
    }
});

app.mount("#app");
