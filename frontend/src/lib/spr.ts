/** Seed-Performance Ratio */
export function spr(seed: number, placement: number): number {
  const f = (x: number) =>
    x === 1
      ? 0
      : Math.floor(Math.log2(x - 1)) +
        Math.ceil(Math.log2((2 / 3) * x));
  return f(seed) - f(placement);
}