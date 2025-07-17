<script setup lang="ts">
import { ref, Ref, computed } from "vue";
import "./styles.css"; // Tailwind Stuff
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
// import { usePosNoise } from "./scripts/usePosNoise";
import { Button } from "primevue";
import Splitter from "primevue/splitter";
import SplitterPanel from "primevue/splitterpanel";
import InputText from "primevue/inputtext";
import ToggleSwitch from "primevue/toggleswitch";
import Select from "primevue/select";

import "primeicons/primeicons.css";

//  <-- === WorkingFile Interface === -->
interface FileStatus {
    old_file_name: string;
    new_file_name: string;
}

//  <-- === Task Type. Contains all Sub Tasks within === -->
type Task =
    | { CustomText: { text: string; at_start: boolean; active: boolean } }
    | { FindAndReplace: { find_text: string; replace_text: string; active: boolean } }
    | { ClearAll: { active: boolean } }
    | { ChangeCase: { case_choice: number; active: boolean } }
    | { NumSequence: { start_num: number; num_padding: number; active: boolean } }
    | { Date: { year: boolean; month: boolean; day: boolean; year_4: boolean; separator: string; active: boolean } }
    | { Time: { hour_24: boolean; ampm: boolean; separator: string; active: boolean } };

const sortChoice = ref();
const metadata = ref([
    { name: "Name", code: "name" },
    { name: "Date Created", code: "dateCreated" },
    { name: "Date Modified", code: "dateModified" },
    { name: "Type", code: "type" },
    { name: "Size", code: "size" },
]);

//  <-- === Add unique ID to each task. Needed for proper animation in the DOM === -->
interface TaskWithId {
    id: number;
    task: Task;
}

//  <-- === Type Guards === -->
const isCustomTextTask = (
    task: Task,
): task is {
    CustomText: { text: string; at_start: boolean; active: boolean };
} => {
    return "CustomText" in task;
};

const isFindAndReplaceTask = (
    task: Task,
): task is {
    FindAndReplace: { find_text: string; replace_text: string; active: boolean };
} => {
    return "FindAndReplace" in task;
};

const isClearAllTask = (
    task: Task,
): task is {
    ClearAll: { active: boolean };
} => {
    return "ClearAll" in task;
};

const isChangeCaseTask = (
    task: Task,
): task is {
    ChangeCase: { case_choice: number; active: boolean };
} => {
    return "ChangeCase" in task;
};

const isNumSequenceTask = (
    task: Task,
): task is {
    NumSequence: { start_num: number; num_padding: number; active: boolean };
} => {
    return "NumSequence" in task;
};

const isDateTask = (
    task: Task,
): task is {
    Date: { year: boolean; month: boolean; day: boolean; year_4: boolean; separator: string; active: boolean };
} => {
    return "Date" in task;
};

const isTimeTask = (
    task: Task,
): task is {
    Time: { hour_24: boolean; ampm: boolean; separator: string; active: boolean };
} => {
    return "Time" in task;
};

//  <-- === Create an array of TaskWithId === -->
const taskList = ref<TaskWithId[]>([]);

//  <-- === Counter for generating unique IDs === -->
let taskIdCounter = 0;

const addCustomText = () => {
    taskList.value.push({
        id: taskIdCounter++,
        task: {
            CustomText: {
                text: "",
                at_start: true,
                active: true,
            },
        },
    });
    user_update_tasks();
};

const addFindReplace = () => {
    taskList.value.push({
        id: taskIdCounter++,
        task: {
            FindAndReplace: {
                find_text: "",
                replace_text: "",
                active: true,
            },
        },
    });
    user_update_tasks();
};

const addClearAll = () => {
    taskList.value.push({
        id: taskIdCounter++,
        task: {
            ClearAll: { active: true },
        },
    });
    user_update_tasks();
};

const addChangeCase = () => {
    taskList.value.push({
        id: taskIdCounter++,
        task: {
            ChangeCase: { case_choice: 0, active: true },
        },
    });
    user_update_tasks();
};

const addNumSequence = () => {
    taskList.value.push({
        id: taskIdCounter++,
        task: {
            NumSequence: { start_num: 0, num_padding: 4, active: true },
        },
    });
    user_update_tasks();
};

const addDate = () => {
    taskList.value.push({
        id: taskIdCounter++,
        task: {
            NumSequence: {
                year: true,
                month: true,
                day: true,
                year_4: true,
                separator: "_",
                active: true,
            },
        },
    });
    user_update_tasks();
};

