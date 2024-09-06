<script lang="ts">
	import type { AccountDetailsWithCode } from '@2fa/rusty';

	export let item: AccountDetailsWithCode;
</script>

<div class="text-center">
	<p class="text-sm font-medium truncate">
		{item.issuer}:
		{item.username}
	</p>
	<h1 class="text-2xl font-medium">{item.code}</h1>
</div>
