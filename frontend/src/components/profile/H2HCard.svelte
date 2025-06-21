<script lang="ts">
    import { fly } from "svelte/transition";

    export let h2h: {
        playerId: string;
        gamerTag: string;
        wins: number;
        losses: number;
        sets: Array<{
            tournament: string;
            event: string;
            score: string;
            winner: "user" | "opponent";
            characters?: { user?: string[]; opponent?: string[] };
        }>;
    };
    export let userName: string;

    let expanded = false;

    $: winRate = (h2h.wins / (h2h.wins + h2h.losses)) * 100;
    $: isWinning = h2h.wins > h2h.losses;

    // Reset expanded state when h2h data changes (page change)
    $: h2h, (expanded = false);
</script>

<article class="wrapper">
    <header
        class="h2h-header"
        role="button"
        tabindex="0"
        aria-expanded={expanded}
        on:click={() => (expanded = !expanded)}
        on:keydown={(e) => e.key === "Enter" && (expanded = !expanded)}
    >
        <div class="h2h-info">
            <strong class="opponent-name">{h2h.gamerTag}</strong>
            <div class="h2h-stats">
                <span class="h2h-record {isWinning ? 'winning' : 'losing'}">
                    {h2h.wins}-{h2h.losses}
                </span>
                <span class="h2h-winrate">
                    {winRate.toFixed(1)}% WR
                </span>
            </div>
        </div>
        <div class="expand-icon" class:rotated={expanded}>▼</div>
    </header>

    {#if expanded}
        <div class="h2h-details" transition:fly={{ y: -10, duration: 200 }}>
            {#each h2h.sets as set, i}
                <div
                    class="set-detail {set.winner === 'user' ? 'won' : 'lost'}"
                >
                    <div class="set-info">
                        <span class="set-event">{set.event}</span>
                        <span class="set-tournament">@ {set.tournament}</span>
                    </div>
                    <div class="set-result">
                        {#if set.characters}
                            <div class="chars-wrapper">
                                <div class="char-group">
                                    {#if set.characters?.user?.length}
                                        {#each set.characters.user as char}
                                            <img
                                                src={char}
                                                alt=""
                                                class="char-icon"
                                            />
                                        {/each}
                                    {:else}
                                        <span class="char-unknown">×</span>
                                    {/if}
                                </div>
                                <span class="vs-text">vs</span>
                                <div class="char-group">
                                    {#if set.characters?.opponent?.length}
                                        {#each set.characters.opponent as char}
                                            <img
                                                src={char}
                                                alt=""
                                                class="char-icon"
                                            />
                                        {/each}
                                    {:else}
                                        <span class="char-unknown">×</span>
                                    {/if}
                                </div>
                            </div>
                        {/if}
                        <span
                            class="score {set.winner === 'user'
                                ? 'win'
                                : 'loss'}"
                        >
                            {set.score}
                        </span>
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</article>

<style>
    .wrapper {
        width: 100%;
        border: 1px solid #e5e7eb;
        border-radius: 8px;
        margin: 0.5rem 0;
        background: #fff;
        transition: border-color 0.12s;
        overflow: hidden;
    }

    .wrapper:hover {
        border-color: #d1d5db;
    }

    .h2h-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0.7rem 1rem;
        cursor: pointer;
        user-select: none;
        gap: 1rem;
    }

    .h2h-header:hover {
        background: #f9fafb;
    }

    .h2h-info {
        display: flex;
        align-items: center;
        gap: 0.8rem;
        flex: 1;
        min-width: 0;
    }

    .opponent-name {
        font-size: 0.95rem;
        color: #111827;
        min-width: 0;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .h2h-stats {
        display: flex;
        align-items: center;
        gap: 0.8rem;
        flex-shrink: 0;
    }

    @media (max-width: 640px) {
        .h2h-header {
            padding: 0.6rem 0.75rem;
        }

        .h2h-info {
            gap: 0.5rem;
        }

        .opponent-name {
            font-size: 0.85rem;
        }

        .h2h-stats {
            gap: 0.4rem;
        }

        .h2h-record {
            font-size: 0.75rem;
            padding: 2px 6px;
        }

        .h2h-winrate {
            font-size: 0.75rem;
        }

        .expand-icon {
            font-size: 0.65rem;
            margin-left: 0.25rem;
        }
    }

    .h2h-record {
        font-weight: 600;
        padding: 2px 8px;
        border-radius: 4px;
        font-size: 0.85rem;
    }

    .h2h-record.winning {
        background: rgba(16, 185, 129, 0.1);
        color: #059669;
    }

    .h2h-record.losing {
        background: rgba(239, 68, 68, 0.1);
        color: #dc2626;
    }

    .h2h-winrate {
        font-size: 0.85rem;
        color: #6b7280;
    }

    .expand-icon {
        font-size: 0.75rem;
        color: #9ca3af;
        transition: transform 0.2s;
    }

    .expand-icon.rotated {
        transform: rotate(180deg);
    }

    .h2h-details {
        border-top: 1px solid #f3f4f6;
        padding: 0.5rem;
    }

    .set-detail {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0.5rem;
        border-radius: 6px;
        margin-bottom: 0.25rem;
        font-size: 0.85rem;
    }

    .set-detail.won {
        background: rgba(16, 185, 129, 0.05);
    }

    .set-detail.lost {
        background: rgba(239, 68, 68, 0.05);
    }

    .set-info {
        display: flex;
        flex-direction: column;
        gap: 2px;
        flex: 1;
    }

    .set-event {
        font-weight: 500;
        color: #374151;
    }

    .set-tournament {
        font-size: 0.8rem;
        color: #6b7280;
    }

    .set-result {
        display: flex;
        align-items: center;
        gap: 0.75rem;
    }

    .chars-wrapper {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .char-group {
        display: flex;
        gap: 2px;
    }

    .char-icon {
        width: 24px;
        height: 24px;
        object-fit: contain;
    }

    .vs-text {
        font-size: 0.75rem;
        color: #9ca3af;
    }

    .score {
        font-weight: 600;
        font-size: 0.9rem;
        padding: 2px 6px;
        border-radius: 4px;
    }

    .score.win {
        color: #059669;
    }

    .score.loss {
        color: #dc2626;
    }

    .char-unknown {
        display: inline-flex;
        width: 24px;
        height: 24px;
        align-items: center;
        justify-content: center;
        color: #9ca3af;
        font-size: 1rem;
        font-weight: bold;
    }
</style>
