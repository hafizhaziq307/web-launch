<script>
    // import { open } from "@tauri-apps/api/dialog";
    import { appDir } from "@tauri-apps/api/path";
    import { readDir, BaseDirectory } from '@tauri-apps/api/fs';
    import { open } from '@tauri-apps/api/shell';
    import { tauri } from '@tauri-apps/api';

    const data = [
        {
            id: 1,
            title: 'php',
            path: 'C:/laragon/www',
            port: "80"
        }
    ];

    // async function openMyDialog() {
    //     let directory = await open({
    //         directory: true,
    //         multiple: false,
    //     });

    //     console.log(directory);

    //     if (directory) {
    //         directory = directory.replace(/\\/g, "/");
    //         console.log(directory);
    //     }
    // }

    async function getProjects() {
        let webServers = Array.from(data);

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

        // console.log(webServers);
        return webServers;
    }

    async function openInFileExplorer(path = null) {
        // path = path.replace(/\//g, "\\");
        path = "C:\\laragon\\www\\portfolio";
        console.log(path);
        await tauri.invoke('show_in_folder', {path});
    }
</script>

<main class="min-h-screen w-full flex h-[100vh]">
    <div id="content" class="flex h-full flex-col w-full">
        <div class="grow bg-gray-100 px-6 divide-y-2 divide-gray-500">
            {#await getProjects()}
                <p>...loading</p>
            {:then items}
                {#each items as item}
                    <article class="space-y-2 py-5">
                        <header class="text-xl font-bold">{item.title}</header>
            
                        <div class="grid grid-cols-6 gap-4">
                            {#each item.entries as entry}
                                <article class="bg-white rounded-md shadow space-y-3">
                                    <header class="px-4 pt-4">
                                        <a href="{'http://localhost:' + item.port + '/' + entry.name}" target="_blank" class="hover:underline hover:text-blue-600">{entry.name}</a>
                                    </header>
                                    <hr>
                                    <footer class="grid grid-cols-2 px-4 pb-2">
                                        <div class="text-center">
                                            <i class="fas fa-code cursor-pointer text-xl text-gray-800 hover:text-gray-600" title="open in code editor"></i>
                                        </div>
                                        <div class="text-center">
                                            <button on:click={ () => openInFileExplorer(entry.path)}>
                                                <i class="fas fa-folder text-xl text-gray-800 hover:text-gray-600" title="open in file explorer"></i>
                                            </button>
                                        </div>
                                    </footer>
                                </article>
                            {/each}
                        </div>
                    </article>
                {/each}
            {:catch error}
                <p style="color: red">{error.message}</p>
            {/await}
                

           
        </div>
    </div>
</main>