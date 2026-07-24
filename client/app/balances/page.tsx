"use client";

import {
  ArrowDownToLine,
  History,
  RefreshCw,
  Search,
  WalletCards,
} from "lucide-react";
import { useRouter } from "next/navigation";
import { useEffect, useMemo, useState } from "react";

import { BalanceTable } from "@/components/balances/balance-table";
import { LedgerActivity } from "@/components/balances/ledger-activity";
import { PortfolioOverview } from "@/components/balances/portfolio-overview";
import { SiteHeader } from "@/components/site-header";
import { Button } from "@/components/ui/button";
import { demoCredit, getBalances, getLedger } from "@/lib/api";
import { getSession } from "@/lib/session";
import type { AssetBalance, LedgerEntry } from "@/lib/types";

export default function BalancesPage() {
  const [balances, setBalances] = useState<AssetBalance[]>([]);
  const [ledger, setLedger] = useState<LedgerEntry[]>([]);
  const [query, setQuery] = useState("");
  const [loading, setLoading] = useState(true);
  const [crediting, setCrediting] = useState(false);
  const [error, setError] = useState("");
  const [notice, setNotice] = useState("");
  const router = useRouter();

  async function loadPortfolio() {
    setLoading(true);
    setError("");
    try {
      const [nextBalances, nextLedger] = await Promise.all([
        getBalances(),
        getLedger(),
      ]);
      setBalances(nextBalances);
      setLedger(nextLedger);
    } catch (caught) {
      if (!getSession()) {
        router.replace("/login");
        return;
      }
      setError(
        caught instanceof Error ? caught.message : "Unable to load balances.",
      );
    } finally {
      setLoading(false);
    }
  }

  async function getTestFunds(assetSymbol?: string) {
    setCrediting(true);
    setError("");
    setNotice("");
    try {
      const result = await demoCredit(assetSymbol);
      setNotice(
        assetSymbol
          ? `Credited demo ${assetSymbol}. Refresh in a moment.`
          : `Credited ${result.credits.length} assets. Refresh in a moment.`,
      );
      window.setTimeout(() => {
        void loadPortfolio();
      }, 1200);
    } catch (caught) {
      setError(
        caught instanceof Error
          ? caught.message
          : "Unable to credit demo funds.",
      );
    } finally {
      setCrediting(false);
    }
  }

  useEffect(() => {
    let active = true;
    Promise.all([getBalances(), getLedger()])
      .then(([nextBalances, nextLedger]) => {
        if (active) {
          setBalances(nextBalances);
          setLedger(nextLedger);
        }
      })
      .catch((caught) => {
        if (active) {
          if (!getSession()) {
            router.replace("/login");
            return;
          }
          setError(
            caught instanceof Error
              ? caught.message
              : "Unable to load balances.",
          );
        }
      })
      .finally(() => {
        if (active) setLoading(false);
      });

    return () => {
      active = false;
    };
  }, [router]);

  const filtered = useMemo(
    () =>
      balances.filter((balance) =>
        `${balance.symbol} ${balance.name}`
          .toLowerCase()
          .includes(query.toLowerCase()),
      ),
    [balances, query],
  );

  return (
    <div className="relative min-h-screen overflow-hidden bg-[#0d0a10]">
      <div
        aria-hidden
        className="pointer-events-none absolute inset-x-0 top-0 h-[520px] bg-[radial-gradient(ellipse_at_top,_rgba(255,111,97,0.14),_transparent_55%),radial-gradient(ellipse_at_80%_0%,_rgba(116,221,189,0.08),_transparent_40%)]"
      />
      <SiteHeader />
      <main className="relative mx-auto max-w-[1380px] px-5 py-12 lg:px-8 lg:py-16">
        <div className="page-reveal flex flex-col justify-between gap-8 pb-10 md:flex-row md:items-end">
          <div>
            <div className="mb-5 flex items-center gap-4 text-[10px] uppercase tracking-[0.24em] text-[#817787]">
              <span className="h-px w-10 bg-[#ff6f61]" />
              Portfolio / Spot
            </div>
            <h1 className="max-w-2xl text-4xl font-semibold leading-[0.95] tracking-[-0.05em] text-[#fff8f5] sm:text-6xl">
              Your inventory,
              <span className="text-[#ff6f61]"> in motion.</span>
            </h1>
            <p className="mt-5 max-w-xl text-base leading-7 text-[#938a98]">
              Spot balances with a live read on what&apos;s free to trade, what
              orders have reserved, and the ledger behind every change.
            </p>
          </div>
          <div className="flex flex-wrap gap-2">
            <Button
              disabled={loading || crediting}
              onClick={() => void getTestFunds()}
            >
              <ArrowDownToLine size={16} />
              {crediting ? "Crediting…" : "Get test funds"}
            </Button>
            <Button disabled={loading} onClick={loadPortfolio} tone="quiet">
              <RefreshCw className={loading ? "animate-spin" : ""} size={16} />
              Refresh
            </Button>
          </div>
        </div>

        {notice ? (
          <div className="mb-4 rounded-2xl border border-[#2a5a4c] bg-[#13211e] px-5 py-4 text-sm text-[#74ddbd]">
            {notice}
          </div>
        ) : null}

        {error ? (
          <div className="rounded-2xl border border-[#6e353f] bg-[#211318] px-5 py-4 text-sm text-[#ff9e96]">
            {error}
          </div>
        ) : loading ? (
          <div className="grid min-h-80 place-items-center rounded-[28px] border border-[#302839] bg-[#141018]/70">
            <RefreshCw className="animate-spin text-[#ff8175]" size={22} />
          </div>
        ) : (
          <>
            <section className="page-reveal" style={{ animationDelay: "80ms" }}>
              <PortfolioOverview balances={balances} ledger={ledger} />
            </section>

            <section
              className="page-reveal mt-12"
              style={{ animationDelay: "140ms" }}
            >
              <div className="mb-4 flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
                <div className="flex items-center gap-3">
                  <span className="grid size-9 place-items-center rounded-xl bg-[#271a20]">
                    <WalletCards className="text-[#ff8175]" size={17} />
                  </span>
                  <div>
                    <h2 className="text-sm font-semibold uppercase tracking-[0.14em] text-[#e6dfe7]">
                      Spot balances
                    </h2>
                    <p className="mt-1 text-xs text-[#716878]">
                      {filtered.length} asset
                      {filtered.length === 1 ? "" : "s"} in view
                    </p>
                  </div>
                </div>
                <label className="flex h-10 items-center gap-2 rounded-full border border-[#342d3b] bg-[#15111a]/90 px-4 text-[#716878] focus-within:border-[#ff6f61]">
                  <Search size={15} />
                  <input
                    className="w-48 bg-transparent text-sm text-[#fff8f5] outline-none placeholder:text-[#5f5665]"
                    onChange={(event) => setQuery(event.target.value)}
                    placeholder="Filter assets"
                    value={query}
                  />
                </label>
              </div>
              <BalanceTable
                balances={filtered}
                crediting={crediting}
                onDemoCredit={(symbol) => void getTestFunds(symbol)}
              />
            </section>

            <section
              className="page-reveal mt-14"
              style={{ animationDelay: "200ms" }}
            >
              <div className="mb-4 flex items-center gap-3">
                <span className="grid size-9 place-items-center rounded-xl bg-[#271a20]">
                  <History className="text-[#ff8175]" size={17} />
                </span>
                <div>
                  <h2 className="text-sm font-semibold uppercase tracking-[0.14em] text-[#e6dfe7]">
                    Recent activity
                  </h2>
                  <p className="mt-1 text-xs text-[#716878]">
                    Append-only ledger of balance changes
                  </p>
                </div>
              </div>
              <LedgerActivity entries={ledger} />
            </section>
          </>
        )}
      </main>
    </div>
  );
}
