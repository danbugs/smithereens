/// <reference types="vite/client" />

export const API_BASE: string = '/api';

export async function fetchEventSets(
  userId: number,
  slug: string
): Promise<any> {
  const res = await fetch(
    `${API_BASE}/event-sets/${encodeURIComponent(slug)}?user_id=${userId}`
  );
  if (!res.ok) throw new Error(res.statusText);
  return (await res.json()).data.event;
}