const addTime = () => {
    taskList.value.push({
        id: taskIdCounter++,
        task: {
            Time: {
                hour_24: true,
                ampm: false,
                separator: "_",
                active: true,
            },
        },
    });
    user_update_tasks();
};

let dirtyFilesSelection: string[] | null = null;
const fileStatusReturn: Ref<FileStatus[]> = ref([]);
const numFileStatusItems = computed(() => fileStatusReturn.value.length);

//  <-- === Opens Files System Dialog === -->
async function open_files() {
    const selectedFiles = await open({
        multiple: true,
        director: true,
    });

    if (selectedFiles) {
        fileStatusReturn.value = await invoke("user_open_files", {
            fileNames: selectedFiles,
        });
    } else {
        console.log("There was no previous selection or current selection. No Updates to make.");
    }
}

//  <-- === Update interface to show latest files === -->
async function user_update_tasks() {
    fileStatusReturn.value = await invoke("user_update_tasks", {
        taskList: taskList.value.map((t) => t.task),
    });
}

//  <-- === Rename Files on the Rust Backend === -->
async function rename_files() {
    if (dirtyFilesSelection) {
        let renameSuccess: string = await invoke("rename_files", {
            fileNames: dirtyFilesSelection,
            taskList: taskList.value.map((t) => t.task),
        });

        if (renameSuccess.startsWith("Rename Error", 0)) {
            console.log(renameSuccess);
        } else {
            clearFiles();
        }
    }
}

//  <-- === All Task Functions === -->
async function clearFiles() {
    fileStatusReturn.value = [];
    await invoke("user_clear_files");
}

async function clearTasks() {
    const loopLength = taskList.value.length;
    for (let i = loopLength - 1; i >= 0; i--) {
        taskList.value.pop();
        if (i > 0) {
            // Don't wait after the last iteration
            await new Promise((resolve) => setTimeout(resolve, 40));
        }
    }
    user_update_tasks();
}
function deleteSelectedTask(index: number) {
    taskList.value.splice(index, 1);
    user_update_tasks();
}
function moveSelectedTaskUp(index: number) {
    if (index > 0) {
        const [item] = taskList.value.splice(index, 1);
        taskList.value.splice(index - 1, 0, item);
        user_update_tasks();
    }
}
function moveSelectedTaskDown(index: number) {
    if (index < taskList.value.length - 1) {
        const [item] = taskList.value.splice(index, 1);
        taskList.value.splice(index + 1, 0, item);
        user_update_tasks();
    }
}

//  <-- === usePosNoise function from external file. === -->
// const sphere1 = usePosNoise({ size: 1.2, speed: 0.0005, amplitude: 175 });
// const sphere2 = usePosNoise({ size: 0.8, speed: 0.0003, amplitude: 100 });
// const sphere3 = usePosNoise({ size: 1.5, speed: 0.0004, amplitude: 175 });
</script>

