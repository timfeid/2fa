<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import Hamburger from 'lucide-svelte/icons/ellipsis-vertical';
	import LogOut from 'lucide-svelte/icons/log-out';

	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger asChild let:builder>
		<Button builders={[builder]} variant="ghost" size="icon">
			<Hamburger size={16} />
		</Button>
	</DropdownMenu.Trigger>
	<DropdownMenu.Content align="end" class="w-56">
		<DropdownMenu.Item href="/logout">
			<LogOut class="mr-2 h-4 w-4" />
			<span>Log out</span>
		</DropdownMenu.Item>
	</DropdownMenu.Content>
</DropdownMenu.Root>
