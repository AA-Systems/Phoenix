"use client";

import {
  CandlestickSeries,
  ColorType,
  createChart,
  type CandlestickData,
  type IChartApi,
  type ISeriesApi,
  type Time,
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

export type ChartCandleOhlc = {
  time: number;
  open: number;
  high: number;
  low: number;
  close: number;
};

function toChartPrice(raw: number, decimals: number) {
  return raw / 10 ** decimals;
}

function isCandleData(data: unknown): data is CandlestickData<Time> {
  return (
    typeof data === "object" &&
    data !== null &&
    "open" in data &&
    "high" in data &&
    "low" in data &&
    "close" in data
  );
}

function humanToRaw(human: number, decimals: number) {
  return Math.round(human * 10 ** decimals);
}

function ohlcFromSeries(
  data: CandlestickData<Time>,
  time: Time,
  quoteDecimals: number,
): ChartCandleOhlc | null {
  const unix =
    typeof time === "number"
      ? time
      : typeof time === "string"
        ? Math.floor(new Date(time).getTime() / 1000)
        : null;
  if (unix === null) return null;
  return {
    time: unix,
    open: humanToRaw(data.open, quoteDecimals),
    high: humanToRaw(data.high, quoteDecimals),
    low: humanToRaw(data.low, quoteDecimals),
    close: humanToRaw(data.close, quoteDecimals),
  };
}

export function PriceChart({
  marketSymbol,
  pair,
  trades,
  onCandleFocus,
}: {
  marketSymbol: string;
  pair: string;
  trades: TradeView[];
  onCandleFocus?: (candle: ChartCandleOhlc | null) => void;
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
  const onCandleFocusRef = useRef(onCandleFocus);
  const quoteDecimalsRef = useRef(0);
  const pinnedTimeRef = useRef<number | null>(null);
  const decimalsBySymbol = useAssetDecimals();
  const { quoteDecimals } = marketDecimals(marketSymbol, decimalsBySymbol);

  onCandleFocusRef.current = onCandleFocus;
  quoteDecimalsRef.current = quoteDecimals;

  useEffect(() => {
    let active = true;
    setLoading(true);
    setError("");
    setSeedReady(false);
    primedSeen.current = false;
    seenTrades.current = new Set();
    pinnedTimeRef.current = null;
    onCandleFocusRef.current?.(null);

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
        vertLines: { color: "rgba(40, 32, 48, 0.6)" },
        horzLines: { color: "rgba(40, 32, 48, 0.6)" },
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
        vertLine: { color: "rgba(255, 111, 97, 0.4)", width: 1, style: 2 },
        horzLine: { color: "rgba(255, 111, 97, 0.4)", width: 1, style: 2 },
      },
    });

    const series = chart.addSeries(CandlestickSeries, {
      upColor: "#74ddbd",
      downColor: "#ff6f61",
      borderUpColor: "#74ddbd",
      borderDownColor: "#ff6f61",
      wickUpColor: "#74ddbd",
      wickDownColor: "#ff6f61",
    });

    chartRef.current = chart;
    seriesRef.current = series;

    const emitFromParam = (param: {
      time?: Time;
      seriesData: Map<unknown, unknown>;
    }) => {
      if (param.time === undefined) {
        if (pinnedTimeRef.current === null) {
          onCandleFocusRef.current?.(null);
        }
        return;
      }
      const data = param.seriesData.get(series);
      if (!isCandleData(data)) {
        if (pinnedTimeRef.current === null) {
          onCandleFocusRef.current?.(null);
        }
        return;
      }
      const ohlc = ohlcFromSeries(data, param.time, quoteDecimalsRef.current);
      onCandleFocusRef.current?.(ohlc);
    };

    chart.subscribeCrosshairMove((param) => {
      if (pinnedTimeRef.current !== null) return;
      emitFromParam(param);
    });

    chart.subscribeClick((param) => {
      if (param.time === undefined) {
        pinnedTimeRef.current = null;
        onCandleFocusRef.current?.(null);
        return;
      }
      const data = param.seriesData.get(series);
      if (!isCandleData(data)) {
        pinnedTimeRef.current = null;
        onCandleFocusRef.current?.(null);
        return;
      }
      const ohlc = ohlcFromSeries(data, param.time, quoteDecimalsRef.current);
      if (!ohlc) return;
      if (pinnedTimeRef.current === ohlc.time) {
        pinnedTimeRef.current = null;
      } else {
        pinnedTimeRef.current = ohlc.time;
      }
      onCandleFocusRef.current?.(ohlc);
    });

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
        <div className="flex items-center gap-1 rounded-lg border border-[#27202f] bg-[#18131f] p-0.5">
          {CANDLE_INTERVALS.map((value) => (
            <button
              className={`rounded px-2 py-0.5 text-[10px] font-semibold transition-all ${
                interval === value
                  ? "border border-[#ff6f61]/30 bg-[#ff6f61]/15 text-[#ff8175] shadow-xs"
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
