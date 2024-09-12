<script lang="ts">
	import { Dialog as DialogPrimitive } from 'bits-ui';
	import Cross2 from 'lucide-svelte/icons/x';
	import * as Dialog from './index.js';
	import { cn, flyAndScale } from '$lib/utils.js';
	import { Button } from '../button/index.js';

	type $$Props = DialogPrimitive.ContentProps & { open?: boolean; noClose?: boolean };

	let className: $$Props['class'] = undefined;
	export let open: $$Props['open'] = true;
	export let transition: $$Props['transition'] = flyAndScale;
	export let showClose: $$Props['noClose'] = false;
	export let transitionConfig: $$Props['transitionConfig'] = {
		duration: 200
	};
	export { className as class };
</script>

<Dialog.Portal>
	<Dialog.Overlay />
	<DialogPrimitive.Content
		{transition}
		{transitionConfig}
		class={cn(
			'bg-background fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border p-5 shadow-lg sm:rounded-lg md:w-full',
			className
		)}
		{...$$restProps}
	>
		<slot />

		{#if showClose}
			<Button
				on:click={() => (open = false)}
				variant="ghost"
				size="icon"
				class="absolute top-3 right-4"
			>
				<Cross2 class="h-4 w-4" />
			</Button>
			<span class="sr-only">Close</span>
		{/if}
		<!-- </DialogPrimitive.Close> -->
	</DialogPrimitive.Content>
</Dialog.Portal>
