"use client";

import Link from "next/link";
import { RefreshCw } from "lucide-react";
import { useEffect, useState } from "react";

import { MarketPairIcons } from "@/components/markets/asset-icon";
import { listMarkets } from "@/lib/api";
import { formatMarketPair, splitMarketSymbol } from "@/lib/markets";
import type { Market } from "@/lib/types";

export function MarketBoard() {
  const [markets, setMarkets] = useState<Market[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false);

  useEffect(() => {
    let active = true;
    listMarkets()
      .then((next) => {
        if (active) setMarkets(next.slice(0, 2));
      })
      .catch(() => {
        if (active) setError(true);
      })
      .finally(() => {
        if (active) setLoading(false);
      });
    return () => {
      active = false;
    };
  }, []);

  return (
    <div className="overflow-hidden rounded-[28px] border border-[#302839] bg-[#15111a] shadow-[0_28px_80px_rgba(0,0,0,0.3)]">
      <div className="flex items-center justify-between border-b border-[#2c2533] px-6 py-5">
        <div>
          <p className="text-xs uppercase tracking-[0.18em] text-[#817787]">
            Market pulse
          </p>
          <p className="mt-1 text-sm text-[#f0e9f0]">Listed pairs</p>
        </div>
        <span className="flex items-center gap-2 rounded-full bg-[#13211e] px-3 py-1.5 text-xs text-[#74ddbd]">
          <span className="size-1.5 rounded-full bg-[#74ddbd]" />
          {error ? "Offline" : "Live catalog"}
        </span>
      </div>

      <div className="grid grid-cols-[1fr_auto] px-6 py-3 text-[10px] uppercase tracking-[0.16em] text-[#716878]">
        <span>Market</span>
        <span className="text-right">Status</span>
      </div>

      {loading ? (
        <div className="grid place-items-center border-t border-[#292230] py-14">
          <RefreshCw className="animate-spin text-[#ff8175]" size={18} />
        </div>
      ) : error || markets.length === 0 ? (
        <div className="border-t border-[#292230] px-6 py-10 text-center text-sm text-[#716878]">
          {error ? "Unable to load markets." : "No markets listed yet."}
        </div>
      ) : (
        markets.map((market) => {
          const [base, quote] = splitMarketSymbol(market.symbol);
          return (
            <Link
              className="grid grid-cols-[1fr_auto] items-center border-t border-[#292230] px-6 py-5 transition-colors hover:bg-[#1b1621]"
              href={`/trade/${encodeURIComponent(market.symbol)}`}
              key={market.id}
            >
              <div className="flex items-center gap-3">
                <MarketPairIcons base={base} quote={quote} size={36} />
                <span className="text-sm font-medium text-[#fff8f5]">
                  {formatMarketPair(market.symbol)}
                </span>
              </div>
              <span
                className={`text-right font-mono text-xs uppercase tracking-[0.12em] ${
                  market.status === "trading"
                    ? "text-[#74ddbd]"
                    : "text-[#e2c07a]"
                }`}
              >
                {market.status}
              </span>
            </Link>
          );
        })
      )}
    </div>
  );
}
