<script lang="ts">
    import { fly } from "svelte/transition";
    import { fetchEventSets } from "../../lib/api";

    /** Single‑tournament collapsible card */
    export let event: any; // { slug, name, tournament, seed, placement, spr }
    export let userId!: number;
    export let setsData: any = null; // Pre-fetched sets data from parent

    /* cross‑card, same‑tab cache */
    const setsCache: Map<string, any> = ((globalThis as any).__sggSets ||=
        new Map());

    let open = false;
    let loading = false;
    let sets: any[] = [];
    let error = "";
    let userEntrantId: number = 0;

    // Create a unique key for this specific event instance
    $: eventKey = `${event.slug}_${event.placement}_${event.seed}`;

    // Reset when event changes
    $: if (eventKey) {
        open = false;
        sets = [];
        error = "";
        userEntrantId = 0;
    }

    async function toggle() {
        open = !open;
        if (!open) return;

        // First check if we have pre-fetched data from parent
        if (setsData) {
            sets = setsData.sets;
            userEntrantId = setsData.entrantId;
            return;
        }

        // Then check the global cache
        if (setsCache.has(event.slug)) {
            const cached = setsCache.get(event.slug)!;
            sets = cached.sets;
            userEntrantId = cached.entrantId;
            return;
        }

        // Finally fetch from API
        loading = true;
        try {
            const data = await fetchEventSets(userId, event.slug);
            sets = data?.userEntrant?.paginatedSets?.nodes ?? [];
            userEntrantId = data?.userEntrant?.id ?? 0;
            // Don't mutate the event object
            setsCache.set(event.slug, { sets, entrantId: userEntrantId });
        } catch (e: any) {
            error = e.message ?? "error";
        } finally {
            loading = false;
        }
    }

    const userWonSet = (s: any) => s.winnerId === userEntrantId;
    const userWonGame = (g: any) => g.winnerId === userEntrantId;
    const charIcon = (sel: any) => sel?.character?.images?.[0]?.url ?? "";

    // Get ordinal suffix
    const getOrdinal = (n: number) => {
        const s = ["th", "st", "nd", "rd"];
        const v = n % 100;
        return n + (s[(v - 20) % 10] || s[v] || s[0]);
    };

    // Get placement class for styling - reactive
    $: getPlacementClass = () => {
        if (!event || !event.placement) return "";
        if (event.placement === 1) return "gold";
        if (event.placement === 2) return "silver";
        if (event.placement === 3) return "bronze";
        return "";
    };

    // Reactive href
    $: href = `https://www.start.gg/${event.slug}`;

    // Make placement values reactive
    $: currentPlacement = event?.placement || null;
    $: currentSeed = event?.seed || null;
    $: currentSpr = event?.spr;
    $: currentNumEntrants = event?.numEntrants || null;
</script>

