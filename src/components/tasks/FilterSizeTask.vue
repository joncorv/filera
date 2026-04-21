<script setup lang="ts">
import InputNumber from "primevue/inputnumber";
import Select from "primevue/select";
import ToggleButton from "primevue/togglebutton";

defineProps<{
    task: {
        FilterSize: {
            greater_than: boolean;
            byte_base_size: number;
            size: number;
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
                <h4 class="text-sm font-semibold text-textprimary m-0">Size Filter</h4>
                <p class="text-xs text-textprimary m-0">Filter by file size</p>
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

            <!-- === Size Number === -->
            <div class="w-full min-w-36">
                <InputNumber v-model="task.FilterSize.size" :id="`size-${index}`"
                    :input-id="`size-inputid-${index}`" fluid show-buttons
                    buttonLayout="horizontal" size="small" class="w-full"
                    @value-change="emit('update')" />
            </div>

            <!-- === File Size Byte Size Dropdown === -->
            <div class="w-full">
                <Select v-model="task.FilterSize.byte_base_size" :options="[
                    { label: 'Bytes', value: 0 },
                    { label: 'Kilobytes', value: 1 },
                    { label: 'Megabytes', value: 2 },
                    { label: 'Gigabytes', value: 3 },
                    { label: 'Terabytes', value: 4 },
                ]" optionLabel="label" optionValue="value" fluid size="small" class="flex-1"
                    @change="emit('update')" />
            </div>

            <!-- === Position at Start or End === -->
            <div class="flex-1">
                <ToggleButton v-model="task.FilterSize.greater_than" onLabel="Greater Than"
                    offLabel="Less Than" size="small" @change="emit('update')" />
            </div>

        </div>
    </div>
</template>
