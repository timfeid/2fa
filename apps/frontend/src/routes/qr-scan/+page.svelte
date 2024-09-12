<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { prepForQrCodeScan, scanForOtpQrs } from '$lib/tauri';
	import { toast } from 'svelte-sonner';

	async function scan() {
		const details = await prepForQrCodeScan();
		const results = await scanForOtpQrs(details);

		processResults(results);
	}

	function noResults() {
		toast.error('Unable to find a valid OTP QR');
	}

	function processResults(input: string[]) {
		if (input.length === 0) {
			return noResults();
		}
	}
</script>

<div class="absolute resizable flex items-end justify-center p-4">
	<Button on:click={scan}>Capture QR</Button>
</div>

<style lang="scss">
	.resizable {
		background: transparent;
		border: 5px red solid;
		width: 100vw;
		height: 100vh;
	}

	:global(body) {
		background: transparent !important;
	}
</style>
