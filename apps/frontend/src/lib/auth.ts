import { redirect } from '@sveltejs/kit';
import { accessToken } from './stores/access-token';
import { forTauri, getAccessTokenFromTauri, saveAccessTokenTauri } from './tauri';
import type { AuthResponse } from '@2fa/rusty';

export function loginRequired({ accessToken }: { accessToken?: string }) {
	if (!accessToken) {
		throw redirect(307, '/login');
	}
}

export async function refreshAccessToken() {
	const response = await fetch('/refresh-token', { method: 'post' });
	const token = await response.text();
	accessToken.set(token);

	return token;
}

export async function getAccessToken() {
	if (forTauri()) {
		return getAccessTokenFromTauri();
	}
}

export async function saveLoginDetails(details: AuthResponse) {
	if (forTauri() && details.refresh_token) {
		return saveAccessTokenTauri(details.refresh_token);
		// return console.log(await invoke('save_login', { refreshToken: details.refresh_token }));
	}

	await fetch(`/save-login`, {
		method: 'post',
		body: JSON.stringify(details),
		headers: {
			'Content-Type': 'application/json'
		}
	});
}
