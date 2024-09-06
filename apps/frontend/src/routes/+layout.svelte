<script lang="ts">
	import '../app.scss';
	import NavLink from '../lib/components/ui/nav-link.svelte';
	import { accessToken } from '../lib/stores/access-token';
	export let data;

	accessToken.set(data.accessToken);
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

<div class="sticky top-0 w-full border-b flex h-12 items-center text-sm">
	<div class="container">
		<ul class="flex space-x-6">
			<li>
				<NavLink href="/">Home</NavLink>
			</li>
			<li>
				<NavLink href="/accounts">Accounts</NavLink>
			</li>
		</ul>
	</div>
</div>
<div class="w-full max-w-md mx-auto md:max-w-2xl pt-6">
	<slot />
</div>
