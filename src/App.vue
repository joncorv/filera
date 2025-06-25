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



interface WorkingFile {
  path: string;
  old_file_name: string;
  new_file_name: string;
  active: boolean;
}


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
        workingFileReturn.value = await invoke("open_files", { fileNames: dirtyFilesSelection });

    } else if (selectedFiles && !dirtyFilesSelection) {
        dirtyFilesSelection = selectedFiles;
        workingFileReturn.value = await invoke("open_files", { fileNames: dirtyFilesSelection });

    } else if (!selectedFiles && dirtyFilesSelection) {
        console.log("There was a previous selection, but no current selection. No updates to make.")
    
    } else {
        console.log("There was no previous selection or current selection. No Updates to make.");
    }

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

</script>

<template>
    <body class="flex flex-col box-border w-screen h-screen m-0 p-0 overflow-hidden"> 
        <p class="text-5xl font-bold py-8">Batch Renamer</p>


        <Splitter style="flex: 1; overflow: hidden">
            <SplitterPanel class="flex flex-col flex-1">
                
                <div id="first" class="">
                    <Button label="Open A Folder" @click="open_folder" icon="pi pi-folder-open" />
                    <Button label="Open Files" @click="open_files" icon="pi pi-folder-open" />
                    <Button label="Clear All Files" @click="clear_selection" icon="pi pi-folder-open" />
                </div>
                
                <div id="second" class="flex-1 overflow-hidden">
                    <!-- <li v-for="item in workingFileReturn">
                        {{ item.old_file_name }}
                    </li> -->

                    <DataTable :value="workingFileReturn" scrollable scrollHeight="flex" size="small" stripedRows tableStyle="min-width: 5rem">
                        <Column selectionMode="multiple"></Column>
                        <Column field="old_file_name" header="Old Name" sortable ></Column>
                        <Column field="new_file_name" header="New Name" sortable ></Column>
                        <!-- <Column field="path" header="Full Path"></Column> -->
                        <!-- <Column field="active" header="Active"></Column> -->
                    </DataTable>

                </div>
                <div id="third" class="">Total files selected: {{ number_of_working_files }}</div>
            </SplitterPanel>



            <SplitterPanel class="flex items-center justify-center"> Panel 2 </SplitterPanel>
        </Splitter>

        <div id="footer" class="h-10 bg-red-300">
            Here is the global footer

        </div>



    </body>
</template>

<style>
:root {


    color: #0f0f0f;
    background-color: #f6f6f6;

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
