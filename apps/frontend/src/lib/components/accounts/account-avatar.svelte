<script lang="ts">
	import type { AccountDetailsWithCode } from '@2fa/rusty';
	import { getColor } from '../../colors';

	export let account: AccountDetailsWithCode;

	$: color = getColor(account.issuer);
	$: secondaryColor = getColor(account.username || account.issuer);
</script>

<div
	style="--secondary-color: rgba({secondaryColor.r}, {secondaryColor.g}, {secondaryColor.b}, 20); --primary-color: rgba({color.r}, {color.g}, {color.b}, 20);"
	class="xs:text-xl rounded h-8 xs:h-12 w-8 xs:w-12 bg-[var(--primary-color)] flex items-center justify-center text-white"
>
	<slot>
		<div>
			{account.issuer.charAt(0)}
		</div>
	</slot>
</div>
