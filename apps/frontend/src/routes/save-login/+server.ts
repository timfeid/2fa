import { setAuthCookies } from '../../lib/auth.js';

export async function POST(req) {
	setAuthCookies(req.cookies, await req.request.json());

	return new Response(null, { status: 201 });
}
