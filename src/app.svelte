<script lang="ts">
    import type DriveDetails from "./types/drive-details";
    import Drive from "./components/drive.svelte";

    import { invoke } from "@tauri-apps/api/tauri";

    const driveDetails: Promise<Array<DriveDetails>> = invoke("get_drive_details");
</script>


<style>
    .drives-container {
        display: grid;
        gap: 1rem;
    }
</style>


<main>
    {#await driveDetails}
        Fetching Drive Details...
    {:then details}
        <div class="drives-container">
            {#each details as detail }
                <Drive driveDetails={ detail } />
            {/each}
        </div>
    {/await}
</main>