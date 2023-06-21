<script lang="ts">
    import type DirectoryRecord from "./types/directory-record";
    import type DriveDetails from "./types/drive-details";
    import Drive from "./components/drive.svelte";

    import { invoke } from "@tauri-apps/api/tauri";

    const driveDetails: Promise<Array<DriveDetails>> = invoke("get_drive_details");

    const getDirectoryRecords = async (path: string): Promise<Array<DirectoryRecord>> => {
        return invoke("read_directory", { path: path });
    };
</script>


<style>
    .directory-contents {
        max-height: 100vh;
        overflow-y: scroll;
    }

    .drives-container {
        display: grid;
        gap: 1rem;
    }
</style>


<main>
    {#await driveDetails}
        { "" }
    {:then details}
        {#if details.length > 0}
            {#await getDirectoryRecords(details[0].path)}
                { "" }
            {:then directoryRecords}
                <div class="directory-contents">
                    {#each directoryRecords as record}
                        <p>{ JSON.stringify(record) }</p>
                    {/each}
                </div>
            {/await}
        {/if}

        <div class="drives-container" style="grid-template-columns: repeat({ details.length }, 1fr);">
            {#each details as detail }
                <Drive driveDetails={ detail } />
            {/each}
        </div>
    {/await}
</main>