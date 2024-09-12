<script lang="ts">
	import type { CreateAccountArgs } from '@2fa/rusty';
	import { createEventDispatcher } from 'svelte';
	import { toast } from 'svelte-sonner';
	import { superForm, type SuperForm } from 'sveltekit-superforms';
	import { client } from '../../client';
	import Account from '../accounts/account.svelte';
	import { Button } from '../ui/button';
	import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from '../ui/dialog';
	import { FormControl, FormDescription, FormField, FormFieldErrors, FormLabel } from '../ui/form';
	import Input from '../ui/input/input.svelte';
	import { createAccountList } from './create-account';

	const dispatch = createEventDispatcher();
	let open = false;

	$: preview = {
		id: '',
		issuer: account?.issuer || '',
		username: account?.username || '',
		code: '123456',
		next_step: '',
		step: '30'
	};

	$: {
		if ($createAccountList.length > 0) {
			open = true;
		}
	}

	let loading = false;
	async function saveAccount() {
		if (account) {
			loading = true;
			console.log(account);
			try {
				const response = await client.mutation(['account.create', account]);
				createAccountList.update((accounts) => {
					accounts.shift();
					return accounts;
				});
				toast.success(`Added your ${account.issuer} account.`);
				open = false;
				dispatch('create', response);
			} catch (e) {
				console.log(e);
			}
			loading = false;
		}
	}

	$: {
		const d = $createAccountList?.at(0);
		if (d) {
			account = d;
		}
	}
	let account: CreateAccountArgs | undefined;

	let form: SuperForm<CreateAccountArgs> | undefined;
	$: form = account ? superForm(account) : undefined;
</script>

<Dialog bind:open>
	<DialogContent bind:open noClose>
		{#if account && form}
			<form method="post" on:submit|preventDefault={saveAccount}>
				<DialogHeader>
					<DialogTitle>Create account</DialogTitle>
				</DialogHeader>

				<div class="py-6">
					<FormField {form} name="issuer">
						<FormControl let:attrs>
							<FormLabel class="mb-0">Issuer</FormLabel>
							<Input {...attrs} bind:value={account.issuer} />
						</FormControl>
						<FormDescription />
						<FormFieldErrors />
					</FormField>
					<FormField {form} name="username">
						<FormControl let:attrs>
							<FormLabel>Account name</FormLabel>
							<Input {...attrs} bind:value={account.username} />
						</FormControl>
						<FormDescription />
						<FormFieldErrors />
					</FormField>

					<div class="pt-2">
						<h2 class="text-sm text-muted mb-1">Preview</h2>
						<Account preview account={preview} />
					</div>
				</div>

				<DialogFooter>
					<Button variant="ghost" on:click={() => (open = false)}>Cancel</Button>
					<Button {loading} type="submit">Save</Button>
				</DialogFooter>
			</form>
		{/if}
	</DialogContent>
</Dialog>