<template>
    <body class="flex flex-col box-border w-screen h-screen m-0 p-0 overflow-hidden z-0">
        <!-- === Green Circles === -->
        <!-- <div
            id="circles-random-positions"
            class="absolute h-screen w-screen overflow-hidden"
        >
            <div
                id="circle_top_left"
                class="absolute w-30 h-30 rounded-full bg-green-400"
                :style="{
                    transform: `translate(${160 + sphere1.x}px, ${sphere1.y}px)`,
                }"
            />
            <div
                id="circle_right"
                class="absolute w-40 h-40 rounded-full bg-green-200"
                :style="{
                    transform: `translate(${1000 + sphere2.x}px, ${sphere2.y}px)`,
                }"
            />
            <div
                id="circle_btm"
                class="absolute w-60 h-60 rounded-full bg-green-400"
                :style="{
                    transform: `translate(${400 + sphere3.x}px, ${520 + sphere3.y}px)`,
                }"
            />
        </div> -->

        <!-- === Master Splitter Panel === -->
        <Splitter style="flex: 1; overflow: hidden; background-color: transparent; z-index: 40">
            <!-- === Left Splitter Panel === -->
            <SplitterPanel
                class="flex flex-col gap-0 m-4 rounded-2xl p-4 border-2 border-white/30 bg-white/20 backdrop-blur-lg shadow-lg z-50"
            >
                <!-- === File Buttons === -->
                <div id="file_buttons" class="flex flex-row items-center justify-start mb-4">
                    <!-- <Button class="mr-4" label="Open A Folder" severity="primary" @click="open_folder" icon="pi pi-folder-open" /> -->
                    <Button class="reg-button" unstyled label="Open Files" @click="open_files" icon="pi pi-file" />
                    <Button
                        class="reg-button"
                        unstyled
                        label="Clear All Files"
                        severity="danger"
                        @click="clearFiles"
                        icon="pi pi-trash"
                    />
                    <Select
                        v-model="sortChoice"
                        :options="metadata"
                        optionLabel="name"
                        placeholder="Sort By"
                        optionValue="code"
                        class="w-full md:w-56"
                    />
                </div>

                <!-- <hr class="border-1 border-white/30 my-4" /> -->

                <Transition mode="out-in">
                    <!-- === No Files Selected === -->
                    <div v-if="!numFileStatusItems" class="flex-1 justify-items-center flex-row ml-4 mb-2">
                        <span class="font-thin text-xs font-mono text-center text-black/50"> no files selected </span>
                    </div>

                    <!-- === File Table === -->
                    <div
                        v-else
                        id="table-container"
                        class="flex-1 flex flex-col mb-2 min-h-0 text-sm bg-white/50 rounded-sm border-1 border-black/10"
                    >
                        <table class="w-full">
                            <thead>
                                <tr>
                                    <th class="px-4 py-2 text-left border-b border-white/30">Old Name</th>
                                    <th class="px-4 py-2 text-left border-b border-white/30">New Name</th>
                                </tr>
                            </thead>
                        </table>

                        <div class="flex-1 overflow-y-auto min-h-0">
                            <table class="w-full">
                                <tbody>
                                    <tr v-for="(item, index) in fileStatusReturn" :key="index">
                                        <td class="px-4 py-2 border-b border-black/10">
                                            {{ item.old_file_name }}
                                        </td>
                                        <td class="px-4 py-2 border-b border-black/10">
                                            {{ item.new_file_name }}
                                        </td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </div>
                </Transition>

                <!-- === Footer -> Total Files Selected === -->
                <div id="selection_info">
                    <span class="font-medium">Total Files Selected:</span>
                    <Transition mode="out-in">
                        <span class="ml-2">{{ numFileStatusItems }}</span>
                    </Transition>
                </div>
            </SplitterPanel>

            <!-- === Right Splitter Panel === -->
            <SplitterPanel class="flex flex-col flex-1">
                <div class="flex flex-row mt-4 mb-2 ml-4">
                    <Button
                        class="reg-button"
                        unstyled
                        severity="primary"
                        @click="addCustomText"
                        label="Add Custom Text"
                        icon="pi pi-arrow-circle-left"
                    />
                    <Button
                        class="reg-button"
                        unstyled
                        severity="primary"
                        @click="addFindReplace"
                        label="Add Find & Replace"
                        icon="pi pi-search"
                    />
                    <Button
                        class="reg-button shadow-xl"
                        unstyled
                        severity="danger"
                        @click="clearTasks"
                        label="Clear All Tasks"
                        icon="pi pi-trash"
                    />
                </div>

                <TransitionGroup tag="div" name="ttasks" class="relative">
                    <div v-for="(item, index) in taskList" :key="item.id" class="ttasks-item mx-4 my-2">
                        <!-- === CustomText Task === -->
                        <template v-if="isCustomTextTask(item.task)">
                            <div
                                class="flex flex-row gap-4 border-2 border-white/30 bg-white/40 rounded-lg p-2 backdrop-blur-lg shadow-lg"
                            >
                                <!-- === Drag Buttons === -->
                                <div class="flex flex-col items-center ml-2 mr-1 gap-1">
                                    <i
                                        class="pi pi-angle-up hover:cursor-pointer text-xs"
                                        :class="{ 'opacity-30': index === 0 }"
                                        @click="moveSelectedTaskUp(index)"
                                    ></i>
                                    <i
                                        class="pi pi-angle-down hover:cursor-pointer text-xs"
                                        :class="{
                                            'opacity-30': index === taskList.length - 1,
                                        }"
                                        @click="moveSelectedTaskDown(index)"
                                    ></i>
                                </div>

                                <!-- === Prefix Text === -->
                                <InputText
                                    placeholder="Prefix Text"
                                    fluid
                                    size="small"
                                    :id="`input-text-${index}`"
                                    v-model="item.task.CustomText.text"
                                    variant="filled"
                                    @input="user_update_tasks"
                                />

                                <!-- === Checkbox === -->
                                <div class="flex items-center gap-2">
                                    <ToggleSwitch
                                        v-model="item.task.CustomText.active"
                                        :inputId="`checkbox-${index}`"
                                        :name="`active-checkbox${index}`"
                                        binary
                                        @change="user_update_tasks"
                                    />
                                </div>

                                <!-- === Delete Button === -->
                                <div class="flex items-center ml-0 mr-2" @click="deleteSelectedTask(index)">
                                    <i
                                        class="pi pi-trash hover:cursor-pointer"
                                        style="font-size: 1.1rem; color: red"
                                    ></i>
                                </div>
                            </div>
                        </template>

                        <!-- === Find & Replace Task === -->
                        <template v-else-if="isFindAndReplaceTask(item.task)">
                            <div
                                class="flex flex-row gap-4 border-2 border-white/30 bg-white/40 rounded-lg p-2 backdrop-blur-lg shadow-lg"
                            >
                                <!-- === Drag Buttons === -->
                                <div class="flex flex-col items-center ml-2 mr-1 gap-1">
                                    <i
                                        class="pi pi-angle-up hover:cursor-pointer text-xs"
                                        :class="{ 'opacity-30': index === 0 }"
                                        @click="moveSelectedTaskUp(index)"
                                    ></i>
                                    <i
                                        class="pi pi-angle-down hover:cursor-pointer text-xs"
                                        :class="{
                                            'opacity-30': index === taskList.length - 1,
                                        }"
                                        @click="moveSelectedTaskDown(index)"
                                    ></i>
                                </div>

                                <!-- === Find Text Field === -->
                                <InputText
                                    id="in_label"
                                    placeholder="Find"
                                    fluid
                                    size="small"
                                    v-model="item.task.FindAndReplace.find_text"
                                    variant="filled"
                                    @input="user_update_tasks"
                                />

                                <!-- ===Replace Text Field === -->
                                <InputText
                                    id="in_label"
                                    placeholder="Replace"
                                    fluid
                                    size="small"
                                    v-model="item.task.FindAndReplace.replace_text"
                                    variant="filled"
                                    @input="user_update_tasks"
                                />

                                <!-- === Checkbox === -->
                                <div class="flex items-center gap-2">
                                    <ToggleSwitch
                                        v-model="item.task.FindAndReplace.active"
                                        :inputId="`active-${index}`"
                                        name="namefindreplaceactive"
                                        binary
                                        @change="user_update_tasks"
                                    />
                                </div>

                                <!-- === Delete Button === -->
                                <div class="flex items-center ml-0 mr-2" @click="deleteSelectedTask(index)">
                                    <i
                                        class="pi pi-trash hover:cursor-pointer"
                                        style="font-size: 1.1rem; color: red"
                                    ></i>
                                </div>
                            </div>
                        </template>

                        <!-- === ClearAll Task === -->
                        <template v-else-if="isClearAllTask(item.task)">
                            <div
                                class="flex flex-row gap-4 border-2 border-white/30 bg-white/40 rounded-lg p-2 backdrop-blur-lg shadow-lg"
                            >
                                <!-- === Drag Buttons === -->
                                <div class="flex flex-col items-center ml-2 mr-1 gap-1">
                                    <i
                                        class="pi pi-angle-up hover:cursor-pointer text-xs"
                                        :class="{ 'opacity-30': index === 0 }"
                                        @click="moveSelectedTaskUp(index)"
                                    ></i>
                                    <i
                                        class="pi pi-angle-down hover:cursor-pointer text-xs"
                                        :class="{
                                            'opacity-30': index === taskList.length - 1,
                                        }"
                                        @click="moveSelectedTaskDown(index)"
                                    ></i>
                                </div>

                                <!-- === Checkbox === -->
                                <div class="flex items-center gap-2">
                                    <ToggleSwitch
                                        v-model="item.task.FindAndReplace.active"
                                        :inputId="`active-${index}`"
                                        name="namefindreplaceactive"
                                        binary
                                        @change="user_update_tasks"
                                    />
                                </div>

                                <!-- === Delete Button === -->
                                <div class="flex items-center ml-0 mr-2" @click="deleteSelectedTask(index)">
                                    <i
                                        class="pi pi-trash hover:cursor-pointer"
                                        style="font-size: 1.1rem; color: red"
                                    ></i>
                                </div>
                            </div>
                        </template>

                        <!-- === ChangeCase Task === -->
                        <template v-else-if="isChangeCaseTask(item.task)">
                            <div
                                class="flex flex-row gap-4 border-2 border-white/30 bg-white/40 rounded-lg p-2 backdrop-blur-lg shadow-lg"
                            >
                                <!-- === Drag Buttons === -->
                                <div class="flex flex-col items-center ml-2 mr-1 gap-1">
                                    <i
                                        class="pi pi-angle-up hover:cursor-pointer text-xs"
                                        :class="{ 'opacity-30': index === 0 }"
                                        @click="moveSelectedTaskUp(index)"
                                    ></i>
                                    <i
                                        class="pi pi-angle-down hover:cursor-pointer text-xs"
                                        :class="{
                                            'opacity-30': index === taskList.length - 1,
                                        }"
                                        @click="moveSelectedTaskDown(index)"
                                    ></i>
                                </div>

                                <!-- === Delete Button === -->
                                <div class="flex items-center ml-0 mr-2" @click="deleteSelectedTask(index)">
                                    <i
                                        class="pi pi-trash hover:cursor-pointer"
                                        style="font-size: 1.1rem; color: red"
                                    ></i>
                                </div>
                            </div>
                        </template>

                        <!-- === NumSequence Task === -->
                        <template v-else-if="isNumSequenceTask(item.task)">
                            <div
                                class="flex flex-row gap-4 border-2 border-white/30 bg-white/40 rounded-lg p-2 backdrop-blur-lg shadow-lg"
                            >
                                <!-- === Drag Buttons === -->
                                <div class="flex flex-col items-center ml-2 mr-1 gap-1">
                                    <i
                                        class="pi pi-angle-up hover:cursor-pointer text-xs"
                                        :class="{ 'opacity-30': index === 0 }"
                                        @click="moveSelectedTaskUp(index)"
                                    ></i>
                                    <i
                                        class="pi pi-angle-down hover:cursor-pointer text-xs"
                                        :class="{
                                            'opacity-30': index === taskList.length - 1,
                                        }"
                                        @click="moveSelectedTaskDown(index)"
                                    ></i>
                                </div>

                                <!-- === Delete Button === -->
                                <div class="flex items-center ml-0 mr-2" @click="deleteSelectedTask(index)">
                                    <i
                                        class="pi pi-trash hover:cursor-pointer"
                                        style="font-size: 1.1rem; color: red"
                                    ></i>
                                </div>
                            </div>
                        </template>

                        <!-- === Date Task === -->
                        <template v-else-if="isDateTask(item.task)">
                            <div
                                class="flex flex-row gap-4 border-2 border-white/30 bg-white/40 rounded-lg p-2 backdrop-blur-lg shadow-lg"
                            >
                                <!-- === Drag Buttons === -->
                                <div class="flex flex-col items-center ml-2 mr-1 gap-1">
                                    <i
                                        class="pi pi-angle-up hover:cursor-pointer text-xs"
                                        :class="{ 'opacity-30': index === 0 }"
                                        @click="moveSelectedTaskUp(index)"
                                    ></i>
                                    <i
                                        class="pi pi-angle-down hover:cursor-pointer text-xs"
                                        :class="{
                                            'opacity-30': index === taskList.length - 1,
                                        }"
                                        @click="moveSelectedTaskDown(index)"
                                    ></i>
                                </div>

                                <!-- === Delete Button === -->
                                <div class="flex items-center ml-0 mr-2" @click="deleteSelectedTask(index)">
                                    <i
                                        class="pi pi-trash hover:cursor-pointer"
                                        style="font-size: 1.1rem; color: red"
                                    ></i>
                                </div>
                            </div>
                        </template>

                        <!-- === Time Task === -->
                        <template v-else-if="isTimeTask(item.task)">
                            <div
                                class="flex flex-row gap-4 border-2 border-white/30 bg-white/40 rounded-lg p-2 backdrop-blur-lg shadow-lg"
                            >
                                <!-- === Drag Buttons === -->
                                <div class="flex flex-col items-center ml-2 mr-1 gap-1">
                                    <i
                                        class="pi pi-angle-up hover:cursor-pointer text-xs"
                                        :class="{ 'opacity-30': index === 0 }"
                                        @click="moveSelectedTaskUp(index)"
                                    ></i>
                                    <i
                                        class="pi pi-angle-down hover:cursor-pointer text-xs"
                                        :class="{
                                            'opacity-30': index === taskList.length - 1,
                                        }"
                                        @click="moveSelectedTaskDown(index)"
                                    ></i>
                                </div>

                                <!-- === Delete Button === -->
                                <div class="flex items-center ml-0 mr-2" @click="deleteSelectedTask(index)">
                                    <i
                                        class="pi pi-trash hover:cursor-pointer"
                                        style="font-size: 1.1rem; color: red"
                                    ></i>
                                </div>
                            </div>
                        </template>
                    </div>
                </TransitionGroup>
            </SplitterPanel>
        </Splitter>

        <!-- === App Footer === -->
        <!-- <div id="footer" class="flex flex-row-reverse m-4 "> -->
        <div
            id="footer"
            class="flex flex-row-reverse gap-0 m-4 rounded-2xl p-4 border-2 border-white/30 bg-white/20 backdrop-blur-lg shadow-lg z-50"
        >
            <!-- === Batch Rename All Files Button === -->
            <Button
                unstyled
                icon="pi pi-trash"
                class="big-button"
                label="Batch Rename All Files"
                size="large"
                @click="rename_files"
            />
        </div>
    </body>
