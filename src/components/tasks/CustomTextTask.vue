<script setup lang="ts">
import InputText from "primevue/inputtext";
import FloatLabel from "primevue/floatlabel";
import ToggleButton from "primevue/togglebutton";

defineProps<{
    task: {
        CustomText: {
            text: string;
            at_start: boolean;
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
                <span class="pi pi-pen-to-square text-textprimary"></span>
                <h4 class="text-sm font-semibold text-textprimary m-0">Custom Text</h4>
                <p class="text-xs text-textprimary m-0">Add text to file names</p>
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
            <!-- === Prefix Text === -->
            <div class="w-full min-w-36">
                <FloatLabel variant="on" class="font-thin">
                    <InputText fluid size="small" placeholder="" :id="`input-text-${index}`"
                        v-model="task.CustomText.text" @input="emit('update')"
                        class="" />
                    <label class="font-thin" for="`input-text-${index}`">Value</label>
                </FloatLabel>
            </div>

            <!-- === Position at Start or End === -->
            <div class="flex-1">
                <ToggleButton v-model="task.CustomText.at_start" onLabel="@ Start"
                    offLabel="@ End" size="small" @change="emit('update')" />
            </div>
        </div>
    </div>
</template>
