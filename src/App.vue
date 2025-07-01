<script setup lang="ts">
import { ref, Ref, computed } from "vue";
import './styles.css'; // Tailwind Stuff
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';

import { Button } from "primevue";
import Splitter from 'primevue/splitter';
import SplitterPanel from 'primevue/splitterpanel';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
// import ColumnGroup from 'primevue/columngroup';   // optional
// import Row from 'primevue/row';                   // optional
import InputText from 'primevue/inputtext';
// import Checkbox from 'primevue/checkbox';
// import CheckboxGroup from 'primevue/checkboxgroup';
import FloatLabel from 'primevue/floatlabel';

import ToggleSwitch from 'primevue/toggleswitch';

import 'primeicons/primeicons.css'







interface WorkingFile {
  path: string;
  old_file_name: string;
  new_file_name: string;
  active: boolean;
}

type Task = 
  | { Prefix: { text: string; active: boolean; } }
  | { Postfix: { text: string; active: boolean; } }
  | { FindAndReplace: { find_text: string; replace_text: string; active: boolean; } };

// Create an array of Task
const taskList = ref<Task[]>([]);

const addPrefix = () => {
  taskList.value.push({
    Prefix: {
      text: '',
      active: true
    }
  });
};

const addFindReplace = () => {
  taskList.value.push({
    FindAndReplace: {
      find_text: '',
      replace_text: '',
      active: true
    }
  });
};

// Add Task to the array like this:
// taskList.value = [
//   {
//     Prefix: {
//       text: 'pre_',
//       active: true
//     }
//   },
//   {
//     Postfix: {
//       text: '_post',
//       active: false
//     }
//   },
//   {
//     FindAndReplace: {
//       find_text: 'mytxt',
//       replace_text: 'WAY_BETTER_STUFF',
//       active: true
//     }
//   }
// ];

let dirtyFilesSelection: string[] | null = null;
// const fileReturnMessage = ref([""]);
const workingFileReturn: Ref<WorkingFile[]> = ref([]);
const number_of_working_files = computed(() => workingFileReturn.value.length);


async function open_files() {

    const selectedFiles = await open({
        multiple: true,
        director: true,
    })
    
    if (selectedFiles && dirtyFilesSelection) {
        dirtyFilesSelection = dirtyFilesSelection.concat(selectedFiles);
        workingFileReturn.value = await invoke("open_files", { fileNames: dirtyFilesSelection, taskList: taskList.value });

    } else if (selectedFiles && !dirtyFilesSelection) {
        dirtyFilesSelection = selectedFiles;
        workingFileReturn.value = await invoke("open_files", { fileNames: dirtyFilesSelection, taskList: taskList.value });

    } else if (!selectedFiles && dirtyFilesSelection) {
        console.log("There was a previous selection, but no current selection. No updates to make.")
    
    } else {
        console.log("There was no previous selection or current selection. No Updates to make.");
    }

}

async function update_files() {
    workingFileReturn.value = await invoke("open_files", { fileNames: dirtyFilesSelection, taskList: taskList.value });

}

async function open_folder() {
    const selectedFolder = await open({
        multiple: false,
        directory: true,
    })
    console.log(selectedFolder)
}

function clear_selection(){
    dirtyFilesSelection = null;
    workingFileReturn.value = [];
}

function clearTask() {
    taskList.value = [];
    update_files();
}


</script>

