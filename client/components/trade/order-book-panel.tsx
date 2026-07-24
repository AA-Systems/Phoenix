"use client";

import { useAssetDecimals } from "@/components/trade/asset-decimals";
import { formatPrice, formatQty, marketDecimals } from "@/lib/trade-format";
import type { OrderBookDepth } from "@/lib/types";

const DEPTH = 18;

export function OrderBookPanel({
  book,
  marketSymbol,
  onPriceClick,
}: {
  book: OrderBookDepth | null;
  marketSymbol: string;
  onPriceClick?: (price: number) => void;
}) {
  const decimalsBySymbol = useAssetDecimals();
  const { baseDecimals, quoteDecimals, quote } = marketDecimals(
    marketSymbol,
    decimalsBySymbol,
  );
  const asks = [...(book?.asks ?? [])].slice(0, DEPTH).reverse();
  const bids = (book?.bids ?? []).slice(0, DEPTH);

  const maxQty = Math.max(
    1,
    ...asks.map((level) => level.quantity),
    ...bids.map((level) => level.quantity),
  );

  return (
    <div className="flex h-full min-h-[320px] flex-col overflow-hidden rounded-lg border border-[#2c2533] bg-[#100d14] lg:min-h-0">
      <div className="border-b border-[#2c2533] px-3 py-2.5 text-[10px] uppercase tracking-[0.16em] text-[#716878]">
        Order book
      </div>
      <div className="grid grid-cols-3 px-3 py-2 text-[10px] uppercase tracking-[0.12em] text-[#57505e]">
        <span>Price ({quote})</span>
        <span className="text-right">Size</span>
        <span className="text-right">Total</span>
      </div>

      <div className="min-h-0 flex-1 overflow-auto font-mono text-xs">
        <div className="flex flex-col justify-end">
          {asks.map((level) => {
            const width = (level.quantity / maxQty) * 100;
            return (
              <button
                className="relative grid w-full grid-cols-3 px-3 py-[3px] text-left hover:bg-[#1b1621]"
                key={`ask-${level.price}`}
                onClick={() => onPriceClick?.(level.price)}
                type="button"
              >
                <span
                  className="pointer-events-none absolute inset-y-0 right-0 bg-[#ff6f61]/12"
                  style={{ width: `${width}%` }}
                />
                <span className="relative text-[#ff8175]">
                  {formatPrice(level.price, quoteDecimals)}
                </span>
                <span className="relative text-right text-[#dcd4de]">
                  {formatQty(level.quantity, baseDecimals)}
                </span>
                <span className="relative text-right text-[#716878]">
                  {level.order_count}
                </span>
              </button>
            );
          })}
        </div>

        <div className="border-y border-[#2c2533] px-3 py-2 text-center text-[11px] text-[#938a98]">
          Spread ·{" "}
          {book?.bids[0] && book?.asks[0]
            ? formatPrice(
                book.asks[0].price - book.bids[0].price,
                quoteDecimals,
              )
            : "—"}
        </div>

        <div>
          {bids.map((level) => {
            const width = (level.quantity / maxQty) * 100;
            return (
              <button
                className="relative grid w-full grid-cols-3 px-3 py-[3px] text-left hover:bg-[#1b1621]"
                key={`bid-${level.price}`}
                onClick={() => onPriceClick?.(level.price)}
                type="button"
              >
                <span
                  className="pointer-events-none absolute inset-y-0 right-0 bg-[#74ddbd]/12"
                  style={{ width: `${width}%` }}
                />
                <span className="relative text-[#74ddbd]">
                  {formatPrice(level.price, quoteDecimals)}
                </span>
                <span className="relative text-right text-[#dcd4de]">
                  {formatQty(level.quantity, baseDecimals)}
                </span>
                <span className="relative text-right text-[#716878]">
                  {level.order_count}
                </span>
              </button>
            );
          })}
        </div>
      </div>
    </div>
  );
}
