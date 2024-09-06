import { client } from '$lib/client';

export async function load() {
	const items = await client.query(['account.list', { search: null }]);

	return { items };
}
