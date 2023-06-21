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
        text-align: center;
        padding: 1em;

        transition: 300ms ease-in background-color;
    }

    .drive:hover {
        background-color: rgb(28, 28, 28);
    }

    header :global(.material-icon) {
        width: 1.75rem;
        fill: #fff;
    }

    header {
        justify-content: center;
        align-items: center;
        max-height: 1em;
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