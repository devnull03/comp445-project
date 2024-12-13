<script lang="ts">
	import { Toaster } from 'svelte-sonner';
	import { ModeWatcher } from 'mode-watcher';
	import '../app.css';
	import DarkModeSwitcher from '$lib/components/DarkModeSwitcher.svelte';

	let { children } = $props();
</script>

<Toaster />
<ModeWatcher />

<div class="fixed top-4 right-4">
	<DarkModeSwitcher />
</div>

{@render children()}
