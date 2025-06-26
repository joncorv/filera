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
import Checkbox from 'primevue/checkbox';
import CheckboxGroup from 'primevue/checkboxgroup';
import FloatLabel from 'primevue/floatlabel';





interface WorkingFile {
  path: string;
  old_file_name: string;
  new_file_name: string;
  active: boolean;
}

type Tasks = 
  | { Prefix: { text: string; active: boolean; } }
  | { Postfix: { text: string; active: boolean; } }
  | { FindAndReplace: { find_text: string; replace_text: string; active: boolean; } };

// Create an array of Tasks
const taskList = ref<Tasks[]>([]);

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

// Add tasks to the array like this:
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

function clearTasks() {
    taskList.value = [];
    update_files();
}


</script>

<template>

    <body class="flex flex-col box-border w-screen h-screen m-0 p-0 overflow-hidden">
        <p class="text-3xl font-bold px-4">Batch Renamer</p>

        <!-- This is the Master Splitter Panel -->
        <Splitter style="flex: 1; overflow: hidden">

            <!-- This is the Left Splitter Panel -->
            <SplitterPanel class="flex flex-col flex-1 gap-2">

                <div id="file_buttons" class="ml-4 my-4">
                    <Button class="mr-4" label="Open A Folder" severity="primary" @click="open_folder" icon="pi pi-folder-open" />
                    <Button class="mr-4" label="Open Files" severity="primary" @click="open_files" icon="pi pi-file" />
                    <Button class="mr-4" label="Clear All Files" severity="danger" @click="clear_selection" icon="pi pi-trash" />
                </div>

                <div id="data_tables" class="flex-1 overflow-hidden">
                    <DataTable :value="workingFileReturn" scrollable scrollHeight="flex" size="small" stripedRows
                        tableStyle="min-width: 5rem">
                        <Column selectionMode="multiple"></Column>
                        <Column field="old_file_name" header="Old Name" sortable></Column>
                        <Column field="new_file_name" header="New Name" sortable></Column>
                        <!-- <Column field="path" header="Full Path"></Column> -->
                        <!-- <Column field="active" header="Active"></Column> -->
                    </DataTable>
                </div>
                <div id="selection_info" class="">Total files selected: {{ number_of_working_files }}</div>
            </SplitterPanel>

            <!-- This is the Right Splitter Panel -->
            <SplitterPanel class="flex flex-col flex-1">

                <div class="flex flex-row my-4 ml-4">
                    <Button class="mr-4" severity="primary" @click="addPrefix" label="Add Prefix" icon="pi pi-arrow-circle-left" />
                    <Button class="mr-4" severity="primary" @click="addFindReplace" label="Add Find & Replace" icon="pi pi-search" />
                    <Button class="mr-4" severity="danger" @click="clearTasks" label="Clear All Tasks" icon="pi pi-trash" />
                </div>

                <div v-for="(item, index) in taskList" :key="index" class="item">

                    <!-- Prefix Task -->
                    <template v-if="item.Prefix">
                        <div class="flex flex-row ml-4 my-2 gap-2">

                            <!-- Prefix Active Checkbox -->
                            <div class="flex items-center gap-2">
                                <Checkbox v-model="item.Prefix.active" :inputId="`checkbox-${index}`" :name="`active-checkbox${index}`" binary @change="update_files" />
                                <label :for="`checkbox-${index}`" >Active</label>
                            </div>

                            <!-- Prefix Text -->
                            <FloatLabel variant="on">
                                <InputText :id="`input-text-${index}`" v-model="item.Prefix.text" variant="filled" @input="update_files" />
                                <label :for="`input-text-${index}`" >Prefix Text</label>
                            </FloatLabel>
                            
                        </div>
                    </template>

                    <!-- Find & Replace Task -->
                    <template v-else-if="item.FindAndReplace">
                        <div class="flex flex-row ml-4 my-2 gap-2">

                            <!-- Active Checkbox -->
                            <div class="flex items-center gap-2">
                                <Checkbox v-model="item.FindAndReplace.active" :inputId="`active-${index}`" name="namefindreplaceactive" binary @change="update_files" />
                                <label :for="`active-${index}`" >Active</label>
                            </div>

                            <!-- Find Text Field -->
                             <FloatLabel variant="on">
                                <InputText id="in_label" v-model="item.FindAndReplace.find_text" variant="filled" @input="update_files" />
                                <label for="in_label">Find</label>
                            </FloatLabel>

                            <!-- Replace Text Field -->
                             <FloatLabel variant="on">
                                <InputText id="in_label" v-model="item.FindAndReplace.replace_text" variant="filled" @input="update_files" />
                                <label for="in_label">Replace</label>
                            </FloatLabel>



                        </div>
                    </template>
                </div>

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


    /* color: #0f0f0f;
    background-color: #f6f6f6; */

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
    }
/* 
.body {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
} */


</style>