<article class="wrapper {open ? 'open' : ''}">
    <header
        class="bar"
        role="button"
        tabindex="0"
        aria-expanded={open}
        on:click={toggle}
        on:keydown={(e) => e.key === "Enter" && toggle()}
    >
        <div class="header-content">
            <div class="title">
                {event.name} — <strong>{event.tournament.name}</strong>
            </div>
            <div class="meta">
                <span>Seed {currentSeed ?? "…"}</span>
                <span class="placement {getPlacementClass()}"
                    >Placement {currentPlacement
                        ? `${getOrdinal(currentPlacement)}${currentNumEntrants ? `/${currentNumEntrants}` : ""}`
                        : "…"}</span
                >
                <span class="spr">SPR {Number.isNaN(currentSpr) ? "…" : currentSpr}</span>
                <a
                    class="ext"
                    {href}
                    target="_blank"
                    aria-label="Open on StartGG"
                    on:click|stopPropagation
                >
                    <img
                        src="https://www.start.gg/__static/images/favicon/favicon.ico"
                        alt="StartGG"
                        class="startgg-icon"
                    />
                </a>
            </div>
        </div>
    </header>

    {#if open}
        <div transition:fly={{ y: 6, duration: 160 }}>
            {#if loading}
                <p class="state">loading sets…</p>
            {:else if error}
                <p class="state err">{error}</p>
            {:else if sets.length === 0}
                <p class="state">No set data.</p>
            {:else}
                <section class="sets">
                    {#each sets as set}
                        <div class="set {userWonSet(set) ? 'win' : ''}">
                            <div class="set-head">
                                {set.fullRoundText} — {set.displayScore ||
                                    (set.winnerId ? "" : "DRAW")}
                            </div>
                            {#if set.games?.length}
                                <div class="games">
                                    {#each set.games as g}
                                        <div
                                            class="game {userWonGame(g)
                                                ? 'win'
                                                : ''}"
                                            title={g.displayScore ?? ""}
                                        >
                                            {#each g.selections as sel, i}
                                                <div class="player-chars">
                                                    {#if charIcon(sel)}
                                                        <img
                                                            class="char"
                                                            src={charIcon(sel)}
                                                            alt=""
                                                        />
                                                    {:else}
                                                        <span
                                                            class="char missing"
                                                            >?</span
                                                        >
                                                    {/if}
                                                    {#if sel.entrant?.id === userEntrantId && userWonGame(g)}
                                                        <span
                                                            class="winner-mark"
                                                            >✓</span
                                                        >
                                                    {/if}
                                                </div>
                                                {#if i === 0}
                                                    <span class="vs">vs</span>
                                                {/if}
                                            {/each}
                                        </div>
                                    {/each}
                                </div>
                            {/if}
                        </div>
                    {/each}
                </section>
            {/if}
        </div>
    {/if}
</article>

<style>
    .wrapper {
        width: 100%;
        max-width: 100%;
        border: 1px solid #e5e7eb;
        border-radius: 8px;
        margin: 0.5rem 0;
        background: #fff;
        transition: border-color 0.12s;
    }
    .wrapper.open {
        border-color: #111827;
    }
    .bar {
        padding: 0.7rem 1rem;
        cursor: pointer;
    }
    .bar:hover {
        background: #f9fafb;
    }
    .header-content {
        display: flex;
        flex-direction: column;
        gap: 0.4rem;
    }
    .title {
        font-size: 0.95rem;
        color: #111827;
    }
    .meta {
        display: flex;
        gap: 0.8rem;
        font-size: 0.8rem;
        color: #4b5563;
        align-items: center;
    }
    .meta > span {
        white-space: nowrap;
    }
    .spr {
        min-width: 60px;
    }
    .ext {
        color: #b91c1c;
        text-decoration: none;
        display: inline-flex;
        align-items: center;
    }
    .ext:hover {
        opacity: 0.8;
    }
    .startgg-icon {
        width: 16px;
        height: 16px;
        vertical-align: middle;
    }

    /* Placement styling */
    .placement.gold {
        background: linear-gradient(135deg, #ffd700, #ffa500);
        color: #fff;
        padding: 2px 8px;
        border-radius: 12px;
        font-weight: 600;
    }
    .placement.silver {
        background: linear-gradient(135deg, #c0c0c0, #999);
        color: #fff;
        padding: 2px 8px;
        border-radius: 12px;
        font-weight: 600;
    }
    .placement.bronze {
        background: linear-gradient(135deg, #cd7f32, #b87333);
        color: #fff;
        padding: 2px 8px;
        border-radius: 12px;
        font-weight: 600;
    }

    .sets {
        padding: 0 1rem 0.8rem;
        display: flex;
        flex-direction: column;
        gap: 6px;
        font-size: 0.86rem;
    }
    .set {
        border: 1px solid #e5e7eb;
        border-radius: 6px;
        padding: 6px 8px;
    }
    .set.win {
        background: rgba(16, 185, 129, 0.09);
        border-color: rgba(16, 185, 129, 0.3);
    } /*green*/
    .set-head {
        font-weight: 600;
        margin-bottom: 4px;
    }
    .games {
        display: flex;
        flex-wrap: wrap;
        gap: 4px;
    }
    .game {
        display: inline-flex;
        align-items: center;
        gap: 2px;
        padding: 2px 4px;
        border: 1px solid #e5e7eb;
        border-radius: 4px;
        font-size: 0.75rem;
    }
    .game.win {
        background: rgba(16, 185, 129, 0.12);
        border-color: rgba(16, 185, 129, 0.3);
    } /*green*/
    .char {
        width: 20px;
        height: 20px;
        object-fit: contain;
        flex-shrink: 0;
    }
    .char.missing {
        display: inline-block;
        width: 20px;
        height: 20px;
        line-height: 20px;
        text-align: center;
        font-size: 14px;
        color: #6b7280;
    }
    .player-chars {
        display: inline-flex;
        align-items: center;
        gap: 2px;
        position: relative;
    }
    .winner-mark {
        color: #10b981;
        font-weight: bold;
        font-size: 0.8rem;
    }
    .vs {
        color: #6b7280;
        font-size: 0.7rem;
        margin: 0 4px;
    }

    .state {
        text-align: center;
        padding: 0.6rem 0;
        font-size: 0.85rem;
        color: #6b7280;
    }
    .state.err {
        color: #dc2626;
    }
</style>
