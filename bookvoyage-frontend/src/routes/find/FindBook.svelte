<script lang="ts">
	import { onMount } from 'svelte';
	import { getAuthHeader } from '../auth.js';
	import L from 'leaflet';

	let books: { book_info: any; book_logs: any }[] = [];
	let map;
	let loading = true;

	onMount(async () => {
		await fetchBooks();
		await loadMap();

		// cleanup function
		return () => {};
	});

	async function fetchList() {
		var book_list_response = await fetch('http://localhost:5173/api/book_list');
		let book_list = await book_list_response.json();

		for (var book of book_list) {
			var book_logs_response = await fetch(
				`http://localhost:5173/api/book_logs/by_id/${book['id']}`,
				{
					headers: {
						Authorization: getAuthHeader()
					}
				}
			);
			var book_logs = await book_logs_response.json();

			books.push({ book_info: book, book_logs: book_logs });
		}

		// svelte only updates on assignments
		books = books;
	}

	async function loadMap() {
		const link = document.createElement('link');
		link.href = 'https://unpkg.com/leaflet@1.9.3/dist/leaflet.css';
		link.rel = 'stylesheet';
		link.type = 'text/css';

		link.onload = () => {
			map = L.map('map').setView([0, 0], 2);
			L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
				attribution:
					'Map data © <a href="https://openstreetmap.org">OpenStreetMap</a> contributors',
				maxZoom: 18
			}).addTo(map);

			loading = false;

			map.on('click', on_map_click);
		};

		document.head.appendChild(link);
	}

	async function fetchBooks() {
		var book_list_response = await fetch('http://localhost:5173/api/book_list', {
			headers: {
				Authorization: getAuthHeader()
			}
		});
		let book_list = await book_list_response.json();

		for (var book of book_list) {
			var book_logs_response = await fetch(
				`http://localhost:5173/api/book_logs/by_id/${book['id']}`,
				{
					headers: {
						Authorization: getAuthHeader()
					}
				}
			);
			var book_logs = await book_logs_response.json();

			books.push({ book_info: book, book_logs: book_logs });
		}

		// svelte only updates on assignments
		books = books;
	}

	const bookCodeLen = 8;
	let bookCode = '';
	let fetchedBook: { book_info: any; book_logs: any } | null = null;

	const onBookCodeInput = async () => {
		fetchedBook = null;

		console.log(bookCode.length);
		if (bookCode.length == bookCodeLen) {
			var book_list_response = await fetch(
				`http://localhost:5173/api/book_list/by_code/${bookCode}`,
				{
					headers: {
						Authorization: getAuthHeader()
					}
				}
			);
			var book_list = await book_list_response.json();

			console.log(book_logs);
			if (book_list.length == 1) {
				let book = book_list[0];

				var book_logs_response = await fetch(
					`http://localhost:5173/api/book_logs/by_id/${book['id']}`,
					{
						headers: {
							Authorization: getAuthHeader()
						}
					}
				);
				var book_logs = await book_logs_response.json();

				fetchedBook = { book_info: book, book_logs: book_logs };
				console.log(fetchedBook);
			}
		}

		updateInputVisibility();
	};

	function updateInputVisibility() {
		if (fetchedBook != null) {
			document.getElementById('addLogForm').style.display = 'block';
			document.getElementById('map').style.display = 'block';
		} else {
			document.getElementById('addLogForm').style.display = 'none';
			document.getElementById('map').style.display = 'none';
		}
	}

	let errors: Record<string, string> = {};

	function isRequired(value: any) {
		return value != null && value !== '';
	}

	async function doPost(data: any) {
		let payload = JSON.stringify({
			code: data['bookCode'],
			review: data['review'],
			lat: parseFloat(data['lat']),
			lon: parseFloat(data['lon'])
		});

		console.log(payload);

		const res = await fetch('/api/add_log', {
			method: 'POST',
			body: payload,
			headers: {
				Authorization: getAuthHeader(),
				'content-type': 'application/json'
			}
		});

		const json = await res.json();
		return json;
	}

	let submitPromise: Promise<any>;

	const handleOnSubmit = (event: any) => {
		const formData = new FormData(event.target as HTMLFormElement);

		let error_flag = false;

		const data: any = {};
		for (let field of formData) {
			const [key, value] = field;
			if (!isRequired(value)) {
				errors[key] = key + ' is required';
				error_flag = true;
			}
			data[key] = value;
		}
		data['bookCode'] = bookCode;

		if (!error_flag) {
			submitPromise = doPost(data);
		}
	};

	export let markerLayer = null;
	function addMarker(lat: String, lng: String) {
		if (loading) {
			return;
		}
		//   const { lat, lng, title, message } = marker;
		markerLayer = L.marker([lat, lng])
			.addTo(map)
			// .bindPopup(`<b>${title}</b><br>${message}`)
			// .openPopup()
			.setOpacity(1.0);
	}
	function removeMarker() {
		if (markerLayer) {
			map.removeLayer(markerLayer);
		}
	}
	let clickLat = null;
	let clickLng = null;
	function on_map_click(event: any) {
		console.log(event);
		let coords = event.latlng;
		console.log(coords);
		clickLat = parseFloat(coords['lat']);
		clickLng = parseFloat(coords['lng']);
		removeMarker();
		addMarker(clickLat, clickLng);
	}
