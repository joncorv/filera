<script setup lang="ts">
import Select from "primevue/select"

defineProps<{
    task: {
        ChangeCase: {
            case_choice: number;
            active: boolean;
        };
    };
    index: number;
    isFirst: boolean;
    isLast: boolean;
}>();

const emit = defineEmits<{
    (e: 'update'): void;
    (e: 'delete', index: number): void;
    (e: 'move-up', index: number): void;
    (e: 'move-down', index: number): void;
}>();
</script>

<template>
    <div class="task-container">
        <!-- === Title and Description === -->
        <div class="flex flex-row items-center justify-between mb-1">
            <div class="flex flex-row items-center gap-2">
                <span class="pi pi-sort-alpha-up text-textprimary"></span>
                <h4 class="min-w-max text-sm font-semibold text-textprimary m-0">Change Case
                </h4>
                <p class="min-w-max text-xs text-textprimary m-0">
                    Convert text case in file names
                </p>
            </div>
            <!-- === Dummy Spacer === -->
            <div class="flex-1"></div>

            <!-- === Close Button === -->
            <i class="pi pi-angle-up text-textprimary hover:cursor-pointer text-sm mr-1"
                :class="{ 'opacity-30': isFirst }" @click="emit('move-up', index)"></i>
            <i class="pi pi-angle-down text-textprimary hover:cursor-pointer text-sm mr-1"
                :class="{ 'opacity-30': isLast }" @click="emit('move-down', index)"></i>
            <!-- === Close Button === -->
            <div class="flex items-center" @click="emit('delete', index)">
                <i class="pi pi-times hover:cursor-pointer text-sm text-textprimary hover:text-red-500 transition-colors"
                    style="font-size: 0.9rem"></i>
            </div>
        </div>

        <!-- === Main Controls === -->
        <div class="flex flex-row gap-3 items-center">

            <!-- === Case Choice Dropdown === -->
            <Select v-model="task.ChangeCase.case_choice" :options="[
                { label: 'lowercase', value: 0 },
                { label: 'UPPERCASE', value: 1 },
                // { label: 'Title Case', value: 2 },
            ]" optionLabel="label" optionValue="value" placeholder="Select case type" size="small" class="flex-1"
                @change="emit('update')" />

        </div>
    </div>
</template>
