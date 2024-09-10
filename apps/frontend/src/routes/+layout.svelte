<script lang="ts">
	import { onMount } from 'svelte';
	import '../app.scss';
	import { getAccessToken } from '../lib/auth';
	import { accessToken } from '../lib/stores/access-token';
	import { goto } from '$app/navigation';
	export let data;

	onMount(async () => {
		const at = await getAccessToken();
		accessToken.set(at || undefined);

		if (!at) {
			return goto('/login');
		}

		return goto('/accounts');
	});
</script>

<svelte:head>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
	<link
		rel="stylesheet"
		media="all"
		href="https://fonts.googleapis.com/css2?family=Bitter:wght@100;200;300;400;500;600;700;800;900&family=Noto+Sans+Mono:wght@400;500;600&family=Open+Sans:ital,wght@0,300;0,400;0,500;0,600;0,700;1,400&display=swap"
	/>
</svelte:head>

<slot />
