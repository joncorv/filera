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
        emoji: "🌿",
        command: () => usePreset(fileraTheme),
    },
    {
        label: "Rose Pine",
        emoji: "🌹",
        command: () => usePreset(rosePineTheme),
    },
    {
        label: "Catppuccin",
        emoji: "🐱",
        command: () => usePreset(catppuccinTheme),
    },
    {
        label: "Tokyo Night",
        emoji: "🌃",
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
    <Menu ref="themeMenuToggle" id="theme_menu" :model="themeMenuItems" :popup="true">
        <template #item="{ item, props }">
            <a class="p-menu-item-link" v-bind="props.action">
                <span class="p-menu-item-icon">{{ item.emoji }}</span>
                <span class="p-menu-item-label">{{ item.label }}</span>
            </a>
        </template>
    </Menu>
</template>
