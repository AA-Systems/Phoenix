"use client";

import { RefreshCw } from "lucide-react";
import { useEffect, useState } from "react";

import { SiteHeader } from "@/components/site-header";
import { AssetDecimalsProvider } from "@/components/trade/asset-decimals";
import {
  PriceChart,
  type ChartCandleOhlc,
} from "@/components/trade/price-chart";
import { OpenOrdersPanel } from "@/components/trade/open-orders-panel";
import { OrderBookPanel } from "@/components/trade/order-book-panel";
import { OrderForm } from "@/components/trade/order-form";
import { TradeHeader } from "@/components/trade/trade-header";
import {
  getBalances,
  getOrderBook,
  getRecentTrades,
  listAssets,
  listMarkets,
  listOpenOrders,
  restoreSession,
} from "@/lib/api";
import { formatMarketPair } from "@/lib/markets";
import { decimalsMapFromAssets } from "@/lib/trade-format";
import { useTradeFeed } from "@/lib/use-trade-feed";
import type { Market, OrderBookDepth, TradeView } from "@/lib/types";

export function TradeWorkspace({ symbol }: { symbol: string }) {
  const marketSymbol = decodeURIComponent(symbol).trim().toUpperCase();
  const [markets, setMarkets] = useState<Market[]>([]);
  const [market, setMarket] = useState<Market | null>(null);
  const [bookSeed, setBookSeed] = useState<OrderBookDepth | null>(null);
  const [tradesSeed, setTradesSeed] = useState<TradeView[] | null>(null);
  const [decimalsBySymbol, setDecimalsBySymbol] = useState<Record<
    string,
    number
  > | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [priceHint, setPriceHint] = useState<number | null>(null);
  const [hintKey, setHintKey] = useState(0);
  const [candleOhlc, setCandleOhlc] = useState<ChartCandleOhlc | null>(null);

  const {
    book,
    trades,
    balances,
    orders,
    setBalances,
    setOrders,
    connected,
    wsError,
  } = useTradeFeed(marketSymbol, bookSeed, tradesSeed);

  useEffect(() => {
    let active = true;
    setLoading(true);
    setError("");
    setPriceHint(null);
    setCandleOhlc(null);
    setTradesSeed(null);

    async function boot() {
      try {
        const [allMarkets, assets, depth, recentTrades] = await Promise.all([
          listMarkets(),
          listAssets(),
          getOrderBook(marketSymbol).catch(() => null),
          getRecentTrades(marketSymbol, 50).catch(() => []),
        ]);
        if (!active) return;

        const found =
          allMarkets.find(
            (item) => item.symbol.toUpperCase() === marketSymbol,
          ) ?? null;
        if (!found) {
          setError(`Market ${marketSymbol} not found.`);
          setMarket(null);
          return;
        }

        setMarkets(allMarkets);
        setMarket(found);
        setDecimalsBySymbol(decimalsMapFromAssets(assets));
        setBookSeed(
          depth ?? {
            market_symbol: marketSymbol,
            bids: [],
            asks: [],
          },
        );
        setTradesSeed(recentTrades);

        const session = await restoreSession();
        if (!active) return;
        if (session) {
          const [nextBalances, nextOrders] = await Promise.all([
            getBalances(),
            listOpenOrders(),
          ]);
          if (!active) return;
          setBalances(nextBalances);
          setOrders(nextOrders);
        } else {
          setBalances(null);
          setOrders(null);
        }
      } catch (caught) {
        if (active) {
          setError(
            caught instanceof Error
              ? caught.message
              : "Unable to load trading desk.",
          );
        }
      } finally {
        if (active) setLoading(false);
      }
    }

    boot();
    return () => {
      active = false;
    };
  }, [marketSymbol, setBalances, setOrders]);

  function onPriceClick(price: number) {
    setPriceHint(price);
    setHintKey((key) => key + 1);
  }

  if (loading) {
    return (
      <div className="relative flex h-dvh flex-col overflow-hidden bg-[#0d0a10]">
        <div
          aria-hidden
          className="pointer-events-none absolute inset-x-0 top-0 h-[220px] bg-[radial-gradient(ellipse_at_top,_rgba(255,111,97,0.08),_transparent_55%)]"
        />
        <SiteHeader variant="desk" />
        <div className="grid flex-1 place-items-center">
          <RefreshCw className="animate-spin text-[#ff8175]" size={22} />
        </div>
      </div>
    );
  }

  if (error || !market || !decimalsBySymbol) {
    return (
      <div className="relative flex h-dvh flex-col overflow-hidden bg-[#0d0a10]">
        <div
          aria-hidden
          className="pointer-events-none absolute inset-x-0 top-0 h-[220px] bg-[radial-gradient(ellipse_at_top,_rgba(255,111,97,0.08),_transparent_55%)]"
        />
        <SiteHeader variant="desk" />
        <div className="mx-auto flex max-w-lg flex-1 items-center px-5 text-center">
          <p className="w-full text-lg text-[#ff9e96]">
            {error || "Market unavailable"}
          </p>
        </div>
      </div>
    );
  }

  const tradingMarkets = markets.filter((item) => item.status === "trading");

  return (
    <div className="relative flex h-dvh flex-col overflow-hidden bg-[#0d0a10]">
      <div
        aria-hidden
        className="pointer-events-none absolute inset-x-0 top-0 h-[220px] bg-[radial-gradient(ellipse_at_top,_rgba(255,111,97,0.08),_transparent_55%)]"
      />
      <SiteHeader variant="desk" />

      <AssetDecimalsProvider value={decimalsBySymbol}>
        <div className="flex min-h-0 flex-1 flex-col">
          <TradeHeader
            book={book}
            candleOhlc={candleOhlc}
            connected={connected}
            market={market}
            markets={tradingMarkets}
          />
          {wsError ? (
            <p className="shrink-0 border-b border-[#6e353f] bg-[#211318] px-3 py-2 text-xs text-[#ff9e96] sm:px-4">
              {wsError}
            </p>
          ) : null}

          <div
            className={[
              "grid min-h-0 flex-1 gap-1.5 p-1.5 sm:p-2",
              "grid-cols-1 auto-rows-auto overflow-auto",
              "md:grid-cols-2 md:grid-rows-[minmax(300px,42vh)_minmax(420px,1fr)_minmax(160px,auto)]",
              "lg:grid-cols-[minmax(0,1fr)_minmax(240px,280px)_minmax(280px,340px)]",
              "lg:grid-rows-[minmax(0,1fr)_minmax(140px,200px)]",
              "lg:overflow-hidden",
              "xl:grid-cols-[minmax(0,1fr)_minmax(260px,300px)_minmax(300px,360px)]",
              "xl:grid-rows-[minmax(0,1fr)_minmax(160px,220px)]",
              "2xl:grid-cols-[minmax(0,1fr)_320px_380px]",
            ].join(" ")}
          >
            <div className="min-h-[280px] md:col-span-2 md:min-h-0 lg:col-span-1 lg:col-start-1 lg:row-start-1">
              <PriceChart
                marketSymbol={market.symbol}
                onCandleFocus={setCandleOhlc}
                pair={formatMarketPair(market.symbol)}
                trades={trades}
              />
            </div>

            <div className="min-h-[360px] md:min-h-0 lg:col-start-2 lg:row-start-1 lg:h-full">
              <OrderBookPanel
                book={book}
                marketSymbol={market.symbol}
                onPriceClick={onPriceClick}
                trades={trades}
              />
            </div>

            <div className="flex h-full min-h-[360px] flex-col md:min-h-0 lg:col-start-3 lg:row-start-1">
              <OrderForm
                balances={balances}
                market={market}
                priceHint={priceHint}
                priceHintNonce={hintKey}
              />
            </div>

            <div className="min-h-[160px] md:col-span-2 lg:col-span-3 lg:col-start-1 lg:row-start-2 lg:min-h-0">
              <OpenOrdersPanel marketSymbol={market.symbol} orders={orders} />
            </div>
          </div>
        </div>
      </AssetDecimalsProvider>
    </div>
  );
}
