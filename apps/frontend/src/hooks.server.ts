import type { Handle } from '@sveltejs/kit';
import { sequence } from '@sveltejs/kit/hooks';
import { isPast } from 'date-fns/isPast';
import { decodeJwt, importSPKI, jwtVerify, type JWTPayload } from 'jose';
import { setAuthCookies } from './lib/auth';
import { refreshAccessToken } from './lib/mutations/auth';

const publicKey = await importSPKI(
	`-----BEGIN PUBLIC KEY-----
  MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAi1SeMzYfXQQsHtHFTvOu
  QBN9+VTPEEipTPLbGC36LVbEmPlk76OfW5kZIdBaA+ZtQzPb9eWSF/6MK55TiVGA
  uLqOT+klxaUxGXC/r3HTz3E2xJDh1CxIrVZjUs5B+A1aEIrEeU6rHxs/xDgce71k
  DaQY2cyF2jmZauiwiNYVUS1m/4Jbcex8EvkaXgu8h0OfdgKUJN8DN3W8b27MmOl0
  F/k4ZKSkHmCqVKBRd9akfcTYTt3+QkS4CI86G4ysMpXH0D+ssAQRxhJY80yFcAs+
  0In4gvlSj63c0JycPMTl7afyZOQd3Dm90nQCZyAIwdbIGzqRyEoUFGHtrBTE0YJH
  1QIDAQAB
  -----END PUBLIC KEY-----`,
	'RS256'
);

const refreshUser: Handle = async ({ event, resolve }) => {
	let refreshToken = event.cookies.get('refresh_token');
	let accessToken = event.cookies.get('access_token');
	let data: JWTPayload | undefined;

	if (accessToken || refreshToken) {
		if (accessToken) {
			try {
				data = (await jwtVerify(accessToken, publicKey, {}))?.payload;
			} catch (e) {
				console.log('oh no', e);
			}
		}

		try {
			if ((!data || isPast(new Date((data as JWTPayload).exp! * 1000))) && refreshToken) {
				const response = await refreshAccessToken(refreshToken);
				setAuthCookies(event.cookies, response);
				accessToken = response.access_token || undefined;
				refreshToken = response.refresh_token || undefined;
			}
		} catch (e) {
			console.log('oh no', e);
		}

		event.locals.accessToken = accessToken;
	}

	return resolve(event);
};

const setUser: Handle = async ({ event, resolve }) => {
	if (event.locals.accessToken) {
		event.locals.user = decodeJwt(event.locals.accessToken);
	}

	return resolve(event);
};

export const handle = sequence(refreshUser, setUser);
