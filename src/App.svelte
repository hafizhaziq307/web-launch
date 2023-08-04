<script>
    import { open } from "@tauri-apps/api/dialog";
    import { exists, readDir, BaseDirectory, writeTextFile, readTextFile } from '@tauri-apps/api/fs';
    import { onMount } from 'svelte';
    import OpenInCodeEditor from './lib/buttons/OpenInCodeEditor.svelte';
    import OpenInFileExplorer from './lib/buttons/OpenInFileExplorer.svelte';
    import CreateModal from './lib/modals/CreateModal.svelte';
    import DeleteConfirm from './lib/modals/DeleteConfirm.svelte';
    import EditModal from './lib/modals/EditModal.svelte';
    import Error from './lib/Error.svelte';

    const fileSaveConfig = {
        filename: "data.json",
        dir: BaseDirectory.AppLocalData,
        // dir: BaseDirectory.Desktop
    };

    let isEditModalOpen = false;
    let isCreateModalOpen = false;
    let isDeleteConfirmOpen = false;
    let selectedRecord = {
        id: null,
        title: null,
        path: null,
        port: null
    };
    let deletedRecord = {
        id: null,
        title: null,
        path: null,
        port: null
    };

    let promise = displayData();

    // create savefile if not exists yet
    onMount(async () => {
        const isExists = await exists(fileSaveConfig.filename, { dir: fileSaveConfig.dir });
        if (!isExists) {
            await writeTextFile(fileSaveConfig.filename, "[]", {dir: fileSaveConfig.dir });
            promise = displayData();
        } 
    });


    

    async function openDialog(e) {
        let directory = await open({
            directory: true,
            multiple: false,
        });

        if (directory) {
            e.target.value = directory.replace(/\\/g, "/");
        } 
    }

    async function readData() {
        const strContents = await readTextFile(fileSaveConfig.filename, { dir: fileSaveConfig.dir });
        return JSON.parse(strContents);
    }

    async function updateData(e) {
        let updatedRecord = {};
        const formData = new FormData(e.target);
        for (const [name, value] of formData) {
            updatedRecord[name] = value;
        }
        
        let data = Array.from(await readData());
        let foundIndex  = data.findIndex(obj => obj.id == updatedRecord.id); 

        if (foundIndex != -1) {
            data[foundIndex] = {
                id: data[foundIndex].id,
                title: updatedRecord.title,
                path: updatedRecord.path,
                port: updatedRecord.port
            };
            await writeTextFile(fileSaveConfig.filename, JSON.stringify(data), {dir: fileSaveConfig.dir });

            closeEditModal();
            promise = displayData();
        }
    }

    async function insertData(e) {
        let createdRecord = {};
        const formData = new FormData(e.target);
        createdRecord.id = String(Date.now());
        for (const [name, value] of formData) {
            createdRecord[name] = value;
        }

        let data = Array.from(await readData());
        data.push(createdRecord);

        await writeTextFile(fileSaveConfig.filename, JSON.stringify(data), {dir: fileSaveConfig.dir });

        closeCreateModal();
        promise = displayData();
    }

    async function deleteData(id) {
        let data = Array.from(await readData());
        const record = data.find(obj => obj.id == id); 

        if (record) {
            data = data.filter(obj => obj.id !== record.id);

            await writeTextFile(fileSaveConfig.filename, JSON.stringify(data), {dir: fileSaveConfig.dir });

            closeCreateModal();
            promise = displayData();
        }
    }

    async function displayData() {
        let webServers = Array.from(await readData());

        for (let i = 0; i < webServers.length; i++) {
            const webServer = webServers[i];
            
            let entries =  await readDir(webServer.path, { dir: fileSaveConfig.dir, recursive: false });

            if (entries) {
                entries = entries.filter(obj => 'children' in obj);

                for (let j = 0; j < entries.length; j++) {
                    const entry = entries[j];
                    entry.path = entry.path.replace(/\\/g, '/');
                }

                webServer.entries = entries;
            }
        }
        return webServers;
    }

    async function openEditModal(id) {
        const data = Array.from(await readData());
        const record = data.find(obj => obj.id == id); 

        if (record) {
            selectedRecord = {
                id: record.id,
                title: record.title,
                path: record.path,
                port: record.port
            };
            isEditModalOpen = true;
        } 
    }

    function closeEditModal() {
        selectedRecord = {
            id: null,
            title: null,
            path: null,
            port: null
        };
        isEditModalOpen = false;
    }

    function openCreateModal() {
        isCreateModalOpen = true;
    }

    function closeCreateModal() {
        isCreateModalOpen = false;
    }

    async function openDeleteConfirm(id) {
        const data = Array.from(await readData());
        const record = data.find(obj => obj.id == id); 
        
        if (record) {
            deletedRecord = {
                id: record.id,
                title: record.title,
                path: record.path,
                port: record.port
            };
            isDeleteConfirmOpen = true;
        } 
    }

    function closeDeleteConfirm() {
        deletedRecord = {
            id: null,
            title: null,
            path: null,
            port: null
        };
        isDeleteConfirmOpen = false;
    }
</script>

<main class="min-h-screen w-full h-[100vh]">
    <div id="content" class="flex h-full flex-col w-full">
        <div class="grow p-3 lg:p-6 bg-gray-100">
            {#await promise}
                <p>...loading</p>
            {:then items}
            <div class="divide-y-2 divide-gray-500">
                {#each items as item}
                    <article class="space-y-2 py-5">
                        <header class="flex gap-2 justify-between items-center">
                            <button on:click={() => openEditModal(item.id)} class="text-2xl font-bold hover:text-red-600" title="Edit">{item.title}</button>
                            <button class="px-4 py-2 rounded text-white bg-red-600 hover:bg-red-500 text-sm font-medium leading-snug" title="Remove" on:click={() => openDeleteConfirm(item.id)}>Remove</button>
                        </header>
            
                        <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
                            {#each item.entries as entry}
                                <article class="bg-white rounded-md shadow space-y-3">
                                    <header class="px-4 pt-4 truncate">
                                        <a href="{ `http://localhost:${item.port}/${entry.name}` }" target="_blank" class="hover:underline hover:text-blue-600">{entry.name}</a>                                        
                                    </header>
                                    <hr>
                                    <footer class="grid grid-cols-2 px-4 pb-2">
                                        <div class="text-center">
                                           <OpenInCodeEditor path={entry.path}/>
                                        </div>
                                        <div class="text-center">
                                            <OpenInFileExplorer path={entry.path}/>
                                        </div>
                                    </footer>
                                </article>
                            {/each}
                        </div>
                    </article>
                {/each}
            </div>

            <div class="max-w-3xl mx-auto mt-10">
                <button class="w-full px-4 py-2 rounded bg-green-600 hover:bg-green-500 font-medium text-white" title="Add more stuff" on:click={openCreateModal}>Add More</button>
            </div>
            {:catch error}
                <Error />
            {/await}
        </div>
    </div>

    <!-- modals -->
    <EditModal isOpen={isEditModalOpen} closeModal={closeEditModal} updateData={updateData} selectedRecord={selectedRecord} openDialog={openDialog} />
    <CreateModal isOpen={isCreateModalOpen} closeModal={closeCreateModal} insertData={insertData} openDialog={openDialog} />
    <DeleteConfirm isOpen={isDeleteConfirmOpen} closeConfirm={closeDeleteConfirm} deleteData={deleteData} deletedRecord={deletedRecord} />
    <!-- end modals -->
</main>