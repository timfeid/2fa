<script lang="ts">
	import Fuse from 'fuse.js';
	import { Card, CardHeader } from '../ui/card';
	import SearchInput from '../ui/search-input/search-input.svelte';
	import AccountDialog from '../accounts/account-dialog.svelte';
	import { onMount } from 'svelte';
	import { client } from '../../client';
	import CardContent from '../ui/card/card-content.svelte';
	import Account from '../accounts/account.svelte';
	import type { AccountDetailsWithCode } from '@2fa/rusty';
	import { Separator } from '../ui/separator';

	onMount(async () => {
		items = await client.query(['account.list', { query: '' }]);
	});

	let items: AccountDetailsWithCode[] = [];
	$: fuse = new Fuse(items, { keys: ['issuer', 'username'] });
	let query = '';
	let selectedItem: AccountDetailsWithCode | undefined = undefined;

	$: filteredItems = query
		? fuse.search(query).map((i) => i.item)
		: [...items].sort((a, b) => (a.issuer > b.issuer ? 1 : -1));

	function openFirst() {
		if (filteredItems.length === 0) {
			return;
		}

		openItem(filteredItems[0]);
	}

	function openItem(item: AccountDetailsWithCode) {
		selectedItem = item;
	}
</script>

{#if selectedItem}
	<AccountDialog bind:account={selectedItem} />
{/if}

<div class="space-y-3 py-3">
	<form on:submit|preventDefault={openFirst} class="px-4">
		<SearchInput bind:value={query}></SearchInput>
		<button type="submit" class="hidden" />
	</form>
	<div class="">
		{#each filteredItems as account}
			<Account {account} />
		{/each}
	</div>
</div>
