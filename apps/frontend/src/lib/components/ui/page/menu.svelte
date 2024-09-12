<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import Plus from 'lucide-svelte/icons/badge-plus';
	import Hamburger from 'lucide-svelte/icons/ellipsis-vertical';
	import LogOut from 'lucide-svelte/icons/log-out';

	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import { isTauri, openQrScanner } from '$lib/tauri';

	async function addAccount() {
		if (isTauri) {
			return await openQrScanner();
		}
	}
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger asChild let:builder>
		<Button builders={[builder]} variant="secondary" size="icon">
			<Hamburger size={12} />
		</Button>
	</DropdownMenu.Trigger>
	<DropdownMenu.Content align="end" class="w-56">
		<DropdownMenu.Item class="cursor-pointer" on:click={addAccount}>
			<Plus class="mr-2 h-4 w-4" />
			<span>Add account</span>
		</DropdownMenu.Item>
		<DropdownMenu.Separator />
		<DropdownMenu.Item href="/logout">
			<LogOut class="mr-2 h-4 w-4" />
			<span>Log out</span>
		</DropdownMenu.Item>
	</DropdownMenu.Content>
</DropdownMenu.Root>
