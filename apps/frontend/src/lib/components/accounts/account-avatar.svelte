<script lang="ts">
	import type { AccountDetails } from '@2fa/rusty';
	import { getColor } from '../../colors';

	export let account: AccountDetails;

	$: color = getColor(account.issuer);
	$: secondaryColor = getColor(account.username || account.issuer);
</script>

<div
	style="--secondary-color: rgba({secondaryColor.r}, {secondaryColor.g}, {secondaryColor.b}, 20); --primary-color: rgba({color.r}, {color.g}, {color.b}, 20);"
	class="text-xl rounded h-10 w-10 bg-[var(--primary-color)] flex items-center justify-center"
>
	<slot>
		<div>
			{account.issuer.charAt(0)}
		</div>
	</slot>
</div>
