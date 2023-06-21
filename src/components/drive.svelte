<script lang="ts">
    import type DriveDetails from "src/types/drive-details";

    export let driveDetails: DriveDetails;

    const bytesToGB = (x: number) => {
        return (x / 1_073_741_824).toFixed(2)
    };
</script>


<style>
    .bold-span {
        font-weight: bold;
    }
</style>


<div class="drive">
    <h2>Drive { driveDetails.path }</h2>

    <p>Available Space: <span class="bold-span">{ bytesToGB(driveDetails.available_space) } GB</span></p>
    <p>Total Space: <span class="bold-span">{ bytesToGB(driveDetails.total_space) } GB</span></p>

    <progress value={ driveDetails.total_space - driveDetails.available_space } max={ driveDetails.total_space } />
</div>