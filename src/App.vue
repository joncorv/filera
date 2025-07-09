<script setup lang="ts">
import { ref, Ref, computed } from "vue";
import './styles.css'; // Tailwind Stuff
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import { usePosNoise } from "./scripts/usePosNoise"
import Button from "primevue";
import Splitter from 'primevue/splitter';
import SplitterPanel from 'primevue/splitterpanel';
import InputText from 'primevue/inputtext';
import ToggleSwitch from 'primevue/toggleswitch';
import Select from 'primevue/select';

import 'primeicons/primeicons.css'


//  <-- === WorkingFile Interface === -->
interface WorkingFile {
    path: string;
    old_file_name: string;
    new_file_name: string;
    active: boolean;
}

//  <-- === Task Type. Contains all Sub Tasks within === -->
type Task =
    | { Prefix: { text: string; active: boolean; } }
    | { Postfix: { text: string; active: boolean; } }
    | { FindAndReplace: { find_text: string; replace_text: string; active: boolean; } };


const sortChoice = ref();
const metadata = ref([
    { name: 'Name', code: 'name' },
    { name: 'Date Created', code: 'dateCreated' },
    { name: 'Date Modified', code: 'dateModified' },
    { name: 'Type', code: 'type' },
    { name: 'Size', code: 'size' }
]);

//  <-- === Add unique ID to each task. Needed for proper animation in the DOM === -->
interface TaskWithId {
    id: number;
    task: Task;
}

//  <-- === Type Guards === -->
const isPrefixTask = (task: Task): task is { Prefix: { text: string; active: boolean; } } => {
    return 'Prefix' in task;
}

const isFindAndReplaceTask = (task: Task): task is { FindAndReplace: { find_text: string; replace_text: string; active: boolean; } } => {
    return 'FindAndReplace' in task;
}

//  <-- === Create an array of TaskWithId === -->
const taskList = ref<TaskWithId[]>([]);

//  <-- === Counter for generating unique IDs === -->
let taskIdCounter = 0;

const addPrefix = () => {
    taskList.value.push({
        id: taskIdCounter++,
        task: {
            Prefix: {
                text: '',
                active: true
            }
        }
    });
    update_files();
};

const addFindReplace = () => {
    taskList.value.push({
        id: taskIdCounter++,
        task: {
            FindAndReplace: {
                find_text: '',
                replace_text: '',
                active: true
            }
        }
    });
    update_files();
};

let dirtyFilesSelection: string[] | null = null;
const workingFileReturn: Ref<WorkingFile[]> = ref([]);
const number_of_working_files = computed(() => workingFileReturn.value.length);


//  <-- === Opens Files System Dialog === -->
async function open_files() {

    const selectedFiles = await open({
        multiple: true,
        director: true,
    })

    if (selectedFiles && dirtyFilesSelection) {
        dirtyFilesSelection = dirtyFilesSelection.concat(selectedFiles);
        workingFileReturn.value = await invoke("open_files", {
            fileNames: dirtyFilesSelection,
            taskList: taskList.value.map(t => t.task)
        });

    } else if (selectedFiles && !dirtyFilesSelection) {
        dirtyFilesSelection = selectedFiles;
        workingFileReturn.value = await invoke("open_files", {
            fileNames: dirtyFilesSelection,
            taskList: taskList.value.map(t => t.task)
        });

    } else if (!selectedFiles && dirtyFilesSelection) {
        console.log("There was a previous selection, but no current selection. No updates to make.")

    } else {
        console.log("There was no previous selection or current selection. No Updates to make.");
    }
}

//  <-- === Update interface to show latest files === -->
async function update_files() {
    if (dirtyFilesSelection) {
        workingFileReturn.value = await invoke("open_files", {
            fileNames: dirtyFilesSelection,
            taskList: taskList.value.map(t => t.task)
        });
    }
}

//  <-- === Rename Files on the Rust Backend === -->
async function rename_files() {
    if (dirtyFilesSelection) {
        let renameSuccess: string = await invoke("rename_files", { fileNames: dirtyFilesSelection, taskList: taskList.value.map(t => t.task) });

        if (renameSuccess.startsWith('Rename Error', 0)) {
            console.log(renameSuccess);
        }
        else {
            clearSelection();
        }
    };
}

