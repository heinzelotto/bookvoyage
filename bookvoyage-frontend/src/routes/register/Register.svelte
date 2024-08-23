<script>
	let responseText = '';
	let username = '';
	let password = '';
	let error = '';

	async function register(username, password) {
		const response = await fetch('/api/register', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ username, password })
		});

		if (response.ok) {
			const data = await response.json();

			responseText = "successfully registered: " + JSON.stringify(data);
		} else {
			throw new Error('Registration failed');
		}
	}

	async function handleSubmit() {
		try {
			await register(username, password);
			// navigate('/');
		} catch (e) {
			error = e.message;
		}
	}
</script>

<form on:submit|preventDefault={handleSubmit}>
	<input bind:value={username} placeholder="Username" required />
	<input bind:value={password} type="password" placeholder="Password" required />
	<button type="submit">Register</button>
</form>

{#if error}
	<p class="error">{error}</p>
{/if}

	<p>{responseText}</p>
