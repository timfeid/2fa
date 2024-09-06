<script lang="ts">
	import { onMount } from 'svelte';
	import '../app.scss';
	export let data;
	onMount(() => {
		localStorage.setItem('access_token', data.accessToken || '');
	});
</script>

<slot />
