export interface Player {
  id: number;
  prefix?: string;
  gamerTag: string;
  url: string;
  user: {
    name?: string;
    discriminator?: string;
    location?: { country?: string };
    images: { type: string; url: string }[];
  };
}