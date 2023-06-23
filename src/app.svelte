<script lang="ts">
    import { type DirectoryData, currentDirectory as currentDirectoryStore } from "./store/directory";
    import type DirectoryRecord from "./types/directory-record";
    import type DriveDetails from "./types/drive-details";
    import Record from "./components/record.svelte";
    import Drive from "./components/drive.svelte";

    import { invoke } from "@tauri-apps/api/tauri";


    const driveDetails: Promise<Array<DriveDetails>> = invoke("get_drive_details");
    let currentDirectory: DirectoryData;

    const getDirectoryRecords = async (path: string): Promise<Array<DirectoryRecord>> => {
        return invoke("read_directory", { path: path });
    };

    const setCurrentDirectoryStoreToRootDrive = async (path: string) => {
        const directoryRecords = await getDirectoryRecords(path);

        const directoryData: DirectoryData = {
            records: directoryRecords,
            path: path,
        };

        currentDirectoryStore.set(directoryData);
    };

    currentDirectoryStore.subscribe((value: DirectoryData) => { currentDirectory = value });
</script>


<style>
    main {
        display: grid;
    }

    .directory-contents {
        overflow-y: hidden;
    }

    .drives-container {
        display: grid;
        gap: 1rem;
    }
</style>


<main>
    <div class="directory-contents">
        {#each currentDirectory.records as directoryRecord}
            <Record record={ directoryRecord } />
        {:else}
            <p>Failed to load files from current directory</p>
        {/each}
    </div>

    {#await driveDetails then details}
        <div class="drives-container" on:load={ setCurrentDirectoryStoreToRootDrive(details[0].path) }>
            {#each details as detail }
                <Drive driveDetails={ detail } />
            {/each}
        </div>
    {/await}
</main>