import { client } from '$lib/client';

export async function load({ params }) {
	const id = params.slug;
	const item = await client.query(['account.find', id]);
	return { item };
}
