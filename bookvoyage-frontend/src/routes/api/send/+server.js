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

    let total = await response.json();

    console.log(total);

    return json(total);
}