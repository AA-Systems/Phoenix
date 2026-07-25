import { Activity, ShieldCheck } from "lucide-react";
import type { ReactNode } from "react";

import { Brand } from "@/components/brand";
import { MarketBoard } from "@/components/market-board";

type AuthShellProps = {
  eyebrow: string;
  title: string;
  description: string;
  children: ReactNode;
};

export function AuthShell({
  eyebrow,
  title,
  description,
  children,
}: AuthShellProps) {
  return (
    <main className="relative grid min-h-screen overflow-hidden bg-[#0d0a10] lg:grid-cols-[minmax(0,1fr)_minmax(480px,0.82fr)]">
      <div
        aria-hidden
        className="pointer-events-none absolute inset-x-0 top-0 h-[520px] bg-[radial-gradient(ellipse_at_top,_rgba(255,111,97,0.14),_transparent_55%),radial-gradient(ellipse_at_80%_0%,_rgba(116,221,189,0.08),_transparent_40%)]"
      />
      <section className="relative flex min-h-screen flex-col px-5 py-6 sm:px-10 lg:px-16">
        <Brand />
        <div className="mx-auto my-auto w-full max-w-lg py-12">
          <div className="mb-4 flex items-center gap-2">
            <span className="size-1.5 rounded-full bg-[#ff6f61] pulse-dot-green" />
            <p className="text-xs font-semibold uppercase tracking-[0.22em] text-[#ff8175]">
              {eyebrow}
            </p>
          </div>
          <h1 className="text-[32px] font-bold leading-[1.08] tracking-[-0.04em] text-[#fff8f5] sm:whitespace-nowrap sm:text-4xl">
            {title}
          </h1>
          <p className="mb-8 mt-4 max-w-md text-base leading-7 text-[#9b929f]">
            {description}
          </p>
          <div className="rounded-[28px] border border-[#302839] bg-[#121016]/90 p-6 shadow-[0_24px_70px_rgba(0,0,0,0.4)] backdrop-blur-md sm:p-8">
            {children}
          </div>
        </div>
        <div className="flex items-center gap-6 text-xs text-[#716878]">
          <span className="flex items-center gap-2">
            <ShieldCheck className="text-[#74ddbd]" size={15} /> EdDSA secured
          </span>
          <span className="flex items-center gap-2">
            <Activity className="text-[#ff8175]" size={15} /> Active session
            guard
          </span>
        </div>
      </section>

      <aside className="relative m-3 hidden rounded-[38px] border border-[#2c2533] bg-[#141018]/90 p-10 shadow-2xl backdrop-blur-md lg:flex lg:flex-col lg:justify-center">
        <div className="mx-auto w-full max-w-xl">
          <div className="mb-4 flex items-center justify-between font-mono text-xs text-[#716878]">
            <span>LIVE / MARKET WINDOW</span>
            <span className="text-[#74ddbd] flex items-center gap-1.5">
              <span className="size-1.5 rounded-full bg-[#74ddbd] pulse-dot-green" />
              Feed connected
            </span>
          </div>
          <MarketBoard />
          <div className="mt-5 grid grid-cols-2 overflow-hidden rounded-2xl border border-[#302839] bg-[#19141e]/90">
            <div className="border-r border-[#302839] p-4">
              <p className="text-[10px] uppercase tracking-[0.16em] text-[#716878]">
                Custody mode
              </p>
              <p className="mt-1 text-sm font-semibold text-[#ded6df]">
                Central ledger
              </p>
            </div>
            <div className="p-4">
              <p className="text-[10px] uppercase tracking-[0.16em] text-[#716878]">
                Settlement
              </p>
              <p className="mt-1 text-sm font-semibold text-[#74ddbd]">
                100% Operational
              </p>
            </div>
          </div>
        </div>
      </aside>
    </main>
  );
}
