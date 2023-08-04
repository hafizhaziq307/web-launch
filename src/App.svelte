<script>
    import { open } from "@tauri-apps/api/dialog";
    import { readDir, BaseDirectory, writeTextFile, readTextFile } from '@tauri-apps/api/fs';
    import { invoke } from "@tauri-apps/api/tauri";

    let promise = displayData();
    let editModalOpen = false;
    let createModalOpen = false;
    let deleteModalOpen = false;
    let selectedRecord = {
        id: null,
        title: null,
        path: null,
        port: null
    };

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
        const strContents = await readTextFile('example.json', { dir: BaseDirectory.Desktop });
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
            await writeTextFile("example.json", JSON.stringify(data), {dir: BaseDirectory.Desktop });

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

        await writeTextFile("example.json", JSON.stringify(data), {dir: BaseDirectory.Desktop });

        closeCreateModal();
        promise = displayData();
    }

    async function deleteData(id) {
        let data = Array.from(await readData());
        const record = data.find(obj => obj.id == id); 

        if (record) {
            data = data.filter(obj => obj.id !== record.id);

            await writeTextFile("example.json", JSON.stringify(data), {dir: BaseDirectory.Desktop });

            closeCreateModal();
            promise = displayData();
        }
    }

    async function displayData() {
        let webServers = Array.from(await readData());

        for (let i = 0; i < webServers.length; i++) {
            const webServer = webServers[i];
            
            let entries =  await readDir(webServer.path, { dir: BaseDirectory.Desktop, recursive: false });

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

    async function openInFileExplorer(path = null) {
        if (path) {
            await invoke('show_in_folder', {path});   
        } 
    }

    async function openInCodeEditor(path = null) {
        if (path) {
            await invoke('show_in_code_editor', {path});   
        } 
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
            editModalOpen = true;
        } 
    }

    function closeEditModal() {
        selectedRecord = {
            id: null,
            title: null,
            path: null,
            port: null
        };
        editModalOpen = false;
    }

    async function openCreateModal() {
        createModalOpen = true;
    }

    function closeCreateModal() {
        createModalOpen = false;
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
                            <button class="px-4 py-2 rounded text-white bg-red-600 hover:bg-red-500 text-sm font-medium leading-snug" title="Remove" on:click={() => deleteData(item.id)}>Remove</button>
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
                                            <button on:click={ () => openInCodeEditor(entry.path) }>
                                                <i class="fas fa-code text-xl text-gray-800 hover:text-gray-600" title="Open in code editor"></i>
                                            </button>
                                        </div>
                                        <div class="text-center">
                                            <button on:click={ () => openInFileExplorer(entry.path) }>
                                                <i class="fas fa-folder text-xl text-gray-800 hover:text-gray-600" title="Open in file explorer"></i>
                                            </button>
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
                <div class="fixed inset-0 flex items-center justify-center">
                    <div class="bg-white border border-gray-200 flex flex-col items-center justify-center px-4 md:px-8 lg:px-24 py-8 rounded-lg shadow-2xl">
                        <div class="text-6xl md:text-7xl lg:text-9xl font-bold tracking-wider text-gray-300">
                            <i class="fas fa-exclamation-triangle"></i>
                        </div>
                        <p class="text-2xl md:text-3xl lg:text-5xl font-bold tracking-wider text-gray-500 mt-4">Error</p>
                        <p class="text-gray-500 mt-6 py-2 border-y-2 text-center">Whoops, something went wrong on our Application.</p>
                    </div>
                </div>
            {/await}
        </div>
    </div>

    <!-- edit modal -->
    <div class={`bg-slate-800 bg-opacity-50 flex justify-center items-center p-6 ${editModalOpen ? 'fixed inset-0' : 'hidden'}` } >
        <!-- overlay -->
        <div class="fixed inset-0 -z-10" on:click={closeEditModal}></div>
        
        <!-- modal -->
        <div class="bg-white px-10 py-6 rounded-md max-w-xl w-full space-y-4">
            <!-- modal header -->
            <header class="mb-4">
                <h1 class="text-xl font-bold text-slate-500">Edit</h1>
            </header>

            <!-- modal content -->
            <form class="row" on:submit|preventDefault={updateData}>
                <div class="space-y-4">
                    <div>
                        <label for="title" class="block text-sm font-bold ml-1 mb-1">Title</label>
                        <input type="text" name="title" id="title" value={selectedRecord.title} autocomplete="off" on:blur={(e) => e.target.value = (e.target.value).trim()} required class="py-2 px-3 block w-full border-2 border-gray-200 rounded-md outline-none text-sm focus:border-blue-500 focus:ring-blue-500 shadow-sm" >
                    </div>
        
                    <div>
                        <label for="path" class="block text-sm font-bold ml-1 mb-1">Path</label>
                        <input type="text" name="path" id="path" value={selectedRecord.path} autocomplete="off" on:blur={(e) => e.target.value = (e.target.value).trim()} required readonly class="py-2 px-3 block w-full border-2 border-gray-200 rounded-md outline-none text-sm focus:border-blue-500 focus:ring-blue-500 shadow-sm" on:click={openDialog}>
                    </div>
        
                    <div>
                        <label for="port" class="block text-sm font-bold ml-1 mb-1">Port</label>
                        <input type="text" name="port" id="port" value={selectedRecord.port} autocomplete="off" on:blur={(e) => e.target.value = (e.target.value).trim()} required class="py-2 px-3 block w-full border-2 border-gray-200 rounded-md outline-none text-sm focus:border-blue-500 focus:ring-blue-500 shadow-sm" >
                    </div>
                </div>

                <div class="text-right">
                    <input type="hidden" name="id" value="{selectedRecord.id}">
                    <button type="submit" class="bg-indigo-600 hover:bg-indigo-500 leading-snug px-4 py-2 rounded-md text-md text-white font-semibold mt-4 text-sm">Save</button>
                </div>
            </form>
        </div>
    </div>

    <!-- create modal -->
    <div class={`bg-slate-800 bg-opacity-50 flex justify-center items-center p-6 ${createModalOpen ? 'fixed inset-0' : 'hidden'}` } >
        <!-- overlay -->
        <div class="fixed inset-0 -z-10" on:click={closeCreateModal}></div>
        
        <!-- modal -->
        <div class="bg-white px-10 py-6 rounded-md max-w-xl w-full space-y-4">
            <!-- modal header -->
            <header class="mb-4">
                <h1 class="text-xl font-bold text-slate-500">Create</h1>
            </header>

            <!-- modal content -->
            <form class="row" on:submit|preventDefault={insertData}>
                <div class="space-y-4">
                    <div>
                        <label for="title" class="block text-sm font-bold ml-1 mb-1">Title</label>
                        <input type="text" name="title" id="title" autocomplete="off" required on:blur={(e) => e.target.value = (e.target.value).trim()} class="py-2 px-3 block w-full border-2 border-gray-200 rounded-md outline-none text-sm focus:border-blue-500 focus:ring-blue-500 shadow-sm" >
                    </div>
        
                    <div>
                        <label for="path" class="block text-sm font-bold ml-1 mb-1">Path</label>
                        <input type="text" name="path" id="path" autocomplete="off" required readonly on:blur={(e) => e.target.value = (e.target.value).trim()} class="py-2 px-3 block w-full border-2 border-gray-200 rounded-md outline-none text-sm focus:border-blue-500 focus:ring-blue-500 shadow-sm" on:click={openDialog}>
                    </div>
        
                    <div>
                        <label for="port" class="block text-sm font-bold ml-1 mb-1">Port</label>
                        <input type="text" name="port" id="port" autocomplete="off" required on:blur={(e) => e.target.value = (e.target.value).trim()} class="py-2 px-3 block w-full border-2 border-gray-200 rounded-md outline-none text-sm focus:border-blue-500 focus:ring-blue-500 shadow-sm" >
                    </div>
                </div>

                <div class="text-right">
                    <button type="submit" class="bg-indigo-600 hover:bg-indigo-500 leading-snug px-4 py-2 rounded-md text-md text-white font-semibold mt-4 text-sm">Save</button>
                </div>
            </form>
        </div>
    </div>
</main>