<template>

    <body class="flex flex-col box-border w-screen h-screen m-0 p-0 overflow-hidden z-0">

        <!-- This is the Master Splitter Panel -->
        <Splitter style="flex: 1; overflow: hidden; background-color: transparent; backdrop-filter: blur(50px); z-index: 40;">

            <!-- This is the Left Splitter Panel -->
            <SplitterPanel class="flex flex-col gap-0 m-4 rounded-2xl p-4 border-2 border-white/30 bg-white/20 backdrop-blur-3xl shadow-lg z-50">


                <div id="file_buttons" class="flex flex-row items-center justify-start">
                    <!-- <Button class="mr-4" label="Open A Folder" severity="primary" @click="open_folder" icon="pi pi-folder-open" /> -->
                    <Button class="reg-button" unstyled label="Open Files" @click="open_files" icon="pi pi-file" />
                    <Button class="reg-button" unstyled label="Clear All Files" severity="danger" @click="clear_selection" icon="pi pi-trash" />
                </div>

                <hr class="border-1 border-white/30 my-4" />

                <Transition mode="out-in">
                <div v-if="!number_of_working_files" class="flex-1 justify-items-center flex-row ml-4 mb-2">
                    <span class="font-thin text-xs font-mono text-center text-black/50"> no files selected </span>
                </div>

               <div v-else id="table-container" class="flex-1 flex flex-col mb-2 min-h-0">
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
                                <tr v-for="(item, index) in workingFileReturn" :key="index" >
                                    <td class="px-4 py-2 border-b">{{ item.old_file_name }}</td>
                                    <td class="px-4 py-2 border-b">{{ item.new_file_name }}</td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
               </div>

                </Transition>

                <div id="selection_info">
                    <span class="font-bold">Total Files Selected:</span>
                    <Transition mode="out-in">
                        <span class="ml-2">{{ number_of_working_files}}</span>
                    </Transition>
                </div>



            </SplitterPanel>

            <!-- This is the Right Splitter Panel -->
            <SplitterPanel class="flex flex-col flex-1">

                <div class="flex flex-row mt-4 mb-2 ml-4">
                    <Button class="reg-button" unstyled  severity="primary" @click="addPrefix" label="Add Prefix" icon="pi pi-arrow-circle-left" />
                    <Button class="reg-button" unstyled  severity="primary" @click="addFindReplace" label="Add Find & Replace" icon="pi pi-search" />
                    <Button class="reg-button shadow-xl" unstyled  severity="danger" @click="clearTask" label="Clear All Task" icon="pi pi-trash" />
                </div>

                <TransitionGroup>
                <div v-for="(item, index) in taskList" :key="index" class="item">

                    <!-- Prefix Task -->
                    <template v-if="item.Prefix">
                        <div class="flex flex-row mx-4 my-2 gap-4 border-2 border-white/30 bg-white/40 rounded-lg p-2 backdrop-blur-3xl shadow-lg">

                            <!-- Drag Button -->
                            <div class="flex items-center ml-2 mr-1">
                                <i class="pi pi-arrows-v hover:cursor-pointer" style="font-size: 1rem"></i>
                            </div>

                            <!-- Prefix Text -->
                            <InputText placeholder="Prefix Text" fluid size="small" :id="`input-text-${index}`" v-model="item.Prefix.text" variant="filled" @input="update_files" />

                            <!-- Checkbox -->
                            <div class="flex items-center gap-2 mr-2">
                                <ToggleSwitch v-model="item.Prefix.active" :inputId="`checkbox-${index}`" :name="`active-checkbox${index}`" binary @change="update_files" />
                            </div>

                        </div>
                    </template>

                    <!-- Find & Replace Task -->
                    <template v-else-if="item.FindAndReplace">
                        <div class="flex flex-row mx-4 my-2 gap-4 border-2 border-white/30 bg-white/40 rounded-lg p-2 backdrop-blur-3xl shadow-lg">

                            <!-- Drag Button -->
                            <div class="flex items-center ml-2 mr-1">
                                <i class="pi pi-arrows-v hover:cursor-pointer" style="font-size: 1rem"></i>
                            </div>

                            <!-- Find Text Field -->
                            <InputText id="in_label" placeholder="Find" fluid size="small"  v-model="item.FindAndReplace.find_text" variant="filled" @input="update_files" />
                                
                            <!-- Replace Text Field -->
                                <InputText id="in_label" placeholder="Replace" fluid size="small"  v-model="item.FindAndReplace.replace_text" variant="filled" @input="update_files" />


                            <!--  Checkbox -->
                            <div class="flex items-center gap-2 mr-2">
                                <ToggleSwitch v-model="item.FindAndReplace.active" :inputId="`active-${index}`" name="namefindreplaceactive" binary @change="update_files" />
                            </div>


                        </div>
                    </template>
                </div>
                </TransitionGroup>

                <!-- <div class="debug">
                    <strong>Current data:</strong>
                    <pre>{{ JSON.stringify(taskList, null, 2) }}</pre>
                </div> -->


            </SplitterPanel>
        </Splitter>

        <div id="footer" class="flex flex-row-reverse m-4">
            <Button label="Batch Rename All Files" size="large" />

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

/* Transition Classes */
.v-enter-active,
.v-leave-active {
  transition: opacity 0.1s ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
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
    padding: calc(var(--spacing) * 4) /* 1rem = 16px */;
    margin-right: calc(var(--spacing) * 4) /* 1rem = 16px */;
    height: 2.5rem;
    border: 2px solid rgba(255, 255, 255, 0.4);
    border-radius: 9999px;
    color: color-mix(in oklab, var(--color-black) /* #000 = #000000 */ 60%, transparent);
    font-weight: 600;
    font-size: var(--text-sm) /* 0.875rem = 14px */;
    line-height: var(--tw-leading, var(--text-sm--line-height) /* calc(1.25 / 0.875) ≈ 1.4286 */);
    background-color: color-mix(in oklab, var(--color-white) /* #fff = #ffffff */ 80%, transparent);
    box-shadow: black 0px 0px 0px 0px, rgba(0, 0, 0, 0.05) 0px 4px 6px -1px, rgba(0, 0, 0, 0.1) 0px 2px 4px -1px;
    
    transition: background-color 0.2s ease;
    &:hover {
        background-color: color-mix(in oklab, var(--color-white) /* #fff = #ffffff */ 100%, transparent);
        cursor: pointer;
    }

}

</style>
