import { FetchTransport, WebsocketTransport, createClient } from '@rspc/client';

import type { Procedures } from '@2fa/rusty';
// import { get } from 'svelte/store';
import { PUBLIC_API_URL } from '$env/static/public';
import { browser } from '$app/environment';

const transport = new FetchTransport(PUBLIC_API_URL, (input, init) =>
	fetch(input, {
		...init,
		headers: {
			authorization: browser ? localStorage.getItem('access_token') || '' : ''
		}
	})
);

export const websocketClient = createClient<Procedures>({
	transport: browser
		? new WebsocketTransport(PUBLIC_API_URL.replace('http', 'ws') + '/ws')
		: transport
});

export const client = createClient<Procedures>({
	transport
});
