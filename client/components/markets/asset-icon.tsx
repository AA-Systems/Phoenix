"use client";

import Image from "next/image";
import { useMemo, useState } from "react";

import { assetIconCandidates, marketInitials } from "@/lib/markets";

function SingleIcon({ symbol, size = 40 }: { symbol: string; size?: number }) {
  const candidates = useMemo(() => assetIconCandidates(symbol), [symbol]);
  const [index, setIndex] = useState(0);
  const src = candidates[index];

  if (!src) {
    return (
      <span
        className="grid place-items-center rounded-full bg-linear-to-br from-[#321f26] to-[#1a131c] font-mono text-xs font-semibold text-[#ff8175] ring-1 ring-[#3a2a32]"
        style={{ width: size, height: size }}
      >
        {marketInitials(symbol)}
      </span>
    );
  }

  return (
    <Image
      alt={symbol}
      className="rounded-full bg-[#1a131c] ring-1 ring-[#3a2a32]"
      height={size}
      onError={() => {
        if (index + 1 < candidates.length) {
          setIndex((current) => current + 1);
        } else {
          setIndex(candidates.length);
        }
      }}
      src={src}
      width={size}
    />
  );
}

/** Overlapping base + quote icons for a market pair. */
export function MarketPairIcons({
  base,
  quote,
  size = 40,
}: {
  base: string;
  quote: string;
  size?: number;
}) {
  const overlap = Math.round(size * 0.32);

  return (
    <div
      className="relative flex shrink-0 items-center"
      style={{ width: size + (quote ? size - overlap : 0), height: size }}
    >
      <div className="absolute left-0 top-0 z-10">
        <SingleIcon size={size} symbol={base} />
      </div>
      {quote ? (
        <div className="absolute top-0 z-0" style={{ left: size - overlap }}>
          <SingleIcon size={size} symbol={quote} />
        </div>
      ) : null}
    </div>
  );
}

export function AssetIcon({
  symbol,
  size = 40,
}: {
  symbol: string;
  size?: number;
}) {
  return <SingleIcon size={size} symbol={symbol} />;
}
