<script setup lang="ts">
import { ref, Ref, computed } from "vue";
import "./styles.css"; // Tailwind Stuff
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { usePosNoise } from "./scripts/usePosNoise";
import { Button } from "primevue";
import Splitter from "primevue/splitter";
import SplitterPanel from "primevue/splitterpanel";
import InputText from "primevue/inputtext";
import ToggleSwitch from "primevue/toggleswitch";
import Select from "primevue/select";
import Menubar from "primevue/menubar";
import Menu from "primevue/menu";
import Dropdown from "primevue/dropdown";
import InputNumber from "primevue/inputnumber";

import "primeicons/primeicons.css";

//  <-- === WorkingFile Interface === -->
interface FileStatus {
    old_file_name: string;
    new_file_name: string;
}

//  <-- === Task Type. Contains all Sub Tasks within === -->
type Task =
    | { CustomText: { text: string; at_start: boolean; active: boolean } }
    | {
          FindAndReplace: {
              find_text: string;
              replace_text: string;
              active: boolean;
          };
      }
    | { ClearAll: { active: boolean } }
    | { ChangeCase: { case_choice: number; active: boolean } }
    | {
          NumSequence: {
              start_num: number;
              num_padding: number;
              at_start: boolean;
              separator: string;
              active: boolean;
          };
      }
    | {
          Date: {
              year: boolean;
              month: boolean;
              day: boolean;
              year_4: boolean;
              separator: string;
              active: boolean;
          };
      }
    | {
          Time: {
              hour_24: boolean;
              separator: string;
              active: boolean;
          };
      };

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
    task: Task
): task is {
    CustomText: { text: string; at_start: boolean; active: boolean };
} => {
    return "CustomText" in task;
};

const isFindAndReplaceTask = (
    task: Task
): task is {
    FindAndReplace: {
        find_text: string;
        replace_text: string;
        active: boolean;
    };
} => {
    return "FindAndReplace" in task;
};

const isClearAllTask = (
    task: Task
): task is {
    ClearAll: { active: boolean };
} => {
    return "ClearAll" in task;
};

const isChangeCaseTask = (
    task: Task
): task is {
    ChangeCase: { case_choice: number; active: boolean };
} => {
    return "ChangeCase" in task;
};

const isNumSequenceTask = (
    task: Task
): task is {
    NumSequence: { start_num: number; num_padding: number; at_start: boolean; separator: string; active: boolean };
} => {
    return "NumSequence" in task;
};

const isDateTask = (
    task: Task
): task is {
    Date: {
        year: boolean;
        month: boolean;
        day: boolean;
        year_4: boolean;
        separator: string;
        active: boolean;
    };
} => {
    return "Date" in task;
};

const isTimeTask = (
    task: Task
): task is {
    Time: {
        hour_24: boolean;
        ampm: boolean;
        separator: string;
        active: boolean;
    };
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
            NumSequence: { start_num: 0, num_padding: 4, at_start: true, separator: "_", active: true },
        },
    });
    user_update_tasks();
};

