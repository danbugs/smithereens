<script lang="ts">
    import type { Player } from "../../types";
    import { flagUrl } from "../../lib/flags";

    export let nodes: Player[] = [];
    export let page = 1;
    export let totalPages = 1;

    /** callback prop supplied by the parent */
    export let onPage: (n: number) => void = () => {};

    const avatar = (p: Player) =>
        p.user?.images?.find((i) => i.type === "profile")?.url ??
        `https://ui-avatars.com/api/?name=${encodeURIComponent(p.gamerTag)}&background=random`;

    // Filter out players without discriminator
    $: filteredNodes = nodes.filter((p) => p.user?.discriminator);
</script>

{#if filteredNodes.length === 0}
    <p class="empty">No players found.</p>
{:else}
    <div class="results-container">
        {#each filteredNodes as p}
            <a
                class="result-row"
                href={`/#/profile/${encodeURIComponent(p.user.discriminator)}`}
                rel="noopener"
            >
                <img class="avatar" src={avatar(p)} alt="" />
                <div class="info">
                    <span class="tag">
                        {#if p.prefix}{p.prefix} |
                        {/if}
                        {p.gamerTag}
                        {#if p.user?.location?.country}
                            <img
                                class="flag-img"
                                src={flagUrl(p.user.location.country)}
                                alt=""
                            />
                        {/if}
                    </span>
                    {#if p.user?.name}
                        <span class="name">{p.user.name}</span>
                    {/if}
                </div>
            </a>
        {/each}
    </div>

    <nav class="pager">
        <button on:click={() => onPage(page - 1)} disabled={page === 1}
            >Prev</button
        >
        <span>{page} / {totalPages}</span>
        <button on:click={() => onPage(page + 1)} disabled={page === totalPages}
            >Next</button
        >
    </nav>
{/if}

<style>
    .results-container {
        width: clamp(280px, 95%, 900px);
        margin: 2rem auto 2.5rem;
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }
    .result-row {
        width: 100%;
        display: flex;
        align-items: center;
        gap: 1rem;
        padding: 0.9rem 1rem;
        border: 1px solid #e5e7eb;
        border-radius: 14px;
        background: #fff;
        text-decoration: none;
        box-shadow: 0 1px 2px rgb(0 0 0 / 4%);
        transition: box-shadow 0.15s;
    }
    .result-row:hover {
        box-shadow: 0 2px 6px rgb(0 0 0 / 8%);
    }
    .avatar {
        width: 48px;
        height: 48px;
        border-radius: 50%;
        object-fit: cover;
    }
    .info {
        display: flex;
        flex-direction: column;
    }
    .tag {
        font-weight: 600;
        color: #111827;
        font-size: 1rem;
        display: flex;
        align-items: center;
        gap: 0.4rem;
    }
    .flag-img {
        width: 24px;
        height: 16px;
        object-fit: contain;
        flex-shrink: 0; /* keep it from stretching */
    }
    .name {
        font-size: 0.85rem;
        color: #6b7280;
        margin-top: 0.1rem;
    }
    .empty {
        text-align: center;
        margin-top: 2rem;
    }
    .pager {
        display: flex;
        justify-content: center;
        gap: 1rem;
        margin-bottom: 4rem;
    }
    .pager button {
        padding: 0.4rem 0.9rem;
        border: 1px solid #d1d5db;
        border-radius: 8px;
        background: #fff;
        cursor: pointer;
    }
    .pager button[disabled] {
        opacity: 0.4;
        cursor: not-allowed;
    }
</style>
