<script lang="ts">
    import { type DirectoryData, readSetDirectory, currentDirectory as currentDirectoryStore } from "./store/directory";
    import type DriveDetails from "./types/drive-details";
    import Record from "./components/record.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import Drive from "./components/drive.svelte";

    const driveDetails: Promise<Array<DriveDetails>> = invoke("get_drive_details");
    let currentDirectory: DirectoryData;

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
        {/each}
    </div>

    {#await driveDetails then details}
        <div class="drives-container" on:load={ readSetDirectory(details[0].path) }>
            {#each details as detail }
                <Drive driveDetails={ detail } />
            {/each}
        </div>
    {/await}
</main>