//  <-- === All Task Functions === -->
function clearSelection() {
    dirtyFilesSelection = null;
    workingFileReturn.value = [];
}
async function clearTasks() {
    const loopLength = taskList.value.length;
    for (let i = loopLength - 1; i >= 0; i--) {
        taskList.value.pop();
        if (i > 0) { // Don't wait after the last iteration
            await new Promise(resolve => setTimeout(resolve, 40));
        }
    }
    update_files();
}
function deleteSelectedTask(index: number) {
    taskList.value.splice(index, 1);
    update_files();
}
function moveSelectedTaskUp(index: number) {
    if (index > 0) {
        const [item] = taskList.value.splice(index, 1);
        taskList.value.splice(index - 1, 0, item);
        update_files();
    }
}
function moveSelectedTaskDown(index: number) {
    if (index < taskList.value.length - 1) {
        const [item] = taskList.value.splice(index, 1);
        taskList.value.splice(index + 1, 0, item);
        update_files();
    }
}

//  <-- === usePosNoise function from external file. === -->
const sphere1 = usePosNoise({ size: 1.2, speed: 0.0005, amplitude: 175 })
const sphere2 = usePosNoise({ size: 0.8, speed: 0.0003, amplitude: 100 })
const sphere3 = usePosNoise({ size: 1.5, speed: 0.0004, amplitude: 175 })

</script>

