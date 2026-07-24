"use client";

import { useState } from "react";

import { useAssetDecimals } from "@/components/trade/asset-decimals";
import { cancelOrder } from "@/lib/api";
import { formatMarketPair } from "@/lib/markets";
import { formatPrice, formatQty, marketDecimals } from "@/lib/trade-format";
import type { OpenOrder } from "@/lib/types";

export function OpenOrdersPanel({
  orders,
  marketSymbol,
}: {
  orders: OpenOrder[] | null;
  marketSymbol: string;
}) {
  const [busyId, setBusyId] = useState<string | null>(null);
  const [error, setError] = useState("");
  const decimalsBySymbol = useAssetDecimals();

  const filtered =
    orders?.filter(
      (order) =>
        order.market_symbol.toUpperCase() === marketSymbol.toUpperCase() &&
        (order.status === "active" || order.status === "partially_filled"),
    ) ?? [];

  async function onCancel(orderId: string) {
    setError("");
    setBusyId(orderId);
    try {
      await cancelOrder(orderId);
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : "Cancel failed.");
    } finally {
      setBusyId(null);
    }
  }

  return (
    <div className="flex h-full min-h-0 flex-col overflow-hidden rounded-lg border border-[#2c2533] bg-[#100d14]">
      <div className="flex shrink-0 items-center justify-between border-b border-[#2c2533] px-4 py-2.5">
        <p className="text-[10px] uppercase tracking-[0.16em] text-[#716878]">
          Open orders
        </p>
        <p className="text-xs text-[#57505e]">{filtered.length} active</p>
      </div>

      {orders === null ? (
        <p className="flex flex-1 items-center justify-center px-4 py-6 text-center text-sm text-[#57505e]">
          Log in to view your open orders.
        </p>
      ) : filtered.length === 0 ? (
        <p className="flex flex-1 items-center justify-center px-4 py-6 text-center text-sm text-[#57505e]">
          No open orders on this market.
        </p>
      ) : (
        <div className="min-h-0 flex-1 overflow-auto">
          <div className="min-w-[720px]">
            <div className="sticky top-0 grid grid-cols-[1.1fr_0.7fr_1fr_1fr_1fr_1fr_auto] bg-[#100d14] px-4 py-2 text-[10px] uppercase tracking-[0.12em] text-[#57505e]">
              <span>Market</span>
              <span>Side</span>
              <span className="text-right">Price</span>
              <span className="text-right">Qty</span>
              <span className="text-right">Filled</span>
              <span className="text-right">Remaining</span>
              <span className="w-20 text-right">Action</span>
            </div>
            {filtered.map((order) => {
              const { baseDecimals, quoteDecimals } = marketDecimals(
                order.market_symbol,
                decimalsBySymbol,
              );
              return (
                <div
                  className="grid grid-cols-[1.1fr_0.7fr_1fr_1fr_1fr_1fr_auto] items-center border-t border-[#292230] px-4 py-2.5 text-sm"
                  key={order.id}
                >
                  <span className="text-[#fff8f5]">
                    {formatMarketPair(order.market_symbol)}
                  </span>
                  <span
                    className={
                      order.order_type === "buy"
                        ? "text-[#74ddbd]"
                        : "text-[#ff8175]"
                    }
                  >
                    {order.order_type}
                  </span>
                  <span className="text-right font-mono text-[#dcd4de]">
                    {formatPrice(order.price, quoteDecimals)}
                  </span>
                  <span className="text-right font-mono text-[#dcd4de]">
                    {formatQty(order.quantity, baseDecimals)}
                  </span>
                  <span className="text-right font-mono text-[#716878]">
                    {formatQty(order.filled_quantity, baseDecimals)}
                  </span>
                  <span className="text-right font-mono text-[#dcd4de]">
                    {formatQty(order.remaining, baseDecimals)}
                  </span>
                  <div className="flex w-20 justify-end">
                    <button
                      className="rounded-full border border-[#3a3142] px-2.5 py-1 text-xs text-[#ff9e96] hover:border-[#ff6f61] disabled:opacity-50"
                      disabled={busyId === order.id}
                      onClick={() => onCancel(order.id)}
                      type="button"
                    >
                      {busyId === order.id ? "…" : "Cancel"}
                    </button>
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      )}
      {error ? (
        <p className="border-t border-[#2c2533] px-4 py-2 text-xs text-[#ff9e96]">
          {error}
        </p>
      ) : null}
    </div>
  );
}
