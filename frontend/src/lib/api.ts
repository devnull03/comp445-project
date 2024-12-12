import { PUBLIC_API_PATH } from "$env/static/public";
import { toast } from "svelte-sonner";

async function getPage(queryID: string, page: number) {

	let res = await fetch(`${PUBLIC_API_PATH}/search-results?query_id=${queryID}&page=${page}`, {
		method: "get"
	})

	if (!res.ok) {
		toast.error("Failed to fetch search results");
	}

	return await res.json();
}


export { getPage };