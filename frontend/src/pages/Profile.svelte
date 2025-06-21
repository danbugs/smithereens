<script lang="ts">
  import { onMount } from "svelte";
  import { API_BASE } from "../lib/api";
  import { flagUrl } from "../lib/flags";
  import { fetchEventSets } from "../lib/api";
  import { spr } from "../lib/spr";

  import EventCard from "../components/profile/EventCard.svelte";
  import H2HCard from "../components/profile/H2HCard.svelte";

  export let params: { slug: string };

  /* ──────────────────────────────────────────────────────────
   *  high‑level state
   * ─────────────────────────────────────────────────────── */
  let loading = true; // full‑page spinner on first paint
  let err = "";
  let user: any = null;

  /* tournaments / paging ---------------------------------- */
  let page = 1;
  let totalPages = 1;
  let events: any[] = [];
  let loadingEvents = false; // spinner for list area
  const pages = new Map<number, any[]>(); // in‑memory this session
  
  /* prefetch progress tracking ----------------------------- */
  let prefetchProgress = 0;
  let showProgressBar = false;
  let prefetchedPages = new Set<number>();
  let totalExpectedTournaments = 0;
  let actualTournamentCount = 0;
  let finalCountKnown = false;

  /* sets cache for instant loading ------------------------- */
  const setsCache = new Map<string, any>(); // eventSlug -> sets data

  /* --------------------------------------------------------
   *  stats buckets (reactive)
   * ------------------------------------------------------*/
  let wins = 0,
    losses = 0;
  let tournaments = 0;
  let winRate = 0;
  let compLabel = "…";
  let topChars: string[] = [];

  /* head-to-head tracking ---------------------------------- */
  const h2hMap = new Map<string, {
    playerId: string,
    gamerTag: string,
    wins: number,
    losses: number,
    sets: Array<{
      tournament: string,
      event: string,
      score: string,
      winner: 'user' | 'opponent',
      characters?: { user?: string[], opponent?: string[] }
    }>
  }>();
  
  let h2hList: any[] = [];
  let h2hPage = 1;
  let h2hSearchQuery = "";
  let h2hFiltered: any[] = [];
  const H2H_PER_PAGE = 10;

  $: if (tournaments) {
    winRate = (wins / (wins + losses)) * 100;
    const avgW = wins / tournaments;
    const avgL = losses / tournaments;
    const wInt = Math.round(avgW);
    let lInt = Math.round(avgL);
    if (lInt > 2) lInt = 2; // cap at 2
    compLabel = `${wInt}-${lInt}er`;
  }

  /* per‑visit processed guard to avoid double counting ------*/
  const processedSlugs = new Set<string>(); // event.slug already tallied
  const charCount = new Map<string, number>();

  function addSetStats(sets: any[], entrantId: number, eventName: string, tournamentName: string) {
    for (const s of sets || []) {
      if (!s?.winnerId || !s?.slots) continue;
      
      // Check if this is a DQ by looking at displayScore or fullRoundText
      const isDQ = (s.displayScore && s.displayScore.toUpperCase().includes('DQ')) || 
                   (s.fullRoundText && s.fullRoundText.toUpperCase().includes('DQ'));
      
      const userWon = s.winnerId === entrantId;
      
      // Only count wins/losses if not a DQ
      if (!isDQ) {
        if (userWon) wins++;
        else losses++;
      }
      
      // Find opponent in the set
      const opponent = s.slots.find((slot: any) => slot.entrant?.id !== entrantId)?.entrant;
      if (opponent && opponent.participants?.[0]?.player?.id) {
        const opponentPlayerId = opponent.participants[0].player.id;
        const opponentName = opponent.name;
        
        // Get or create h2h entry
        if (!h2hMap.has(opponentPlayerId)) {
          h2hMap.set(opponentPlayerId, {
            playerId: opponentPlayerId,
            gamerTag: opponentName,
            wins: 0,
            losses: 0,
            sets: []
          });
        }
        
        const h2h = h2hMap.get(opponentPlayerId)!;
        
        // Only count h2h wins/losses if not a DQ
        if (!isDQ) {
          if (userWon) h2h.wins++;
          else h2h.losses++;
        }
        
        // Extract character data
        const userChars: string[] = [];
        const opponentChars: string[] = [];
        
        for (const g of s.games || []) {
          for (const sel of g.selections || []) {
            if (sel.entrant?.id === entrantId && sel.character?.images?.[0]?.url) {
              userChars.push(sel.character.images[0].url);
            } else if (sel.entrant?.id === opponent.id && sel.character?.images?.[0]?.url) {
              opponentChars.push(sel.character.images[0].url);
            }
          }
        }
        
        // Add set details (including DQs for reference, but marked)
        h2h.sets.push({
          tournament: tournamentName,
          event: eventName,
          score: s.displayScore || "",
          winner: userWon ? 'user' : 'opponent',
          characters: userChars.length || opponentChars.length ? {
            user: [...new Set(userChars)],
            opponent: [...new Set(opponentChars)]
          } : undefined
        });
      }
      
      // Character usage tracking
      for (const g of s.games || []) {
        for (const sel of g.selections || []) {
          if (sel.entrant?.id === entrantId) {
            const icon = sel.character?.images?.[0]?.url;
            if (icon) charCount.set(icon, (charCount.get(icon) || 0) + 1);
          }
        }
      }
    }
  }
  
  function finalizeStats() {
    // Finalize character rankings
    const sorted = [...charCount.entries()].sort((a, b) => b[1] - a[1]);
    topChars = sorted.slice(0, 2).map(([u]) => u);
    
    // Finalize h2h list
    h2hList = Array.from(h2hMap.values())
      .sort((a, b) => (b.wins + b.losses) - (a.wins + a.losses));
    filterH2H();
  }
  
  function filterH2H() {
    if (!h2hSearchQuery.trim()) {
      h2hFiltered = h2hList;
    } else {
      const query = h2hSearchQuery.toLowerCase();
      h2hFiltered = h2hList.filter(h => 
        h.gamerTag.toLowerCase().includes(query)
      );
    }
    h2hPage = 1; // Reset to first page on filter
  }
  
  $: h2hSearchQuery, filterH2H();
  $: h2hPaginated = h2hFiltered.slice(
    (h2hPage - 1) * H2H_PER_PAGE,
    h2hPage * H2H_PER_PAGE
  );
  $: h2hTotalPages = Math.ceil(h2hFiltered.length / H2H_PER_PAGE);

  /* avatar / banner helpers --------------------------------*/
  const pImage = (t: string) =>
    user?.images?.find((i: any) => i.type === t)?.url;
  const tag = () =>
    (user.player.prefix ? `${user.player.prefix} | ` : "") +
    user.player.gamerTag;
  const avatar = () =>
    pImage("profile") || 
    `https://ui-avatars.com/api/?name=${encodeURIComponent(user.player.gamerTag)}&background=random`;

  /* enrich event with sets and cache them ------------------*/
  async function enrichStats(list: any[], uid: number) {
    // Create a new array to store enriched events
    const enrichedList = await Promise.all(
      list.map(async (ev) => {
        if (processedSlugs.has(ev.slug)) {
          return ev; // Return as-is if already processed
        }
        
        try {
          const d = await fetchEventSets(uid, ev.slug);
          const ent = d.userEntrant;
          
          // Cache the sets data for EventCard to use
          setsCache.set(ev.slug, {
            sets: ent.paginatedSets?.nodes || [],
            entrantId: ent.id
          });
          
          addSetStats(ent.paginatedSets?.nodes || [], ent.id, ev.name, ev.tournament.name);
          tournaments++;
          actualTournamentCount++; // Track actual count
          processedSlugs.add(ev.slug);
          
          // Return a new object with enriched data
          return {
            ...ev,
            seed: ent.initialSeedNum,
            placement: ent.standing?.placement ?? 0,
            spr: spr(ent.initialSeedNum, ent.standing?.placement ?? 0),
            _statsDone: true
          };
        } catch {
          // On error, return the original event
          return ev;
        }
      }),
    );
    
    // Replace the original array contents with enriched data
    list.length = 0;
    list.push(...enrichedList);
    
    finalizeStats();
  }

  /* first load ---------------------------------------------*/
  onMount(() => loadProfile());

  async function loadProfile() {
    loading = true;
    try {
      const r = await fetch(
        `${API_BASE}/profile/${encodeURIComponent(params.slug)}?page=1`,
      );
      if (!r.ok) throw new Error(r.statusText);
      const j = await r.json();
      user = j.data.user;
      totalPages = user.events.pageInfo.totalPages;
      
      // Calculate expected total tournaments (assuming ~10 per page, last page might have fewer)
      const firstPageCount = user.events.nodes.length;
      totalExpectedTournaments = totalPages > 1 
        ? (totalPages - 1) * 10 + firstPageCount 
        : firstPageCount;

      // render fast with first page
      await loadPage(1, user.events.nodes);
      
      // Show progress bar for remaining pages
      if (totalPages > 1) {
        showProgressBar = true;
        // Start background prefetch
        prefetchAllPages();
      }
    } catch (e: any) {
      err = e.message;
    } finally {
      loading = false;
    }
  }

  async function prefetchAllPages() {
    for (let p = 2; p <= totalPages; p++) {
      if (!prefetchedPages.has(p)) {
        await silentPrefetch(p);
        // Update progress
        prefetchProgress = (prefetchedPages.size / (totalPages - 1)) * 100;
      }
    }
    // Hide progress bar when done
    showProgressBar = false;
    finalCountKnown = true;
  }

  async function silentPrefetch(n: number) {
    if (pages.has(n)) return;
    try {
      const r = await fetch(
        `${API_BASE}/profile/${encodeURIComponent(params.slug)}?page=${n}`,
      );
      const nodes = (await r.json()).data.user.events.nodes;
      // Create fresh copies to avoid mutations
      const freshEvents = nodes.map((e: any) => ({...e}));
      await enrichStats(freshEvents, user.id);
      pages.set(n, freshEvents);
      prefetchedPages.add(n);
    } catch {
      /* ignore */
    }
  }

  /* page navigation – prefer memory before network ----------*/
  async function loadPage(n: number, initial?: any[]) {
    if (n < 1 || (totalPages && n > totalPages)) return;
    
    // Only update page if we're not in the middle of loading
    if (!loadingEvents) {
      page = n;
    }

    // If already in memory, use fresh copies
    if (pages.has(n)) {
      // Always provide fresh copies to prevent cross-contamination
      events = pages.get(n)!.map(e => ({...e}));
      page = n; // Update page after successful load
      return;
    }

    // If this is the initial load, use provided data
    if (initial && n === 1) {
      const freshEvents = initial.map(e => ({...e}));
      await enrichStats(freshEvents, user.id);
      pages.set(n, freshEvents);
      events = freshEvents.map(e => ({...e}));
      page = n; // Update page after successful load
      return;
    }

    // Otherwise fetch from network
    loadingEvents = true;
    try {
      const r = await fetch(
        `${API_BASE}/profile/${encodeURIComponent(params.slug)}?page=${n}`,
      );
      const nodes = (await r.json()).data.user.events.nodes;
      const freshEvents = nodes.map((e: any) => ({...e}));
      await enrichStats(freshEvents, user.id);
      pages.set(n, freshEvents);
      events = freshEvents.map(e => ({...e}));
      page = n; // Update page after successful load
      
      // Mark as prefetched if it wasn't already
      if (!prefetchedPages.has(n)) {
        prefetchedPages.add(n);
        prefetchProgress = (prefetchedPages.size / (totalPages - 1)) * 100;
      }
    } catch {
      /* show whatever is already there */
      // Don't update page on error
    } finally {
      loadingEvents = false;
    }
  }
  
  const goto = (n: number) => loadPage(n);

  // Helper to get cached sets for EventCard
  export function getCachedSets(eventSlug: string) {
    return setsCache.get(eventSlug);
  }
