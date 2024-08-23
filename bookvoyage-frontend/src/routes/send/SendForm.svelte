<script lang="ts">
	import { onMount } from 'svelte';
	import { getAuthHeader } from '../auth.js';
	import L from 'leaflet';

	let map;
	let loading = true;

	onMount(async () => {
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
	});

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

	//   $: if (!loading) {
	//     addMarkers();
	//   }

	let bookCodeString = '';
	const getRandomBookCode = async () => {
		bookCodeString = '';
		var response = await fetch('http://localhost:5173/api/code');
		var result = await response.json();
		console.log(result);
		bookCodeString = result.code_string;
		return result;
	};
	let bookCodePromise = getRandomBookCode();

	let errors: Record<string, string> = {};

	function isRequired(value: any) {
		return value != null && value !== '';
	}

	async function doPost(data: any) {
		let payload = JSON.stringify({
			title: data['title'],
			author: data['author'],
			review: data['review'],
			code: data['bookCode'],
			lat: parseFloat(data['lat']),
			lon: parseFloat(data['lon'])
		});

		console.log(payload);

		const res = await fetch('/api/send', {
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

		if (!error_flag) {
			submitPromise = doPost(data);
		}
	};

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

<!-- {#if loading}
  <p>Loading map...</p>
{:else} -->
<!-- {/if} -->

<form on:submit={handleOnSubmit}>
	<div class="form-control w-full max-w-xs">
		<label class="label" for="title">
			<span class="label-text">Title</span>
			{#if errors.title}
				<span class="text-red label-text-alt"> {errors.title}</span>
			{/if}
		</label>
		<input
			id="title"
			name="title"
			type="text"
			placeholder="Title"
			class="input input-bordered w-full max-w-xs"
		/>
	</div>

	<div class="form-control w-full max-w-xs">
		<label class="label" for="author">
			<span class="label-text">Author</span>
			{#if errors.author}
				<span class="text-red label-text-alt"> {errors.author}</span>
			{/if}
		</label>
		<input
			id="author"
			name="author"
			type="text"
			placeholder="Author"
			class="input input-bordered w-full max-w-xs"
		/>
	</div>

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

	<div class="form-control w-full max-w-xs">
		<label class="label" for="bookCode">
			<span class="label-text">Book Code</span>
			{#if errors.bookCode}
				<span class="text-red label-text-alt"> {errors.bookCode}</span>
			{/if}
		</label>
		<input
			id="bookCode"
			name="bookCode"
			type="text"
			readOnly="true"
			bind:value={bookCodeString}
			class="input input-bordered w-full max-w-xs"
		/>
		{#await bookCodePromise}
			<strong>Generating BookCode...</strong>
		{:catch err}
			<p><small style="color: red"> {err} </small></p>
		{/await}
		<button on:click={async () => await getRandomBookCode()} type="button" class="btn btn-xs">
			Regenerate
		</button>
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
	<button id="submitButton" type="submit" class="btn">Bon Voyage</button>
</form>

{#if submitPromise != null}
	{#await submitPromise then submitResponse}
		Backend answers: <strong>{submitResponse.response}</strong>
	{:catch err}
		<h2>Error while loading submit response.</h2>
	{/await}
{/if}

<!-- <div class="example">
    <svelte:component this={LeafletMap} events="{['click']}" on:click={on_map_click} options={mapOptions}>
        <svelte:component this={TileLayer} url={tileUrl} options={tileLayerOptions}/>
        {#if clickLat != null && clickLng != null}
            <svelte:component this={Marker} latLng={[clickLat, clickLng]} />        
        {/if}
    </svelte:component>
</div> -->
<div id="map" style="height: 500px; width: 100%;" />

<style>
	/* input[type=text], textarea {
        padding: 10px;
        margin:10px 0;
        border: none;
        border-radius:20px;
        box-shadow:0 0 15px 4px rgba(0,0,0,0.06);
        width:100%;
        font-family: inherit;
        font-size: inherit;
    }
    textarea {
        resize:vertical;
    }
    #submitButton {
        padding: 15px;
        margin:20px 0;
        border: none;
        cursor:pointer;
        border-radius: 20px;
        color:white;
        background-color: #04AA6D;
        box-shadow:0 0 15px 4px rgba(0,0,0,0.06);
        font-family: inherit;
        font-size: inherit;
    } */
	.example {
		height: 500px;
		width: 500px;
	}
</style>
