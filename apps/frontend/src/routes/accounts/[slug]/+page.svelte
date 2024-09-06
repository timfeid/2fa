<script lang="ts">
	import Account from '../../../lib/components/pages/account.svelte';
	export let data;
	$: id = data.id;
</script>

<Account {id}></Account>
