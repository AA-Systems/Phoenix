"use client";

import {
  Activity,
  CandlestickChart,
  CheckCircle2,
  RefreshCw,
  Search,
  Zap,
} from "lucide-react";
import { useEffect, useMemo, useState } from "react";

import { MarketsTable } from "@/components/markets/markets-table";
import { SiteHeader } from "@/components/site-header";
import { Button } from "@/components/ui/button";
import { listMarkets } from "@/lib/api";
import type { Market } from "@/lib/types";

export default function MarketsPage() {
  const [markets, setMarkets] = useState<Market[]>([]);
  const [query, setQuery] = useState("");
  const [statusFilter, setStatusFilter] = useState<
    "all" | "trading" | "halted"
  >("all");
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  async function loadMarkets() {
    setLoading(true);
    setError("");
    try {
      setMarkets(await listMarkets());
    } catch (caught) {
      setError(
        caught instanceof Error ? caught.message : "Unable to load markets.",
      );
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    let active = true;
    listMarkets()
      .then((next) => {
        if (active) setMarkets(next);
      })
      .catch((caught) => {
        if (active) {
          setError(
            caught instanceof Error
              ? caught.message
              : "Unable to load markets.",
          );
        }
      })
      .finally(() => {
        if (active) setLoading(false);
      });
    return () => {
      active = false;
    };
  }, []);

  const filtered = useMemo(
    () =>
      markets.filter((market) => {
        const matchesQuery = `${market.symbol} ${market.name} ${market.status}`
          .toLowerCase()
          .includes(query.toLowerCase());
        const matchesStatus =
          statusFilter === "all" || market.status === statusFilter;
        return matchesQuery && matchesStatus;
      }),
    [markets, query, statusFilter],
  );

  const tradingCount = markets.filter((m) => m.status === "trading").length;
  const haltedCount = markets.filter((m) => m.status === "halted").length;

  return (
    <div className="relative min-h-screen overflow-hidden bg-[#0d0a10]">
      <div
        aria-hidden
        className="pointer-events-none absolute inset-x-0 top-0 h-[520px] bg-[radial-gradient(ellipse_at_top,_rgba(255,111,97,0.14),_transparent_55%),radial-gradient(ellipse_at_80%_0%,_rgba(116,221,189,0.08),_transparent_40%)]"
      />
      <SiteHeader />
      <main className="relative mx-auto max-w-[1380px] px-5 py-10 lg:px-8 lg:py-14">
        <div className="page-reveal flex flex-col justify-between gap-8 pb-10 md:flex-row md:items-end">
          <div>
            <div className="mb-4 flex items-center gap-4 text-[10px] uppercase tracking-[0.24em] text-[#817787]">
              <span className="h-px w-10 bg-[#ff6f61]" />
              Spot / Markets
            </div>
            <h1 className="max-w-2xl text-4xl font-semibold leading-[0.95] tracking-[-0.05em] text-[#fff8f5] sm:text-6xl">
              Every pair,
              <span className="text-[#ff6f61]"> one board.</span>
            </h1>
            <p className="mt-4 max-w-xl text-base leading-7 text-[#938a98]">
              Live listings from the exchange catalog — status, tick size, and
              minimums for each spot market.
            </p>
          </div>
          <Button disabled={loading} onClick={loadMarkets} tone="quiet">
            <RefreshCw className={loading ? "animate-spin" : ""} size={16} />
            Refresh
          </Button>
        </div>

        {/* Top Summary Cards */}
        <div className="mb-10 grid gap-4 sm:grid-cols-3">
          <div className="rounded-2xl border border-[#2b2434] bg-[#141018]/80 p-5 shadow-lg backdrop-blur-md">
            <div className="flex items-center justify-between text-xs text-[#8e8594]">
              <span className="uppercase tracking-wider font-mono">
                Listed Markets
              </span>
              <CandlestickChart className="text-[#ff8175]" size={16} />
            </div>
            <div className="mt-3 flex items-baseline gap-2">
              <span className="text-3xl font-bold text-[#fff8f5]">
                {markets.length}
              </span>
              <span className="text-xs text-[#74ddbd] font-medium">
                {tradingCount} Active
              </span>
            </div>
          </div>

          <div className="rounded-2xl border border-[#2b2434] bg-[#141018]/80 p-5 shadow-lg backdrop-blur-md">
            <div className="flex items-center justify-between text-xs text-[#8e8594]">
              <span className="uppercase tracking-wider font-mono">
                Order Engine
              </span>
              <Zap className="text-[#74ddbd]" size={16} />
            </div>
            <div className="mt-3 flex items-baseline gap-2">
              <span className="text-3xl font-bold text-[#fff8f5]">
                Ultra-low
              </span>
              <span className="text-xs text-[#8e8594]">Sub-ms matching</span>
            </div>
          </div>

          <div className="rounded-2xl border border-[#2b2434] bg-[#141018]/80 p-5 shadow-lg backdrop-blur-md">
            <div className="flex items-center justify-between text-xs text-[#8e8594]">
              <span className="uppercase tracking-wider font-mono">Status</span>
              <CheckCircle2 className="text-[#74ddbd]" size={16} />
            </div>
            <div className="mt-3 flex items-center gap-2">
              <span className="size-2 rounded-full bg-[#74ddbd] pulse-dot-green" />
              <span className="text-xl font-bold text-[#fff8f5]">
                100% Operational
              </span>
            </div>
          </div>
        </div>

        {error ? (
          <div className="rounded-2xl border border-[#6e353f] bg-[#211318] px-5 py-4 text-sm text-[#ff9e96]">
            {error}
          </div>
        ) : loading ? (
          <div className="grid min-h-80 place-items-center rounded-[28px] border border-[#302839] bg-[#141018]/70">
            <RefreshCw className="animate-spin text-[#ff8175]" size={22} />
          </div>
        ) : (
          <section className="page-reveal" style={{ animationDelay: "80ms" }}>
            <div className="mb-5 flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
              <div className="flex items-center gap-2">
                <button
                  className={`rounded-full px-4 py-1.5 text-xs font-semibold transition-all ${
                    statusFilter === "all"
                      ? "bg-[#ff6f61] text-[#160e12] shadow-sm"
                      : "bg-[#18131f] text-[#8e8594] border border-[#2b2434] hover:bg-[#231d2c] hover:text-[#ded6df]"
                  }`}
                  onClick={() => setStatusFilter("all")}
                >
                  All ({markets.length})
                </button>
                <button
                  className={`rounded-full px-4 py-1.5 text-xs font-semibold transition-all ${
                    statusFilter === "trading"
                      ? "bg-[#74ddbd] text-[#0c1b16] shadow-sm"
                      : "bg-[#18131f] text-[#8e8594] border border-[#2b2434] hover:bg-[#231d2c] hover:text-[#ded6df]"
                  }`}
                  onClick={() => setStatusFilter("trading")}
                >
                  Trading ({tradingCount})
                </button>
                {haltedCount > 0 && (
                  <button
                    className={`rounded-full px-4 py-1.5 text-xs font-semibold transition-all ${
                      statusFilter === "halted"
                        ? "bg-[#e2c07a] text-[#1a140a] shadow-sm"
                        : "bg-[#18131f] text-[#8e8594] border border-[#2b2434] hover:bg-[#231d2c] hover:text-[#ded6df]"
                    }`}
                    onClick={() => setStatusFilter("halted")}
                  >
                    Halted ({haltedCount})
                  </button>
                )}
              </div>

              <label className="flex h-10 items-center gap-2.5 rounded-full border border-[#342d3b] bg-[#15111a]/90 px-4 text-[#716878] transition-all focus-within:border-[#ff6f61] focus-within:shadow-[0_0_12px_rgba(255,111,97,0.2)]">
                <Search size={15} />
                <input
                  className="w-48 bg-transparent text-sm text-[#fff8f5] outline-none placeholder:text-[#5f5665]"
                  onChange={(event) => setQuery(event.target.value)}
                  placeholder="Search market or asset..."
                  value={query}
                />
              </label>
            </div>
            <MarketsTable markets={filtered} />
          </section>
        )}
      </main>
    </div>
  );
}
