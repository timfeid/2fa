<script lang="ts">
	import Account from '$lib/components/pages/account.svelte';

	export let data;
</script>

<Account item={data.item}></Account>
