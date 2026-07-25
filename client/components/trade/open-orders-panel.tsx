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
    <div className="flex h-full min-h-0 flex-col overflow-hidden rounded-xl border border-[#2c2533] bg-[#100d14]">
      <div className="flex shrink-0 items-center justify-between border-b border-[#2c2533] px-4 py-2.5 bg-[#140f1a]">
        <div className="flex items-center gap-2">
          <span className="text-[10px] uppercase tracking-[0.18em] font-semibold text-[#8e8594]">
            Open Orders
          </span>
          <span className="rounded-full bg-[#241d2c] border border-[#342a3e] px-2 py-0.5 font-mono text-[10px] font-semibold text-[#fff8f5]">
            {filtered.length}
          </span>
        </div>
      </div>

      {orders === null ? (
        <p className="flex flex-1 items-center justify-center px-4 py-6 text-center text-sm text-[#716878]">
          Log in to view your active open orders.
        </p>
      ) : filtered.length === 0 ? (
        <p className="flex flex-1 items-center justify-center px-4 py-6 text-center text-sm text-[#716878]">
          No open orders on this market pair.
        </p>
      ) : (
        <div className="min-h-0 flex-1 overflow-auto">
          <div className="min-w-[720px]">
            <div className="sticky top-0 grid grid-cols-[1.1fr_0.7fr_1fr_1fr_1fr_1fr_auto] bg-[#17121e] border-b border-[#241e2b] px-4 py-2 text-[10px] uppercase tracking-[0.12em] font-mono text-[#716878]">
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
                  className="grid grid-cols-[1.1fr_0.7fr_1fr_1fr_1fr_1fr_auto] items-center border-b border-[#241e2b] px-4 py-2.5 text-xs transition-colors hover:bg-[#1a1422]"
                  key={order.id}
                >
                  <span className="font-semibold text-[#fff8f5]">
                    {formatMarketPair(order.market_symbol)}
                  </span>
                  <div>
                    <span
                      className={`inline-block rounded px-2 py-0.5 font-mono text-[10px] font-bold uppercase tracking-wider ${
                        order.order_type === "buy"
                          ? "bg-[#13211e] text-[#74ddbd] border border-[#1b3d34]"
                          : "bg-[#271a20] text-[#ff8175] border border-[#481f25]"
                      }`}
                    >
                      {order.order_type}
                    </span>
                  </div>
                  <span className="text-right font-mono font-medium text-[#ded6df]">
                    {formatPrice(order.price, quoteDecimals)}
                  </span>
                  <span className="text-right font-mono text-[#ded6df]">
                    {formatQty(order.quantity, baseDecimals)}
                  </span>
                  <span className="text-right font-mono text-[#716878]">
                    {formatQty(order.filled_quantity, baseDecimals)}
                  </span>
                  <span className="text-right font-mono text-[#ded6df]">
                    {formatQty(order.remaining, baseDecimals)}
                  </span>
                  <div className="flex w-20 justify-end">
                    <button
                      className="rounded-full border border-[#3b2e47] bg-[#1a1322] px-3 py-1 text-[11px] font-semibold text-[#ff8175] transition-all duration-200 hover:border-[#ff6f61] hover:bg-[#ff6f61] hover:text-[#160e12] disabled:opacity-50"
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