</template>

<style>
:root {
    background-image: url("src/assets/v960-ning-30.jpg");
    background-size: cover;
    background-position: center;
    background-repeat: no-repeat;
    background-attachment: fixed;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
}

.body {
    background-image: url("src/assets/636977f64331639884919566b3ec074a_edit1.jpg");
    background-size: cover;
    background-position: center;
    background-repeat: no-repeat;
    background-attachment: fixed;
}

/* TransitionGroup styles */
.ttasks-item {
    transition: all 0.1s ease;
}

.ttasks-move,
.ttasks-enter-active,
.ttasks-leave-active {
    transition: all 0.1s ease;
}

.ttasks-enter-from {
    opacity: 0;
    transform: translateY(30px);
}

.ttasks-leave-to {
    opacity: 0;
    transform: translateX(0px);
}

.ttasks-leave-active {
    position: absolute;
    left: 0;
    right: 0;
}

.p-splitter {
    --p-splitter-border-width: 0px;
    --p-splitter-border-color: transparent;
    --p-splitter-gutter-background: transparent;
}

.p-datatable {
    --p-datatable-row-background: transparent;
    --p-datatable-header-cell-background: transparent;
    --p-datatable-body-cell-border-color: rgba(255, 255, 255, 0);
    --p-datatable-body-cell-border-width: 0px;
}

