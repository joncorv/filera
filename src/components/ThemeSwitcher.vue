<script setup lang="ts">
import { ref } from "vue";
import Button from "primevue/button";
import Menu from "primevue/menu";
import { usePreset } from "@primeuix/themes";
import { fileraTheme, rosePineTheme, catppuccinTheme, tokyoNightTheme } from "../themes";

const themeMenuToggle = ref();
const toggleThemeMenu = (event: Event) => {
    themeMenuToggle.value.toggle(event);
};

const themeMenuItems = ref([
    {
        label: "Filera",
        icon: "pi pi-bolt",
        command: () => usePreset(fileraTheme),
    },
    {
        label: "Rose Pine",
        icon: "pi pi-heart",
        command: () => usePreset(rosePineTheme),
    },
    {
        label: "Catppuccin",
        icon: "pi pi-star-fill",
        command: () => usePreset(catppuccinTheme),
    },
    {
        label: "Tokyo Night",
        icon: "pi pi-moon",
        command: () => usePreset(tokyoNightTheme),
    },
]);
</script>

<template>
    <Button
        type="button"
        label="Theme"
        size="small"
        icon="pi pi-palette"
        class="min-w-max"
        severity="secondary"
        @click="toggleThemeMenu"
        aria-haspopup="true"
        aria-controls="theme_menu"
    />
    <Menu ref="themeMenuToggle" id="theme_menu" :model="themeMenuItems" :popup="true" />
</template>
