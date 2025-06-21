<script lang="ts">
    import { push } from "svelte-spa-router";

    export let logoSrc = "/favicon.ico";
    let query = "";

    function handleSubmit(e: SubmitEvent) {
        e.preventDefault();
        const tag = query.trim();
        if (!tag) return;

        push(`/search/${encodeURIComponent(tag)}`);

        query = "";
    }
</script>

<header class="site-header">
    <a href="/#/" class="brand">
        <img src={logoSrc} alt="Smithereens logo" width="28" height="28" />
        <span>Smithereens</span>
    </a>

    <form class="search-form" on:submit|preventDefault={handleSubmit}>
        <input
            class="search-input"
            type="search"
            placeholder="Search for a player"
            bind:value={query}
        />
    </form>
</header>
