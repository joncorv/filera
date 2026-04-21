<script setup lang="ts">
import InputText from "primevue/inputtext";
import { Button } from "primevue";
import Chip from "primevue/chip";
import ToggleButton from "primevue/togglebutton";

defineProps<{
    task: {
        FilterDocType: {
            inclusive: boolean;
            doc_types: string[];
            newDocTypeInput?: string;
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
                <h4 class="text-sm font-semibold text-textprimary m-0">File Type Filter</h4>
                <p class="text-xs text-textprimary m-0">Filter out files by Document Type</p>
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

            <div class="flex gap-2">
                <InputText v-model="task.FilterDocType.newDocTypeInput"
                    :id="`doc-type-input-${index}`" size="small" placeholder="Enter doc type"
                    @keyup.enter="() => {
                        if (task.FilterDocType.newDocTypeInput?.trim()) {
                            task.FilterDocType.doc_types.push(task.FilterDocType.newDocTypeInput.trim());
                            task.FilterDocType.newDocTypeInput = '';
                            emit('update');
                        }
                    }" />
                <Button label="Add" size="small" @click="() => {
                    if (task.FilterDocType.newDocTypeInput?.trim()) {
                        task.FilterDocType.doc_types.push(task.FilterDocType.newDocTypeInput.trim());
                        task.FilterDocType.newDocTypeInput = '';
                        emit('update');
                    }
                }" />
            </div>

            <div class="w-full flex flex-row items-center ">
                <div v-for="(foo, chipIndex) in task.FilterDocType.doc_types"
                    :key="`${foo}-${chipIndex}`">
                    <Chip :label="foo" removable
                        @remove="task.FilterDocType.doc_types.splice(chipIndex, 1); emit('update')" />
                </div>
            </div>

            <!-- === Position at Start or End === -->
            <div class="flex-1">
                <ToggleButton v-model="task.FilterDocType.inclusive" onLabel="Inclusive"
                    offLabel="Exclusive" size="small" @change="emit('update')" />
            </div>

        </div>
    </div>
</template>
