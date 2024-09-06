import { client } from '$lib/client';
import { loginRequired } from '../../../lib/auth.js';

export async function load({ locals }) {
	loginRequired(locals);
	const items = await client.query(['account.list', { search: null }]);

	return { items };
}
