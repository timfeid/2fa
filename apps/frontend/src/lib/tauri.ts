import { invoke, isTauri as forTauri } from '@tauri-apps/api/core';
import { client } from './client';
import { browser } from '$app/environment';

export const isTauri = browser && forTauri();

export async function getRefreshTokenFromTauri() {
	return (await invoke('get_refresh_token')) as string | null;
}

export async function saveRefreshTokenTauri(token: string) {
	const response = await invoke('set_refresh_token', { token });

	return response;
}

export async function getAccessTokenWithTauri() {
	const refreshToken = await getRefreshTokenFromTauri();
	if (refreshToken) {
		const response = await client.mutation(['authentication.refresh_token', refreshToken]);
		return response.access_token;
	}
}
