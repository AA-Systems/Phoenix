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
    <div className="flex h-full min-h-[180px] flex-col overflow-hidden rounded-lg border border-[#2c2533] bg-[#100d14] lg:min-h-0">
      <div className="border-b border-[#2c2533] px-3 py-2.5 text-[10px] uppercase tracking-[0.16em] text-[#716878]">
        Recent trades
      </div>
      <div className="grid grid-cols-3 px-3 py-2 text-[10px] uppercase tracking-[0.12em] text-[#57505e]">
        <span>Price</span>
        <span className="text-right">Size</span>
        <span className="text-right">Time</span>
      </div>
      <div className="flex-1 overflow-auto font-mono text-xs">
        {trades.length === 0 ? (
          <p className="px-3 py-8 text-center text-[#57505e]">
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
              <div className="grid grid-cols-3 px-3 py-[4px]" key={trade.id}>
                <span className={up ? "text-[#74ddbd]" : "text-[#ff8175]"}>
                  {formatPrice(trade.price, quoteDecimals)}
                </span>
                <span className="text-right text-[#dcd4de]">
                  {formatQty(trade.quantity, baseDecimals)}
                </span>
                <span className="text-right text-[#716878]">{time}</span>
              </div>
            );
          })
        )}
      </div>
    </div>
  );
}
