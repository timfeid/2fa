<script lang="ts">
	import { onMount } from 'svelte';
	import { client } from '../../client';
	import type { AccountDetailsWithCode } from '@2fa/rusty';

	export let id: string;

	let item: AccountDetailsWithCode | undefined = undefined;
	onMount(async () => {
		item = await client.query(['account.find', id]);
	});
</script>

{#if item}
	<p class="text-sm font-medium text-gray-900 truncate">
		{item.id}
		{item.issuer}
		{item.username}
	</p>
	<h1 class="text-2xl font-medium">{item.code}</h1>
{/if}
