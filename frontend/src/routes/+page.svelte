<script lang="ts">
	import type { PageData, ActionData } from './$types';
	import { fly, slide, scale, fade } from 'svelte/transition';
	import { onMount } from 'svelte';

	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { toast } from 'svelte-sonner';

	let content: string[] = $state([]);
	let inputFocus: boolean = $state(false);
	let buttonHover: boolean = $state(false);

	let query = $state('');

	let { data, form }: { data: PageData; form: ActionData } = $props();
	onMount(() => {
		console.log(data, form);
		if (form?.error) {
			toast.error(form?.message || "")
		}	
	})
</script>

<main class="flex flex-col items-center justify-center h-screen">
	<form
		method="POST"
		class="group flex gap-1 transition-all duration-300 ease-in-out sticky top-12"
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

	<section class="h-auto flex flex-col overflow-auto max-h-[90vh] mt-16">


		{#key content}
			{#each content as item, idx (idx)}
				<div transition:slide>{item}</div>
			{/each}
		{/key}
	</section>
</main>
