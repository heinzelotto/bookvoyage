<script>
	import { user, token, isAuthenticated } from '../../stores.js';
	import { login, logout } from '../auth.js';

	// import { navigate } from 'svelte-routing';

	let username = '';
	let password = '';
	let error = '';

	async function handleSubmit() {
		try {
			await login(username, password);
			// navigate('/');
		} catch (e) {
			error = e.message;
		}
	}

	async function handleLogout() {
		try {
			await logout();
			// navigate('/');
		} catch (e) {
			error = e.message;
		}
	}
</script>

<form on:submit|preventDefault={handleSubmit}>
	<input bind:value={username} placeholder="Username" required />
	<input bind:value={password} type="password" placeholder="Password" required />
	<button type="submit">Login</button>
</form>

<form on:submit|preventDefault={handleLogout}>
	<button type="submit">Logout</button>
</form>

{#if error}
	<p class="error">{error}</p>
{/if}

{#if $isAuthenticated}
	<p>{$isAuthenticated}</p>
{/if}

{#if $user}
	<p>{$user.username}</p>
{/if}

{#if $token}
	<p>{$token.token}</p>
{/if}
