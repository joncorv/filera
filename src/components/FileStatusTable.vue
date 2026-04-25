<script setup lang="ts">
import { ref, Ref } from "vue";
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

const lastSelectedRow: Ref<number | null> = ref(null);
const selectedRows: Ref<number[]> = ref<number[]>([]);

function rowClick(index: number) {
    console.log("clicked on index: ", index);
    lastSelectedRow.value = index;
    selectedRows.value.push(index);
}

function rowShiftClick(index: number) {
    console.log("shift-click function begin", "current index", index, "previous index", lastSelectedRow.value);

    // if there wasn't a previous selection, just run the regular click function
    if (lastSelectedRow.value == null) {
        rowClick(index);
    } else {
        // iterate from smallest index until greatest index
        let start: number = Math.min(lastSelectedRow.value, index);
        let end: number = Math.max(lastSelectedRow.value, index);
        console.log("shift-click", "start", start, "end", end);

        // add
        for (let i = start; i <= end; i++) {
            selectedRows.value.push(i);
            console.log("pushing index:", i);
        }
    }
}
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
                        v-for="(item, index) in fileStatuses"
                        :key="index"
                        @click.exact="rowClick(index)"
                        @click.shift="rowShiftClick(index)"
                    >
                        <!-- <template v-if="item.active"> -->
                        <!--     <td class="px-4 py-2 border-b border-bordercolor"> -->
                        <!--         {{ item.old_file_name }} -->
                        <!--     </td> -->
                        <!--     <td class="px-4 py-2 border-b border-bordercolor"> -->
                        <!--         {{ item.new_file_name }} -->
                        <!--     </td> -->
                        <!-- </template> -->

                        <template v-if="item.selected">
                            <td class="px-4 py-2 border-b border-bordercolor text-red-500">
                                {{ item.old_file_name }}
                            </td>
                            <td class="px-4 py-2 border-b border-bordercolor text-red-500">
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
