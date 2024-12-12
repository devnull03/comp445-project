<script lang="ts">
	import type { PageData, ActionData } from './$types';
	import { fly, slide, scale, fade } from 'svelte/transition';
	import { onMount } from 'svelte';

	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { toast } from 'svelte-sonner';
	import * as Table from '$lib/components/ui/table';
	import * as Pagination from '$lib/components/ui/pagination';
	import type { SearchResultsRes } from '$lib/model';
	import { getPage } from '$lib/api';

	let inputFocus: boolean = $state(false);
	let buttonHover: boolean = $state(false);
	let loadDelay: boolean = $state(true);

	let query: string = $state('');
	let searchResults: SearchResultsRes | undefined = $state();

	const handlePagination = async (page: number) => {
		loadDelay = true;
		let result = await getPage(searchResults?.search_id as string, page);

		if (result.error) {
			toast.error(result.message || '');
		} else {
			searchResults = result;
		}
        loadDelay = false;
	};

	let { data, form }: { data: PageData; form: ActionData } = $props();
	onMount(() => {
		console.log(form);

		if (form?.error) {
			toast.error(form?.message || '');
		}

		query = (form?.queryText as string) || '';

		searchResults = form?.result;

		setTimeout(() => {
			loadDelay = false;
		}, 100);
	});
</script>

<main class="flex flex-col items-center justify-center h-screen">
	<section class="flex flex-col gap-8 sticky top-12">
		<h1 class="text-3xl font-light self-center">Search Engine</h1>
		<form
			method="POST"
			class="group flex gap-1 transition-all duration-300 ease-in-out"
			onsubmit={(e) => {
				if (query === '') {
					toast.error('Invalid search');
					e.preventDefault();
				}
			}}
		>
			<Input
				type="text"
				placeholder="what do you wanna search?"
				class="rounded-lg {inputFocus
					? 'w-[70vw]'
					: 'w-[45vw]'} transition-all ease-in-out duration-300"
				onfocus={() => {
					inputFocus = true;
				}}
				onblur={() => {
					inputFocus = buttonHover;
				}}
				name="query"
				bind:value={query}
				defaultValue={form?.queryText}
			/>
			<Button
				onmouseover={() => {
					buttonHover = true;
				}}
				onmouseleave={() => {
					buttonHover = false;
				}}
				type="submit">Search</Button
			>
		</form>
	</section>

	{#if !loadDelay && searchResults}
		<section transition:slide class="h-auto flex flex-col overflow-auto max-h-[90vh] mt-16 px-16">
			<Table.Root class="w-[99%] mb-16">
				<Table.Caption>Search Results for "{form?.queryText}"</Table.Caption>
				<Table.Header>
					<Table.Row>
						<Table.Head class="w-[100px]">Document ID</Table.Head>
						<Table.Head class="w-[35%]">Title</Table.Head>
						<Table.Head>Content</Table.Head>
						<Table.Head class="text-right">Real</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each searchResults.data as item, idx (idx)}
						<Table.Row class="cursor-pointer">
							<Table.Cell class="font-medium">{item.data.id}</Table.Cell>
							<Table.Cell>{item.data.title}</Table.Cell>
							<Table.Cell
								>{item.data.text.slice(0, Math.min(300, item.data.text.length))}...
								<i class="font-light">click to read more</i>
							</Table.Cell>
							<Table.Cell class="text-right">{item.data.label}</Table.Cell>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>

			<Pagination.Root
				count={searchResults.number_of_results}
				perPage={20}
				let:pages
				let:currentPage
				class="my-4"
				onPageChange={handlePagination}
			>
				<Pagination.Content>
					<Pagination.Item>
						<Pagination.PrevButton />
					</Pagination.Item>
					{#each pages as page (page.key)}
						{#if page.type === 'ellipsis'}
							<Pagination.Item>
								<Pagination.Ellipsis />
							</Pagination.Item>
						{:else}
							<Pagination.Item>
								<Pagination.Link {page} isActive={currentPage == page.value}>
									{page.value}
								</Pagination.Link>
							</Pagination.Item>
						{/if}
					{/each}
					<Pagination.Item>
						<Pagination.NextButton />
					</Pagination.Item>
				</Pagination.Content>
			</Pagination.Root>
		</section>
	{/if}
</main>
