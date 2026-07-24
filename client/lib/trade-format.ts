import { displayAmount } from "@/lib/balances";
import { splitMarketSymbol } from "@/lib/markets";

export type DecimalsBySymbol = Record<string, number>;

export function decimalsMapFromAssets(
  assets: { symbol: string; decimals: number }[],
): DecimalsBySymbol {
  const map: DecimalsBySymbol = {};
  for (const asset of assets) {
    map[asset.symbol.trim().toUpperCase()] = asset.decimals;
  }
  return map;
}

export function assetDecimals(
  symbol: string,
  decimalsBySymbol: DecimalsBySymbol,
): number {
  const key = symbol.trim().toUpperCase();
  const value = decimalsBySymbol[key];
  if (value === undefined) {
    throw new Error(`Unknown asset decimals for ${key}`);
  }
  return value;
}

export function marketDecimals(
  marketSymbol: string,
  decimalsBySymbol: DecimalsBySymbol,
) {
  const [base, quote] = splitMarketSymbol(marketSymbol);
  return {
    base,
    quote,
    baseDecimals: assetDecimals(base, decimalsBySymbol),
    quoteDecimals: assetDecimals(quote, decimalsBySymbol),
  };
}

export function formatPrice(price: number, quoteDecimals: number) {
  return displayAmount(price, quoteDecimals);
}

export function formatQty(quantity: number, baseDecimals: number) {
  return displayAmount(quantity, baseDecimals);
}

export function parseHumanToRaw(
  value: string,
  decimals: number,
): number | null {
  const trimmed = value.trim();
  if (!trimmed || !/^\d+(\.\d+)?$/.test(trimmed)) return null;
  const [whole, frac = ""] = trimmed.split(".");
  if (frac.length > decimals) return null;
  const padded = frac.padEnd(decimals, "0");
  const raw =
    BigInt(whole) * BigInt(10) ** BigInt(decimals) + BigInt(padded || "0");
  if (raw > BigInt(Number.MAX_SAFE_INTEGER)) return null;
  return Number(raw);
}

export function rawToHumanInput(raw: number, decimals: number): string {
  const neg = raw < 0;
  const abs = Math.abs(raw);
  const whole = Math.floor(abs / 10 ** decimals);
  const frac = String(abs % 10 ** decimals).padStart(decimals, "0");
  const trimmed = frac.replace(/0+$/, "");
  const text = trimmed ? `${whole}.${trimmed}` : String(whole);
  return neg ? `-${text}` : text;
}
