import type DirectoryRecord from "src/types/directory-record";
import { writable, type Writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/tauri";


export type DirectoryData = {
    records: Array<DirectoryRecord>,
    path: string,
}


export const currentDirectory: Writable<DirectoryData> = writable({
    records: [  ],
    path: ""
});


/** Reads a directory and populates the `currentDirectory` store value with the directory's records. */
export const readSetDirectory = async (path: string) => {
    const directoryRecords: Array<DirectoryRecord> = await invoke("read_directory", { path: path });
    const directoryData: DirectoryData = { records: directoryRecords, path: path };

    currentDirectory.set(directoryData);
};