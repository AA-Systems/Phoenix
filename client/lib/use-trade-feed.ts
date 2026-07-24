"use client";

import { useCallback, useEffect, useRef, useState } from "react";

import { restoreSession } from "@/lib/api";
import type {
  AssetBalance,
  ExchangeEvent,
  OpenOrder,
  OrderBookDepth,
  OrderBookLevelDelta,
  PriceLevel,
  TradeView,
} from "@/lib/types";

const WS_URL = process.env.NEXT_PUBLIC_WS_URL ?? "ws://localhost:3002/ws";

type HubMessage =
  | { type: "ready" }
  | { type: "authenticated"; user_id: string }
  | { type: "subscribed"; channel: string; market?: string }
  | { type: "unsubscribed"; channel: string; market?: string }
  | { type: "error"; message: string }
  | { type: "pong" }
  | { type: "event"; event: ExchangeEvent };

function applyLevelDelta(
  levels: PriceLevel[],
  side: "bid" | "ask",
  update: OrderBookLevelDelta,
): PriceLevel[] {
  const next = levels.filter((level) => level.price !== update.price);
  if (update.quantity > 0) {
    next.push({
      price: update.price,
      quantity: update.quantity,
      order_count: update.order_count,
    });
  }
  next.sort((a, b) => (side === "bid" ? b.price - a.price : a.price - b.price));
  return next;
}

function mergeTrades(seed: TradeView[], live: TradeView[]): TradeView[] {
  const byId = new Map<string, TradeView>();
  for (const trade of seed) byId.set(trade.id, trade);
  for (const trade of live) {
    if (!byId.has(trade.id)) byId.set(trade.id, trade);
  }
  return [...byId.values()]
    .sort(
      (a, b) =>
        new Date(b.created_at).getTime() - new Date(a.created_at).getTime(),
    )
    .slice(0, 80);
}

export function useTradeFeed(
  marketSymbol: string,
  bookSeed: OrderBookDepth | null,
  tradesSeed: TradeView[] | null = null,
) {
  const [book, setBook] = useState<OrderBookDepth | null>(bookSeed);
  const [trades, setTrades] = useState<TradeView[]>([]);
  const [balances, setBalances] = useState<AssetBalance[] | null>(null);
  const [orders, setOrders] = useState<OpenOrder[] | null>(null);
  const [connected, setConnected] = useState(false);
  const [wsError, setWsError] = useState("");
  const socketRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    setBook(bookSeed);
  }, [bookSeed]);

  useEffect(() => {
    if (tradesSeed === null) {
      setTrades([]);
      return;
    }
    setTrades((prev) => mergeTrades(tradesSeed, prev));
  }, [tradesSeed]);

  useEffect(() => {
    let closed = false;
    let socket: WebSocket | null = null;

    async function connect() {
      const session = await restoreSession();
      if (closed) return;

      socket = new WebSocket(WS_URL);
      socketRef.current = socket;

      socket.onopen = () => {
        setConnected(true);
        setWsError("");
      };

      socket.onclose = () => {
        setConnected(false);
      };

      socket.onerror = () => {
        setWsError("WebSocket connection failed");
      };

      socket.onmessage = (event) => {
        let message: HubMessage;
        try {
          message = JSON.parse(event.data) as HubMessage;
        } catch {
          return;
        }

        if (message.type === "ready") {
          socket?.send(
            JSON.stringify({
              op: "subscribe",
              channel: "orderbook",
              market: marketSymbol,
            }),
          );
          socket?.send(
            JSON.stringify({
              op: "subscribe",
              channel: "trades",
              market: marketSymbol,
            }),
          );
          if (session?.accessToken) {
            socket?.send(
              JSON.stringify({ op: "auth", token: session.accessToken }),
            );
          }
          return;
        }

        if (message.type === "authenticated") {
          socket?.send(
            JSON.stringify({ op: "subscribe", channel: "balances" }),
          );
          socket?.send(
            JSON.stringify({ op: "subscribe", channel: "open_orders" }),
          );
          return;
        }

        if (message.type === "error") {
          setWsError(message.message);
          return;
        }

        if (message.type !== "event") return;
        const payload = message.event;

        if (payload.type === "order_book_updated") {
          if (
            payload.market_symbol.toUpperCase() !== marketSymbol.toUpperCase()
          ) {
            return;
          }
          setBook((prev) => {
            const base: OrderBookDepth = prev ?? {
              market_symbol: marketSymbol,
              bids: [],
              asks: [],
            };
            let bids = base.bids;
            let asks = base.asks;
            for (const update of payload.updates) {
              if (update.side === "bid") {
                bids = applyLevelDelta(bids, "bid", update);
              } else {
                asks = applyLevelDelta(asks, "ask", update);
              }
            }
            return { ...base, bids, asks };
          });
        }

        if (payload.type === "trade_executed") {
          if (
            payload.trade.market_symbol.toUpperCase() !==
            marketSymbol.toUpperCase()
          ) {
            return;
          }
          setTrades((prev) => {
            if (prev.some((trade) => trade.id === payload.trade.id)) {
              return prev;
            }
            return [payload.trade, ...prev].slice(0, 80);
          });
        }

        if (payload.type === "balance_updated") {
          setBalances(payload.balances);
        }

        if (payload.type === "open_orders_updated") {
          setOrders(payload.orders);
        }
      };
    }

    connect();

    return () => {
      closed = true;
      socket?.close();
      socketRef.current = null;
    };
  }, [marketSymbol]);

  const resetPrivate = useCallback(() => {
    setBalances(null);
    setOrders(null);
  }, []);

  return {
    book,
    trades,
    balances,
    orders,
    setBalances,
    setOrders,
    connected,
    wsError,
    resetPrivate,
  };
}
