<script lang="ts">
	import type { AccountDetails, AccountDetailsWithCode } from '@2fa/rusty';
	import Link from 'lucide-svelte/icons/link';
	import { Button } from '../ui/button';
	import {
		DialogContent,
		DialogDescription,
		DialogFooter,
		Dialog,
		DialogHeader,
		DialogTitle
	} from '../ui/dialog';
	import Separator from '../ui/separator/separator.svelte';
	import { onMount } from 'svelte';
	import { client } from '../../client';
	import { Skeleton } from '../ui/skeleton';

	let open = true;
	export let account: AccountDetails | undefined;

	let accountWithCode: AccountDetailsWithCode | undefined = undefined;

	onMount(async () => {
		if (!account) {
			open = false;
			return;
		}

		// await new Promise((resolve) => setTimeout(() => resolve(true), 5000));
		accountWithCode = await client.query(['account.find', account.id]);
	});

	$: if (!open) {
		account = undefined;
	}
</script>

<Dialog bind:open>
	<DialogContent>
		{#if account}
			<DialogHeader>
				<DialogTitle>{account.issuer}</DialogTitle>
				<DialogDescription>{account.username}</DialogDescription>
			</DialogHeader>
			<Separator></Separator>
			<div class="flex space-x-2 items-center">
				{#if !accountWithCode}
					<Skeleton class="h-[32px] w-[86px] rounded-full" />
				{:else}
					<div class="text-2xl font-mono">{accountWithCode?.code}</div>
				{/if}
				<button disabled={!accountWithCode} class=" disabled:text-gray-700">
					<Link class="w-4 h-4" />
				</button>
			</div>
			<DialogFooter>
				<Button>Edit</Button>
			</DialogFooter>
		{/if}
	</DialogContent>
</Dialog>
