"use client";

import { useState } from "react";
import { useAssetDecimals } from "@/components/trade/asset-decimals";
import { formatPrice, formatQty, marketDecimals } from "@/lib/trade-format";
import type { OrderBookDepth, TradeView } from "@/lib/types";

const DEFAULT_DEPTH = 14;

export function OrderBookPanel({
  book,
  trades = [],
  marketSymbol,
  onPriceClick,
}: {
  book: OrderBookDepth | null;
  trades?: TradeView[];
  marketSymbol: string;
  onPriceClick?: (price: number) => void;
}) {
  const [mainTab, setMainTab] = useState<"book" | "trades">("book");
  const [viewMode, setViewMode] = useState<"all" | "bids" | "asks">("all");
  const decimalsBySymbol = useAssetDecimals();
  const { baseDecimals, quoteDecimals, quote } = marketDecimals(
    marketSymbol,
    decimalsBySymbol,
  );

  const depth = viewMode === "all" ? DEFAULT_DEPTH : 22;
  const asks = [...(book?.asks ?? [])].slice(0, depth).reverse();
  const bids = (book?.bids ?? []).slice(0, depth);

  const maxQty = Math.max(
    1,
    ...asks.map((level) => level.quantity),
    ...bids.map((level) => level.quantity),
  );

  const spread =
    book?.bids[0] && book?.asks[0]
      ? book.asks[0].price - book.bids[0].price
      : null;

  return (
    <div className="flex h-full min-h-[340px] flex-col overflow-hidden rounded-xl border border-[#2c2533] bg-[#100d14] lg:min-h-0">
      {/* Top Header Tabs */}
      <div className="flex items-center justify-between border-b border-[#2c2533] px-3 py-2 bg-[#140f1a]">
        <div className="flex items-center gap-1.5">
          <button
            className={`rounded-lg px-2.5 py-0.5 text-[10px] font-bold uppercase tracking-[0.12em] transition-all ${
              mainTab === "book"
                ? "bg-[#271f30] text-[#fff8f5] border border-[#3d3248] shadow-xs"
                : "text-[#716878] hover:text-[#bbb1bf]"
            }`}
            onClick={() => setMainTab("book")}
            type="button"
          >
            Order Book
          </button>
          <button
            className={`rounded-lg px-2.5 py-0.5 text-[10px] font-bold uppercase tracking-[0.12em] transition-all ${
              mainTab === "trades"
                ? "bg-[#271f30] text-[#fff8f5] border border-[#3d3248] shadow-xs"
                : "text-[#716878] hover:text-[#bbb1bf]"
            }`}
            onClick={() => setMainTab("trades")}
            type="button"
          >
            Market Trades
          </button>
        </div>

        {mainTab === "book" ? (
          <div className="flex items-center gap-1 rounded-lg bg-[#18131f] p-0.5 border border-[#27202f]">
            <button
              className={`rounded px-2 py-0.5 text-[10px] font-semibold transition-all ${
                viewMode === "all"
                  ? "bg-[#271f30] text-[#fff8f5]"
                  : "text-[#716878] hover:text-[#bbb1bf]"
              }`}
              onClick={() => setViewMode("all")}
              type="button"
            >
              All
            </button>
            <button
              className={`rounded px-2 py-0.5 text-[10px] font-semibold transition-all ${
                viewMode === "bids"
                  ? "bg-[#13211e] text-[#74ddbd]"
                  : "text-[#716878] hover:text-[#74ddbd]"
              }`}
              onClick={() => setViewMode("bids")}
              type="button"
            >
              Bids
            </button>
            <button
              className={`rounded px-2 py-0.5 text-[10px] font-semibold transition-all ${
                viewMode === "asks"
                  ? "bg-[#271a20] text-[#ff8175]"
                  : "text-[#716878] hover:text-[#ff8175]"
              }`}
              onClick={() => setViewMode("asks")}
              type="button"
            >
              Asks
            </button>
          </div>
        ) : (
          <div className="flex items-center gap-1.5 font-mono text-[10px] text-[#74ddbd]">
            <span className="size-1.5 rounded-full bg-[#74ddbd] pulse-dot-green" />
            <span>Live Feed</span>
          </div>
        )}
      </div>

      {mainTab === "book" ? (
        <>
          <div className="grid grid-cols-3 px-3 py-1.5 text-[9px] uppercase tracking-[0.1em] font-mono text-[#716878] border-b border-[#241e2b]">
            <span>Price ({quote})</span>
            <span className="text-right">Size</span>
            <span className="text-right">Orders</span>
          </div>

          <div className="min-h-0 flex-1 overflow-auto font-mono text-[10px] leading-tight select-none">
            {(viewMode === "all" || viewMode === "asks") && (
              <div className="flex flex-col justify-end">
                {asks.map((level) => {
                  const width = (level.quantity / maxQty) * 100;
                  return (
                    <button
                      className="group relative grid w-full grid-cols-3 px-3 py-[2px] text-left transition-colors hover:bg-[#1f1725]"
                      key={`ask-${level.price}`}
                      onClick={() => onPriceClick?.(level.price)}
                      type="button"
                    >
                      <span
                        className="pointer-events-none absolute inset-y-0 right-0 bg-gradient-to-l from-[#ff6f61]/25 to-transparent transition-all duration-300"
                        style={{ width: `${width}%` }}
                      />
                      <span className="relative font-semibold text-[#ff8175] group-hover:underline">
                        {formatPrice(level.price, quoteDecimals)}
                      </span>
                      <span className="relative text-right text-[#ded6df]">
                        {formatQty(level.quantity, baseDecimals)}
                      </span>
                      <span className="relative text-right text-[#716878]">
                        {level.order_count}
                      </span>
                    </button>
                  );
                })}
              </div>
            )}

            <div className="my-0.5 border-y border-[#2c2533] bg-[#140f1a] px-3 py-1 flex items-center justify-between text-[10px] font-mono">
              <span className="text-[#8e8594]">Spread</span>
              <span className="font-semibold text-[#ded6df]">
                {spread !== null ? formatPrice(spread, quoteDecimals) : "—"}
              </span>
            </div>

            {(viewMode === "all" || viewMode === "bids") && (
              <div>
                {bids.map((level) => {
                  const width = (level.quantity / maxQty) * 100;
                  return (
                    <button
                      className="group relative grid w-full grid-cols-3 px-3 py-[2px] text-left transition-colors hover:bg-[#14231f]"
                      key={`bid-${level.price}`}
                      onClick={() => onPriceClick?.(level.price)}
                      type="button"
                    >
                      <span
                        className="pointer-events-none absolute inset-y-0 right-0 bg-gradient-to-l from-[#74ddbd]/25 to-transparent transition-all duration-300"
                        style={{ width: `${width}%` }}
                      />
                      <span className="relative font-semibold text-[#74ddbd] group-hover:underline">
                        {formatPrice(level.price, quoteDecimals)}
                      </span>
                      <span className="relative text-right text-[#ded6df]">
                        {formatQty(level.quantity, baseDecimals)}
                      </span>
                      <span className="relative text-right text-[#716878]">
                        {level.order_count}
                      </span>
                    </button>
                  );
                })}
              </div>
            )}
          </div>
        </>
      ) : (
        <>
          <div className="grid grid-cols-3 px-3 py-1.5 text-[9px] uppercase tracking-[0.1em] font-mono text-[#716878] border-b border-[#241e2b]">
            <span>Price ({quote})</span>
            <span className="text-right">Size</span>
            <span className="text-right">Time</span>
          </div>
          <div className="min-h-0 flex-1 overflow-auto font-mono text-[10px] leading-tight select-none">
            {trades.length === 0 ? (
              <p className="px-3 py-12 text-center text-[10px] text-[#716878]">
                Waiting for live market fills…
              </p>
            ) : (
              trades.map((trade, index) => {
                const prev = trades[index + 1];
                const up = !prev || trade.price >= prev.price;
                const time = new Date(trade.created_at).toLocaleTimeString(
                  "en-US",
                  {
                    hour12: false,
                    hour: "2-digit",
                    minute: "2-digit",
                    second: "2-digit",
                  },
                );
                return (
                  <div
                    className="grid grid-cols-3 px-3 py-[2px] hover:bg-[#1a1422] transition-colors"
                    key={trade.id}
                  >
                    <span
                      className={`font-semibold ${
                        up ? "text-[#74ddbd]" : "text-[#ff8175]"
                      }`}
                    >
                      {formatPrice(trade.price, quoteDecimals)}
                    </span>
                    <span className="text-right text-[#ded6df]">
                      {formatQty(trade.quantity, baseDecimals)}
                    </span>
                    <span className="text-right text-[9px] text-[#716878]">
                      {time}
                    </span>
                  </div>
                );
              })
            )}
          </div>
        </>
      )}
    </div>
  );
}
