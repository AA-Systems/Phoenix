"use client";

import { RefreshCw, Search, ShieldCheck, WalletCards } from "lucide-react";
import { useRouter } from "next/navigation";
import { useEffect, useMemo, useState } from "react";

import { BalanceTable } from "@/components/balances/balance-table";
import { SiteHeader } from "@/components/site-header";
import { Button } from "@/components/ui/button";
import { getBalances } from "@/lib/api";
import { getSession } from "@/lib/session";
import type { AssetBalance } from "@/lib/types";

export default function BalancesPage() {
  const [balances, setBalances] = useState<AssetBalance[]>([]);
  const [query, setQuery] = useState("");
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const router = useRouter();

  async function loadBalances() {
    setLoading(true);
    setError("");
    try {
      setBalances(await getBalances());
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

  useEffect(() => {
    if (!getSession()) {
      router.replace("/login");
      return;
    }

    let active = true;
    getBalances()
      .then((data) => {
        if (active) setBalances(data);
      })
      .catch((caught) => {
        if (active) {
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

  const lockedAssets = balances.filter((balance) => balance.locked > 0).length;

  return (
    <div className="min-h-screen bg-[#0d0a10]">
      <SiteHeader />
      <main className="mx-auto max-w-[1380px] px-5 py-12 lg:px-8 lg:py-16">
        <div className="flex flex-col justify-between gap-8 pb-10 md:flex-row md:items-end">
          <div>
            <p className="text-xs font-semibold uppercase tracking-[0.2em] text-[#ff8175]">
              Portfolio / Spot
            </p>
            <h1 className="mt-4 text-4xl font-semibold tracking-[-0.045em] text-[#fff8f5] sm:text-6xl">
              Asset inventory
            </h1>
            <p className="mt-4 max-w-xl text-[#938a98]">
              Native asset balances only. Available funds can trade; locked
              funds are reserved by orders.
            </p>
          </div>
          <Button disabled={loading} onClick={loadBalances} tone="quiet">
            <RefreshCw className={loading ? "animate-spin" : ""} size={16} />
            Refresh ledger
          </Button>
        </div>

        <div className="grid gap-3 sm:grid-cols-3">
          <div className="rounded-3xl border border-[#302839] bg-[#141018] p-6">
            <p className="text-[10px] uppercase tracking-[0.18em] text-[#716878]">
              Assets held
            </p>
            <p className="mt-3 font-mono text-2xl text-[#fff8f5]">
              {balances.length.toString().padStart(2, "0")}
            </p>
          </div>
          <div className="rounded-3xl border border-[#302839] bg-[#141018] p-6">
            <p className="text-[10px] uppercase tracking-[0.18em] text-[#716878]">
              Assets locked
            </p>
            <p className="mt-3 font-mono text-2xl text-[#fff8f5]">
              {lockedAssets.toString().padStart(2, "0")}
            </p>
          </div>
          <div className="rounded-3xl border border-[#302839] bg-[#141018] p-6">
            <p className="text-[10px] uppercase tracking-[0.18em] text-[#716878]">
              Ledger status
            </p>
            <p className="mt-3 flex items-center gap-2 text-sm text-[#74ddbd]">
              <ShieldCheck size={16} /> Reconciled
            </p>
          </div>
        </div>

        <section className="mt-12">
          <div className="mb-4 flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
            <div className="flex items-center gap-3">
              <span className="grid size-9 place-items-center rounded-xl bg-[#271a20]">
                <WalletCards className="text-[#ff8175]" size={17} />
              </span>
              <h2 className="text-sm font-semibold uppercase tracking-[0.14em] text-[#e6dfe7]">
                Spot balances
              </h2>
            </div>
            <label className="flex h-10 items-center gap-2 rounded-full border border-[#342d3b] bg-[#15111a] px-4 text-[#716878] focus-within:border-[#ff6f61]">
              <Search size={15} />
              <input
                className="w-48 bg-transparent text-sm text-[#fff8f5] outline-none placeholder:text-[#5f5665]"
                onChange={(event) => setQuery(event.target.value)}
                placeholder="Filter assets"
                value={query}
              />
            </label>
          </div>

          {error ? (
            <div className="rounded-2xl border border-[#6e353f] bg-[#211318] px-5 py-4 text-sm text-[#ff9e96]">
              {error}
            </div>
          ) : loading ? (
            <div className="grid min-h-72 place-items-center rounded-[28px] border border-[#302839] bg-[#141018]">
              <RefreshCw className="animate-spin text-[#ff8175]" size={22} />
            </div>
          ) : (
            <BalanceTable balances={filtered} />
          )}
        </section>
      </main>
    </div>
  );
}
