<script>
	import { page } from '$app/stores';
	import logo from '$lib/images/logo_small.png';
	import '../app.css';
	import { isAuthenticated } from '../stores.js';
	import { logout } from './auth.js';

	function handleLogout() {
    	logout();
    	// navigate('/login');
	}
</script>

<div class="navbar bg-base-100">
	<div class="flex-1">
		<a href="/" class="btn btn-ghost normal-case text-xl">
			BookVoyage
			<!-- <img src={logo} alt="BookVoyage" /> -->
		</a>
	</div>
	<div class="flex-none">
		<ul class="menu menu-horizontal px-1">
			<li aria-current={$page.url.pathname === '/' ? 'page' : undefined} />
			<li aria-current={$page.url.pathname === '/about' ? 'page' : undefined}>
				<a href="/about">About</a>
			</li>
			<li aria-current={$page.url.pathname === '/list' ? 'page' : undefined}>
				<a href="/list">List</a>
			</li>
			<li aria-current={$page.url.pathname === '/send' ? 'page' : undefined}>
				<a href="/send">Send</a>
			</li>
			<li aria-current={$page.url.pathname === '/find' ? 'page' : undefined}>
				<a href="/find">Find</a>
			</li>
			{#if $isAuthenticated}
				<li aria-current={$page.url.pathname === '/profile' ? 'page' : undefined}>
					<a href="/profile">Profile</a>
				</li>
				<li>
					<button on:click={handleLogout}>Logout</button>
				</li>
			{:else}
				<li aria-current={$page.url.pathname === '/login' ? 'page' : undefined}>
					<a href="/login">Login</a>
				</li>
				<li aria-current={$page.url.pathname === '/register' ? 'page' : undefined}>
					<a href="/register">Register</a>
				</li>
			{/if}
		</ul>
	</div>
</div>
