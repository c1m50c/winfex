<script lang="ts">
    import QuestionMark from "./material-icons/question-mark.svelte";
    import type DirectoryRecord from "src/types/directory-record";
    import { readSetDirectory } from "../store/directory";
    import Folder from "./material-icons/folder.svelte";
    import File from "./material-icons/file.svelte";

    export let record: DirectoryRecord;

    const setCurrentDirectoryToRecordDirectory = async () => {
        if (!record.is_directory) {
            return;
        }

        readSetDirectory(record.path);
    };
</script>


<style>
    .record {
        display: flex;
        gap: 0.5rem;
        padding: 0 0.5rem;
        flex-direction: row;
    }

    .record:hover {
        background-color: rgb(18, 18, 18);
    }

    .record :global(.material-icon) {
        fill: rgb(248, 248, 248);
        width: 1rem;
    }

    .record :global(.folder) {
        fill: rgb(230, 187, 70);
    }
</style>


<div class="record" on:dblclick={ setCurrentDirectoryToRecordDirectory }>
    {#if record.is_directory !== undefined}
        {#if record.is_directory}
            <Folder />
        {:else}
            <File />
        {/if}
    {:else}
        <QuestionMark />
    {/if}

    <p style="font-weight: bold">{ record.name }</p>
    
    {#if record.size > 0}
        <p style="margin-left: auto; text-align: right; opacity: 50%;">{ record.size }</p>
    {/if}
</div>