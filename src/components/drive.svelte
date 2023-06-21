<script lang="ts">
    import type DriveDetails from "src/types/drive-details";
    import Storage from "./material-icons/storage.svelte";
    import ProgressBar from "./progress-bar.svelte";

    export let driveDetails: DriveDetails;

    const bytesToGB = (x: number) => {
        return (x / 1_073_741_824).toFixed(2)
    };
</script>


<style>
    .drive {
        background-color: rgb(18, 18, 18);
        border-radius: 1em;
        text-align: center;
        padding: 1em;
    }

    header :global(.material-icon) {
        fill: #fff;
        width: 1.75rem;
    }

    header {
        justify-content: center;
        align-items: center;
        display: flex;
        gap: 0.5em;
    }
</style>


<div class="drive">
    <header>
        <Storage />
        <h2>{ driveDetails.path }</h2>
    </header>

    <p>{ bytesToGB(driveDetails.available_space) } / { bytesToGB(driveDetails.total_space) } GB</p>

    <ProgressBar value={ driveDetails.total_space - driveDetails.available_space } maximum={ driveDetails.total_space } />
</div>