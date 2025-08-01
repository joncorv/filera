<script setup lang="ts">
import { ref, Ref, computed } from "vue";
import "./styles.css"; // Tailwind Stuff
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { Button } from "primevue";
import Splitter from "primevue/splitter";
import SplitterPanel from "primevue/splitterpanel";
import InputText from "primevue/inputtext";
import ToggleSwitch from "primevue/toggleswitch";
import ToggleButton from "primevue/togglebutton";
import Select from "primevue/select";
// import Menubar from "primevue/menubar";
import Menu from "primevue/menu";
import Dropdown from "primevue/dropdown";
import InputNumber from "primevue/inputnumber";
import IconField from "primevue/iconfield";
import InputIcon from "primevue/inputicon";
import FloatLabel from "primevue/floatlabel";
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
    task: Task,
): task is {
    CustomText: { text: string; at_start: boolean; active: boolean };
} => {
    return "CustomText" in task;
};

const isFindAndReplaceTask = (
    task: Task,
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
    NumSequence: {
        start_num: number;
        num_padding: number;
        at_start: boolean;
        separator: string;
        active: boolean;
    };
} => {
    return "NumSequence" in task;
};

const isDateTask = (
    task: Task,
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
    task: Task,
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
            NumSequence: {
                start_num: 0,
                num_padding: 4,
                at_start: true,
                separator: "_",
                active: true,
            },
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
const numTaskListItems = computed(() => taskList.value.length);

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

// NOTE: this used to be an iterative delete, but it's unncessary
async function clearTasks() {
    taskList.value = [];
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

const taskMenuItems = ref([
    {
        label: "Find & Replace",
        icon: "pi pi-search",
        command: () => addFindReplace(),
    },
    {
        label: "Number Sequence",
        icon: "pi pi-sort-numeric-down",
        command: () => addNumSequence(),
    },
    {
        label: "Custom Text",
        icon: "pi pi-arrow-circle-left",
        command: () => addCustomText(),
    },
    {
        label: "Change Case",
        icon: "pi pi-sort-alpha-down",
        command: () => addChangeCase(),
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
    {
        label: "Clear All",
        icon: "pi pi-eraser",
        command: () => addClearAll(),
    },
]);

const templateMenuItems = ref([
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
]);

const taskMenuToggle = ref();
const taskMenuToggleFunction = (event: any) => {
    taskMenuToggle.value.toggle(event);
};

const templateMenuToggle = ref();
const templateMenuToggleFunction = (event: any) => {
    templateMenuToggle.value.toggle(event);
};

// Template functions - implement these as needed
const applyTemplate = (templateType: string) => {
    console.log(`Apply ${templateType} template`);
    // TODO: Implement template functionality
};

// const showTemplates = () => {
//     console.log("View templates");
//     // TODO: Impliment view templates functionality
// };

const createTemplate = () => {
    console.log("Create custom template");
    // TODO: Implement custom template creation
};

// const showHelp = () => {
//     console.log("Show help");
//     // TODO: Implement help functionality
// };

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
    <body class="flex flex-col box-border w-screen h-screen m-0 p-0 overflow-hidden z-0 bg-white/5">
        <!-- === Global Top Menubar === -->
        <!-- <div -->
        <!--     id="left-menu-buttons" -->
        <!--     class="flex flex-row p-2 border-b-1 border-white/30 items-center justify-between w-full z-50 bg-black" -->
        <!-- > -->
        <!--     <div class="items-center gap-4"> -->
        <!--         <Button size="small" icon="pi pi-file" label="Open Files" @click="open_files" class="mr-2" /> -->
        <!--         <Button size="small" icon="pi pi-folder-open" label="Open Folders" @click="open_files" /> -->
        <!--     </div> -->
        <!---->
        <!--     <div id="right-menu-buttons" class="items-center gap-2"> -->
        <!--         <Button -->
        <!--             type="button" -->
        <!--             size="small" -->
        <!--             icon="pi pi-bookmark" -->
        <!--             variant="outlined" -->
        <!--             @click="showTemplates" -->
        <!--             v-tooltip="'Templates'" -->
        <!--             class="p-button-secondary p-button-rounded mr-2" -->
        <!--         /> -->
        <!--         <Button -->
        <!--             type="button" -->
        <!--             size="small" -->
        <!--             icon="pi pi-cog" -->
        <!--             variant="outlined" -->
        <!--             @click="showSettings" -->
        <!--             v-tooltip="'Settings'" -->
        <!--             class="p-button-secondary p-button-rounded mr-2" -->
        <!--         /> -->
        <!--         <Button size="small" icon="pi pi-check" label="Batch Rename Files" @click="rename_files" /> -->
        <!--     </div> -->
        <!-- </div> -->

        <!-- === Master Splitter Panel === -->
        <Splitter style="flex: 1; overflow: hidden; background-color: transparent; z-index: 40">
            <!-- === Left Splitter Panel === -->
            <SplitterPanel
                class="flex flex-2/3 flex-col gap-0 ml-2 mr-0.5 mt-0 mb-2 border-1 rounded-lg border-white/20 shadow-sm z-50 bg-black/40"
            >
                <!-- === Left SplitterPanel Menubar === -->
                <div id="file_buttons" class="flex flex-row items-center gap-2 justify-start m-2">
                    <!-- <div class="items-center flex flex-row gap-4"> -->
                    <Button
                        size="small"
                        icon="pi pi-file"
                        label="Open Files"
                        @click="open_files"
                        variant="outlined"
                        class="min-w-max"
                    />
                    <Button
                        size="small"
                        icon="pi pi-folder-open"
                        label="Open Folders"
                        variant="outlined"
                        @click="open_files"
                        class="min-w-max"
                    />
                    <!-- </div> -->
                    <!-- === Search Field === -->
                    <IconField class="flex-3/4 w-full">
                        <InputIcon class="pi pi-search" />
                        <InputText v-model="value1" placeholder="Search your files..." size="small" class="w-full" />
                    </IconField>

                    <!-- === Sort Select === -->
                    <Select
                        v-model="sortChoice"
                        :options="metadata"
                        size="small"
                        optionLabel="name"
                        placeholder="Sort By"
                        optionValue="code"
                        class="w-full flex-1/4"
                    />

                    <!-- === Hamburger Select === -->
                    <Button
                        icon="pi pi-replay"
                        class="whitespace-nowrap flex-none"
                        @click="clearFiles"
                        variant="outlined"
                        severity="secondary"
                        size="small"
                    />
                </div>

                <hr class="border-white/30" />

                <Transition mode="out-in">
                    <!-- === No Files Selected === -->
                    <div
                        v-if="!numFileStatusItems"
                        class="flex flex-1 bg-white/3 flex-col justify-center items-center w-full h-full whitespace-nowrap"
                    >
                        <span class="text-center mb-2 text-gray-400">Drag your files here.</span>
                        <span class="text-center text-sm text-gray-600"
                            >NOTE: drag functionality is not implimented,</span
                        >
                        <span class="text-center text-sm text-gray-600">Please use the add buttons above.</span>
                    </div>

                    <!-- === File Table === -->
                    <div v-else id="table-container" class="flex-1/2 flex flex-col mb-2 min-h-0 text-sm">
                        <table class="w-full text-white/90">
                            <thead>
                                <tr>
                                    <th class="px-4 pt-2.5 pb-1.5 text-left border-b border-white/20">Old Name</th>
                                    <th class="px-4 pt-2.5 pb-1.5 text-left border-b border-white/20">New Name</th>
                                </tr>
                            </thead>
                        </table>

                        <div class="flex-1/2 overflow-y-auto min-h-0 text-white/70">
                            <table class="w-full">
                                <tbody>
                                    <tr v-for="(item, index) in fileStatusReturn" :key="index">
                                        <td class="px-4 py-2 border-b border-b-white/10">
                                            {{ item.old_file_name }}
                                        </td>
                                        <td class="px-4 py-2 border-b border-b-white/10">
                                            {{ item.new_file_name }}
                                        </td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </div>
                </Transition>
                <footer
                    id="footer_left_panel"
                    class="flex flex-row py-2 px-2 bg-black/50 border-t-1 rounded-b-lg border-white/20 text-sm text-gray-400"
                >
                    <div id="total-files-selected">
                        <span class="">Total Files Selected: </span>
                        <Transition mode="out-in">
                            <span>{{ numFileStatusItems }}</span>
                        </Transition>
                    </div>

                    <div id="separator" class="flex-1"></div>
                </footer>
            </SplitterPanel>

            <!-- === Right Splitter Panel === -->
            <SplitterPanel
                class="flex flex-col flex-1/3 ml-0.5 mb-2 mr-2 bg-black/40 rounded-lg border-1 border-white/20"
            >
                <!-- === Right SplitterPanel Menubar === -->
                <div id="file_buttons" class="flex flex-row m-2 gap-2 items-center justify-start">
                    <!-- <Menu id="customTextOverlayMenu" :model="customTextMenuItems" popup="true" /> -->

                    <Button
                        type="button"
                        label="File Tasks & Effects"
                        size="small"
                        icon="pi pi-plus"
                        class="min-w-max"
                        variant="outlined"
                        @click="taskMenuToggleFunction"
                        aria-haspopup="true"
                        aria-controls="custom_text_menu"
                    />
                    <Menu ref="taskMenuToggle" id="custom_text_menu" :model="taskMenuItems" :popup="true" />

                    <!-- === Separator === -->
                    <div class="flex-1"></div>

                    <Button
                        type="button"
                        label="Templates"
                        variant="outlined"
                        severity="secondary"
                        size="small"
                        icon="pi pi-bookmark"
                        class="min-w-max"
                        @click="templateMenuToggleFunction"
                        aria-haspopup="true"
                        aria-controls="custom_text_menu"
                    />
                    <Menu ref="templateMenuToggle" id="template_menu" :model="templateMenuItems" :popup="true" />

                    <!-- === Hamburger Select === -->
                    <Button
                        icon="pi pi-trash"
                        class="whitespace-nowrap flex-none"
                        @click="clearTasks"
                        variant="outlined"
                        severity="secondary"
                        size="small"
                    />
                </div>

                <hr class="border-white/20" />

                <TransitionGroup tag="div" name="ttasks" class="h-full relative overflow-y-auto bg-white/3">
                    <!-- === No Files Selected === -->
                    <div
                        v-if="!numTaskListItems"
                        class="flex flex-1 flex-col justify-center items-center w-full h-full whitespace-nowrap"
                    >
                        <span class="text-center mb-2 text-gray-400">File tasks & effects go here.</span>
                        <span class="text-center text-sm text-gray-600">Please use the add buttons above.</span>
                    </div>

                    <div
                        v-else
                        v-for="(item, index) in taskList"
                        :key="item.id"
                        class="ttasks-item mx-2 my-2 whitespace-nowrap"
                    >
                        <!-- === CustomText Task === -->
                        <template v-if="isCustomTextTask(item.task)">
                            <div class="task-container">
                                <!-- === Title and Description === -->
                                <div class="flex flex-row items-center justify-between mb-1">
                                    <div class="flex flex-row items-center gap-2">
                                        <span class="pi pi-pen-to-square"></span>
                                        <h4 class="text-sm font-semibold text-gray-200 m-0">Custom Text</h4>
                                        <p class="text-xs text-gray-400 m-0">Add text to file names</p>
                                    </div>
                                    <!-- === Dummy Spacer === -->
                                    <div class="flex-1"></div>

                                    <!-- === Close Button === -->
                                    <i
                                        class="pi pi-angle-up hover:cursor-pointer text-sm mr-1"
                                        :class="{ 'opacity-30': index === 0 }"
                                        @click="moveSelectedTaskUp(index)"
                                    ></i>
                                    <i
                                        class="pi pi-angle-down hover:cursor-pointer text-sm mr-1"
                                        :class="{
                                            'opacity-30': index === taskList.length - 1,
                                        }"
                                        @click="moveSelectedTaskDown(index)"
                                    ></i>
                                    <!-- === Close Button === -->
                                    <div class="flex items-center" @click="deleteSelectedTask(index)">
                                        <i
                                            class="pi pi-times hover:cursor-pointer text-sm text-gray-600 hover:text-red-500 transition-colors"
                                            style="font-size: 0.9rem"
                                        ></i>
                                    </div>
                                </div>

                                <!-- === Main Controls === -->
                                <div class="flex flex-row gap-3 items-center">
                                    <!-- === Prefix Text === -->

                                    <div class="w-full min-w-36">
                                        <FloatLabel variant="on" class="font-thin">
                                            <InputText
                                                fluid
                                                size="small"
                                                placeholder=""
                                                :id="`input-text-${index}`"
                                                v-model="item.task.CustomText.text"
                                                variant="outlined"
                                                @input="user_update_tasks"
                                                class=""
                                            />
                                            <label class="font-thin" for="`input-text-${index}`">Value</label>
                                        </FloatLabel>
                                    </div>

                                    <div class="flex-1">
                                        <ToggleButton
                                            v-model="item.task.CustomText.at_start"
                                            onLabel="Position at Beginning"
                                            offLabel="Position at End"
                                            size="small"
                                            @change="user_update_tasks"
                                        />
                                    </div>
                                </div>
                            </div>
                        </template>

                        <!-- === Find & Replace Task === -->
                        <template v-else-if="isFindAndReplaceTask(item.task)">
                            <div class="task-container">
                                <!-- === Title and Description === -->
                                <div class="flex flex-row items-center justify-between mb-1">
                                    <div class="flex flex-row items-center gap-2">
                                        <span class="pi pi-search"></span>
                                        <h4 class="text-sm font-semibold text-gray-200 m-0">Find & Replace</h4>
                                        <p class="text-xs text-gray-400 m-0">Replace text in file names</p>
                                    </div>
                                    <!-- === Dummy Spacer === -->
                                    <div class="flex-1"></div>

                                    <!-- === Close Button === -->
                                    <i
                                        class="pi pi-angle-up hover:cursor-pointer text-sm mr-1"
                                        :class="{ 'opacity-30': index === 0 }"
                                        @click="moveSelectedTaskUp(index)"
                                    ></i>
                                    <i
                                        class="pi pi-angle-down hover:cursor-pointer text-sm mr-1"
                                        :class="{
                                            'opacity-30': index === taskList.length - 1,
                                        }"
                                        @click="moveSelectedTaskDown(index)"
                                    ></i>
                                    <!-- === Close Button === -->
                                    <div class="flex items-center" @click="deleteSelectedTask(index)">
                                        <i
                                            class="pi pi-times hover:cursor-pointer text-sm text-gray-600 hover:text-red-500 transition-colors"
                                            style="font-size: 0.9rem"
                                        ></i>
                                    </div>
                                </div>

                                <!-- === Main Controls === -->
                                <div class="flex flex-row gap-3 items-center">
                                    <!-- === Find Text Field === -->

                                    <div class="w-full min-w-36">
                                        <FloatLabel variant="on" class="font-thin">
                                            <InputText
                                                :id="`find-text-${index}`"
                                                fluid
                                                placeholder=""
                                                size="small"
                                                v-model="item.task.FindAndReplace.find_text"
                                                variant="outlined"
                                                @input="user_update_tasks"
                                                class="flex-1"
                                            />
                                            <label class="font-thin" for="`find-text-${index}`">Find</label>
                                        </FloatLabel>
                                    </div>

                                    <!-- === Replace Text Field === -->

                                    <div class="w-full min-w-36">
                                        <FloatLabel variant="on" class="font-thin">
                                            <InputText
                                                :id="`replace-text-${index}`"
                                                fluid
                                                placeholder=""
                                                size="small"
                                                v-model="item.task.FindAndReplace.replace_text"
                                                variant="outlined"
                                                @input="user_update_tasks"
                                                class="flex-1"
                                            />
                                            <label class="font-thin" for="`replace-text-${index}`">Replace</label>
                                        </FloatLabel>
                                    </div>
                                </div>
                            </div>
                        </template>

                        <!-- === ClearAll Task === -->
                        <template v-else-if="isClearAllTask(item.task)">
                            <div class="task-container">
                                <!-- === Title and Description === -->
                                <div class="flex flex-row items-center justify-between mb-1">
                                    <div class="flex flex-row items-center gap-2">
                                        <span class="pi pi-eraser"></span>
                                        <h4 class="text-sm font-semibold text-gray-200 m-0">Clear All</h4>
                                        <p class="text-xs text-gray-400 m-0">Remove all text from file names</p>
                                    </div>
                                    <!-- === Dummy Spacer === -->
                                    <div class="flex-1"></div>

                                    <!-- === Close Button === -->
                                    <i
                                        class="pi pi-angle-up hover:cursor-pointer text-sm mr-1"
                                        :class="{ 'opacity-30': index === 0 }"
                                        @click="moveSelectedTaskUp(index)"
                                    ></i>
                                    <i
                                        class="pi pi-angle-down hover:cursor-pointer text-sm mr-1"
                                        :class="{
                                            'opacity-30': index === taskList.length - 1,
                                        }"
                                        @click="moveSelectedTaskDown(index)"
                                    ></i>
                                    <!-- === Close Button === -->
                                    <div class="flex items-center" @click="deleteSelectedTask(index)">
                                        <i
                                            class="pi pi-times hover:cursor-pointer text-sm text-gray-600 hover:text-red-500 transition-colors"
                                            style="font-size: 0.9rem"
                                        ></i>
                                    </div>
                                </div>

                                <!-- === Main Controls === -->
                                <!-- <div class="flex flex-row gap-3 items-center"> -->
                                <!--     <div class="flex-1 flex items-center"> -->
                                <!--         <p class="text-sm text-gray-700 m-0"> -->
                                <!--             This will remove all existing text from the file names -->
                                <!--         </p> -->
                                <!--     </div> -->

                                <!-- <!-- === Active Toggle === -->
                                <!-- <div class="flex items-center gap-2 whitespace-nowrap"> -->
                                <!--     <label :for="`active-${index}`" class="text-xs text-gray-600">Active</label> -->
                                <!--     <ToggleSwitch -->
                                <!--         v-model="item.task.ClearAll.active" -->
                                <!--         :inputId="`active-${index}`" -->
                                <!--         :name="`clear-all-active-${index}`" -->
                                <!--         binary -->
                                <!--         @change="user_update_tasks" -->
                                <!--     /> -->
                                <!-- </div> -->
                                <!-- </div> -->
                            </div>
                        </template>

                        <!-- === ChangeCase Task === -->
                        <template v-else-if="isChangeCaseTask(item.task)">
                            <div class="task-container">
                                <!-- === Title and Description === -->
                                <div class="flex flex-row items-center justify-between mb-1">
                                    <div class="flex flex-row items-center gap-2">
                                        <span class="pi pi-sort-alpha-up"></span>
                                        <h4 class="min-w-max text-sm font-semibold text-gray-200 m-0">Change Case</h4>
                                        <p class="min-w-max text-xs text-gray-400 m-0">
                                            Convert text case in file names
                                        </p>
                                    </div>
                                    <!-- === Dummy Spacer === -->
                                    <div class="flex-1"></div>

                                    <!-- === Close Button === -->
                                    <i
                                        class="pi pi-angle-up hover:cursor-pointer text-sm mr-1"
                                        :class="{ 'opacity-30': index === 0 }"
                                        @click="moveSelectedTaskUp(index)"
                                    ></i>
                                    <i
                                        class="pi pi-angle-down hover:cursor-pointer text-sm mr-1"
                                        :class="{
                                            'opacity-30': index === taskList.length - 1,
                                        }"
                                        @click="moveSelectedTaskDown(index)"
                                    ></i>
                                    <!-- === Close Button === -->
                                    <div class="flex items-center" @click="deleteSelectedTask(index)">
                                        <i
                                            class="pi pi-times hover:cursor-pointer text-sm text-gray-600 hover:text-red-500 transition-colors"
                                            style="font-size: 0.9rem"
                                        ></i>
                                    </div>
                                </div>

                                <!-- === Main Controls === -->
                                <div class="flex flex-row gap-3 items-center">
                                    <!-- === Case Choice Dropdown === -->
                                    <Dropdown
                                        v-model="item.task.ChangeCase.case_choice"
                                        :options="[
                                            { label: 'lowercase', value: 0 },
                                            { label: 'UPPERCASE', value: 1 },
                                            // { label: 'Title Case', value: 2 },
                                        ]"
                                        optionLabel="label"
                                        optionValue="value"
                                        placeholder="Select case type"
                                        size="small"
                                        class="flex-1"
                                        @change="user_update_tasks"
                                    />

                                    <!-- <!-- === Active Toggle === -->
                                    <!-- <div class="flex items-center gap-2 whitespace-nowrap"> -->
                                    <!--     <label :for="`active-${index}`" class="text-xs text-gray-600">Active</label> -->
                                    <!--     <ToggleSwitch -->
                                    <!--         v-model="item.task.ChangeCase.active" -->
                                    <!--         :inputId="`active-${index}`" -->
                                    <!--         :name="`change-case-active-${index}`" -->
                                    <!--         binary -->
                                    <!--         @change="user_update_tasks" -->
                                    <!--     /> -->
                                    <!-- </div> -->
                                </div>
                            </div>
                        </template>

                        <!-- === NumSequence Task === -->
                        <template v-else-if="isNumSequenceTask(item.task)">
                            <div class="task-container">
                                <!-- === Title and Description === -->
                                <div class="flex flex-row items-center justify-between mb-1">
                                    <div class="flex flex-row items-center gap-2">
                                        <span class="pi pi-sort-numeric-down"></span>
                                        <h4 class="text-sm font-semibold text-gray-200 m-0">Number Sequence</h4>
                                        <p class="text-xs text-gray-400 m-0">Add sequential numbers to file names</p>
                                    </div>

                                    <!-- === Dummy Spacer === -->
                                    <div class="flex-1"></div>

                                    <!-- === Close Button === -->
                                    <i
                                        class="pi pi-angle-up hover:cursor-pointer text-sm mr-1"
                                        :class="{ 'opacity-30': index === 0 }"
                                        @click="moveSelectedTaskUp(index)"
                                    ></i>
                                    <i
                                        class="pi pi-angle-down hover:cursor-pointer text-sm mr-1"
                                        :class="{
                                            'opacity-30': index === taskList.length - 1,
                                        }"
                                        @click="moveSelectedTaskDown(index)"
                                    ></i>
                                    <!-- === Close Button === -->
                                    <div class="flex items-center" @click="deleteSelectedTask(index)">
                                        <i
                                            class="pi pi-times hover:cursor-pointer text-sm text-gray-600 hover:text-red-500 transition-colors"
                                            style="font-size: 0.9rem"
                                        ></i>
                                    </div>
                                </div>

                                <!-- === Main Controls === -->
                                <div class="flex flex-row gap-2 items-center">
                                    <!-- === Start Number === -->
                                    <div class="w-full min-w-36">
                                        <FloatLabel variant="on" class="">
                                            <InputNumber
                                                v-model="item.task.NumSequence.start_num"
                                                :id="`start-num-${index}`"
                                                :input-id="`start-num-inputid-${index}`"
                                                fluid
                                                show-buttons
                                                buttonLayout="horizontal"
                                                size="small"
                                                class="w-full"
                                                @value-change="user_update_tasks"
                                            />
                                            <label for="`start-num-inputid-${index}`">Start #</label>
                                        </FloatLabel>
                                    </div>

                                    <!-- === Padding === -->
                                    <div class="w-full min-w-36">
                                        <FloatLabel variant="on" class="">
                                            <InputNumber
                                                v-model="item.task.NumSequence.num_padding"
                                                :id="`padding-${index}`"
                                                :input-id="`padding-inputid-${index}`"
                                                size="small"
                                                fluid
                                                show-buttons
                                                buttonLayout="horizontal"
                                                class="w-full"
                                                @value-change="user_update_tasks"
                                            />
                                            <label for="`padding-${index}`">Padding</label>
                                        </FloatLabel>
                                    </div>

                                    <!-- === Separator === -->
                                    <div class="flex-1">
                                        <FloatLabel variant="on" class="">
                                            <InputText
                                                v-model="item.task.NumSequence.separator"
                                                :id="`separator-${index}`"
                                                size="small"
                                                class="w-21"
                                                @input="user_update_tasks"
                                            />
                                            <label for="`separator-${index}`">Separator</label>
                                        </FloatLabel>
                                    </div>

                                    <div class="flex-1">
                                        <ToggleButton
                                            v-model="item.task.NumSequence.at_start"
                                            onLabel="Position at Beginning"
                                            offLabel="Position at End"
                                            size="small"
                                            class="flex-1"
                                            @change="user_update_tasks"
                                        />
                                    </div>
                                </div>
                            </div>
                        </template>

                        <!-- === Date Task === -->
                        <template v-else-if="isDateTask(item.task)">
                            <div class="task-container">
                                <!-- === Title and Description === -->
                                <div class="flex flex-row items-center justify-between mb-1">
                                    <div class="flex flex-row items-center gap-2">
                                        <span class="pi pi-calendar"></span>
                                        <h4 class="text-sm font-semibold text-gray-200 m-0">Date</h4>
                                        <p class="text-xs text-gray-400 m-0">Add current date to file names</p>
                                    </div>

                                    <!-- === Dummy Spacer === -->
                                    <div class="flex-1"></div>

                                    <!-- === Close Button === -->
                                    <i
                                        class="pi pi-angle-up hover:cursor-pointer text-sm mr-1"
                                        :class="{ 'opacity-30': index === 0 }"
                                        @click="moveSelectedTaskUp(index)"
                                    ></i>
                                    <i
                                        class="pi pi-angle-down hover:cursor-pointer text-sm mr-1"
                                        :class="{
                                            'opacity-30': index === taskList.length - 1,
                                        }"
                                        @click="moveSelectedTaskDown(index)"
                                    ></i>
                                    <!-- === Close Button === -->
                                    <div class="flex items-center" @click="deleteSelectedTask(index)">
                                        <i
                                            class="pi pi-times hover:cursor-pointer text-sm text-gray-600 hover:text-red-500 transition-colors"
                                            style="font-size: 0.9rem"
                                        ></i>
                                    </div>
                                </div>

                                <!-- === Main Controls === -->
                                <div class="flex flex-row gap-3 items-center">
                                    <!-- === Date Components === -->
                                    <div class="flex items-center gap-2 whitespace-nowrap">
                                        <label :for="`year-${index}`" class="text-xs text-gray-600">Year</label>
                                        <ToggleSwitch
                                            v-model="item.task.Date.year"
                                            :inputId="`year-${index}`"
                                            :name="`year-${index}`"
                                            binary
                                            @change="user_update_tasks"
                                        />
                                    </div>

                                    <div class="flex items-center gap-2 whitespace-nowrap">
                                        <label :for="`month-${index}`" class="text-xs text-gray-600">Month</label>
                                        <ToggleSwitch
                                            v-model="item.task.Date.month"
                                            :inputId="`month-${index}`"
                                            :name="`month-${index}`"
                                            binary
                                            @change="user_update_tasks"
                                        />
                                    </div>

                                    <div class="flex items-center gap-2 whitespace-nowrap">
                                        <label :for="`day-${index}`" class="text-xs text-gray-600">Day</label>
                                        <ToggleSwitch
                                            v-model="item.task.Date.day"
                                            :inputId="`day-${index}`"
                                            :name="`day-${index}`"
                                            binary
                                            @change="user_update_tasks"
                                        />
                                    </div>

                                    <div class="flex items-center gap-2 whitespace-nowrap">
                                        <label :for="`year4-${index}`" class="text-xs text-gray-600">4-Digit</label>
                                        <ToggleSwitch
                                            v-model="item.task.Date.year_4"
                                            :inputId="`year4-${index}`"
                                            :name="`year4-${index}`"
                                            binary
                                            @change="user_update_tasks"
                                        />
                                    </div>

                                    <!-- === Separator === -->
                                    <InputText
                                        v-model="item.task.Date.separator"
                                        :id="`separator-${index}`"
                                        placeholder="Sep"
                                        size="small"
                                        class="w-12"
                                        @input="user_update_tasks"
                                    />

                                    <!-- === Active Toggle === -->
                                    <!-- <div class="flex items-center gap-2 whitespace-nowrap"> -->
                                    <!--     <label :for="`active-${index}`" class="text-xs text-gray-600">Active</label> -->
                                    <!--     <ToggleSwitch -->
                                    <!--         v-model="item.task.Date.active" -->
                                    <!--         :inputId="`active-${index}`" -->
                                    <!--         :name="`date-active-${index}`" -->
                                    <!--         binary -->
                                    <!--         @change="user_update_tasks" -->
                                    <!--     /> -->
                                    <!-- </div> -->
                                </div>
                            </div>
                        </template>

                        <!-- === Time Task === -->
                        <template v-else-if="isTimeTask(item.task)">
                            <div class="task-container">
                                <!-- === Title and Description === -->
                                <div class="flex flex-row items-center justify-between mb-1">
                                    <div class="flex flex-row items-center gap-2">
                                        <span class="pi pi-clock"></span>
                                        <h4 class="text-sm font-semibold text-gray-200 m-0">Time</h4>
                                        <p class="text-xs text-gray-400 m-0">Add current time to file names</p>
                                    </div>

                                    <!-- === Dummy Spacer === -->
                                    <div class="flex-1"></div>

                                    <!-- === Close Button === -->
                                    <i
                                        class="pi pi-angle-up hover:cursor-pointer text-sm mr-1"
                                        :class="{ 'opacity-30': index === 0 }"
                                        @click="moveSelectedTaskUp(index)"
                                    ></i>
                                    <i
                                        class="pi pi-angle-down hover:cursor-pointer text-sm mr-1"
                                        :class="{
                                            'opacity-30': index === taskList.length - 1,
                                        }"
                                        @click="moveSelectedTaskDown(index)"
                                    ></i>
                                    <!-- === Close Button === -->
                                    <div class="flex items-center" @click="deleteSelectedTask(index)">
                                        <i
                                            class="pi pi-times hover:cursor-pointer text-sm text-gray-600 hover:text-red-500 transition-colors"
                                            style="font-size: 0.9rem"
                                        ></i>
                                    </div>
                                </div>

                                <!-- === Main Controls === -->
                                <div class="flex flex-row gap-3 items-center">
                                    <!-- === Time Format === -->
                                    <div class="flex items-center gap-2 whitespace-nowrap">
                                        <label :for="`hour24-${index}`" class="text-xs text-gray-600">24 Hour</label>
                                        <ToggleSwitch
                                            v-model="item.task.Time.hour_24"
                                            :inputId="`hour24-${index}`"
                                            :name="`hour24-${index}`"
                                            binary
                                            @change="user_update_tasks"
                                        />
                                    </div>

                                    <!-- === Separator === -->
                                    <InputText
                                        v-model="item.task.Time.separator"
                                        :id="`separator-${index}`"
                                        placeholder="Sep"
                                        size="small"
                                        class="w-12"
                                        @input="user_update_tasks"
                                    />

                                    <!-- === Active Toggle === -->
                                    <!-- <div class="flex items-center gap-2 whitespace-nowrap"> -->
                                    <!--     <label :for="`active-${index}`" class="text-xs text-gray-600">Active</label> -->
                                    <!--     <ToggleSwitch -->
                                    <!--         v-model="item.task.Time.active" -->
                                    <!--         :inputId="`active-${index}`" -->
                                    <!--         :name="`time-active-${index}`" -->
                                    <!--         binary -->
                                    <!--         @change="user_update_tasks" -->
                                    <!--     /> -->
                                    <!-- </div> -->
                                </div>
                            </div>
                        </template>
                    </div>
                </TransitionGroup>
                <footer
                    id="footer_right_panel"
                    class="flex flex-row py-2 px-2 gap-2 bg-black/15 border-t-1 rounded-b-lg border-white/20 text-sm text-gray-400"
                >
                    <div id="total-files-selected" class="flex flex-col">
                        <span class="font-bold">File Destination</span>
                        <span class="">Files replaced in place</span>
                    </div>

                    <div id="separator" class="flex-1"></div>

                    <Button
                        size="small"
                        icon="pi pi-folder-open"
                        variant="outlined"
                        label="Change Output Directory"
                        @click="rename_files"
                    />
                    <Button size="small" icon="pi pi-check-square" label="Batch Rename Files" @click="rename_files" />
                </footer>
            </SplitterPanel>
        </Splitter>

        <!-- === Footer -> Total Files Selected === -->
        <!-- <footer -->
        <!--     class="flex flex-row py-2 px-2 bg-black/50 border-t-1 rounded-b-lg border-white/20 text-sm text-gray-400" -->
        <!-- > -->
        <!--     <div id="total-files-selected"> -->
        <!--         <span class="">Total Files Selected: </span> -->
        <!--         <Transition mode="out-in"> -->
        <!--             <span>{{ numFileStatusItems }}</span> -->
        <!--         </Transition> -->
        <!--     </div> -->
        <!---->
        <!--     <div id="separator" class="flex-1"></div> -->
        <!---->
        <!--     <!-- <Button size="medium" icon="pi pi-check" label="Batch Rename Files" @click="rename_files" /> -->
        <!-- -->
        <!-- </footer> -->
    </body>
