<script setup lang="ts">
import FloatLabel from "primevue/floatlabel";
import DatePicker from "primevue/datepicker";
import ToggleButton from "primevue/togglebutton";

defineProps<{
    task: {
        FilterTime: {
            before: boolean;
            time: string;
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
                <span class="pi pi-clock text-textprimary"></span>
                <h4 class="text-sm font-semibold text-textprimary m-0">Time Filter</h4>
                <p class="text-xs text-textprimary m-0">Filter before or after a date</p>
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

            <!-- === Start Date Picker === -->
            <div class="w-full max-w-78">
                <FloatLabel variant="on">
                    <DatePicker v-model="task.FilterTime.time" :id="`separator-${index}`"
                        fluid size="small" @input="emit('update')" />
                    <label for="`name-${index}`">Date</label>
                </FloatLabel>
            </div>

            <div class="w-full bg-red"> </div>

            <!-- === Position at Start or End === -->
            <div class="flex-1">
                <ToggleButton v-model="task.FilterTime.before" onLabel="Filter Older Dates"
                    offLabel="Filter Newer Dates" size="small" @change="emit('update')" />
            </div>

        </div>
    </div>
</template>
