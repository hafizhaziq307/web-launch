<script>
    import { invoke } from "@tauri-apps/api/tauri";
    export let path;

    async function openInFileExplorer(path = null) {
        if (path) {
            await invoke('show_in_folder', {path});   
        } 
    }
</script>

<button on:click={ () => openInFileExplorer(path) }>
    <i class="fas fa-folder text-xl text-gray-800 hover:text-gray-600" title="Open in file explorer"></i>
</button>