</template>

<style>
:root {
    /* background-image: url("src/assets/v960-ning-30.jpg"); */
    /* background-image: black; */
    background-size: cover;
    background-position: center;
    background-repeat: no-repeat;
    background-attachment: fixed;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 50%;
}

.body {
    /* background-image: url("src/assets/636977f64331639884919566b3ec074a_edit1.jpg"); */
    /* background-color: white; */
    background-size: cover;
    background-position: center;
    background-repeat: no-repeat;
    background-attachment: fixed;
}

/* Cross-platform CSS foundation */
*,
*::before,
*::after {
    box-sizing: border-box;
    /* margin: 0;
    padding: 0; */
}

html {
    font-size: 15px; /* PrimeVue design system base */
    line-height: 1.5;
    -webkit-text-size-adjust: 100%;
    -ms-text-size-adjust: 100%;
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

.p-floatlabel {
    --p-floatlabel-font-weight: 400;
    --p-floatlabel-font-size: 12rem;
    --p-floatlabel-in-input-padding-top: 6rem;
    --p-floatlabel-in-input-padding-bottom: 6rem;
}

.task-container {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    border: 1px solid rgba(255, 255, 255, 0.2);
    background-color: rgba(15, 15, 15, 1);
    border-radius: 0.5rem;
    padding: 0.75rem;
    overflow: hidden;
}
</style>
