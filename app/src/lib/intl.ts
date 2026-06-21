export const sortCollator = new Intl.Collator(undefined, {
  numeric: true,
  sensitivity: "variant",
  usage: "sort",
});

export function compare(a: string, b: string): number {
  return sortCollator.compare(a, b);
}
