<script setup lang="ts">
import type { FileStatusResponse } from "../types";
import Button from "primevue/button";

defineProps<{
    fileStatusResponse: FileStatusResponse;
}>();

const emit = defineEmits<{
    userFilestatusClick: [index: number];
    userFilestatusCtrlClick: [index: number];
    userFilestatusShiftClick: [index: number];
    userFilestatusSelectionClear: [];
    userFilestatusSelectionDelete: [];
}>();

//
</script>

<template>
    <div
        v-if="fileStatusResponse.statuses.length < 1"
        class="flex flex-1 flex-col justify-center items-center w-full h-full whitespace-nowrap bg-panelbody select-none"
    >
        <span class="text-center -mt-4 mb-1 text-textprimary">Your files live here</span>
        <span class="text-center text-sm text-textsecondary">Drag and drop here, or use the buttons above</span>
    </div>

    <div v-else id="table-container" class="flex-1/2 flex flex-col mb-0 min-h-0 text-sm bg-panelbody select-none">
        <table class="w-full table-fixed text-textprimary bg-panelheader/50">
            <colgroup>
                <col class="w-1/2" />
                <col class="w-1/2" />
            </colgroup>
            <thead>
                <tr>
                    <th class="px-4 pt-2.5 pb-1.5 text-left border-b border-bordercolor">Old Name</th>
                    <th class="px-4 pt-2.5 pb-1.5 text-left border-b border-bordercolor">New Name</th>
                </tr>
            </thead>
        </table>

        <div class="flex-1/2 overflow-y-auto min-h-0 text-textprimary">
            <table class="w-full table-fixed">
                <colgroup>
                    <col class="w-1/2" />
                    <col class="w-1/2" />
                </colgroup>
                <tbody>
                    <tr
                        v-for="(item, index) in fileStatusResponse.statuses"
                        :key="index"
                        @click.exact="emit('userFilestatusClick', index)"
                        @click.ctrl="emit('userFilestatusCtrlClick', index)"
                        @click.meta="emit('userFilestatusCtrlClick', index)"
                        @click.shift="emit('userFilestatusShiftClick', index)"
                        class="cursor-pointer"
                        :class="item.selected ? 'bg-accenthover' : ''"
                    >
                        <td
                            class="px-4 py-2 border-b border-bordercolor break-words"
                            :class="{
                                'font-medium text-highlighttext': item.selected,
                                'text-textprimary': !item.selected,
                                'opacity-50': !item.selected && !item.active,
                                italic: !item.active,
                            }"
                        >
                            {{ item.old_file_name }}
                        </td>
                        <td
                            class="px-4 py-2 border-b border-bordercolor break-words"
                            :class="{
                                'font-medium text-highlighttext': item.selected,
                                'text-textprimary': !item.selected,
                                'opacity-50': !item.selected && !item.active,
                                italic: !item.active,
                            }"
                        >
                            {{ item.new_file_name }}
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>

    <div
        v-if="fileStatusResponse.stats.selected > 0"
        class="flex justify-center gap-2 p-2 bg-panelheader/50 border-t border-bordercolor"
    >
        <Button
            size="small"
            severity="secondary"
            icon="pi pi-eraser"
            label="Clear Selection"
            @click="emit('userFilestatusSelectionClear')"
        />
        <Button
            size="small"
            severity="danger"
            icon="pi pi-trash"
            label="Remove Files"
            @click="emit('userFilestatusSelectionDelete')"
        />
    </div>
</template>
