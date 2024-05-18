import { json } from '@sveltejs/kit';

/** @type {import('./$types').RequestHandler} */
export async function GET({ url }) {
    try {
        const res = await fetch("http://127.0.0.1:8080/code");

        if (!res.ok) {
            // If the response is not OK, throw an error
            throw new Error(`HTTP error! status: ${res.status}`);
        }

        const code = await res.json();

        console.log(code);

        return json(code);
    } catch (error) {
        console.error('Fetch error:', error);

        // Check the error type
        let errorMessage = 'Failed to fetch data';
        if (error instanceof TypeError) {
            errorMessage = 'Network error or CORS issue';
        } else if (error instanceof SyntaxError) {
            errorMessage = 'Invalid JSON response';
        } else {
            errorMessage = error.message;
        }

        // Return an error response
        return json({ error: errorMessage }, { status: 500 });
    }
}
