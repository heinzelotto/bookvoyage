import { json } from '@sveltejs/kit';

/** @type {import('./$types').RequestHandler} */
export async function POST({ request }) {
    const req = await request.json();

    const response = await fetch("http://localhost:8080/send", {
        method: 'POST',
        body: JSON.stringify(req),
        headers: {
            'content-type': 'application/json'
        }
    });

    if (!response.ok) {
        // TODO: handle error status code without failing later
        console.log(await response.text());
    }

    let total = await response.json();

    console.log(total);

    return json(total);
}