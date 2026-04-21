<script setup lang="ts">
import FloatLabel from "primevue/floatlabel";
import DatePicker from "primevue/datepicker";
import ToggleButton from "primevue/togglebutton";

defineProps<{
    task: {
        FilterTimePeriod: {
            inclusive: boolean;
            start_time: Date | null;
            end_time: Date | null;
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
                <h4 class="text-sm font-semibold text-textprimary m-0">Time Period Filter</h4>
                <p class="text-xs text-textprimary m-0">Filter by period of time</p>
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
            <div class="w-full">
                <FloatLabel variant="on">
                    <DatePicker class="" fluid v-model="task.FilterTimePeriod.start_time" :id="`separator-${index}`"
                        size="small" @update:modelValue="emit('update')" />
                    <label for="`name-${index}`">Start Date</label>
                </FloatLabel>
            </div>

            <!-- === End Date Picker === -->
            <div class="w-full">
                <FloatLabel variant="on">
                    <DatePicker class="" fluid v-model="task.FilterTimePeriod.end_time" :id="`separator-${index}`"
                        size="small" @update:modelValue="emit('update')" />
                    <label for="`name-${index}`">End Date</label>
                </FloatLabel>
            </div>

            <!-- === Position at Start or End === -->
            <div class="flex-1">
                <ToggleButton v-model="task.FilterTimePeriod.inclusive" onLabel="Filter Matching Dates"
                    offLabel="Filter Non-Matching Dates" size="small" @change="emit('update')" />
            </div>

        </div>
    </div>
</template>
