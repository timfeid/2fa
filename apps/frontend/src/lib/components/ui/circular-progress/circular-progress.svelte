<script lang="ts">
	export let size = 100;
	export let backgroundColor = 'text-primary/20';
	export let color = 'text-primary';
	export let strokeWidth = 10;
	export let progress: number;

	$: radius = (size - strokeWidth) / 2;
	$: circumference = radius * 2 * Math.PI;
	$: offset = circumference - (progress / 100) * circumference;
</script>

<div class="relative inline-flex items-center justify-center">
	<svg width={size} height={size} class="transform -rotate-90">
		<circle
			class={`${backgroundColor} transition-all duration-300 ease-in-out`}
			stroke="currentColor"
			stroke-width={strokeWidth}
			fill="transparent"
			r={radius}
			cx={size / 2}
			cy={size / 2}
		/>
		<circle
			class={`${color} transition-all duration-300 ease-in-out`}
			stroke="currentColor"
			stroke-width={strokeWidth}
			stroke-dasharray={circumference}
			stroke-dashoffset={offset}
			stroke-linecap="round"
			fill="transparent"
			r={radius}
			cx={size / 2}
			cy={size / 2}
		/>
	</svg>
	<div class="absolute">
		<slot />
	</div>
</div>
