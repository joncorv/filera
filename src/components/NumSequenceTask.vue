<script setup lang="ts">
import InputText from "primevue/inputtext";
import InputNumber from "primevue/inputnumber";
import FloatLabel from "primevue/floatlabel";
import ToggleButton from "primevue/togglebutton";

defineProps<{
    task: {
        NumSequence: {
            start_num: number;
            num_padding: number;
            at_start: boolean;
            separator: string;
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
                <span class="pi pi-sort-numeric-down text-textprimary"></span>
                <h4 class="text-sm font-semibold text-textprimary m-0">Number Sequence</h4>
                <p class="text-xs text-textprimary m-0">Add sequential numbers to file names</p>
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
        <div class="flex flex-row gap-2 items-center">
            <!-- === Start Number === -->
            <div class="w-full min-w-36">
                <FloatLabel variant="on" class="">
                    <InputNumber v-model="task.NumSequence.start_num"
                        :id="`start-num-${index}`" :input-id="`start-num-inputid-${index}`"
                        fluid show-buttons buttonLayout="horizontal" size="small" class="w-full"
                        @value-change="emit('update')" />
                    <label for="`start-num-inputid-${index}`">Start #</label>
                </FloatLabel>
            </div>

            <!-- === Padding === -->
            <div class="w-full min-w-36">
                <FloatLabel variant="on" class="">
                    <InputNumber v-model="task.NumSequence.num_padding"
                        :id="`padding-${index}`" :input-id="`padding-inputid-${index}`"
                        size="small" fluid show-buttons buttonLayout="horizontal" class="w-full"
                        @value-change="emit('update')" />
                    <label for="`padding-${index}`">Padding</label>
                </FloatLabel>
            </div>

            <!-- === Separator === -->
            <div class="flex-1">
                <FloatLabel variant="on" class="">
                    <InputText v-model="task.NumSequence.separator"
                        :id="`separator-${index}`" size="small" class="w-21"
                        @input="emit('update')" />
                    <label for="`separator-${index}`">Separator</label>
                </FloatLabel>
            </div>

            <div class="flex-1">
                <ToggleButton v-model="task.NumSequence.at_start" onLabel="@ Start"
                    offLabel="@ End" size="small" class="flex-1" @change="emit('update')" />
            </div>
        </div>
    </div>
</template>
