<script lang="ts">
	import { writable } from 'svelte/store';
	import { client } from '../lib/client';
	import { onMount } from 'svelte';

	const version = writable();
	onMount(() => client.query(['version']).then(version.set));
</script>

{$version}
