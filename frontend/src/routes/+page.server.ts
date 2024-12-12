import type { Actions } from './$types';
import { API_PATH } from '$env/static/private';
import { fail } from '@sveltejs/kit';
import type { SearchResultsRes } from '$lib/model';

export const actions = {
	default: async ({ request }) => {
		const formData = await request.formData();
		const query = formData.get('query');

		try {
			const response = await fetch(`${API_PATH}/search?search_text=${query}`, {
				method: 'GET',
			});

			const result = await response.json();
			if (!response.ok) {
				return fail(response.status, { query, message: result.message, error: true });
			}

			return {
				success: true,
				queryText: query,
				result: result as SearchResultsRes
			};
		} catch (error: any) {
			return fail(500, { query, error: 'Fetch failed: ' + error.message });
		}
	}
} satisfies Actions;