.reg-button {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 0.5rem;
    padding: calc(var(--spacing) * 4);
    margin-right: calc(var(--spacing) * 4);
    height: 2.5rem;
    border: 2px solid rgba(255, 255, 255, 0.4);
    border-radius: 9999px;
    color: color-mix(in oklab, var(--color-black) 60%, transparent);
    font-weight: 600;
    font-size: var(--text-sm);
    line-height: var(--tw-leading, var(--text-sm--line-height));
    background-color: color-mix(in oklab, var(--color-white) 80%, transparent);
    box-shadow:
        black 0px 0px 0px 0px,
        rgba(0, 0, 0, 0.05) 0px 4px 6px -1px,
        rgba(0, 0, 0, 0.1) 0px 2px 4px -1px;
    transition: background-color 0.2s ease;
    backdrop-filter: blur(10px);

    &:hover {
        background-color: color-mix(in oklab, var(--color-white) 100%, transparent);
        cursor: pointer;
    }
}

.big-button {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 0.5rem;
    padding: calc(var(--spacing) * 4);
    margin-right: calc(var(--spacing) * 4);
    height: 3.5rem;
    border: 2px solid rgba(255, 255, 255, 0.4);
    border-radius: 15px;
    color: color-mix(in oklab, var(--color-black) 60%, transparent);
    font-weight: 600;
    font-size: var(--text-med);
    line-height: var(--tw-leading, var(--text-sm--line-height));
    background-color: color-mix(in oklab, var(--color-green-400) 80%, transparent);
    box-shadow:
        black 0px 0px 0px 0px,
        rgba(0, 0, 0, 0.05) 0px 4px 6px -1px,
        rgba(0, 0, 0, 0.1) 0px 2px 4px -1px;
    transition: background-color 0.2s ease;
    backdrop-filter: blur(10px);

    &:hover {
        background-color: color-mix(in oklab, var(--color-green-500) 60%, transparent);
        cursor: pointer;
    }
}
</style>
