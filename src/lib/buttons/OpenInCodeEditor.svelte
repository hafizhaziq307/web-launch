<script>
    import { invoke } from "@tauri-apps/api/tauri";
    export let path;

    async function openInCodeEditor(path = null) {
        if (path) {
            await invoke('show_in_code_editor', {path});   
        } 
    }
</script>

<button on:click={ () => openInCodeEditor(path) }>
    <i class="fas fa-code text-xl text-gray-800 hover:text-gray-600" title="Open in code editor"></i>
</button>