import { writable } from "svelte/store";
import type { SearchResultsRes } from "./model";


export const currentSearchResult = writable<SearchResultsRes>()

