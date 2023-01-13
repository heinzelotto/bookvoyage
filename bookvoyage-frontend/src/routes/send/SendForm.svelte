<script lang="ts">
    import PickAPlace from 'svelte-pick-a-place';

    import { onMount } from 'svelte';

    let map;
    let leafletPromise: Promise<any>;

    onMount(() => {
        const link = document.createElement('link');
        link.rel = 'stylesheet';
        link.href = 'https://unpkg.com/leaflet@1.9.3/dist/leaflet.css';
        link.onload = () => loadMap();

        document.head.appendChild(link);

        return () => {
            map.remove();
            link.parentNode.removeChild(link);
        };
    });

    // TODO: read https://svelte.dev/repl/62271e8fda854e828f26d75625286bc3?version=3.55.1
    async function loadMap() {
        leafletPromise = await import('leaflet');
        // map = L.map('map').setView([49, 12], 13);
        // L.tileLayer('https://a.tile.openstreetmap.org/{z}/{x}/{y}.png ', {
        //     attribution:
        //         'Map data &copy; <a href="https://www.openstreetmap.org/">OpenStreetMap</a> contributors, <a href="https://creativecommons.org/licenses/by-sa/2.0/">CC-BY-SA</a>',
        //     maxZoom: 18,
        // }).addTo(map);
    }

    let bookCodeString = "";
    const getRandomBookCode = async() => {
        bookCodeString = "";
        var response = await fetch('http://localhost:5173/api/code');
        var result = await response.json();
        console.log(result);
        bookCodeString = result.code_string;
        return result;
    }
    let bookCodePromise = getRandomBookCode();

    let errors: Record<string, string> = {};

    function isRequired(value: any) {
        return value != null && value !== ""
    }

    async function doPost(data: any) {
        console.log(JSON.stringify({
                    title: data["title"],
                    author: data["author"],
                    review: data["review"],
                    code: data["bookCode"],
                    coords: data["coords"],
                }));

        const res = await fetch('/api/send', {
                method: 'POST',
                body: JSON.stringify({
                    title: data["title"],
                    author: data["author"],
                    review: data["review"],
                    code: data["bookCode"],
                    coords: data["coords"],
                })
            });

            const json = await res.json();
            return json;
    }

    let submitPromise: Promise<any>;

    const handleOnSubmit = (event: any) => {
        const formData = new FormData(event.target as HTMLFormElement);

        let error_flag = false;

        const data : any = {};
        for (let field of formData) {
            const [key, value] = field;
            if (!isRequired(value)) {
                errors[key] = key + ' is required'
                error_flag = true
            }
            data[key] = value;
        }

        if (!error_flag) {
            submitPromise = doPost(data);
        }
    }

    let coordsString = "";
    function handleMapUpdate({ detail }) {
        let coords = detail.geometry.coordinates;
        coordsString = coords[0] + " " + coords[1];
        console.log('Update event! ', detail.geometry.coordinates);
    }

</script>

<form on:submit={handleOnSubmit}>
    <div>
        <label for="title"> <strong>Title</strong></label>
        <input id="title" name="title" type="text" placeholder="Title" />
        {#if errors.title}
            <p><small style="color: red"> { errors.title } </small></p>
        {/if}
    </div>

    <div>
        <label for="author"> <strong>Author</strong></label>
        <input id="author" name="author" type="text" placeholder="Author" />
        {#if errors.author}
            <p><small style="color: red"> { errors.author } </small></p>
        {/if}
    </div>

    <div>
        <label for="review"> <strong>Comment/Review</strong></label>
        <textarea id="review" name="review" placeholder="What did you experience with this 📕?" />
        {#if errors.review}
            <p><small style="color: red"> { errors.review } </small></p>
        {/if}
    </div>

    <div>
        <label for="bookCode"> <strong>Book Code</strong></label>
        <button on:click={async() => await getRandomBookCode()} type="button" aria-label="Reset counter">
            Regenerate
        </button>
        <input id="bookCode" name="bookCode" type="text" readOnly="true" bind:value="{bookCodeString}"/>
        {#await bookCodePromise}
            <strong>Generating BookCode...</strong>
        {:then bookCode}
        {:catch err}
            <p><small style="color: red"> { err } </small></p>
        {/await}
        {#if errors.bookCode}
            <p><small style="color: red"> { errors.bookCode } </small></p>
        {/if}
    </div>

    <div>
        <label for="coords"> <strong>Coordinates</strong></label>
        <input id="coords" name="coords" type="text" placeholder="Choose on map" readOnly="true" bind:value="{coordsString}"/>
        {#if errors.coords}
            <p><small style="color: red"> { errors.coords } </small></p>
        {/if}
    </div>

    <br />
    <button id="submitButton" type="submit">Bon Voyage</button>
</form>

{#if submitPromise != null}
{#await submitPromise}
{:then submitResponse}
    Backend answers: <strong>{submitResponse.response}</strong>
{:catch err}
    <h2>Error while loading submit response.</h2>
{/await}
{/if}

{#if leafletPromise != null}
{#await leafletPromise}
waiting leafletPromise..
{:then leaflet}
<div class="map-container">
    <PickAPlace leaflet={leaflet} on:update={handleMapUpdate} on:save={() =>
    console.log('On save!')} /></div>
{/await}
{:else}
<strong>Loading map...</strong>
{/if}

<style>
    input[type=text], textarea {
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
    }

    .map-container {
        height:500px;        
    }
</style>