<script lang="ts">
	import Accounts from '$lib/components/pages/accounts.svelte';

	export let data;
</script>

<Accounts items={data.items} />
