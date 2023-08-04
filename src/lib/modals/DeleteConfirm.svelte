<script>
    export let isOpen;
    export let closeConfirm;
    export let deletedRecord;
    export let deleteData;
</script>

<div class={`bg-slate-800 bg-opacity-50 flex justify-center items-center p-6 ${isOpen ? 'fixed inset-0' : 'hidden'}` } >
    <!-- overlay -->
    <div class="fixed inset-0 -z-10" on:click={closeConfirm}></div>
    
    <!-- modal -->
    <div class="bg-white px-10 py-6 rounded-md max-w-xl w-full space-y-4">
        <!-- modal header -->
        <header>
            <h1 class="text-xl font-bold text-slate-500">Are you sure want to delete this? </h1>
        </header>

        <!-- modal content -->
        <table class="border-collapse border border-slate-500 w-full">
            <tr>
                <td class="w-1/3 bg-slate-300 border border-slate-700 p-2 font-bold tracking-wide text-sm">Title</td>
                <td class="w-2/3 border border-slate-700 p-2 text-sm">{deletedRecord.title}</td>
            </tr>
            <tr>
                <td class="w-1/3 bg-slate-300 border border-slate-700 p-2 font-bold tracking-wide text-sm">Path</td>
                <td class="w-2/3 border border-slate-700 p-2 text-sm tracking-wide">{deletedRecord.path}</td>
            </tr>
            <tr>
                <td class="w-1/3 bg-slate-300 border border-slate-700 p-2 font-bold tracking-wide text-sm">Port</td>
                <td class="w-2/3 border border-slate-700 p-2 text-sm">{deletedRecord.port}</td>
            </tr>
        </table>

        <footer class="text-right">
            <button class="bg-red-600 hover:bg-red-500 leading-snug px-4 py-2 rounded-md text-md text-white font-semibold text-sm" on:click={() => deleteData(deletedRecord.id)}>Delete</button>
        </footer>
    </div>
</div>