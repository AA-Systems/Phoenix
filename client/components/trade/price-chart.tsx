"use client";

import {
  CandlestickSeries,
  ColorType,
  createChart,
  type IChartApi,
  type ISeriesApi,
  type UTCTimestamp,
} from "lightweight-charts";
import { useEffect, useMemo, useRef, useState } from "react";

import { useAssetDecimals } from "@/components/trade/asset-decimals";
import { getCandles } from "@/lib/api";
import {
  applyTradeToCandles,
  CANDLE_INTERVALS,
  type Candle,
  type CandleInterval,
} from "@/lib/candles";
import { marketDecimals } from "@/lib/trade-format";
import type { TradeView } from "@/lib/types";

function toChartPrice(raw: number, decimals: number) {
  return raw / 10 ** decimals;
}

export function PriceChart({
  marketSymbol,
  pair,
  trades,
}: {
  marketSymbol: string;
  pair: string;
  trades: TradeView[];
}) {
  const [interval, setInterval] = useState<CandleInterval>("1m");
  const [candles, setCandles] = useState<Candle[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [seedReady, setSeedReady] = useState(false);
  const seenTrades = useRef<Set<string>>(new Set());
  const primedSeen = useRef(false);
  const containerRef = useRef<HTMLDivElement>(null);
  const chartRef = useRef<IChartApi | null>(null);
  const seriesRef = useRef<ISeriesApi<"Candlestick"> | null>(null);
  const decimalsBySymbol = useAssetDecimals();
  const { quoteDecimals } = marketDecimals(marketSymbol, decimalsBySymbol);

  useEffect(() => {
    let active = true;
    setLoading(true);
    setError("");
    setSeedReady(false);
    primedSeen.current = false;
    seenTrades.current = new Set();

    getCandles(marketSymbol, interval, 200)
      .then((rows) => {
        if (!active) return;
        setCandles(rows);
        setSeedReady(true);
      })
      .catch((caught) => {
        if (!active) return;
        setError(
          caught instanceof Error ? caught.message : "Unable to load candles.",
        );
        setCandles([]);
        setSeedReady(true);
      })
      .finally(() => {
        if (active) setLoading(false);
      });

    return () => {
      active = false;
    };
  }, [marketSymbol, interval]);

  useEffect(() => {
    if (!seedReady) return;

    // History is already in DB candles — mark current feed as seen once.
    if (!primedSeen.current) {
      for (const trade of trades) {
        seenTrades.current.add(trade.id);
      }
      primedSeen.current = true;
      return;
    }

    if (trades.length === 0) return;

    setCandles((prev) => {
      let next = prev;
      for (const trade of trades) {
        if (seenTrades.current.has(trade.id)) continue;
        if (trade.market_symbol.toUpperCase() !== marketSymbol.toUpperCase()) {
          continue;
        }
        seenTrades.current.add(trade.id);
        next = applyTradeToCandles(next, trade, marketSymbol, interval);
      }
      return next;
    });
  }, [trades, marketSymbol, interval, seedReady]);

  const chartData = useMemo(
    () =>
      candles.map((candle) => ({
        time: Math.floor(
          new Date(candle.bucket_start).getTime() / 1000,
        ) as UTCTimestamp,
        open: toChartPrice(candle.open, quoteDecimals),
        high: toChartPrice(candle.high, quoteDecimals),
        low: toChartPrice(candle.low, quoteDecimals),
        close: toChartPrice(candle.close, quoteDecimals),
      })),
    [candles, quoteDecimals],
  );

  useEffect(() => {
    const el = containerRef.current;
    if (!el) return;

    const chart = createChart(el, {
      autoSize: true,
      layout: {
        background: { type: ColorType.Solid, color: "#100d14" },
        textColor: "#938a98",
      },
      grid: {
        vertLines: { color: "rgba(48,40,57,0.55)" },
        horzLines: { color: "rgba(48,40,57,0.55)" },
      },
      rightPriceScale: {
        borderColor: "#2c2533",
      },
      timeScale: {
        borderColor: "#2c2533",
        timeVisible: true,
        secondsVisible: false,
      },
      crosshair: {
        vertLine: { color: "rgba(255,129,117,0.35)" },
        horzLine: { color: "rgba(255,129,117,0.35)" },
      },
    });

    const series = chart.addSeries(CandlestickSeries, {
      upColor: "#1f8f6f",
      downColor: "#d14b4b",
      borderUpColor: "#1f8f6f",
      borderDownColor: "#d14b4b",
      wickUpColor: "#74ddbd",
      wickDownColor: "#ff8175",
    });

    chartRef.current = chart;
    seriesRef.current = series;

    return () => {
      chart.remove();
      chartRef.current = null;
      seriesRef.current = null;
    };
  }, []);

  useEffect(() => {
    const series = seriesRef.current;
    const chart = chartRef.current;
    if (!series || !chart) return;
    series.setData(chartData);
    if (chartData.length > 0) {
      chart.timeScale().scrollToRealTime();
    }
  }, [chartData]);

  return (
    <div className="relative flex h-full min-h-[260px] flex-col overflow-hidden rounded-lg border border-[#2c2533] bg-[#100d14] sm:min-h-[300px] lg:min-h-0">
      <div className="flex items-center justify-between gap-3 border-b border-[#2c2533] px-3 py-2 sm:px-4">
        <p className="text-[10px] uppercase tracking-[0.16em] text-[#716878]">
          Chart · {pair}
        </p>
        <div className="flex items-center gap-1">
          {CANDLE_INTERVALS.map((value) => (
            <button
              className={`rounded-md px-2 py-1 text-[11px] font-medium transition-colors ${
                interval === value
                  ? "bg-[#271a20] text-[#ff8175]"
                  : "text-[#716878] hover:text-[#dcd4de]"
              }`}
              key={value}
              onClick={() => setInterval(value)}
              type="button"
            >
              {value}
            </button>
          ))}
        </div>
      </div>

      <div className="relative min-h-0 flex-1">
        <div className="absolute inset-0" ref={containerRef} />
        {loading ? (
          <div className="absolute inset-0 grid place-items-center bg-[#100d14]/70 text-xs text-[#716878]">
            Loading candles…
          </div>
        ) : null}
        {!loading && error ? (
          <div className="absolute inset-0 grid place-items-center bg-[#100d14]/80 px-4 text-center text-xs text-[#ff9e96]">
            {error}
          </div>
        ) : null}
        {!loading && !error && chartData.length === 0 ? (
          <div className="pointer-events-none absolute inset-0 grid place-items-center text-xs text-[#57505e]">
            No trades yet — candles appear after fills.
          </div>
        ) : null}
      </div>
    </div>
  );
}
