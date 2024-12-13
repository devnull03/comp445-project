<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';

	import type { RecordResponse, SimilarityInfoFull } from '$lib/model';
	import { currentSearchResults } from '$lib/stores.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import { goto } from '$app/navigation';
	import * as Table from '$lib/components/ui/table';

	let selectedResult: RecordResponse | undefined = $state();
	let selectedSimilarResult: SimilarityInfoFull | undefined = $state();

	const loadData = () => {
		if ($page.url.searchParams.get('parent')) {
			selectedResult = $currentSearchResults?.data.find(
				(result) =>
					result.data.id === Number.parseInt($page.url.searchParams.get('parent') as string)
			);

			selectedSimilarResult = selectedResult?.similar_docs.find(
				(result) => result.doc.id === Number.parseInt($page.params.id)
			);

			console.log('similarResult', selectedSimilarResult, selectedResult);
		} else {
			selectedResult = $currentSearchResults?.data.find(
				(result) => result.data.id === Number.parseInt($page.params.id)
			);
			selectedSimilarResult = undefined;
			console.log('selectedResult', selectedResult);
		}
	};

	$effect(() => {
		loadData();
	});

	onMount(() => {
		loadData();
	});
</script>

<Button
	class="fixed top-4 left-4"
	onclick={loadData}
	href={selectedSimilarResult ? `./${selectedResult?.data.id}` : '../../'}>Back</Button
>

{#snippet disp_res(
	id: number,
	title: string,
	label: 1 | 0,
	text: string,
	similarity: number | null = null
)}
	<div class="flex flex-col items-center gap-2">
		<h1>{title}</h1>
		<div class="flex gap-2">
			ID: {id}
			<Separator orientation="vertical"></Separator> label: {label ? 'Real' : 'Fake'}
			{#if similarity !== null}
				<Separator orientation="vertical"></Separator> similarity: {similarity}
			{/if}
		</div>
	</div>
	<Separator></Separator>
	<div class="pb-16">
		<p>{text}</p>
	</div>
{/snippet}

{#key $page.params.id}
	{#if selectedResult}
		<main class="h-screen flex flex-col px-16 py-24 gap-6">
			{#if selectedSimilarResult}
				{@render disp_res(
					selectedSimilarResult.doc.id,
					selectedSimilarResult.doc.title,
					selectedSimilarResult.doc.label,
					selectedSimilarResult.doc.text,
					selectedSimilarResult.similarity
				)}
			{:else}
				{@render disp_res(
					selectedResult.data.id,
					selectedResult.data.title,
					selectedResult.data.label,
					selectedResult.data.text
				)}
				{#if selectedResult.similar_docs.length > 0}
					<h2 class="text-2xl font-semibold">Similar Documents</h2>

					<div>
						<Table.Root class="w-[99%] mb-16">
							<!-- <Table.Caption>??</Table.Caption> -->
							<Table.Header>
								<Table.Row>
									<Table.Head class="w-[100px]">Document ID</Table.Head>
									<Table.Head class="w-[35%]">Title</Table.Head>
									<Table.Head>Content</Table.Head>
									<Table.Head class="text-right">Real</Table.Head>
									<Table.Head class="text-right">Similarity</Table.Head>
								</Table.Row>
							</Table.Header>
							<Table.Body>
								{#each selectedResult.similar_docs as item}
									<Table.Row
										class="cursor-pointer"
										onclick={() => {
											goto(`./${item.doc.id}?parent=${selectedResult?.data.id}`);
										}}
									>
										<Table.Cell class="font-medium">{item.doc.id}</Table.Cell>
										<Table.Cell>{item.doc.title}</Table.Cell>
										<Table.Cell
											>{item.doc.text.slice(0, Math.min(100, item.doc.text.length))}...
											<i class="font-light">click to read more</i>
										</Table.Cell>
										<Table.Cell class="text-right">{item.doc.label}</Table.Cell>
										<Table.Cell class="text-right">{item.similarity}</Table.Cell>
									</Table.Row>
								{/each}
							</Table.Body>
						</Table.Root>
					</div>
				{/if}
			{/if}
		</main>
	{:else}
		<main class="h-screen flex flex-col gap-4 items-center justify-center">
			<h1>Document ID not found</h1>
			<sub class="text-gray-400">Document might not have been loaded yet</sub>
		</main>
	{/if}
{/key}

<style type="postcss">
	h1 {
		@apply text-3xl font-semibold text-center;
	}
</style>
