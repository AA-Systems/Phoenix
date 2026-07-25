import { ArrowRight, Boxes, CandlestickChart, ShieldCheck } from "lucide-react";
import Image from "next/image";
import Link from "next/link";

import { HeroActions } from "@/components/hero-actions";
import { MarketBoard } from "@/components/market-board";
import { PhoenixCore } from "@/components/phoenix-core";
import { SiteFooter } from "@/components/site-footer";
import { SiteHeader } from "@/components/site-header";
import { Button } from "@/components/ui/button";

const surfaces = [
  {
    icon: CandlestickChart,
    number: "01",
    title: "Trade the book",
    copy: "Live candles, depth, and a ticket on one desk — place limits and watch the market move in real time.",
    imagePath: "/images/trade-desk.png",
    detail: "Chart · order book · ticket",
    href: "/markets",
  },
  {
    icon: Boxes,
    number: "02",
    title: "Know your inventory",
    copy: "Available and locked funds stay separate, visible, and backed by an append-only ledger.",
    imagePath: "/images/balances.png",
    detail: "Available · locked · ledger",
    href: "/balances",
  },
];

export default function Home() {
  return (
    <div className="min-h-screen bg-[#0d0a10]">
      <SiteHeader />

      <main>
        <section className="relative mt-3 overflow-hidden border-b border-[#2c2533]">
          <div className="mx-auto grid min-h-[760px] max-w-[1380px] lg:grid-cols-[1.02fr_0.98fr]">
            <div className="page-reveal flex flex-col justify-center px-6 py-24 sm:px-10 lg:border-r lg:border-[#241f2a] lg:px-12">
              <div className="mb-9 flex items-center gap-4 text-[10px] uppercase tracking-[0.24em] text-[#817787]">
                <span className="h-px w-10 bg-[#ff6f61]" />
                PHEONIX spot exchange / 2026
              </div>
              <h1 className="max-w-3xl text-6xl font-semibold leading-[0.9] tracking-[-0.07em] text-[#fff8f5] sm:text-7xl lg:text-[96px]">
                Clear eyes.
                <br />
                <span className="text-[#ff6f61]">Live markets.</span>
              </h1>
              <p className="mt-8 max-w-xl text-lg leading-8 text-[#a198a5]">
                A spot exchange where every balance has a history, every session
                has a boundary, and the market stays in focus.
              </p>
              <HeroActions />
              <div className="mt-16 grid max-w-xl grid-cols-2 border-y border-[#2c2533] text-xs">
                <span className="flex items-center gap-3 border-r border-[#2c2533] py-4 pr-4 text-[#8d8492]">
                  <ShieldCheck className="text-[#74ddbd]" size={15} />
                  EdDSA sessions
                </span>
                <span className="flex items-center gap-3 py-4 pl-4 text-[#8d8492]">
                  <Boxes className="text-[#ff8175]" size={15} />
                  Append-only ledger
                </span>
              </div>
            </div>

            <div className="relative flex min-h-[640px] items-center overflow-hidden px-6 py-20 sm:px-10 lg:px-12">
              <PhoenixCore />
              <div className="phoenix-terminal relative z-10 ml-auto w-full max-w-xl">
                <div className="mb-4 flex items-center justify-between font-mono text-[10px] uppercase tracking-[0.16em] text-[#716878]">
                  <span>Terminal / Spot</span>
                  <span className="text-[#74ddbd]">Feed active</span>
                </div>
                <MarketBoard />
                <div className="mt-5 flex justify-between border-l border-[#ff6f61] pl-4 font-mono text-[10px] uppercase tracking-[0.14em] text-[#716878]">
                  <span>Live market catalog</span>
                  <span>UTC +00:00</span>
                </div>
              </div>
            </div>
          </div>
          <div className="flex items-center gap-4 border-t border-[#241f2a] px-6 py-3 font-mono text-[9px] uppercase tracking-[0.2em] text-[#57505e]">
            <span className="size-1.5 rounded-full bg-[#ff6f61]" />
            Built for controlled movement
            <span className="h-px flex-1 bg-[#241f2a]" />
            PHX / CORE 01
          </div>
        </section>

        <section
          id="product"
          className="mx-auto max-w-[1380px] px-5 py-28 lg:px-8 lg:py-40"
        >
          <div className="grid gap-10 pb-20 lg:grid-cols-[0.7fr_1.3fr]">
            <div>
              <p className="text-[10px] uppercase tracking-[0.24em] text-[#ff8175]">
                01 / Product
              </p>
              <p className="mt-3 max-w-xs text-sm leading-6 text-[#716878]">
                Two surfaces that stay honest — the desk where you trade, and
                the ledger where you account.
              </p>
            </div>
            <h2 className="max-w-4xl text-4xl font-semibold leading-[1.04] tracking-tighter text-[#fff8f5] sm:text-6xl">
              The exchange,
              <br />
              as it actually looks.
            </h2>
          </div>

          <div>
            {surfaces.map(
              (
                { icon: Icon, number, title, copy, imagePath, detail, href },
                index,
              ) => (
                <article
                  className={`group grid gap-12 border-t border-[#302839] py-16 lg:items-center lg:py-24 ${
                    index % 2 === 1
                      ? "lg:grid-cols-[0.72fr_1.28fr]"
                      : "lg:grid-cols-[1.28fr_0.72fr]"
                  }`}
                  key={number}
                >
                  <div
                    className={`relative min-h-[320px] overflow-hidden rounded-[18px] border border-[#3b3243] bg-[#15111a] transition-transform duration-500 group-hover:-translate-y-1 sm:min-h-[400px] lg:min-h-[480px] ${
                      index % 2 === 1 ? "lg:order-2" : ""
                    }`}
                  >
                    <Image
                      alt={`${title} — Pheonix screenshot`}
                      className="object-cover object-top transition-transform duration-700 group-hover:scale-[1.015]"
                      fill
                      sizes="(min-width: 1024px) 60vw, 100vw"
                      src={imagePath}
                    />
                    <span className="absolute bottom-4 left-4 border border-[#3b3243] bg-[#0d0a10]/90 px-3 py-2 font-mono text-[9px] uppercase tracking-[0.16em] text-[#817787] backdrop-blur">
                      Surface / {number}
                    </span>
                  </div>

                  <div className="px-2 sm:px-8 lg:px-10">
                    <div className="flex items-center gap-4">
                      <Icon
                        className="text-[#ff8175]"
                        size={20}
                        strokeWidth={1.4}
                      />
                      <span className="h-px w-10 bg-[#49343d]" />
                      <span className="font-mono text-[10px] text-[#716878]">
                        {number} / 02
                      </span>
                    </div>
                    <h3 className="mt-10 text-4xl font-semibold tracking-[-0.045em] text-[#fff8f5]">
                      {title}
                    </h3>
                    <p className="mt-6 max-w-md text-base leading-7 text-[#938a98]">
                      {copy}
                    </p>
                    <div className="mt-10 flex flex-wrap items-center justify-between gap-4 border-t border-[#302839] pt-5">
                      <p className="text-[10px] uppercase tracking-[0.18em] text-[#74ddbd]">
                        {detail}
                      </p>
                      <Link
                        className="text-sm font-medium text-[#ff8175] transition-colors hover:text-[#ffb0a8]"
                        href={href}
                      >
                        Open →
                      </Link>
                    </div>
                  </div>
                </article>
              ),
            )}
          </div>
        </section>

        <section className="relative overflow-hidden border-y border-[#302839] bg-[#141018]">
          <div className="absolute bottom-0 left-[10%] top-0 w-px bg-[#241f2a]" />
          <div className="absolute bottom-0 right-[10%] top-0 w-px bg-[#241f2a]" />
          <div className="relative mx-auto flex max-w-[1180px] flex-col items-start justify-between gap-10 px-7 py-24 sm:flex-row sm:items-end lg:py-28">
            <div>
              <p className="flex items-center gap-3 text-[10px] uppercase tracking-[0.22em] text-[#ff8175]">
                <span className="size-1.5 rounded-full bg-[#ff6f61] shadow-[0_0_12px_#ff6f61]" />
                Ready when you are
              </p>
              <h2 className="mt-5 max-w-3xl text-4xl font-semibold leading-[1.02] tracking-tighter text-[#fff8f5] sm:text-6xl">
                Enter the market
                <br />
                with a clear position.
              </h2>
            </div>
            <Link href="/signup">
              <Button className="h-13 px-7">
                Open account <ArrowRight size={17} />
              </Button>
            </Link>
          </div>
        </section>
      </main>

      <SiteFooter />
    </div>
  );
}
