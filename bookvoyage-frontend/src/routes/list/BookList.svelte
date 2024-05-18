<script lang="ts">
	import { onMount } from 'svelte';
	import L from 'leaflet';

	let books: { book_info: any; book_logs: any }[] = [];
	let map;
	let loading = true;

	onMount(async () => {
		await fetchList();
		await loadMap();

		// cleanup function
		return () => {};
	});

	async function fetchList() {
		var book_list_response = await fetch('http://localhost:5173/api/book_list');
		let book_list = await book_list_response.json();

		for (var book of book_list) {
			var book_logs_response = await fetch(
				`http://localhost:5173/api/book_logs/?book_id=${book['id']}`
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
		};

		document.head.appendChild(link);
	}

	//   export let markers = [
	//     { lat: 25.04776, lng: 121.53185, title: 'Taipei', message: 'Welcome to <a href="https://en.wikipedia.org/wiki/Taipei" target="_blank">Taipei!</a>',  },
	//   ];

	function addMarkers() {
		if (loading) {
			return;
		}
		books.forEach((book) => {
			console.log(book);
			const { book_info, book_logs } = book;
			const { id, title, author, code } = book_info;
			for (var book_log of book_logs) {
				const { id, commenter, lat, lon, comment } = book_log;
				L.marker([lat, lon])
					.addTo(map)
					.bindPopup(`<b>${title}</b><br>${id}`)
					.openPopup()
					.setOpacity(1.0);
			}
		});
	}

	$: if (!loading) {
		addMarkers();
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

<div id="map" style="height: 500px; width: 100%;" />

<style>
</style>
