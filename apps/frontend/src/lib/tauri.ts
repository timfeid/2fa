import { env } from '$env/dynamic/public';
import { invoke } from '@tauri-apps/api/core';

export function forTauri() {
	return !!env.PUBLIC_FOR_TAURI;
}

export async function getAccessTokenFromTauri() {
	return (await invoke('get_access_token')) as string | null;
}

export async function saveAccessTokenTauri(token: string) {
	const response = await invoke('set_access_token', { token });

	console.log(response);

	return response;
}
