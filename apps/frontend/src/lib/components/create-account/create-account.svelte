<script lang="ts">
	import type { CreateAccountArgs } from '@2fa/rusty';
	import { superForm, type SuperForm } from 'sveltekit-superforms';
	import Account from '../accounts/account.svelte';
	import { FormControl, FormDescription, FormField, FormFieldErrors, FormLabel } from '../ui/form';
	import Input from '../ui/input/input.svelte';

	$: preview = {
		id: '',
		issuer: account?.issuer || '',
		username: account?.username || '',
		code: 'XXXXXX',
		next_step: '',
		step: '30'
	};

	export let account: CreateAccountArgs;

	let form: SuperForm<CreateAccountArgs> | undefined;
	$: form = account ? superForm(account) : undefined;
</script>

{#if form}
	<FormField {form} name="issuer">
		<FormControl let:attrs>
			<FormLabel class="mb-0">Issuer</FormLabel>
			<Input {...attrs} bind:value={account.issuer} />
		</FormControl>
		<FormDescription />
		<FormFieldErrors />
	</FormField>
	<FormField {form} name="username">
		<FormControl let:attrs>
			<FormLabel>Account name</FormLabel>
			<Input {...attrs} bind:value={account.username} />
		</FormControl>
		<FormDescription />
		<FormFieldErrors />
	</FormField>

	<div class="pt-2">
		<h2 class="text-sm text-muted mb-1">Preview</h2>
		<Account preview account={preview} />
	</div>
{/if}
