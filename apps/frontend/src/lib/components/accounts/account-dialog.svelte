<script lang="ts">
	import type { AccountDetailsWithCode } from '@2fa/rusty';
	import { Button } from '../ui/button';
	import {
		Dialog,
		DialogContent,
		DialogDescription,
		DialogFooter,
		DialogHeader,
		DialogTitle
	} from '../ui/dialog';
	import Separator from '../ui/separator/separator.svelte';
	import Account from './account.svelte';

	let open = true;
	export let account: AccountDetailsWithCode | undefined;

	$: if (!open) {
		account = undefined;
	}
</script>

<Dialog bind:open>
	<DialogContent bind:open>
		{#if account}
			<DialogHeader>
				<DialogTitle>{account.issuer}</DialogTitle>
				<DialogDescription>{account.username}</DialogDescription>
			</DialogHeader>
			<Separator></Separator>
			<Account {account} />
			<DialogFooter>
				<Button>Edit</Button>
			</DialogFooter>
		{/if}
	</DialogContent>
</Dialog>
