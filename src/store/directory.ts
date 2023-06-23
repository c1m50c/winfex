import type DirectoryRecord from "src/types/directory-record";
import { writable, type Writable } from "svelte/store";


export type DirectoryData = {
    records: Array<DirectoryRecord>;
    path: string;
}


export const currentDirectory: Writable<DirectoryData> = writable({
    records: [  ],
    path: ""
});