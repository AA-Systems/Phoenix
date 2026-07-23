import type { AssetBalance, LedgerEntry } from "@/lib/types";

export function displayAmount(value: number, decimals: number) {
  return new Intl.NumberFormat("en-US", {
    maximumFractionDigits: Math.min(decimals, 8),
  }).format(value / 10 ** decimals);
}

export function displayDelta(value: number, decimals: number) {
  const sign = value > 0 ? "+" : "";
  return `${sign}${displayAmount(value, decimals)}`;
}

export function portfolioStats(
  balances: AssetBalance[],
  ledger: LedgerEntry[],
) {
  const lockedAssets = balances.filter((b) => b.locked > 0).length;
  const freeAssets = balances.filter(
    (b) => b.available > 0 && b.locked === 0,
  ).length;

  const ratios = balances
    .map((b) => {
      const total = b.available + b.locked;
      return total > 0 ? b.locked / total : null;
    })
    .filter((ratio): ratio is number => ratio !== null);

  const lockedShare =
    ratios.length > 0
      ? Math.round(
          (ratios.reduce((sum, ratio) => sum + ratio, 0) / ratios.length) * 100,
        )
      : 0;

  return {
    assetCount: balances.length,
    lockedAssets,
    freeAssets,
    lockedShare,
    ledgerCount: ledger.length,
  };
}

/** Last 7 day buckets of ledger event counts (oldest → newest). */
export function activityBuckets(entries: LedgerEntry[], days = 7) {
  const today = new Date();
  today.setHours(0, 0, 0, 0);

  const buckets = Array.from({ length: days }, (_, index) => {
    const day = new Date(today);
    day.setDate(today.getDate() - (days - 1 - index));
    return {
      key: day.toISOString().slice(0, 10),
      label: day.toLocaleDateString("en-US", { weekday: "short" }),
      count: 0,
      deposits: 0,
      locks: 0,
    };
  });

  const byKey = new Map(buckets.map((b) => [b.key, b]));

  for (const entry of entries) {
    const day = new Date(entry.created_at);
    day.setHours(0, 0, 0, 0);
    const key = day.toISOString().slice(0, 10);
    const bucket = byKey.get(key);
    if (!bucket) continue;
    bucket.count += 1;
    if (entry.entry_type === "deposit") bucket.deposits += 1;
    if (entry.entry_type === "lock" || entry.entry_type === "unlock") {
      bucket.locks += 1;
    }
  }

  return buckets;
}
