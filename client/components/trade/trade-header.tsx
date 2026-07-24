"use client";

import Link from "next/link";
import { ChevronDown } from "lucide-react";
import { useEffect, useRef, useState } from "react";

import { MarketPairIcons } from "@/components/markets/asset-icon";
import { useAssetDecimals } from "@/components/trade/asset-decimals";
import { formatMarketPair, splitMarketSymbol } from "@/lib/markets";
import { formatPrice, marketDecimals } from "@/lib/trade-format";
import type { Market, OrderBookDepth } from "@/lib/types";

export function TradeHeader({
  market,
  markets,
  book,
  connected,
}: {
  market: Market;
  markets: Market[];
  book: OrderBookDepth | null;
  connected: boolean;
}) {
  const [open, setOpen] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);
  const [base, quote] = splitMarketSymbol(market.symbol);
  const decimalsBySymbol = useAssetDecimals();
  const { quoteDecimals } = marketDecimals(market.symbol, decimalsBySymbol);

  const bestBid = book?.bids[0]?.price;
  const bestAsk = book?.asks[0]?.price;
  const mid =
    bestBid !== undefined && bestAsk !== undefined
      ? (bestBid + bestAsk) / 2
      : (bestBid ?? bestAsk);

  useEffect(() => {
    function onClick(event: MouseEvent) {
      if (!menuRef.current?.contains(event.target as Node)) setOpen(false);
    }
    document.addEventListener("mousedown", onClick);
    return () => document.removeEventListener("mousedown", onClick);
  }, []);

  return (
    <div className="flex shrink-0 flex-wrap items-center justify-between gap-3 border-b border-[#2c2533] bg-[#100d14] px-3 py-2.5 sm:gap-4 sm:px-4">
      <div
        className="relative flex min-w-0 flex-wrap items-center gap-3 sm:gap-5"
        ref={menuRef}
      >
        <button
          className="flex items-center gap-3 rounded-lg border border-[#302839] bg-[#17131d] px-2.5 py-1.5 text-left hover:border-[#ff6f61]/60 sm:px-3 sm:py-2"
          onClick={() => setOpen((value) => !value)}
          type="button"
        >
          <MarketPairIcons base={base} quote={quote} size={32} />
          <div>
            <p className="text-sm font-semibold text-[#fff8f5]">
              {formatMarketPair(market.symbol)}
            </p>
            <p className="text-[10px] uppercase tracking-[0.14em] text-[#716878]">
              {market.status}
            </p>
          </div>
          <ChevronDown className="text-[#716878]" size={16} />
        </button>

        {open && (
          <div className="absolute left-0 top-full z-30 mt-2 max-h-72 w-64 overflow-auto rounded-2xl border border-[#302839] bg-[#15111a] py-2 shadow-2xl">
            {markets.map((item) => {
              const [b, q] = splitMarketSymbol(item.symbol);
              return (
                <Link
                  className="flex items-center gap-3 px-3 py-2.5 hover:bg-[#1b1621]"
                  href={`/trade/${encodeURIComponent(item.symbol)}`}
                  key={item.id}
                  onClick={() => setOpen(false)}
                >
                  <MarketPairIcons base={b} quote={q} size={28} />
                  <span className="text-sm text-[#fff8f5]">
                    {formatMarketPair(item.symbol)}
                  </span>
                </Link>
              );
            })}
          </div>
        )}

        <div className="hidden sm:block">
          <p className="text-[10px] uppercase tracking-[0.16em] text-[#716878]">
            Mid
          </p>
          <p className="font-mono text-xl font-semibold text-[#fff8f5]">
            {mid !== undefined ? formatPrice(mid, quoteDecimals) : "—"}
          </p>
        </div>
        <div className="hidden md:block">
          <p className="text-[10px] uppercase tracking-[0.16em] text-[#716878]">
            Best bid
          </p>
          <p className="font-mono text-sm text-[#74ddbd]">
            {bestBid !== undefined ? formatPrice(bestBid, quoteDecimals) : "—"}
          </p>
        </div>
        <div className="hidden md:block">
          <p className="text-[10px] uppercase tracking-[0.16em] text-[#716878]">
            Best ask
          </p>
          <p className="font-mono text-sm text-[#ff8175]">
            {bestAsk !== undefined ? formatPrice(bestAsk, quoteDecimals) : "—"}
          </p>
        </div>
      </div>

      <div className="flex items-center gap-3 text-xs text-[#716878]">
        <span
          className={`flex items-center gap-2 rounded-full px-3 py-1.5 ${
            connected
              ? "bg-[#13211e] text-[#74ddbd]"
              : "bg-[#271a20] text-[#ff8175]"
          }`}
        >
          <span
            className={`size-1.5 rounded-full ${
              connected ? "bg-[#74ddbd]" : "bg-[#ff8175]"
            }`}
          />
          {connected ? "Live" : "Connecting"}
        </span>
        <span className="hidden font-mono uppercase tracking-[0.14em] sm:inline">
          Spot · Limit
        </span>
      </div>
    </div>
  );
}