</script>

{#if loading}
  <div class="page-spinner"></div>
{:else if err}
  <p class="error">{err}</p>
{:else}
  <div class="banner" style={`background-image:url(${pImage("banner")})`}></div>

  <section class="card">
    <img
      class="avatar"
      src={avatar()}
      alt="profile"
    />
    <div class="meta">
      <h1>{tag()}</h1>
      <p class="loc">
        {#if user.location}
          {user.location.city ? `${user.location.city}, ` : ""}
          {user.location.state ? `${user.location.state}, ` : ""}
          {user.location.country}
          {#if user.location.country}
            <img class="flag" src={flagUrl(user.location.country)} alt />
          {/if}
        {/if}
      </p>
      <div class="socials">
        {#each user.authorizations as a}
          {#if a.url?.includes("twitter.com")}
            <a class="social" href={a.url} target="_blank"
              ><img
                src="https://cdn.jsdelivr.net/npm/simple-icons@11/icons/x.svg"
                alt
              /><span>@{a.externalUsername}</span></a
            >
          {:else if a.url?.includes("twitch.tv")}
            <a class="social" href={a.url} target="_blank"
              ><img
                src="https://cdn.jsdelivr.net/npm/simple-icons@11/icons/twitch.svg"
                alt
              /><span>{a.externalUsername}</span></a
            >
          {:else if a.url}
            <a class="social" href={a.url} target="_blank"
              ><img
                src="https://cdn.jsdelivr.net/npm/simple-icons@11/icons/discord.svg"
                alt
              /><span>{a.externalUsername}</span></a
            >
          {:else}
            <span class="social no-link"
              ><img
                src="https://cdn.jsdelivr.net/npm/simple-icons@11/icons/discord.svg"
                alt
              /><span>{a.externalUsername}</span></span
            >
          {/if}
        {/each}
      </div>
    </div>
  </section>

  <div class="stats-row">
    {#if showProgressBar}
      <div class="progress-container">
        <div class="progress-bar" style="width: {prefetchProgress}%"></div>
        <span class="progress-text">Loading stats and head-to-head data... {Math.round(prefetchProgress)}%</span>
      </div>
    {/if}
    <div class="stat-box">
      <span>WIN RATE</span><b
        >{wins + losses ? winRate.toFixed(1) + "%" : "…"}</b
      >
    </div>
    <div class="stat-box">
      <span>WIN–LOSSES</span><b>{wins ? `${wins}-${losses}` : "…"}</b>
    </div>
    <div class="stat-box">
      <span>COMPETITOR TYPE</span><b>{wins ? compLabel : "…"}</b>
    </div>
    <div class="stat-box">
      <span>TOURNAMENTS ENTERED</span><b>{finalCountKnown ? actualTournamentCount : `~${totalExpectedTournaments}`}</b>
    </div>
    {#if topChars.length}
      <div class="stat-box char-box">
        <span>MAIN CHARACTER{topChars.length > 1 ? "S" : ""}</span>
        <b
          >{#each topChars as src}<img
              class="char-icon"
              {src}
              alt="character"
            />{/each}</b
        >
      </div>
    {/if}
  </div>

  <h2 class="sect-title">Tournaments</h2>
  <div class="tournaments-container">
    {#if loadingEvents}
      <p style="text-align:center;margin-top:1rem">Loading tournaments…</p>
    {:else if pages.has(page)}
      {#each pages.get(page) as ev}
        <EventCard event={ev} userId={user.id} setsData={getCachedSets(ev.slug)} />
      {/each}
    {:else}
      {#each events as ev}
        <EventCard event={ev} userId={user.id} setsData={getCachedSets(ev.slug)} />
      {/each}
    {/if}
  </div>

  <nav class="pager">
    <button on:click={() => goto(page - 1)} disabled={page === 1}>Prev</button>
    <button class:active={page === 1} on:click={() => goto(1)}>1</button>
    {#if page > 3}<span>…</span>{/if}
    {#each [page - 1, page, page + 1] as n}
      {#if n > 1 && n < totalPages}
        <button class:active={n === page} on:click={() => goto(n)}>{n}</button>
      {/if}
    {/each}
    {#if page < totalPages - 2}<span>…</span>{/if}
    {#if totalPages > 1}
      <button
        class:active={page === totalPages}
        on:click={() => goto(totalPages)}>{totalPages}</button
      >
    {/if}
    <button on:click={() => goto(page + 1)} disabled={page === totalPages}
      >Next</button
    >
  </nav>

  <h2 class="sect-title">Head-to-Heads</h2>
  <div class="h2h-container">
    {#if showProgressBar}
      <p class="placeholder">Loading head-to-head data...</p>
    {:else if h2hList.length === 0}
      <p class="placeholder">No head-to-head data yet</p>
    {:else}
      <div class="h2h-search">
        <input
          type="text"
          placeholder="Search opponents by gamertag..."
          bind:value={h2hSearchQuery}
          class="search-input"
        />
      </div>
      
      {#each h2hPaginated as h2h}
        <H2HCard {h2h} userName={tag()} />
      {/each}
      
      {#if h2hTotalPages > 1}
        <nav class="pager h2h-pager">
          <button on:click={() => h2hPage--} disabled={h2hPage === 1}>Prev</button>
          <span>Page {h2hPage} of {h2hTotalPages}</span>
          <button on:click={() => h2hPage++} disabled={h2hPage === h2hTotalPages}>Next</button>
        </nav>
      {/if}
    {/if}
  </div>
{/if}

<style>
  /* styles from your previous version */
  .loc {
    display: inline-flex;
    align-items: center;
    gap: 4px;
  }
  .page-spinner {
    height: 40vh;
    display: flex;
    justify-content: center;
    align-items: center;
  }
  .page-spinner::after {
    content: "";
    width: 38px;
    height: 38px;
    border: 4px solid #e5e7eb;
    border-top-color: #b91c1c;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }
  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
  .error {
    color: #dc2626;
    text-align: center;
    margin-top: 2rem;
  }
  .banner {
    height: 140px;
    background-size: cover;
    background-position: center;
    border-bottom: 1px solid #e5e7eb;
  }
  .card {
    max-width: 900px;
    margin: -72px auto 1.5rem;
    padding: 1rem 1.2rem;
    background: #fff;
    border-radius: 12px;
    display: flex;
    gap: 1rem;
    align-items: center;
    box-shadow: 0 2px 6px #0002;
    border: 1px solid #e5e7eb;
  }
  .avatar {
    width: 96px;
    height: 96px;
    border-radius: 50%;
    object-fit: cover;
    flex-shrink: 0;
  }
  .meta h1 {
    margin: 0;
    font-size: 1.6rem;
    line-height: 1.25;
    color: #111827;
  }
  .meta p {
    margin: 0.15rem 0 0.3rem;
    font-size: 0.9rem;
    color: #6b7280;
  }
  .flag {
    width: 24px;
    height: 16px;
    vertical-align: middle;
    margin-left: 4px;
  }
  .socials {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin-top: 4px;
  }
  .social {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    color: #b91c1c;
    text-decoration: none;
    font-size: 0.85rem;
  }
  .social img {
    width: 18px;
    height: 18px;
    opacity: 0.85;
  }
  .social:hover img {
    opacity: 1;
  }
  .no-link {
    cursor: default;
  }
  .stats-row {
    max-width: 900px;
    margin: 1rem auto 2rem;
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    padding: 0 0.5rem;
    justify-content: center;
    position: relative;
  }
  @media (max-width: 480px) {
    .stats-row {
      margin-top: 1rem;
    }
  }
  .stat-box {
    flex: 1 1 140px;
    border: 1px solid #e5e7eb;
    border-radius: 10px;
    background: #fff;
    text-align: center;
    padding: 0.6rem 0.4rem;
    color: #4b5563;
    font-size: 0.75rem;
    font-weight: 600;
  }
  .stat-box b {
    display: block;
    font-size: 1.25rem;
    color: #111827;
    margin-top: 2px;
  }
  .progress-container {
    position: absolute;
    top: -30px;
    left: 0;
    right: 0;
    height: 20px;
    background: #f3f4f6;
    border-radius: 10px;
    overflow: hidden;
    box-shadow: inset 0 1px 3px rgba(0,0,0,0.1);
  }
  .progress-bar {
    height: 100%;
    background: #b91c1c;
    transition: width 0.3s ease;
    border-radius: 10px;
  }
  .progress-text {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    font-size: 0.75rem;
    color: #374151;
    font-weight: 600;
    white-space: nowrap;
    background: rgba(255, 255, 255, 0.9);
    padding: 2px 8px;
    border-radius: 10px;
    box-shadow: 0 1px 3px rgba(0,0,0,0.2);
  }
  .sect-title {
    text-align: center;
    margin: 0 0 0.5rem;
    color: #374151;
  }
  .pager {
    display: flex;
    gap: 0.4rem;
    justify-content: center;
    margin: 1rem auto 2rem;
    max-width: 900px;
    padding: 0 1rem;
  }
  
  @media (max-width: 640px) {
    .pager {
      padding: 0 0.5rem;
    }
  }
  .pager button {
    padding: 0.3rem 0.6rem;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    background: #fff;
    cursor: pointer;
  }
  .pager button.active {
    background: #2563eb;
    color: #fff;
  }
  .placeholder {
    color: #9ca3af;
    text-align: center;
    margin: 2rem 0;
  }
  .char-box b {
    display: flex;
    gap: 6px;
    justify-content: center;
    align-items: center;
  }
  .char-icon {
    width: 28px;
    height: 28px;
    object-fit: contain;
  }
  .tournaments-container {
    width: 100%;
    max-width: 900px;
    margin: 0 auto;
    padding: 0 1rem;
  }
  .h2h-container {
    width: 100%;
    max-width: 900px;
    margin: 0 auto;
    padding: 0 1rem;
  }
  .h2h-search {
    margin-bottom: 1rem;
  }
  .search-input {
    width: 100%;
    padding: 0.5rem 1rem;
    border: 1px solid #e5e7eb;
    border-radius: 8px;
    font-size: 0.9rem;
    outline: none;
    transition: border-color 0.2s;
  }
  .search-input:focus {
    border-color: #b91c1c;
  }
  .h2h-pager {
    margin-top: 1rem;
    margin-bottom: 2rem;
  }
  
  /* Mobile responsive */
  @media (max-width: 640px) {
    .tournaments-container,
    .h2h-container {
      padding: 0 0.5rem;
    }
  }
</style>