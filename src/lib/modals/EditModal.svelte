<script>
    export let isOpen;
    export let closeModal;
    export let updateData;
    export let selectedRecord;
    export let openDialog;
</script>

<div class={`bg-slate-800 bg-opacity-50 flex justify-center items-center p-6 ${isOpen ? 'fixed inset-0' : 'hidden'}` } >
    <!-- overlay -->
    <div class="fixed inset-0 -z-10" on:click={closeModal}></div>
    
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