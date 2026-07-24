/** Icons live at /public/assets/{SYMBOL}.png or .svg — resolved at runtime. */

export function formatMarketPair(symbol: string): string {
  return symbol.replace(/[_/]/g, " / ").replace(/\s+/g, " ").trim();
}

export function marketInitials(symbol: string): string {
  const [base] = symbol.split(/[_/]/);
  return (base ?? symbol).slice(0, 2).toUpperCase();
}

export function splitMarketSymbol(symbol: string): [string, string] {
  const parts = symbol.split(/[_/]/).filter(Boolean);
  return [parts[0]?.toUpperCase() ?? symbol, parts[1]?.toUpperCase() ?? ""];
}

export function assetIconCandidates(symbol: string): string[] {
  const key = symbol.trim().toUpperCase();
  if (!key) return [];
  return [`/assets/${key}.png`, `/assets/${key}.svg`];
}
