<script lang="ts">
    export let maximum: number;
    export let value: number;

    const valueAsPercentage = () => {
        return (value / maximum * 100).toFixed(2);
    }
</script>


<style>
    .progress-bar {
        position: relative;
        background-color: rgb(12, 12, 12);
        min-height: 1em;
        min-width: 7em;
    }

    .progress-indicator {
        min-height: 1em;
    }

    .progress-value {
        position: absolute;
        font-size: 0.75em;

        left: 50%;
        top: -25%;
        transform: translate(-50%, -50%);
    }
</style>


<div class="progress-bar">
    <p class="progress-value">{ valueAsPercentage() }% <span style="font-weight: bold">FULL</span></p>
    <div class="progress-indicator" style="width: { valueAsPercentage() }%; background-color: color-mix(in srgb, #ff1a1a { valueAsPercentage() }%, #1aff1a);" />
</div>