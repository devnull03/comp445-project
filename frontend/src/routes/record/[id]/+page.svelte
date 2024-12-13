<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';

	import type { RecordResponse, SimilarityInfoFull } from '$lib/model';
	import { currentSearchResults } from '$lib/stores.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';

	let selectedResult: RecordResponse | undefined = $state();

	onMount(() => {
		selectedResult = $currentSearchResults?.data.find(
			(result) => result.data.id === Number.parseInt($page.params.id)
		);
	});
</script>

<Button class="fixed top-4 left-4" href="../../">Back</Button>

{#if selectedResult}
	<main class="h-screen flex flex-col px-16 py-24 gap-6">
		<div class="flex flex-col items-center gap-2">
			<h1>{selectedResult.data.title}</h1>
			<div class="flex gap-2">
				ID: {selectedResult.data.id}
				<Separator orientation="vertical"></Separator> label: {selectedResult.data.label
					? 'Real'
					: 'Fake'}
			</div>
		</div>
		<Separator></Separator>
		<div class="pb-24">
			<p>{selectedResult.data.text}</p>
		</div>
	</main>
{:else}
	<main class="h-screen flex items-center justify-center">
		<h1>Document ID not found</h1>
		<sub class="text-gray-400">Document might not have been loaded yet</sub>
	</main>
{/if}

<style type="postcss">
	h1 {
		@apply text-3xl font-semibold text-center;
	}
</style>
