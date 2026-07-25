import type { Market } from "@/lib/types";

/** Icons live at /public/assets/{SYMBOL}.png or .svg — resolved at runtime. */

/** Featured pairs always appear first in market lists / pickers. */
const PINNED_MARKET_SYMBOLS = ["SOL_USDC", "BTC_USDC"] as const;

export function sortMarketsPinned(markets: Market[]): Market[] {
  const rank = (symbol: string) => {
    const key = symbol.trim().toUpperCase();
    const idx = PINNED_MARKET_SYMBOLS.indexOf(
      key as (typeof PINNED_MARKET_SYMBOLS)[number],
    );
    return idx === -1 ? PINNED_MARKET_SYMBOLS.length : idx;
  };

  return [...markets].sort((a, b) => {
    const byPin = rank(a.symbol) - rank(b.symbol);
    if (byPin !== 0) return byPin;
    return a.symbol.localeCompare(b.symbol);
  });
}

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
