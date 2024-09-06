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
	import CircularProgress from '../ui/circular-progress/circular-progress.svelte';

	let open = true;
	export let account: AccountDetails | undefined;

	let accountWithCode: AccountDetailsWithCode | undefined = undefined;
	let timeout: NodeJS.Timeout | undefined;
	let progress = 0;
	let ttl = 0;

	onMount(() => {
		if (!account) {
			open = false;
			return;
		}

		refresh(account);

		updateTtn();

		return () => clearTimeout(timeout);
	});

	async function refresh(account: AccountDetails) {
		accountWithCode = await client.query(['account.find', account.id]);
		updateTtn();
	}

	function updateTtn() {
		clearTimeout(timeout);
		if (accountWithCode) {
			ttn(accountWithCode);
		}
		timeout = setTimeout(updateTtn, 1000);
	}

	async function ttn(account: AccountDetailsWithCode) {
		const now = new Date().getTime() / 1000;
		const seconds = +account.next_step - now;
		ttl = Math.ceil(Math.max(seconds, 0));
		progress = (seconds / +account.step) * 100;
		if (ttl < 1) {
			await refresh(account);
		}
	}

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
			<div class="flex pb-4">
				<Button
					variant="ghost"
					disabled={!accountWithCode}
					class="py-4 h-auto disabled:text-gray-700"
				>
					{#if !accountWithCode}
						<Skeleton class="h-[32px] w-[86px] rounded-full" />
					{:else}
						<div class="text-4xl font-mono tracking-[.5rem]">
							{accountWithCode.code}
						</div>
					{/if}
					<Link class="w-4 h-4" />
				</Button>
				<div class="ml-auto flex flex-col justify-center items-end space-y-2 text-xs">
					<CircularProgress size={48} strokeWidth={2} {progress} display="{ttl}s" />
				</div>
			</div>
			<DialogFooter>
				<Button>Edit</Button>
			</DialogFooter>
		{/if}
	</DialogContent>
</Dialog>
