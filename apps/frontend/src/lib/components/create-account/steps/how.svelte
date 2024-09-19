<script lang="ts">
	import { Pencil, ScanQrCode } from 'lucide-svelte';
	import { toast } from 'svelte-sonner';
	import { isTauri, openQrScanner } from '../../../tauri';
	import { Button } from '../../ui/button';

	function qr() {
		if (isTauri) {
			openQrScanner();
		}
	}

	function manual() {
		toast.info('Coming soon');
	}
</script>

<div class="grid grid-cols-1 gap-1 w-full">
	<Button
		variant="ghost"
		class="h-auto flex flex-col py-12 items-center space-y-3 rounded-l"
		on:click={qr}
	>
		<ScanQrCode size="32" />
		<div>Scan QR code</div>
		<div class="text-muted">Scan your screen for a valid OTP</div>
	</Button>
	<Button
		variant="ghost"
		class="h-auto flex flex-col py-12 items-center space-y-3 rounded-r"
		on:click={manual}
	>
		<Pencil size="32" />
		<div>Enter manually</div>
		<div class="text-muted">Need to add from elsewhere?</div>
	</Button>
</div>
