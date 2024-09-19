<script lang="ts">
	import type { AccountDetailsWithCode } from '@2fa/rusty';
	import Link from 'lucide-svelte/icons/link';
	import Edit from 'lucide-svelte/icons/settings';
	import { onMount } from 'svelte';
	import { client } from '../../client';
	import { Button } from '../ui/button';
	import CircularProgress from '../ui/circular-progress/circular-progress.svelte';
	import { toast } from 'svelte-sonner';

	import { getColor } from '../../colors';
	import { copyToClipboard } from '../../utils';
	import AccountAvatar from './account-avatar.svelte';

	export let account: AccountDetailsWithCode;

	let accountWithCode: AccountDetailsWithCode = account;
	$: accountWithCode = account;
	let timeout: NodeJS.Timeout;
	let progress = 0;
	let ttl = 0;

	$: color = getColor(account.issuer);
	$: secondaryColor = getColor(account.username || account.issuer);

	onMount(() => {
		updateTtn();

		return () => clearTimeout(timeout);
	});

	async function copy() {
		await copyToClipboard(accountWithCode.code);
		toast.success(`Copied code for ${accountWithCode.issuer}`);
	}

	async function refresh(account: AccountDetailsWithCode) {
		accountWithCode = await client.query(['account.find', account.id]);
		updateTtn();
	}

	function updateTtn() {
		clearTimeout(timeout);
		if (accountWithCode && accountWithCode.id) {
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
	export let preview = false;
</script>

<div
	class=" relative py-1 border-b dark:border-b-0 xs:py-2 px-4 group rounded overflow-hidden text-sm hover:bg-stone-100 dark:hover:bg-stone-900/50 flex items-center w-full max-w-full"
	class:bg-stone-900={preview}
	class:!py-4={preview}
	class:preview
	style="--secondary-color: rgba({secondaryColor.r}, {secondaryColor.g}, {secondaryColor.b}, 20); --primary-color: rgba({color.r}, {color.g}, {color.b}, 20);"
>
	<div class=" flex-shrink-0">
		<AccountAvatar {account}>
			{#if !preview}
				<div class="p-1 w-full h-full">
					<CircularProgress color="text-white/50" strokeWidth={3} {progress}>
						<div class="hidden group-hover:block text-xs">{ttl}</div>
						<div class="block group-hover:hidden">
							{account.issuer.charAt(0)}
						</div>
					</CircularProgress>
				</div>
			{:else}
				<div class="block group-hover:hidden">
					{account.issuer.charAt(0)}
				</div>
			{/if}
		</AccountAvatar>
	</div>
	<div class="ml-2 xs:ml-3 whitespace-nowrap min-w-0 text-xs xs:text-base">
		<div class="overflow-ellipsis overflow-hidden">
			{account.issuer}
		</div>
		{#if account.username}
			<div class="text-muted overflow-ellipsis overflow-hidden font-thin">{account.username}</div>
		{/if}
	</div>
	<Button
		class="ml-auto pr-0 pl-8 relative group group-hover:scale-90 transition duration-300 flex-shrink-0"
		variant="ghost"
		on:click={copy}
	>
		<Link class="hidden group-hover:block absolute left-3" size="12" />

		<div class="code rounded xs:text-xl font-mono tracking-[.5rem]">
			{accountWithCode.code}
		</div>
	</Button>
	{#if !preview}
		<div class="w-6 flex-shrink-0">
			<Button class="h-8 hidden group-hover:flex px-2 py-0" variant="ghost">
				<Edit size="12" />
			</Button>
		</div>
	{/if}
</div>

<style lang="scss">
	:global(.dark) {
		.code {
			filter: brightness(3) grayscale(0.8);
		}
	}
	.code {
		background: linear-gradient(90deg, var(--primary-color), var(--secondary-color));
		background-clip: text;
		filter: brightness(-3) grayscale(0.8);
		-webkit-background-clip: text;
		color: transparent;
	}
	.preview {
		pointer-events: none;
	}
</style>
