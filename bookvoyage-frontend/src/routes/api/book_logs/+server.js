import { json } from '@sveltejs/kit';

/** @type {import('./$types').RequestHandler} */
export async function GET({ url }) {
    const book_id = Number(url.searchParams.get('book_id') ?? '1');
    const res = await fetch(`http://localhost:8080/book_logs/${book_id}`);
    let code = await res.json();

    console.log(code);

    return json(code);
}