<template>

    <body class="flex flex-col box-border w-screen h-screen m-0 p-0 overflow-hidden z-0">

        <!-- === Green Circles === -->
        <div id="circles-random-positions" class="absolute h-screen w-screen overflow-hidden">
            <div id="circle_top_left" class="absolute w-30 h-30 rounded-full bg-green-400"
                :style="{ transform: `translate(${160 + sphere1.x}px, ${sphere1.y}px)` }" />
            <div id="circle_right" class="absolute w-40 h-40 rounded-full bg-green-400"
                :style="{ transform: `translate(${1000 + sphere2.x}px, ${sphere2.y}px)` }" />
            <div id="circle_btm" class="absolute w-60 h-60 rounded-full bg-green-400"
                :style="{ transform: `translate(${400 + sphere3.x}px, ${520 + sphere3.y}px)` }" />
        </div>

        <!-- === Master Splitter Panel === -->
        <Splitter style="flex: 1; overflow: hidden; background-color: transparent; ; z-index: 40;">

            <!-- === Left Splitter Panel === -->
            <SplitterPanel
                class="flex flex-col gap-0 m-4 rounded-2xl p-4 border-2 border-white/30 bg-white/20 backdrop-blur-lg shadow-lg z-50">


                <!-- === File Buttons === -->
                <div id="file_buttons" class="flex flex-row items-center justify-start mb-4">
                    <!-- <Button class="mr-4" label="Open A Folder" severity="primary" @click="open_folder" icon="pi pi-folder-open" /> -->
                    <Button class="reg-button" unstyled label="Open Files" @click="open_files" icon="pi pi-file" />
                    <Button class="reg-button" unstyled label="Clear All Files" severity="danger"
                        @click="clearSelection" icon="pi pi-trash" />
                    <Select v-model="sortChoice" :options="metadata" optionLabel="name" placeholder="Sort By"
                        optionValue="code" class="w-full md:w-56" />


                </div>

                <!-- <hr class="border-1 border-white/30 my-4" /> -->

                <Transition mode="out-in">
                    <!-- === No Files Selected === -->
                    <div v-if="!number_of_working_files" class="flex-1 justify-items-center flex-row ml-4 mb-2">
                        <span class="font-thin text-xs font-mono text-center text-black/50"> no files selected </span>
                    </div>

                    <!-- === File Table === -->
                    <div v-else id="table-container"
                        class="flex-1 flex flex-col mb-2 min-h-0 text-sm bg-white/50 rounded-sm border-1 border-black/10">
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
                                    <tr v-for="(item, index) in workingFileReturn" :key="index">
                                        <td class="px-4 py-2 border-b border-black/10">{{ item.old_file_name }}</td>
                                        <td class="px-4 py-2 border-b border-black/10">{{ item.new_file_name }}</td>
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
                        <span class="ml-2">{{ number_of_working_files }}</span>
                    </Transition>
                </div>

            </SplitterPanel>

            <!-- === Right Splitter Panel === -->
            <SplitterPanel class="flex flex-col flex-1">

                <div class="flex flex-row mt-4 mb-2 ml-4">
                    <Button class="reg-button" unstyled severity="primary" @click="addPrefix" label="Add Prefix"
                        icon="pi pi-arrow-circle-left" />
                    <Button class="reg-button" unstyled severity="primary" @click="addFindReplace"
                        label="Add Find & Replace" icon="pi pi-search" />
                    <Button class="reg-button shadow-xl" unstyled severity="danger" @click="clearTasks"
                        label="Clear All Tasks" icon="pi pi-trash" />
                </div>

                <TransitionGroup tag="div" name="ttasks" class="relative">
                    <div v-for="(item, index) in taskList" :key="item.id" class="ttasks-item mx-4 my-2">

                        <!-- === Prefix Task === -->
                        <template v-if="isPrefixTask(item.task)">
                            <div
                                class="flex flex-row gap-4 border-2 border-white/30 bg-white/40 rounded-lg p-2 backdrop-blur-lg shadow-lg">

                                <!-- === Drag Buttons === -->
                                <div class="flex flex-col items-center ml-2 mr-1 gap-1">
                                    <i class="pi pi-angle-up hover:cursor-pointer text-xs"
                                        :class="{ 'opacity-30': index === 0 }" @click="moveSelectedTaskUp(index)"></i>
                                    <i class="pi pi-angle-down hover:cursor-pointer text-xs"
                                        :class="{ 'opacity-30': index === taskList.length - 1 }"
                                        @click="moveSelectedTaskDown(index)"></i>
                                </div>

                                <!-- === Prefix Text === -->
                                <InputText placeholder="Prefix Text" fluid size="small" :id="`input-text-${index}`"
                                    v-model="item.task.Prefix.text" variant="filled" @input="update_files" />

                                <!-- === Checkbox === -->
                                <div class="flex items-center gap-2">
                                    <ToggleSwitch v-model="item.task.Prefix.active" :inputId="`checkbox-${index}`"
                                        :name="`active-checkbox${index}`" binary @change="update_files" />
                                </div>

                                <!-- === Delete Button === -->
                                <div class="flex items-center ml-0 mr-2" @click="deleteSelectedTask(index)">
                                    <i class="pi pi-trash hover:cursor-pointer"
                                        style="font-size: 1.1rem; color: red;"></i>
                                </div>

                            </div>
                        </template>

                        <!-- === Find & Replace Task === -->
                        <template v-else-if="isFindAndReplaceTask(item.task)">
                            <div
                                class="flex flex-row gap-4 border-2 border-white/30 bg-white/40 rounded-lg p-2 backdrop-blur-lg shadow-lg">

                                <!-- === Drag Buttons === -->
                                <div class="flex flex-col items-center ml-2 mr-1 gap-1">
                                    <i class="pi pi-angle-up hover:cursor-pointer text-xs"
                                        :class="{ 'opacity-30': index === 0 }" @click="moveSelectedTaskUp(index)"></i>
                                    <i class="pi pi-angle-down hover:cursor-pointer text-xs"
                                        :class="{ 'opacity-30': index === taskList.length - 1 }"
                                        @click="moveSelectedTaskDown(index)"></i>
                                </div>

                                <!-- === Find Text Field === -->
                                <InputText id="in_label" placeholder="Find" fluid size="small"
                                    v-model="item.task.FindAndReplace.find_text" variant="filled"
                                    @input="update_files" />

                                <!-- ===Replace Text Field === -->
                                <InputText id="in_label" placeholder="Replace" fluid size="small"
                                    v-model="item.task.FindAndReplace.replace_text" variant="filled"
                                    @input="update_files" />


                                <!-- === Checkbox === -->
                                <div class="flex items-center gap-2">
                                    <ToggleSwitch v-model="item.task.FindAndReplace.active" :inputId="`active-${index}`"
                                        name="namefindreplaceactive" binary @change="update_files" />
                                </div>

                                <!-- === Delete Button === -->
                                <div class="flex items-center ml-0 mr-2" @click="deleteSelectedTask(index)">
                                    <i class="pi pi-trash hover:cursor-pointer"
                                        style="font-size: 1.1rem; color: red;"></i>
                                </div>

                            </div>
                        </template>
                    </div>
                </TransitionGroup>
            </SplitterPanel>
        </Splitter>

        <!-- === App Footer === -->
        <!-- <div id="footer" class="flex flex-row-reverse m-4 "> -->
        <div id="footer"
            class="flex flex-row-reverse gap-0 m-4 rounded-2xl p-4 border-2 border-white/30 bg-white/20 backdrop-blur-lg shadow-lg z-50 ">

            <!-- === Batch Rename All Files Button === -->
            <Button unstyled icon="pi pi-trash" class="big-button" label="Batch Rename All Files" size="large"
                @click="rename_files" />
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
    --p-datatable-body-cell-border-color: rgba(255, 255, 255, 0.0);
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
    box-shadow: black 0px 0px 0px 0px, rgba(0, 0, 0, 0.05) 0px 4px 6px -1px, rgba(0, 0, 0, 0.1) 0px 2px 4px -1px;
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
    box-shadow: black 0px 0px 0px 0px, rgba(0, 0, 0, 0.05) 0px 4px 6px -1px, rgba(0, 0, 0, 0.1) 0px 2px 4px -1px;
    transition: background-color 0.2s ease;
    backdrop-filter: blur(10px);

    &:hover {
        background-color: color-mix(in oklab, var(--color-green-500) 60%, transparent);
        cursor: pointer;
    }
}
</style>