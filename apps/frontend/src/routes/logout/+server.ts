import { logout } from '$lib/auth.js';

export function GET(request) {
	logout(request.cookies);

	return new Response(null, {
		status: 307,
		headers: {
			location: '/'
		}
	});
}
