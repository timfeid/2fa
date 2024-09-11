<script lang="ts">
	import NavLink from '$lib/components/ui/nav-link.svelte';
	import Menu from '$lib/components/ui/page/menu.svelte';
</script>

<div class="sticky top-0 w-full border-b flex h-12 items-center text-sm bg-secondary">
	<div class="max-w-md mx-auto items-center flex w-full pl-4">
		<!-- <ul class="flex space-x-6 items-center">
			<li>
				<NavLink href="/accounts">Accounts</NavLink>
			</li>
		</ul> -->
		<div class="text-muted">2FA</div>
		<div class="ml-auto">
			<Menu />
		</div>
	</div>
</div>
<div class="w-full max-w-md mx-auto">
	<slot />
</div>
