<script>

</script>

<ol>
    <li>File 1</li>
    <li>File 2</li>
</ol>

<style>
    ol {
        list-style-type: none;
        display: flex;
        flex-direction: column;
        justify-content: left;
        align-items: flex-start;
        width:auto;
        padding: 0;
        margin: 0;
        color: var(--text-default-color);
        background-color: var(--dark-bg-color);
    }

    li:last-child {
        border-bottom: 1px solid var(--primary-color)
    }

    li {
        border: 1px solid var(--primary-color);
        border-bottom: none;
        overflow-x: hidden;
        width: 10vw;
        white-space: nowrap;
    }

    li:hover, li:focus {
        background-color: var(--dark-highlight-color);
    }
</style>