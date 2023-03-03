<script>
	import Header from './Header.svelte';
	import { page } from '$app/stores';
	import '../app.css';
</script>

<div class="flex flex-col min-h-screen">
	<Header />

	<div id="bla" class="flex flex-1 w-full max-w-3xl p-1.0 mx-auto box-border">
		<slot />
	</div>

	<footer class="flex flex-1 flex-col justify-center items-center p-12">
		<p>visit <a href="https://kit.svelte.dev" class="font-bold">kit.svelte.dev</a> to learn SvelteKit</p>
	</footer>
</div>

<style>
	@media (min-width: 480px) {
		footer {
			padding: 12px 0;
		}
	}
</style>
