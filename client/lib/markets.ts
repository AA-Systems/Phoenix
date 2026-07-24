/** Static icons in /public/assets/{SYMBOL}.png|svg */

const KNOWN = new Set(["SOL", "USDC", "USD", "INR"]);

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

export function assetIconSrc(symbol: string): string | null {
  const key = symbol.trim().toUpperCase();
  if (!KNOWN.has(key)) return null;
  if (key === "INR") return "/assets/INR.svg";
  return `/assets/${key}.png`;
}
