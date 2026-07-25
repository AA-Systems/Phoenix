"use client";

import Link from "next/link";
import { ChevronDown, Search } from "lucide-react";
import { useEffect, useRef, useState } from "react";

import { MarketPairIcons } from "@/components/markets/asset-icon";
import { useAssetDecimals } from "@/components/trade/asset-decimals";
import type { ChartCandleOhlc } from "@/components/trade/price-chart";
import { formatMarketPair, splitMarketSymbol } from "@/lib/markets";
import { formatPrice, marketDecimals } from "@/lib/trade-format";
import type { Market, OrderBookDepth } from "@/lib/types";

export function TradeHeader({
  market,
  markets,
  book,
  connected,
  candleOhlc,
}: {
  market: Market;
  markets: Market[];
  book: OrderBookDepth | null;
  connected: boolean;
  candleOhlc?: ChartCandleOhlc | null;
}) {
  const [open, setOpen] = useState(false);
  const [filter, setFilter] = useState("");
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

  const filteredMarkets = markets.filter(
    (m) =>
      m.symbol.toLowerCase().includes(filter.toLowerCase()) ||
      m.name.toLowerCase().includes(filter.toLowerCase()),
  );

  return (
    <div className="relative z-30 flex shrink-0 flex-wrap items-center justify-between gap-3 border-b border-[#2c2533] bg-[#100d14]/95 px-3 py-2 sm:gap-4 sm:px-5 backdrop-blur-md">
      <div
        className="relative flex min-w-0 flex-wrap items-center gap-3 sm:gap-6"
        ref={menuRef}
      >
        <button
          aria-expanded={open}
          aria-haspopup="listbox"
          className="flex items-center gap-3 rounded-xl border border-[#302839] bg-[#17131d] px-3 py-1.5 text-left transition-all duration-200 hover:border-[#ff6f61] hover:shadow-[0_0_12px_rgba(255,111,97,0.2)]"
          onClick={() => setOpen((value) => !value)}
          type="button"
        >
          <MarketPairIcons base={base} quote={quote} size={32} />
          <div>
            <div className="flex items-center gap-1.5">
              <p className="text-base font-bold text-[#fff8f5]">
                {formatMarketPair(market.symbol)}
              </p>
              <ChevronDown className="text-[#8e8594]" size={14} />
            </div>
            <p className="text-[10px] font-mono uppercase tracking-[0.14em] text-[#74ddbd]">
              Spot Market
            </p>
          </div>
        </button>

        {open && (
          <div
            className="absolute left-0 top-full z-50 mt-2 w-72 overflow-hidden rounded-2xl border border-[#382f42] bg-[#15111a] py-2 shadow-[0_20px_50px_rgba(0,0,0,0.5)] backdrop-blur-md"
            role="listbox"
          >
            <div className="border-b border-[#261e2f] px-3 pb-2 pt-1">
              <div className="flex items-center gap-2 rounded-lg border border-[#2b2434] bg-[#100d14] px-2.5 py-1 text-xs text-[#8e8594]">
                <Search size={13} />
                <input
                  autoFocus
                  className="w-full bg-transparent text-xs text-[#fff8f5] outline-none placeholder:text-[#5f5665]"
                  onChange={(e) => setFilter(e.target.value)}
                  placeholder="Search pair..."
                  value={filter}
                />
              </div>
            </div>
            <div className="max-h-64 overflow-y-auto">
              {filteredMarkets.length === 0 ? (
                <p className="px-3.5 py-4 text-center text-xs text-[#716878]">
                  No markets found
                </p>
              ) : (
                filteredMarkets.map((item) => {
                  const [b, q] = splitMarketSymbol(item.symbol);
                  const isCurrent = item.id === market.id;
                  return (
                    <Link
                      className={`flex items-center justify-between px-3.5 py-2.5 transition-colors ${
                        isCurrent
                          ? "bg-[#251e2d] text-[#ff8175]"
                          : "text-[#fff8f5] hover:bg-[#1b1621]"
                      }`}
                      href={`/trade/${encodeURIComponent(item.symbol)}`}
                      key={item.id}
                      onClick={() => setOpen(false)}
                    >
                      <div className="flex items-center gap-3">
                        <MarketPairIcons base={b} quote={q} size={26} />
                        <div>
                          <p className="text-xs font-semibold">
                            {formatMarketPair(item.symbol)}
                          </p>
                          <p className="text-[10px] text-[#716878]">
                            {item.name}
                          </p>
                        </div>
                      </div>
                      <span className="font-mono text-[10px] uppercase text-[#74ddbd]">
                        {item.status}
                      </span>
                    </Link>
                  );
                })
              )}
            </div>
          </div>
        )}

        {candleOhlc ? (
          <CandleOhlcStrip candle={candleOhlc} quoteDecimals={quoteDecimals} />
        ) : (
          <>
            <div className="hidden sm:block">
              <p className="text-[10px] uppercase tracking-[0.16em] text-[#716878]">
                Index / Mid
              </p>
              <p className="font-mono text-xl font-bold text-[#fff8f5]">
                {mid !== undefined ? formatPrice(mid, quoteDecimals) : "—"}
              </p>
            </div>
            <div className="hidden border-l border-[#241e2b] pl-4 md:block">
              <p className="text-[10px] uppercase tracking-[0.16em] text-[#716878]">
                Best Bid
              </p>
              <p className="font-mono text-sm font-semibold text-[#74ddbd]">
                {bestBid !== undefined
                  ? formatPrice(bestBid, quoteDecimals)
                  : "—"}
              </p>
            </div>
            <div className="hidden border-l border-[#241e2b] pl-4 md:block">
              <p className="text-[10px] uppercase tracking-[0.16em] text-[#716878]">
                Best Ask
              </p>
              <p className="font-mono text-sm font-semibold text-[#ff8175]">
                {bestAsk !== undefined
                  ? formatPrice(bestAsk, quoteDecimals)
                  : "—"}
              </p>
            </div>
          </>
        )}
      </div>

      <div className="flex items-center gap-3 text-xs text-[#716878]">
        <span
          className={`flex items-center gap-2 rounded-full border px-3 py-1 text-[11px] font-medium ${
            connected
              ? "border-[#1b3d34] bg-[#13211e] text-[#74ddbd]"
              : "border-[#481f25] bg-[#271a20] text-[#ff8175]"
          }`}
        >
          <span
            className={`size-1.5 rounded-full ${
              connected ? "bg-[#74ddbd] pulse-dot-green" : "bg-[#ff8175]"
            }`}
          />
          {connected ? "Feed Active" : "Connecting..."}
        </span>
      </div>
    </div>
  );
}

function CandleOhlcStrip({
  candle,
  quoteDecimals,
}: {
  candle: ChartCandleOhlc;
  quoteDecimals: number;
}) {
  const up = candle.close >= candle.open;
  const tone = up ? "text-[#74ddbd]" : "text-[#ff8175]";
  const items = [
    ["Open", candle.open],
    ["High", candle.high],
    ["Low", candle.low],
    ["Close", candle.close],
  ] as const;

  return (
    <div className="flex min-w-0 flex-wrap items-end gap-x-3 gap-y-1 border-l border-[#241e2b] pl-3 sm:pl-4">
      {items.map(([label, value]) => (
        <div className="min-w-0" key={label}>
          <p className="text-[9px] font-mono uppercase tracking-[0.14em] text-[#716878]">
            {label}
          </p>
          <p className={`font-mono text-xs font-semibold sm:text-sm ${tone}`}>
            {formatPrice(value, quoteDecimals)}
          </p>
        </div>
      ))}
    </div>
  );
}
