<script setup lang="ts">
import InputText from "primevue/inputtext";
import FloatLabel from "primevue/floatlabel";

defineProps<{
    task: {
        FindAndReplace: {
            find_text: string;
            replace_text: string;
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
                <span class="pi pi-search text-textprimary"></span>
                <h4 class="text-sm font-semibold text-textprimary m-0">Find & Replace</h4>
                <p class="text-xs text-textprimary m-0">Replace text in file names</p>
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
            <!-- === Find Text Field === -->
            <div class="w-full min-w-36">
                <FloatLabel variant="on" class="font-thin">
                    <InputText :id="`find-text-${index}`" fluid placeholder="" size="small"
                        v-model="task.FindAndReplace.find_text" @input="emit('update')"
                        class="flex-1" />
                    <label class="font-thin" for="`find-text-${index}`">Find</label>
                </FloatLabel>
            </div>

            <!-- === Replace Text Field === -->
            <div class="w-full min-w-36">
                <FloatLabel variant="on" class="font-thin">
                    <InputText :id="`replace-text-${index}`" fluid placeholder="" size="small"
                        v-model="task.FindAndReplace.replace_text"
                        @input="emit('update')" class="flex-1" />
                    <label class="font-thin" for="`replace-text-${index}`">Replace</label>
                </FloatLabel>
            </div>
        </div>
    </div>
</template>
