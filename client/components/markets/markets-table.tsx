import Link from "next/link";
import { ArrowUpRight, CandlestickChart } from "lucide-react";

import { MarketPairIcons } from "@/components/markets/asset-icon";
import { formatMarketPair, splitMarketSymbol } from "@/lib/markets";
import type { Market, MarketStatus } from "@/lib/types";

const statusTone: Record<MarketStatus, string> = {
  trading: "bg-[#13211e] text-[#74ddbd] border border-[#1b3d34]",
  halted: "bg-[#2a2214] text-[#e2c07a] border border-[#48371c]",
  archived: "bg-[#221f26] text-[#716878] border border-[#302839]",
};

export function MarketsTable({ markets }: { markets: Market[] }) {
  if (markets.length === 0) {
    return (
      <div className="rounded-[28px] border border-dashed border-[#3a3142] bg-[#141018]/80 px-6 py-20 text-center shadow-lg">
        <div className="mx-auto grid size-14 place-items-center rounded-2xl bg-[#271a20] text-[#ff8175] shadow-[0_0_20px_rgba(255,111,97,0.15)]">
          <CandlestickChart size={24} />
        </div>
        <h2 className="mt-5 text-lg font-medium text-[#fff8f5]">
          No markets listed
        </h2>
        <p className="mx-auto mt-2 max-w-sm text-sm leading-6 text-[#817884]">
          Seed the exchange or add a market from the admin API to see pairs
          here.
        </p>
      </div>
    );
  }

  return (
    <div className="overflow-hidden rounded-[28px] border border-[#302839] bg-[#141018]/90 shadow-[0_20px_60px_rgba(0,0,0,0.35)] backdrop-blur-md">
      <div className="overflow-x-auto">
        <div className="min-w-[820px]">
          <div className="grid grid-cols-[1.6fr_0.9fr_1fr_1fr_1fr_auto] border-b border-[#302839] bg-[#19141e]/90 px-6 py-4 text-[10px] uppercase tracking-[0.18em] font-semibold text-[#8e8594]">
            <span>Market Pair</span>
            <span>Status</span>
            <span className="text-right">Price Tick</span>
            <span className="text-right">Step Size</span>
            <span className="text-right">Min Order</span>
            <span className="w-28 text-right">Action</span>
          </div>

          {markets.map((market, index) => {
            const [base, quote] = splitMarketSymbol(market.symbol);

            return (
              <div
                className="group page-reveal grid grid-cols-[1.6fr_0.9fr_1fr_1fr_1fr_auto] items-center border-b border-[#241e2b] px-6 py-5 transition-colors duration-200 last:border-b-0 hover:bg-[#1f1927]/90"
                key={market.id}
                style={{ animationDelay: `${60 + index * 40}ms` }}
              >
                <div className="flex items-center gap-3.5">
                  <MarketPairIcons base={base} quote={quote} />
                  <div>
                    <p className="text-base font-semibold text-[#fff8f5] group-hover:text-[#ff8a7f] transition-colors">
                      {formatMarketPair(market.symbol)}
                    </p>
                    <p className="mt-0.5 text-xs text-[#716878]">
                      {market.name}
                    </p>
                  </div>
                </div>

                <div>
                  <span
                    className={`inline-flex items-center gap-1.5 rounded-full px-3 py-1 text-[10px] font-semibold uppercase tracking-[0.14em] ${statusTone[market.status]}`}
                  >
                    {market.status === "trading" && (
                      <span className="size-1.5 rounded-full bg-[#74ddbd] pulse-dot-green" />
                    )}
                    {market.status === "halted" && (
                      <span className="size-1.5 rounded-full bg-[#e2c07a] pulse-dot-amber" />
                    )}
                    {market.status}
                  </span>
                </div>

                <p className="text-right font-mono text-sm text-[#eee7ef]">
                  {market.price_tick_size}
                </p>
                <p className="text-right font-mono text-sm text-[#eee7ef]">
                  {market.quantity_step_size}
                </p>
                <p className="text-right font-mono text-sm text-[#eee7ef]">
                  {market.min_order_quantity}
                </p>

                <div className="flex w-28 justify-end">
                  {market.status === "trading" ? (
                    <Link
                      className="inline-flex items-center gap-1.5 rounded-full border border-[#ff6f61]/40 bg-[#ff6f61]/10 px-4 py-1.5 text-xs font-medium text-[#ff8175] transition-all duration-200 hover:border-[#ff6f61] hover:bg-[#ff6f61] hover:text-[#160e12] hover:shadow-[0_0_16px_rgba(255,111,97,0.4)]"
                      href={`/trade/${encodeURIComponent(market.symbol)}`}
                    >
                      Trade
                      <ArrowUpRight size={14} />
                    </Link>
                  ) : (
                    <span className="px-4 py-1.5 text-xs text-[#57505e]">
                      —
                    </span>
                  )}
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}
