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

	export let account: AccountDetailsWithCode;

	let accountWithCode: AccountDetailsWithCode = account;
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
</script>

<div
	class=" relative group py-2 px-4 rounded overflow-hidden text-sm hover:bg-stone-900/50"
	style="--secondary-color: rgba({secondaryColor.r}, {secondaryColor.g}, {secondaryColor.b}, 20); --primary-color: rgba({color.r}, {color.g}, {color.b}, 20);"
>
	<div class="flex items-center">
		<div class="flex flex-col justify-center items-end space-y-2 font-mono">
			<div
				class="text-xl rounded h-10 w-10 bg-[var(--primary-color)] flex items-center justify-center"
			>
				<CircularProgress color="text-white/50" size={32} strokeWidth={2} {progress}>
					<div class="hidden group-hover:block text-xs">{ttl}</div>
					<div class="block group-hover:hidden">
						{account.issuer.charAt(0)}
					</div>
				</CircularProgress>
			</div>
		</div>
		<div class="flex-grow ml-3">
			<div>
				{account.issuer}
			</div>
			{#if account.username}
				<div class="text-muted">{account.username}</div>
			{/if}
		</div>
		<div class="flex items-center">
			<Button
				class="pr-0 pl-8 relative group group-hover:scale-90 transition duration-300"
				variant="ghost"
				on:click={copy}
			>
				<Link class="hidden group-hover:block absolute left-3" size="12" />

				<div class="code rounded text-xl font-mono tracking-[.5rem]">
					{accountWithCode.code}
				</div>
			</Button>
			<div class="w-6">
				<Button class="h-8 hidden group-hover:flex px-2 py-0" variant="ghost">
					<Edit size="12" />
				</Button>
			</div>
		</div>
	</div>
</div>

<style lang="scss">
	.code {
		background: linear-gradient(90deg, var(--primary-color), var(--secondary-color));
		background-clip: text;
		filter: brightness(3) grayscale(0.8);
		-webkit-background-clip: text;
		color: transparent;
	}
</style>
