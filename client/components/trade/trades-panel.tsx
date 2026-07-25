"use client";

import { useAssetDecimals } from "@/components/trade/asset-decimals";
import { formatPrice, formatQty, marketDecimals } from "@/lib/trade-format";
import type { TradeView } from "@/lib/types";

export function TradesPanel({
  trades,
  marketSymbol,
}: {
  trades: TradeView[];
  marketSymbol: string;
}) {
  const decimalsBySymbol = useAssetDecimals();
  const { baseDecimals, quoteDecimals } = marketDecimals(
    marketSymbol,
    decimalsBySymbol,
  );

  return (
    <div className="flex h-full min-h-[180px] flex-col overflow-hidden rounded-xl border border-[#2c2533] bg-[#100d14] lg:min-h-0">
      <div className="flex items-center justify-between border-b border-[#2c2533] px-3.5 py-2">
        <span className="text-[10px] uppercase tracking-[0.18em] font-semibold text-[#8e8594]">
          Recent Trades
        </span>
        <span className="size-1.5 rounded-full bg-[#74ddbd] pulse-dot-green" />
      </div>
      <div className="grid grid-cols-3 px-3.5 py-2 text-[10px] uppercase tracking-[0.12em] font-mono text-[#716878] border-b border-[#241e2b]">
        <span>Price</span>
        <span className="text-right font-mono">Size</span>
        <span className="text-right font-mono">Time</span>
      </div>
      <div className="flex-1 overflow-auto font-mono text-xs select-none">
        {trades.length === 0 ? (
          <p className="px-3 py-8 text-center text-[#716878]">
            Waiting for live fills…
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
                className="grid grid-cols-3 px-3.5 py-[4px] hover:bg-[#1a1422] transition-colors"
                key={trade.id}
              >
                <span
                  className={`font-semibold ${up ? "text-[#74ddbd]" : "text-[#ff8175]"}`}
                >
                  {formatPrice(trade.price, quoteDecimals)}
                </span>
                <span className="text-right text-[#ded6df]">
                  {formatQty(trade.quantity, baseDecimals)}
                </span>
                <span className="text-right text-[#716878] text-[11px]">
                  {time}
                </span>
              </div>
            );
          })
        )}
      </div>
    </div>
  );
}
