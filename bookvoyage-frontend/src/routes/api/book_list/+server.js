import { json } from '@sveltejs/kit';

/** @type {import('./$types').RequestHandler} */
export async function GET({ url }) {
    const book_code = url.searchParams.get('book_code');
    
    let book_list;

    if (book_code != null) {
        const res = await fetch(`http://localhost:8080/book_list/by_code/${book_code}`);
        book_list = await res.json();
    } else {
        const res = await fetch("http://localhost:8080/book_list");
        book_list = await res.json();    
    }

    console.log(book_list);

    return json(book_list);
}