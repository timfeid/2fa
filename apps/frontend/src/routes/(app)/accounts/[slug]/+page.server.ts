import { client } from '$lib/client';
import { loginRequired } from '../../../../lib/auth.js';

export async function load({ params, locals }) {
	loginRequired(locals);
	const id = params.slug;
	const item = await client.query(['account.find', id]);
	return { item };
}