const addDate = () => {
    taskList.value.push({
        id: taskIdCounter++,
        task: {
            Date: {
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
const sphere1 = usePosNoise({ size: 1.2, speed: 0.0005, amplitude: 175 });
const sphere2 = usePosNoise({ size: 0.8, speed: 0.0003, amplitude: 100 });
const sphere3 = usePosNoise({ size: 1.5, speed: 0.0004, amplitude: 175 });

const items = ref([
    {
        label: "Custom Text",
        icon: "pi pi-pencil",
        items: [
            {
                label: "Custom Text",
                icon: "pi pi-arrow-circle-left",
                command: () => addCustomText(),
            },
            {
                label: "Number Sequence",
                icon: "pi pi-sort-numeric-down",
                command: () => addNumSequence(),
            },
            {
                label: "Date",
                icon: "pi pi-calendar",
                command: () => addDate(),
            },
            {
                label: "Time",
                icon: "pi pi-clock",
                command: () => addTime(),
            },
        ],
    },
    {
        label: "Effects",
        icon: "pi pi-cog",
        items: [
            {
                label: "Find & Replace",
                icon: "pi pi-search",
                command: () => addFindReplace(),
            },
            {
                label: "Clear All",
                icon: "pi pi-eraser",
                command: () => addClearAll(),
            },
            {
                label: "Change Case",
                icon: "pi pi-sort-alpha-down",
                command: () => addChangeCase(),
            },
        ],
    },
    {
        label: "Templates",
        icon: "pi pi-bookmark",
        items: [
            {
                label: "Photo Rename",
                icon: "pi pi-image",
                command: () => applyTemplate("photo"),
            },
            {
                label: "Document Cleanup",
                icon: "pi pi-file-pdf",
                command: () => applyTemplate("document"),
            },
            {
                label: "Music Library",
                icon: "pi pi-volume-up",
                command: () => applyTemplate("music"),
            },
            { separator: true },
            {
                label: "Custom Template",
                icon: "pi pi-plus",
                command: () => createTemplate(),
            },
        ],
    },
]);

// Template functions - implement these as needed
const applyTemplate = (templateType) => {
    console.log(`Apply ${templateType} template`);
    // TODO: Implement template functionality
};

const createTemplate = () => {
    console.log("Create custom template");
    // TODO: Implement custom template creation
};

const showHelp = () => {
    console.log("Show help");
    // TODO: Implement help functionality
};

const showSettings = () => {
    console.log("Show settings");
    // TODO: Implement settings functionality
};

const exportConfig = () => {
    console.log("Export configuration");
    // TODO: Implement config export functionality
};
</script>

<template>
    <body class="flex flex-col box-border w-screen h-screen m-0 p-0 overflow-hidden z-0">
        <!-- === Simple Menubar === -->
        <div class="z-50">
            <div class="card border-none rounded-none shadow-none">
                <Menubar :model="items">
                    <template #start>
                        <div class="flex items-center gap-2">
                            <Button icon="pi pi-file" label="Open Files" @click="open_files" class="p-button-outlined p-button-secondary" />
                            <Button icon="pi pi-folder-open" label="Open Folders" @click="open_files" class="p-button-outlined p-button-secondary" />
                            <div class="border-l border-gray-300 h-8 mx-3"></div>
                        </div>
                    </template>

                    <template #end>
                        <div class="flex items-center gap-2">
                            <Button type="button" icon="pi pi-bookmark" @click="showTemplates" v-tooltip="'Templates'" class="p-button-outlined p-button-secondary p-button-rounded" />
                            <Button type="button" icon="pi pi-cog" @click="showSettings" v-tooltip="'Settings'" class="p-button-outlined p-button-secondary p-button-rounded" />
                            <Button icon="pi pi-check" label="Batch Rename" @click="rename_files" class="p-button-success" />
                        </div>
                    </template>
                </Menubar>
            </div>
        </div>

        <!-- === Master Splitter Panel === -->
        <Splitter style="flex: 1; overflow: hidden; background-color: transparent; z-index: 40">
            <!-- === Left Splitter Panel === -->
            <SplitterPanel class="flex flex-col gap-0 m-4 rounded-2xl p-4 border-2 border-white/30 bg-white/20 backdrop-blur-lg shadow-lg z-50">
                <!-- === File Buttons === -->
                <div id="file_buttons" class="flex flex-row items-center justify-start mb-4">
                    <!-- <Button class="mr-4" label="Open A Folder" severity="primary" @click="open_folder" icon="pi pi-folder-open" /> -->
                    <!-- <Button class="reg-button" label="Open Files" @click="open_files" icon="pi pi-file" />
                    <Button class="reg-button" label="Clear All Files" severity="danger" @click="clearFiles" icon="pi pi-trash" /> -->
                    <Select v-model="sortChoice" :options="metadata" optionLabel="name" placeholder="Sort By" optionValue="code" class="w-full md:w-56" />
                </div>

                <!-- <hr class="border-1 border-white/30 my-4" /> -->

                <Transition mode="out-in">
                    <!-- === No Files Selected === -->
                    <div v-if="!numFileStatusItems" class="flex-1 justify-items-center flex-row ml-4 mb-2">
                        <span class="font-thin text-xs font-mono text-center text-black/50"> no files selected </span>
                    </div>

                    <!-- === File Table === -->
                    <div v-else id="table-container" class="flex-1 flex flex-col mb-2 min-h-0 text-sm bg-white/50 rounded-sm border-1 border-black/10">
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
                <TransitionGroup tag="div" name="ttasks" class="relative">
                    <div v-for="(item, index) in taskList" :key="item.id" class="ttasks-item mx-4 my-2">
                        <!-- === CustomText Task === -->
                        <template v-if="isCustomTextTask(item.task)">
                            <div class="flex flex-col gap-2 border-2 border-white/30 bg-white/40 rounded-lg p-3 backdrop-blur-lg shadow-lg">
                                <!-- === Title and Description === -->
                                <div class="flex flex-row items-center justify-between mb-1">
                                    <div class="flex flex-row items-center gap-2">
                                        <h4 class="text-sm font-semibold text-gray-800 m-0">Custom Text</h4>
                                        <p class="text-xs text-gray-500 m-0">Add text to file names</p>
                                    </div>
                                    <!-- === Close Button === -->
                                    <div class="flex items-center" @click="deleteSelectedTask(index)">
                                        <i class="pi pi-times hover:cursor-pointer text-sm text-gray-600 hover:text-red-500 transition-colors" style="font-size: 0.9rem"></i>
                                    </div>
                                </div>

                                <!-- === Main Controls === -->
                                <div class="flex flex-row gap-3 items-center">
                                    <!-- === Drag Buttons === -->
                                    <div class="flex flex-col items-center ml-2 mr-1 gap-1">
                                        <i class="pi pi-angle-up hover:cursor-pointer text-xs" :class="{ 'opacity-30': index === 0 }" @click="moveSelectedTaskUp(index)"></i>
                                        <i
                                            class="pi pi-angle-down hover:cursor-pointer text-xs"
                                            :class="{
                                                'opacity-30': index === taskList.length - 1,
                                            }"
                                            @click="moveSelectedTaskDown(index)"
                                        ></i>
                                    </div>

                                    <!-- === Prefix Text === -->
                                    <InputText placeholder="Custom Text" fluid size="small" :id="`input-text-${index}`" v-model="item.task.CustomText.text" variant="filled" @input="user_update_tasks" class="flex-1" />

                                    <!-- === Position Toggle === -->
                                    <div class="flex items-center gap-2 whitespace-nowrap">
                                        <label :for="`at-start-${index}`" class="text-xs text-gray-600">At Start</label>
                                        <ToggleSwitch v-model="item.task.CustomText.at_start" :inputId="`at-start-${index}`" :name="`at-start-${index}`" binary @change="user_update_tasks" />
                                    </div>

                                    <!-- === Active Toggle === -->
                                    <div class="flex items-center gap-2 whitespace-nowrap">
                                        <label :for="`active-${index}`" class="text-xs text-gray-600">Active</label>
                                        <ToggleSwitch v-model="item.task.CustomText.active" :inputId="`active-${index}`" :name="`active-checkbox${index}`" binary @change="user_update_tasks" />
                                    </div>
                                </div>
                            </div>
                        </template>

                        <!-- === Find & Replace Task === -->
                        <template v-else-if="isFindAndReplaceTask(item.task)">
                            <div class="flex flex-col gap-2 border-2 border-white/30 bg-white/40 rounded-lg p-3 backdrop-blur-lg shadow-lg">
                                <!-- === Title and Description === -->
                                <div class="flex flex-row items-center justify-between mb-1">
                                    <div class="flex flex-row items-center gap-2">
                                        <h4 class="text-sm font-semibold text-gray-800 m-0">Find & Replace</h4>
                                        <p class="text-xs text-gray-500 m-0">Replace text in file names</p>
                                    </div>
                                    <!-- === Close Button === -->
                                    <div class="flex items-center" @click="deleteSelectedTask(index)">
                                        <i class="pi pi-times hover:cursor-pointer text-sm text-gray-600 hover:text-red-500 transition-colors" style="font-size: 0.9rem"></i>
                                    </div>
                                </div>

                                <!-- === Main Controls === -->
                                <div class="flex flex-row gap-3 items-center">
                                    <!-- === Drag Buttons === -->
                                    <div class="flex flex-col items-center ml-2 mr-1 gap-1">
                                        <i class="pi pi-angle-up hover:cursor-pointer text-xs" :class="{ 'opacity-30': index === 0 }" @click="moveSelectedTaskUp(index)"></i>
                                        <i
                                            class="pi pi-angle-down hover:cursor-pointer text-xs"
                                            :class="{
                                                'opacity-30': index === taskList.length - 1,
                                            }"
                                            @click="moveSelectedTaskDown(index)"
                                        ></i>
                                    </div>

                                    <!-- === Find Text Field === -->
                                    <InputText :id="`find-text-${index}`" placeholder="Find" fluid size="small" v-model="item.task.FindAndReplace.find_text" variant="filled" @input="user_update_tasks" class="flex-1" />

                                    <!-- === Replace Text Field === -->
                                    <InputText :id="`replace-text-${index}`" placeholder="Replace" fluid size="small" v-model="item.task.FindAndReplace.replace_text" variant="filled" @input="user_update_tasks" class="flex-1" />

                                    <!-- === Active Toggle === -->
                                    <div class="flex items-center gap-2 whitespace-nowrap">
                                        <label :for="`active-${index}`" class="text-xs text-gray-600">Active</label>
                                        <ToggleSwitch v-model="item.task.FindAndReplace.active" :inputId="`active-${index}`" :name="`find-replace-active-${index}`" binary @change="user_update_tasks" />
                                    </div>
                                </div>
                            </div>
                        </template>

                        <!-- === ClearAll Task === -->
                        <template v-else-if="isClearAllTask(item.task)">
                            <div class="flex flex-col gap-2 border-2 border-white/30 bg-white/40 rounded-lg p-3 backdrop-blur-lg shadow-lg">
                                <!-- === Title and Description === -->
                                <div class="flex flex-row items-center justify-between mb-1">
                                    <div class="flex flex-row items-center gap-2">
                                        <h4 class="text-sm font-semibold text-gray-800 m-0">Clear All</h4>
                                        <p class="text-xs text-gray-500 m-0">Remove all text from file names</p>
                                    </div>
                                    <!-- === Close Button === -->
                                    <div class="flex items-center" @click="deleteSelectedTask(index)">
                                        <i class="pi pi-times hover:cursor-pointer text-sm text-gray-600 hover:text-red-500 transition-colors" style="font-size: 0.9rem"></i>
                                    </div>
                                </div>

                                <!-- === Main Controls === -->
                                <div class="flex flex-row gap-3 items-center">
                                    <!-- === Drag Buttons === -->
                                    <div class="flex flex-col items-center ml-2 mr-1 gap-1">
                                        <i class="pi pi-angle-up hover:cursor-pointer text-xs" :class="{ 'opacity-30': index === 0 }" @click="moveSelectedTaskUp(index)"></i>
                                        <i
                                            class="pi pi-angle-down hover:cursor-pointer text-xs"
                                            :class="{
                                                'opacity-30': index === taskList.length - 1,
                                            }"
                                            @click="moveSelectedTaskDown(index)"
                                        ></i>
                                    </div>

                                    <!-- === Description Text === -->
                                    <div class="flex-1 flex items-center">
                                        <p class="text-sm text-gray-700 m-0">This will remove all existing text from the file names</p>
                                    </div>

                                    <!-- === Active Toggle === -->
                                    <div class="flex items-center gap-2 whitespace-nowrap">
                                        <label :for="`active-${index}`" class="text-xs text-gray-600">Active</label>
                                        <ToggleSwitch v-model="item.task.ClearAll.active" :inputId="`active-${index}`" :name="`clear-all-active-${index}`" binary @change="user_update_tasks" />
                                    </div>
                                </div>
                            </div>
                        </template>

                        <!-- === ChangeCase Task === -->
                        <template v-else-if="isChangeCaseTask(item.task)">
                            <div class="flex flex-col gap-2 border-2 border-white/30 bg-white/40 rounded-lg p-3 backdrop-blur-lg shadow-lg">
                                <!-- === Title and Description === -->
                                <div class="flex flex-row items-center justify-between mb-1">
                                    <div class="flex flex-row items-center gap-2">
                                        <h4 class="text-sm font-semibold text-gray-800 m-0">Change Case</h4>
                                        <p class="text-xs text-gray-500 m-0">Convert text case in file names</p>
                                    </div>
                                    <!-- === Close Button === -->
                                    <div class="flex items-center" @click="deleteSelectedTask(index)">
                                        <i class="pi pi-times hover:cursor-pointer text-sm text-gray-600 hover:text-red-500 transition-colors" style="font-size: 0.9rem"></i>
                                    </div>
                                </div>

                                <!-- === Main Controls === -->
                                <div class="flex flex-row gap-3 items-center">
                                    <!-- === Drag Buttons === -->
                                    <div class="flex flex-col items-center ml-2 mr-1 gap-1">
                                        <i class="pi pi-angle-up hover:cursor-pointer text-xs" :class="{ 'opacity-30': index === 0 }" @click="moveSelectedTaskUp(index)"></i>
                                        <i
                                            class="pi pi-angle-down hover:cursor-pointer text-xs"
                                            :class="{
                                                'opacity-30': index === taskList.length - 1,
                                            }"
                                            @click="moveSelectedTaskDown(index)"
                                        ></i>
                                    </div>

                                    <!-- === Case Choice Dropdown === -->
                                    <Dropdown
                                        v-model="item.task.ChangeCase.case_choice"
                                        :options="[
                                            { label: 'lowercase', value: 0 },
                                            { label: 'UPPERCASE', value: 1 },
                                            { label: 'Title Case', value: 2 },
                                            { label: 'camelCase', value: 3 },
                                        ]"
                                        optionLabel="label"
                                        optionValue="value"
                                        placeholder="Select case type"
                                        class="flex-1"
                                        @change="user_update_tasks"
                                    />

                                    <!-- === Active Toggle === -->
                                    <div class="flex items-center gap-2 whitespace-nowrap">
                                        <label :for="`active-${index}`" class="text-xs text-gray-600">Active</label>
                                        <ToggleSwitch v-model="item.task.ChangeCase.active" :inputId="`active-${index}`" :name="`change-case-active-${index}`" binary @change="user_update_tasks" />
                                    </div>
                                </div>
                            </div>
                        </template>

                        <!-- === NumSequence Task === -->
                        <template v-else-if="isNumSequenceTask(item.task)">
                            <div class="flex flex-col gap-2 border-2 border-white/30 bg-white/40 rounded-lg p-3 backdrop-blur-lg shadow-lg">
                                <!-- === Title and Description === -->
                                <div class="flex flex-row items-center justify-between mb-1">
                                    <div class="flex flex-row items-center gap-2">
                                        <h4 class="text-sm font-semibold text-gray-800 m-0">Number Sequence</h4>
                                        <p class="text-xs text-gray-500 m-0">Add sequential numbers to file names</p>
                                    </div>
                                    <!-- === Close Button === -->
                                    <div class="flex items-center" @click="deleteSelectedTask(index)">
                                        <i class="pi pi-times hover:cursor-pointer text-sm text-gray-600 hover:text-red-500 transition-colors" style="font-size: 0.9rem"></i>
                                    </div>
                                </div>

                                <!-- === Main Controls === -->
                                <div class="flex flex-row gap-3 items-center">
                                    <!-- === Drag Buttons === -->
                                    <div class="flex flex-col items-center ml-2 mr-1 gap-1">
                                        <i class="pi pi-angle-up hover:cursor-pointer text-xs" :class="{ 'opacity-30': index === 0 }" @click="moveSelectedTaskUp(index)"></i>
                                        <i
                                            class="pi pi-angle-down hover:cursor-pointer text-xs"
                                            :class="{
                                                'opacity-30': index === taskList.length - 1,
                                            }"
                                            @click="moveSelectedTaskDown(index)"
                                        ></i>
                                    </div>

                                    <!-- === Start Number === -->
                                    <InputNumber v-model="item.task.NumSequence.start_num" :id="`start-num-${index}`" placeholder="Start" size="small" class="w-20" @input="user_update_tasks" />

                                    <!-- === Padding === -->
                                    <InputNumber v-model="item.task.NumSequence.num_padding" :id="`padding-${index}`" placeholder="Padding" size="small" class="w-20" :min="1" :max="10" @input="user_update_tasks" />

                                    <!-- === Separator === -->
                                    <InputText v-model="item.task.NumSequence.separator" :id="`separator-${index}`" placeholder="Sep" size="small" class="w-12" @input="user_update_tasks" />

                                    <!-- === Position Toggle === -->
                                    <div class="flex items-center gap-2 whitespace-nowrap">
                                        <label :for="`at-start-${index}`" class="text-xs text-gray-600">At Start</label>
                                        <ToggleSwitch v-model="item.task.NumSequence.at_start" :inputId="`at-start-${index}`" :name="`at-start-${index}`" binary @change="user_update_tasks" />
                                    </div>

                                    <!-- === Active Toggle === -->
                                    <div class="flex items-center gap-2 whitespace-nowrap">
                                        <label :for="`active-${index}`" class="text-xs text-gray-600">Active</label>
                                        <ToggleSwitch v-model="item.task.NumSequence.active" :inputId="`active-${index}`" :name="`num-sequence-active-${index}`" binary @change="user_update_tasks" />
                                    </div>
                                </div>
                            </div>
                        </template>

                        <!-- === Date Task === -->
                        <template v-else-if="isDateTask(item.task)">
                            <div class="flex flex-col gap-2 border-2 border-white/30 bg-white/40 rounded-lg p-3 backdrop-blur-lg shadow-lg">
                                <!-- === Title and Description === -->
                                <div class="flex flex-row items-center justify-between mb-1">
                                    <div class="flex flex-row items-center gap-2">
                                        <h4 class="text-sm font-semibold text-gray-800 m-0">Date</h4>
                                        <p class="text-xs text-gray-500 m-0">Add current date to file names</p>
                                    </div>
                                    <!-- === Close Button === -->
                                    <div class="flex items-center" @click="deleteSelectedTask(index)">
                                        <i class="pi pi-times hover:cursor-pointer text-sm text-gray-600 hover:text-red-500 transition-colors" style="font-size: 0.9rem"></i>
                                    </div>
                                </div>

                                <!-- === Main Controls === -->
                                <div class="flex flex-row gap-3 items-center">
                                    <!-- === Drag Buttons === -->
                                    <div class="flex flex-col items-center ml-2 mr-1 gap-1">
                                        <i class="pi pi-angle-up hover:cursor-pointer text-xs" :class="{ 'opacity-30': index === 0 }" @click="moveSelectedTaskUp(index)"></i>
                                        <i
                                            class="pi pi-angle-down hover:cursor-pointer text-xs"
                                            :class="{
                                                'opacity-30': index === taskList.length - 1,
                                            }"
                                            @click="moveSelectedTaskDown(index)"
                                        ></i>
                                    </div>

                                    <!-- === Date Components === -->
                                    <div class="flex items-center gap-2 whitespace-nowrap">
                                        <label :for="`year-${index}`" class="text-xs text-gray-600">Year</label>
                                        <ToggleSwitch v-model="item.task.Date.year" :inputId="`year-${index}`" :name="`year-${index}`" binary @change="user_update_tasks" />
                                    </div>

                                    <div class="flex items-center gap-2 whitespace-nowrap">
                                        <label :for="`month-${index}`" class="text-xs text-gray-600">Month</label>
                                        <ToggleSwitch v-model="item.task.Date.month" :inputId="`month-${index}`" :name="`month-${index}`" binary @change="user_update_tasks" />
                                    </div>

                                    <div class="flex items-center gap-2 whitespace-nowrap">
                                        <label :for="`day-${index}`" class="text-xs text-gray-600">Day</label>
                                        <ToggleSwitch v-model="item.task.Date.day" :inputId="`day-${index}`" :name="`day-${index}`" binary @change="user_update_tasks" />
                                    </div>

                                    <div class="flex items-center gap-2 whitespace-nowrap">
                                        <label :for="`year4-${index}`" class="text-xs text-gray-600">4-Digit</label>
                                        <ToggleSwitch v-model="item.task.Date.year_4" :inputId="`year4-${index}`" :name="`year4-${index}`" binary @change="user_update_tasks" />
                                    </div>

                                    <!-- === Separator === -->
                                    <InputText v-model="item.task.Date.separator" :id="`separator-${index}`" placeholder="Sep" size="small" class="w-12" @input="user_update_tasks" />

                                    <!-- === Active Toggle === -->
                                    <div class="flex items-center gap-2 whitespace-nowrap">
                                        <label :for="`active-${index}`" class="text-xs text-gray-600">Active</label>
                                        <ToggleSwitch v-model="item.task.Date.active" :inputId="`active-${index}`" :name="`date-active-${index}`" binary @change="user_update_tasks" />
                                    </div>
                                </div>
                            </div>
                        </template>

                        <!-- === Time Task === -->
                        <template v-else-if="isTimeTask(item.task)">
                            <div class="flex flex-col gap-2 border-2 border-white/30 bg-white/40 rounded-lg p-3 backdrop-blur-lg shadow-lg">
                                <!-- === Title and Description === -->
                                <div class="flex flex-row items-center justify-between mb-1">
                                    <div class="flex flex-row items-center gap-2">
                                        <h4 class="text-sm font-semibold text-gray-800 m-0">Time</h4>
                                        <p class="text-xs text-gray-500 m-0">Add current time to file names</p>
                                    </div>
                                    <!-- === Close Button === -->
                                    <div class="flex items-center" @click="deleteSelectedTask(index)">
                                        <i class="pi pi-times hover:cursor-pointer text-sm text-gray-600 hover:text-red-500 transition-colors" style="font-size: 0.9rem"></i>
                                    </div>
                                </div>

                                <!-- === Main Controls === -->
                                <div class="flex flex-row gap-3 items-center">
                                    <!-- === Drag Buttons === -->
                                    <div class="flex flex-col items-center ml-2 mr-1 gap-1">
                                        <i class="pi pi-angle-up hover:cursor-pointer text-xs" :class="{ 'opacity-30': index === 0 }" @click="moveSelectedTaskUp(index)"></i>
                                        <i
                                            class="pi pi-angle-down hover:cursor-pointer text-xs"
                                            :class="{
                                                'opacity-30': index === taskList.length - 1,
                                            }"
                                            @click="moveSelectedTaskDown(index)"
                                        ></i>
                                    </div>

                                    <!-- === Time Format === -->
                                    <div class="flex items-center gap-2 whitespace-nowrap">
                                        <label :for="`hour24-${index}`" class="text-xs text-gray-600">24 Hour</label>
                                        <ToggleSwitch v-model="item.task.Time.hour_24" :inputId="`hour24-${index}`" :name="`hour24-${index}`" binary @change="user_update_tasks" />
                                    </div>

                                    <!-- === Separator === -->
                                    <InputText v-model="item.task.Time.separator" :id="`separator-${index}`" placeholder="Sep" size="small" class="w-12" @input="user_update_tasks" />

                                    <!-- === Active Toggle === -->
                                    <div class="flex items-center gap-2 whitespace-nowrap">
                                        <label :for="`active-${index}`" class="text-xs text-gray-600">Active</label>
                                        <ToggleSwitch v-model="item.task.Time.active" :inputId="`active-${index}`" :name="`time-active-${index}`" binary @change="user_update_tasks" />
                                    </div>
                                </div>
                            </div>
                        </template>
                    </div>
                </TransitionGroup>
            </SplitterPanel>
        </Splitter>
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
