<script lang="ts">
	import NavLink from '$lib/components/ui/nav-link.svelte';
	import Menu from '$lib/components/ui/page/menu.svelte';
</script>

<div class="sticky top-0 w-full border-b flex h-12 items-center text-sm">
	<div class="max-w-md mx-auto flex w-full px-4">
		<ul class="flex space-x-6 items-center">
			<li>
				<NavLink href="/accounts">Accounts</NavLink>
			</li>
		</ul>
		<div class="ml-auto">
			<Menu />
		</div>
	</div>
</div>
<div class="w-full max-w-md mx-auto pt-4 px-4">
	<slot />
</div>
