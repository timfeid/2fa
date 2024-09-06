import { redirect } from '@sveltejs/kit';

export function load({ locals }) {
	if (locals.accessToken) {
		throw redirect(307, '/accounts');
	}

	throw redirect(307, '/login');
}
