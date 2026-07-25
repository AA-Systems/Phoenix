"use client";

import { useId, useMemo } from "react";

import { activityBuckets, portfolioStats } from "@/lib/balances";
import type { AssetBalance, LedgerEntry } from "@/lib/types";

type Props = {
  balances: AssetBalance[];
  ledger: LedgerEntry[];
};

export function PortfolioOverview({ balances, ledger }: Props) {
  const gradientId = useId();
  const stats = useMemo(
    () => portfolioStats(balances, ledger),
    [balances, ledger],
  );
  const buckets = useMemo(() => activityBuckets(ledger), [ledger]);
  const maxCount = Math.max(1, ...buckets.map((b) => b.count));

  const freeShare = Math.max(0, 100 - stats.lockedShare);
  const circumference = 2 * Math.PI * 34;
  const lockedLength = (stats.lockedShare / 100) * circumference;

  return (
    <div className="grid gap-4 lg:grid-cols-[1.05fr_0.95fr]">
      <div className="relative overflow-hidden rounded-[28px] border border-[#302839] bg-[#141018]/90 p-6 sm:p-7 shadow-[0_20px_50px_rgba(0,0,0,0.3)] backdrop-blur-md">
        <div
          aria-hidden
          className="pointer-events-none absolute -right-16 -top-20 size-64 rounded-full bg-[#ff6f61]/10 blur-3xl"
        />
        <div className="relative flex flex-col gap-8 sm:flex-row sm:items-center">
          <div className="relative mx-auto size-[148px] shrink-0 sm:mx-0">
            <svg className="size-full -rotate-90" viewBox="0 0 88 88">
              <circle
                cx="44"
                cy="44"
                fill="none"
                r="34"
                stroke="#241c28"
                strokeWidth="8"
              />
              <circle
                className="ring-draw"
                cx="44"
                cy="44"
                fill="none"
                r="34"
                stroke="#74ddbd"
                strokeDasharray={`${circumference - lockedLength} ${circumference}`}
                strokeLinecap="round"
                strokeWidth="8"
              />
              <circle
                className="ring-draw"
                cx="44"
                cy="44"
                fill="none"
                r="34"
                stroke="#ff6f61"
                strokeDasharray={`${lockedLength} ${circumference}`}
                strokeDashoffset={-(circumference - lockedLength)}
                strokeLinecap="round"
                strokeWidth="8"
                style={{ animationDelay: "120ms" }}
              />
            </svg>
            <div className="absolute inset-0 grid place-items-center text-center">
              <div>
                <p className="font-mono text-2xl font-bold tracking-tight text-[#fff8f5]">
                  {stats.lockedShare}%
                </p>
                <p className="mt-0.5 text-[10px] font-mono uppercase tracking-[0.16em] text-[#8e8594]">
                  Reserved
                </p>
              </div>
            </div>
          </div>

          <div className="min-w-0 flex-1">
            <div className="flex items-center gap-2">
              <span className="size-1.5 rounded-full bg-[#ff6f61] pulse-dot-green" />
              <p className="text-[10px] font-semibold uppercase tracking-[0.18em] text-[#ff8175]">
                Liquidity Mix
              </p>
            </div>
            <h2 className="mt-2 text-2xl font-bold tracking-[-0.04em] text-[#fff8f5]">
              Available vs Reserved
            </h2>
            <p className="mt-2 max-w-sm text-sm leading-6 text-[#938a98]">
              Average share reserved by active orders across your assets — each
              asset weighted equally.
            </p>

            <div className="mt-5 space-y-3">
              <LegendRow
                color="#74ddbd"
                label="Available"
                value={`${freeShare}% · ${stats.freeAssets} assets free`}
              />
              <LegendRow
                color="#ff6f61"
                label="Locked"
                value={`${stats.lockedShare}% · ${stats.lockedAssets} assets reserved`}
              />
            </div>
          </div>
        </div>

        <div className="relative mt-8 space-y-3.5 border-t border-[#2a2330] pt-6">
          <p className="text-[10px] font-mono uppercase tracking-[0.18em] text-[#8e8594]">
            Asset Allocation
          </p>
          {balances.length === 0 ? (
            <p className="text-sm text-[#817884]">No balances loaded yet.</p>
          ) : (
            balances.map((balance, index) => {
              const total = balance.available + balance.locked;
              const availablePct =
                total > 0 ? (balance.available / total) * 100 : 0;
              const lockedPct = total > 0 ? (balance.locked / total) * 100 : 0;
              return (
                <div key={balance.asset_id}>
                  <div className="mb-1.5 flex items-baseline justify-between gap-3">
                    <span className="font-mono text-xs font-semibold text-[#fff8f5]">
                      {balance.symbol}
                    </span>
                    <span className="text-[10px] font-mono text-[#8e8594]">
                      {lockedPct > 0
                        ? `${Math.round(lockedPct)}% locked`
                        : "100% available"}
                    </span>
                  </div>
                  <div className="flex h-2 overflow-hidden rounded-full bg-[#1d1722] p-0.5">
                    <span
                      className="bar-grow h-full rounded-full bg-[#74ddbd]"
                      style={{
                        width: `${availablePct}%`,
                        animationDelay: `${index * 60}ms`,
                      }}
                    />
                    <span
                      className="bar-grow h-full rounded-full bg-[#ff6f61]"
                      style={{
                        width: `${lockedPct}%`,
                        animationDelay: `${index * 60 + 40}ms`,
                      }}
                    />
                  </div>
                </div>
              );
            })
          )}
        </div>
      </div>

      <div className="relative overflow-hidden rounded-[28px] border border-[#302839] bg-[#141018] p-6 sm:p-7">
        <div
          aria-hidden
          className="pointer-events-none absolute -left-10 bottom-0 size-48 rounded-full bg-[#74ddbd]/08 blur-3xl"
        />
        <div className="relative">
          <p className="text-[10px] font-semibold uppercase tracking-[0.18em] text-[#ff8175]">
            Ledger pulse
          </p>
          <h2 className="mt-2 text-2xl font-semibold tracking-[-0.04em] text-[#fff8f5]">
            Activity this week
          </h2>
          <p className="mt-2 max-w-sm text-sm leading-6 text-[#8d8492]">
            How often your balances moved — deposits, locks, and other ledger
            events by day.
          </p>

          <div className="mt-8">
            <svg
              className="h-36 w-full"
              viewBox="0 0 320 120"
              role="img"
              aria-label="Weekly ledger activity chart"
            >
              <defs>
                <linearGradient id={gradientId} x1="0" x2="0" y1="0" y2="1">
                  <stop offset="0%" stopColor="#ff6f61" stopOpacity="0.35" />
                  <stop offset="100%" stopColor="#ff6f61" stopOpacity="0" />
                </linearGradient>
              </defs>
              {buckets.map((bucket, index) => {
                const x = 24 + index * 44;
                const height = (bucket.count / maxCount) * 72;
                const y = 88 - height;
                return (
                  <g key={bucket.key}>
                    <rect
                      className="bar-grow"
                      fill={`url(#${gradientId})`}
                      height={Math.max(height, bucket.count > 0 ? 4 : 0)}
                      rx="6"
                      style={{ animationDelay: `${index * 70}ms` }}
                      width="22"
                      x={x}
                      y={y}
                    />
                    <rect
                      className="bar-grow"
                      fill="#ff6f61"
                      height={Math.max(height * 0.35, bucket.count > 0 ? 3 : 0)}
                      rx="4"
                      style={{ animationDelay: `${index * 70 + 40}ms` }}
                      width="22"
                      x={x}
                      y={y}
                    />
                    <text
                      fill="#716878"
                      fontSize="9"
                      textAnchor="middle"
                      x={x + 11}
                      y="108"
                    >
                      {bucket.label}
                    </text>
                  </g>
                );
              })}
            </svg>
          </div>

          <div className="mt-4 grid grid-cols-3 gap-3 border-t border-[#2a2330] pt-5">
            <Metric label="Entries" value={String(stats.ledgerCount)} />
            <Metric
              label="Assets"
              value={String(stats.assetCount).padStart(2, "0")}
            />
            <Metric
              label="Peak day"
              value={String(Math.max(0, ...buckets.map((b) => b.count)))}
            />
          </div>
        </div>
      </div>
    </div>
  );
}

function LegendRow({
  color,
  label,
  value,
}: {
  color: string;
  label: string;
  value: string;
}) {
  return (
    <div className="flex items-center justify-between gap-4 text-sm">
      <span className="flex items-center gap-2 text-[#cfc6d3]">
        <span
          className="size-2.5 rounded-full"
          style={{ backgroundColor: color }}
        />
        {label}
      </span>
      <span className="font-mono text-xs text-[#8d8492]">{value}</span>
    </div>
  );
}

function Metric({ label, value }: { label: string; value: string }) {
  return (
    <div>
      <p className="text-[10px] uppercase tracking-[0.14em] text-[#716878]">
        {label}
      </p>
      <p className="mt-1 font-mono text-lg text-[#fff8f5]">{value}</p>
    </div>
  );
}
