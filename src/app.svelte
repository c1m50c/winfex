<script lang="ts">
    import Drive from "./components/drive.svelte";
import type DriveDetails from "./types/drive-details";
    import { invoke } from "@tauri-apps/api/tauri";

    const driveDetails: Promise<Array<DriveDetails>> = invoke("get_drive_details");
</script>


<main>
    <h1>Winfex</h1>
    {#await driveDetails}
        Fetching Drive Details...
    {:then details}
        {#each details as detail }
            <Drive driveDetails={ detail } />
        {/each}
    {/await}
</main>