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

    const bookCodeLen = 8;
    let bookCode = "";
    let fetchedBook:{book_info: any, book_logs: any} | null = null;

    const onBookCodeInput = async() => {
        fetchedBook = null;

        console.log(bookCode.length);
        if (bookCode.length == bookCodeLen) {
            var book_list_response = await fetch(`http://localhost:5173/api/book_list/?book_code=${bookCode}`);
            var book_list = await book_list_response.json();

            console.log(book_logs);
            if (book_list.length == 1) {
                let book = book_list[0];

                var book_logs_response = await fetch(`http://localhost:5173/api/book_logs/?book_id=${book["id"]}`);
                var book_logs = await book_logs_response.json();

                fetchedBook = {"book_info": book, "book_logs": book_logs};
                console.log(fetchedBook);
            }
        }
    }

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

<label for="bookCode"><strong>Enter Book Code</strong></label>
<input id="bookCode" name="bookCode" type="text" placeholder="Enter Book Code" bind:value="{bookCode}"
on:input={onBookCodeInput}>

{#if fetchedBook != null}
We got the book!
<li>{fetchedBook.book_info.id} {fetchedBook.book_info.title} {fetchedBook.book_info.author} {fetchedBook.book_info.code}
    <ol>
        {#each fetchedBook.book_logs as book_log}
        <li>{book_log.id} {book_log.commenter} {book_log.lat} {book_log.lon} {book_log.comment}</li>
        {/each}
    </ol>
    </li>
{/if}

<style>
    input[type=text] {
        padding: 10px;
        margin:10px 0;
        border: none;
        border-radius:20px;
        box-shadow:0 0 15px 4px rgba(0,0,0,0.06);
        width:100%;
        font-family: inherit;
        font-size: inherit;
    }
</style>