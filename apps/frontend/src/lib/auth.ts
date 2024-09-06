import type { AuthResponse } from '@2fa/rusty';
import type { Cookies } from '@sveltejs/kit';
import { add } from 'date-fns';

export function setAuthCookies(cookies: Cookies, response: AuthResponse) {
	cookies.set('refresh_token', response.refresh_token || '', {
		expires: add(new Date(), { days: 365 }),
		path: '/'
	});
	cookies.set('access_token', response.access_token || '', {
		expires: add(new Date(), { days: 1 }),
		path: '/'
	});
}

export async function refreshAccessToken() {
	const response = await fetch('/refresh-token', { method: 'post' });
	return await response.text();
}
