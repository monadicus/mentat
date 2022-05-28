import manifest from 'cryptocurrency-icons/manifest.json';

export type ManifestEntry = { symbol: string; name: string; color: string };

const MANIFEST: ManifestEntry[] = manifest;
const GENERIC_ENTRY = MANIFEST.find(e => e.symbol === 'GENERIC');
const MANIFEST_SYMBOLS = Object.fromEntries(MANIFEST.map(e => [e.symbol, e]));
const MANIFEST_NAMES = Object.fromEntries(MANIFEST.map(e => [e.name, e]));

export function findCrypto(search: string) {
  return MANIFEST_SYMBOLS[search] ?? MANIFEST_NAMES[search] ?? GENERIC_ENTRY;
}

export function getCryptoIcon(
  item: ManifestEntry | string,
  coloring: 'black' | 'color' | 'icon' | 'white' = 'color'
): string {
  if (typeof item === 'string') item = findCrypto(item);

  return `/assets/cryptocurrency-icons/${coloring}/${item.symbol.toLowerCase()}.svg`;
}
