import { json } from '@sveltejs/kit';

/** @type {import('./$types').RequestHandler} */
export async function GET({ url }) {
    const res = await fetch("http://localhost:8080/book_list");
    let code = await res.json();

    console.log(code);

    return json(code);
}