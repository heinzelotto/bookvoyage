<script lang="ts">
    import { onMount } from 'svelte';

    let books :{book_info: any, book_logs: any}[] = [];

    onMount(async () => {
        var book_list_response = await fetch('http://localhost:5173/api/book_list');
        let book_list = await book_list_response.json();

        for (var book of book_list) {
            var book_logs_response = await fetch(`http://localhost:5173/api/book_logs/?book_id=${book["id"]}`);
            var book_logs = await book_logs_response.json();


            books.push({"book_info": book, "book_logs": book_logs});
        }

        // svelte only updates on assignments
        books = books;

        // cleanup function
        return () => {};
    });

</script>

<ul>
    {#each books as book}
    <li>{book.book_info.id} {book.book_info.title} {book.book_info.author} {book.book_info.code}
    <ol>
        {#each book.book_logs as book_log}
        <li>{book_log.id} {book_log.commenter} {book_log.lat} {book_log.lon} {book_log.comment}</li>
        {/each}
    </ol>
    </li>
{/each}
</ul>

<style>

</style>