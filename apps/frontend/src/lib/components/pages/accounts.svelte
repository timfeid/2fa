<script lang="ts">
	import type { AccountDetails } from '@2fa/rusty';
	import Fuse from 'fuse.js';
	import { Card, CardHeader } from '../ui/card';
	import SearchInput from '../ui/search-input/search-input.svelte';
	import AccountDialog from '../accounts/account-dialog.svelte';

	export let items: AccountDetails[];
	const fuse = new Fuse(items, { keys: ['issuer', 'username'] });
	let query = '';
	let selectedItem: AccountDetails | undefined = undefined;

	$: filteredItems = query
		? fuse.search(query).map((i) => i.item)
		: [...items].sort((a, b) => (a.issuer > b.issuer ? 1 : -1));

	function openFirst() {
		if (filteredItems.length === 0) {
			return;
		}

		openItem(filteredItems[0]);
	}

	function openItem(item: AccountDetails) {
		selectedItem = item;
	}
</script>

{#if selectedItem}
	<AccountDialog bind:account={selectedItem} />
{/if}

<div class="space-y-4">
	<form on:submit|preventDefault={openFirst}>
		<SearchInput bind:value={query}></SearchInput>
		<button type="submit" class="hidden" />
	</form>
	<!-- <Separator class="h-[3px]" /> -->
	<div class="shadow-md overflow-hidden grid grid-cols-1 gap-2">
		{#each filteredItems as item}
			<button on:click={() => openItem(item)}>
				<Card
					class="dark:bg-stone-900/10 dark:hover:bg-stone-900/25 dark:hover:text-white text-gray-300"
				>
					<CardHeader class="space-y-1 text-sm">
						<div>{item.issuer}</div>
					</CardHeader>
				</Card>
			</button>
		{/each}
	</div>
</div>
