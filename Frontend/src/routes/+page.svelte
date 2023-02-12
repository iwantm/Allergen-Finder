<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<h1>Search</h1>

<form action="?/search" method="POST">
	<div>
		<label for="barcode">Barcode</label>
		<input id="barcode" name="barcode" type="text" required />
	</div>
	<button type="submit">Search</button>
	{#if data.product}
		{#each Object.entries(data.product) as [title, paragraph]}
			<h1>{title}</h1>
			<p>{paragraph}</p>
		{/each}
	{/if}
</form>
