<script setup lang="ts">
// import { ref, Ref } from "vue";
import type { FileStatus } from "../types";
import type { PropType } from "vue";

const props = defineProps({
    fileStatuses: {
        type: Array as PropType<FileStatus[]>,
        required: true,
    },
    numFileStatuses: {
        type: Number,
        required: true,
    },
});

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
        v-if="numFileStatuses < 1"
        class="flex flex-1 flex-col justify-center items-center w-full h-full whitespace-nowrap bg-panelbody"
    >
        <span class="text-center -mt-4 mb-1 text-textprimary">Your files live here</span>
        <span class="text-center text-sm text-textsecondary">Please use the open buttons above</span>
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
                        v-for="(item, index) in fileStatuses"
                        :key="index"
                        @click.exact="emit('userFilestatusClick', index)"
                        @click.ctrl="emit('userFilestatusCtrlClick', index)"
                        @click.meta="emit('userFilestatusCtrlClick', index)"
                        @click.shift="emit('userFilestatusShiftClick', index)"
                        class="cursor-pointer"
                        :class="item.selected ? 'bg-primary/20' : ''"
                    >
                        <td
                            class="px-4 py-2 border-b border-bordercolor break-words"
                            :class="{
                                'text-primary font-medium': item.selected,
                                'text-textprimary': !item.selected,
                                'opacity-50': !item.selected && !item.active,
                                'italic': !item.active,
                            }"
                        >
                            {{ item.old_file_name }}
                        </td>
                        <td
                            class="px-4 py-2 border-b border-bordercolor break-words"
                            :class="{
                                'text-primary font-medium': item.selected,
                                'text-textprimary': !item.selected,
                                'opacity-50': !item.selected && !item.active,
                                'italic': !item.active,
                            }"
                        >
                            {{ item.new_file_name }}
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>
</template>