</script>

<ul>
	{#each books as book}
		<li>
			{book.book_info.id}
			{book.book_info.title}
			{book.book_info.author}
			{book.book_info.code}
			<ol>
				{#each book.book_logs as book_log}
					<li>
						{book_log.id}
						{book_log.commenter}
						{book_log.lat}
						{book_log.lon}
						{book_log.comment}
					</li>
				{/each}
			</ol>
		</li>
	{/each}
</ul>

<label for="bookCode"><strong>Enter Book Code</strong></label>
<input
	class="input input-bordered input-primary w-full max-w-xs"
	id="bookCode"
	name="bookCode"
	type="text"
	placeholder="Enter Book Code"
	bind:value={bookCode}
	on:input={onBookCodeInput}
/>

{#if fetchedBook != null}
	We got the book!
	<li>
		{fetchedBook.book_info.id}
		{fetchedBook.book_info.title}
		{fetchedBook.book_info.author}
		{fetchedBook.book_info.code}
		<ol>
			{#each fetchedBook.book_logs as book_log}
				<li>{book_log.id} {book_log.commenter} {book_log.lat} {book_log.lon} {book_log.comment}</li>
			{/each}
		</ol>
	</li>
{/if}

<div id="map" style="height: 500px; width: 100%; display: none;" />
<form id="addLogForm" style="display: none;" on:submit={handleOnSubmit}>
	<div class="form-control w-full max-w-xs">
		<label class="label" for="review">
			<span class="label-text">Comment/Review</span>
			{#if errors.review}
				<span class="text-red label-text-alt"> {errors.review}</span>
			{/if}
		</label>
		<textarea
			id="review"
			name="review"
			class="textarea textarea-bordered h-24"
			placeholder="What did you experience with this 📕?"
		/>
	</div>

	<div>
		<label for="lat"> <strong>Latitude</strong></label>
		<input
			id="lat"
			name="lat"
			type="text"
			placeholder="Choose on map"
			readOnly="true"
			bind:value={clickLat}
		/>
		{#if errors.lat}
			<p><small style="color: red"> {errors.lat} </small></p>
		{/if}
	</div>
	<div>
		<label for="lon"> <strong>Longitude</strong></label>
		<input
			id="lon"
			name="lon"
			type="text"
			placeholder="Choose on map"
			readOnly="true"
			bind:value={clickLng}
		/>
		{#if errors.lon}
			<p><small style="color: red"> {errors.lon} </small></p>
		{/if}
	</div>

	<br />
	<button id="submitButton" type="submit" class="btn">Add Log</button>
</form>

{#if submitPromise != null}
	{#await submitPromise then submitResponse}
		Backend answers: <strong>{submitResponse.response}</strong>
	{:catch err}
		<h2>Error while loading submit response.</h2>
	{/await}
{/if}

<!-- <style>
    input[type=text] {
        padding: 10px;
        margin:10px 0;
        border: none;
        border-radius:20px;
        box-shadow:0 0 15px 4px rgba(0,0,0,0.06);
        width:100%;
        font-family: inherit;
        font-size: inherit;
    }
</style> -->
