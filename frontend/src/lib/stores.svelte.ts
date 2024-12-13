import { writable } from "svelte/store";
import type { SearchResultsRes } from "./model";

export const currentSearchResults = writable<SearchResultsRes | undefined>();
export const currentQuery = writable<string>("");
