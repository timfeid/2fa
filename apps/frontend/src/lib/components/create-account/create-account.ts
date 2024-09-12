import type { CreateAccountArgs } from '@2fa/rusty';
import { writable } from 'svelte/store';
import { z } from 'zod';

export const formSchema = z.object({
	url: z.string().min(1),
	issuer: z.string().min(1).max(255),
	username: z.string().min(1).max(255)
});

export const createAccountList = writable<CreateAccountArgs[]>([]);

export function createAccount(account: CreateAccountArgs) {
	createAccountList.update((accounts) => {
		accounts.push(account);
		return accounts;
	});
}
