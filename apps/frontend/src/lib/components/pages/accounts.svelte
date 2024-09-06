<script lang="ts">
	import { onMount } from 'svelte';
	import { client } from '../../client';
	import type { AccountDetails } from '@2fa/rusty';

	let items: AccountDetails[] = [];
	onMount(async () => {
		items = await client.query(['account.list', { search: null }]);
	});
</script>

<div class="w-full max-w-md mx-auto bg-white shadow-md overflow-hidden md:max-w-2xl">
	<ul class="divide-y divide-gray-200">
		{#each items as item}
			<li class="p-4 flex space-x-4 justify-center">
				<a href="/accounts/{item.id}" class="flex-1 min-w-0 flex items-center">
					<p class="text-sm font-medium text-gray-900 truncate">
						{item.id}
						{item.issuer}
						{item.username}
					</p>
				</a>
			</li>
		{/each}
	</ul>
</div>
