<script setup lang="ts">
import { ref, Ref } from "vue";
import type { FileStatus } from "../types";
import type { PropType } from "vue";

const props = defineProps({
    data: {
        type: Array as PropType<FileStatus[]>,
        required: true,
    },
    numItems: {
        type: Number,
        required: true,
    },
});

const lastSelectedRow: Ref<number | null> = ref(null);
const selectedRows: Ref<number[]> = ref<number[]>([]);

function rowClick(index: number) {
    lastSelectedRow.value = index;
    selectedRows.value.push(index);
}

function rowShiftClick(index: number) {
    if (lastSelectedRow.value) {
        let start: number = 0;
        let end: number = 0;

        if (lastSelectedRow.value > index) {
            start = index;
            end = lastSelectedRow.value;
        } else {
            start = lastSelectedRow.value;
            end = index;
        }

        for (let i = start; i <= end; i++) {
            selectedRows.value.push(i);
        }
    } else {
        selectedRows.value.push(index);
    }

    console.log("result of shift click: ", selectedRows.value);
}
</script>

<template>
    <div
        v-if="numItems < 1"
        class="flex flex-1 flex-col justify-center items-center w-full h-full whitespace-nowrap bg-panelbody"
    >
        <span class="text-center -mt-4 mb-1 text-textprimary">Your files live here</span>
        <span class="text-center text-sm text-textsecondary">Please use the open buttons above</span>
    </div>

    <div v-else id="table-container" class="flex-1/2 flex flex-col mb-0 min-h-0 text-sm bg-panelbody select-none">
        <table class="w-full text-textprimary bg-panelheader/50">
            <thead>
                <tr>
                    <th class="px-4 pt-2.5 pb-1.5 text-left border-b border-bordercolor">Old Name</th>
                    <th class="px-4 pt-2.5 pb-1.5 text-left border-b border-bordercolor">New Name</th>
                </tr>
            </thead>
        </table>

        <div class="flex-1/2 overflow-y-auto min-h-0 text-textprimary">
            <table class="w-full">
                <tbody>
                    <tr
                        v-for="(item, index) in data"
                        :key="index"
                        @click="rowClick(index)"
                        @click.shift="rowShiftClick(index)"
                    >
                        <template v-if="item.active">
                            <td class="px-4 py-2 border-b border-bordercolor">
                                {{ item.old_file_name }}
                            </td>
                            <td class="px-4 py-2 border-b border-bordercolor">
                                {{ item.new_file_name }}
                            </td>
                        </template>

                        <template v-else>
                            <td class="px-4 py-2 border-b border-bordercolor bg-panelfooter italic opacity-50">
                                {{ item.old_file_name }}
                            </td>
                            <td class="px-4 py-2 border-b border-bordercolor bg-panelfooter italic opacity-50">
                                {{ item.new_file_name }}
                            </td>
                        </template>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>
</template>
