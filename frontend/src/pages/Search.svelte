<script lang="ts">
    import Results from "../components/search/Results.svelte";
    import { API_BASE } from "../lib/api";
    import type { Player } from "../types";

    export let params: { player?: string } = {};

    let nodes: Player[] = [];
    let page = 1;
    let totalPages = 1;
    let loading = false;

    /** fetch whenever player or page changes */
    $: if (params.player) runSearch(params.player, page);

    async function runSearch(tag: string, pg: number) {
        loading = true;
        try {
            const res = await fetch(`${API_BASE}/search`, {
                method: "POST",
                headers: { "content-type": "application/json" },
                body: JSON.stringify({ gamer_tag: tag, page: pg }),
            });
            if (!res.ok) throw new Error(`API ${res.status}`);
            const json = await res.json();
            nodes = json.data?.players?.nodes ?? [];
            totalPages = json.data?.players?.pageInfo?.totalPages ?? 1;
        } catch (e) {
            console.error(e);
            nodes = [];
        } finally {
            loading = false;
        }
    }
</script>

{#if loading}
    <p style="text-align:center;margin-top:2rem">Searching…</p>
{:else}
    <Results
        {nodes}
        {page}
        {totalPages}
        onPage={(n) => {
            if (n >= 1 && n <= totalPages) page = n;
        }}
    />
{/if}
