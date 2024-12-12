import type { Actions } from './$types';
import { API_PATH } from '$env/static/private';
import { fail } from '@sveltejs/kit';

export const actions = {
	default: async ({ request }) => {
		const formData = await request.formData();
		const query = formData.get('query');

		if (query === "") {
			return fail(400, { query, missing: true })
		}

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
				result
			};
		} catch (error: any) {
			return fail(500, { query, error: 'Fetch failed: ' + error.message });
		}
	}
} satisfies Actions;

