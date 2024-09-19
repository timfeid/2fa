<script lang="ts">
	import { Pencil, ScanQrCode } from 'lucide-svelte';
	import { createAccountList, showCreateAccountModal } from '../create-account/create-account';
	import {
		Dialog,
		DialogContent,
		DialogDescription,
		DialogFooter,
		DialogHeader,
		DialogTitle
	} from '../ui/dialog';
	import { Button } from '../ui/button';
	import type { CreateAccountArgs } from '@2fa/rusty';
	import How from '../create-account/steps/how.svelte';
	import CreateAccount from '../create-account/create-account.svelte';
	import { toast } from 'svelte-sonner';
	import { client } from '../../client';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();
	let createAccount: CreateAccountArgs | undefined;
	$: {
		const account = $createAccountList?.at(0);
		if (account) {
			createAccount = account;
		}
	}

	let steps = [
		{
			description: 'How would you like to do that?',
			component: How
		},
		{
			description: 'Enter details',
			component: CreateAccount
		}
	];

	$: step = createAccount ? steps[1] : steps[0];

	function cancel() {
		showCreateAccountModal.set(false);
		createAccount = undefined;
	}

	let loading = false;
	async function saveAccount() {
		if (createAccount) {
			loading = true;
			console.log(createAccount);
			try {
				const response = await client.mutation(['account.create', createAccount]);
				createAccountList.update((accounts) => {
					accounts.shift();
					return accounts;
				});
				toast.success(`Added your ${createAccount.issuer} account.`);
				dispatch('create', response);
				cancel();
			} catch (e) {
				console.log(e);
			}
			loading = false;
		}
	}
</script>

<Dialog bind:open={$showCreateAccountModal} closeOnOutsideClick={false}>
	<DialogContent bind:open={$showCreateAccountModal}>
		<form class="w-full grid" method="post" on:submit|preventDefault={saveAccount}>
			<DialogHeader>
				<DialogTitle>Create account</DialogTitle>
				<DialogDescription>{step.description}</DialogDescription>
			</DialogHeader>
			<div class="py-4">
				<svelte:component this={step.component} account={createAccount} />
			</div>
			<DialogFooter>
				<Button variant="ghost" on:click={cancel}>Cancel</Button>
				{#if createAccount}
					<Button {loading} type="submit">Save</Button>
				{/if}
			</DialogFooter>
		</form>
	</DialogContent>
</Dialog>
