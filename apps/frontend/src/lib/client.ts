import { FetchTransport, WebsocketTransport, createClient } from '@rspc/client';
import type { Procedures } from '@2fa/rusty';
import { PUBLIC_API_URL } from '$env/static/public';
import { browser } from '$app/environment';
import { get } from 'svelte/store';
import { accessToken } from './stores/access-token';
import { refreshAccessToken } from './auth';
import { decodeJwt } from 'jose';
import { isPast } from 'date-fns/isPast';

const transport = new FetchTransport(PUBLIC_API_URL, async (input, init) => {
	const at = get(accessToken);
	if (at) {
		const payload = decodeJwt(at);
		if (browser && isPast(new Date(payload.exp! * 1000))) {
			await refreshAccessToken();
		}
	}

	return fetch(input, {
		...init,
		headers: {
			authorization: get(accessToken) || ''
		}
	});
});

export const websocketClient = createClient<Procedures>({
	transport: browser
		? new WebsocketTransport(PUBLIC_API_URL.replace('http', 'ws') + '/ws')
		: transport
});

export const client = createClient<Procedures>({
	transport
});
