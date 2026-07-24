"use client";

import Link from "next/link";
import { useEffect, useMemo, useState } from "react";

import { AssetIcon } from "@/components/markets/asset-icon";
import { useAssetDecimals } from "@/components/trade/asset-decimals";
import { Button } from "@/components/ui/button";
import { createOrder } from "@/lib/api";
import { displayAmount } from "@/lib/balances";
import {
  formatPrice,
  marketDecimals,
  parseHumanToRaw,
  rawToHumanInput,
} from "@/lib/trade-format";
import type { AssetBalance, Market, OrderType } from "@/lib/types";

const PCT_STEPS = [0, 25, 50, 75, 100] as const;

export function OrderForm({
  market,
  balances,
  priceHint,
  priceHintNonce = 0,
  onSubmitted,
}: {
  market: Market;
  balances: AssetBalance[] | null;
  priceHint: number | null;
  priceHintNonce?: number;
  onSubmitted?: () => void;
}) {
  const [side, setSide] = useState<OrderType>("buy");
  const [price, setPrice] = useState("");
  const [quantity, setQuantity] = useState("");
  const [pct, setPct] = useState(0);
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState("");
  const [ok, setOk] = useState("");
  const decimalsBySymbol = useAssetDecimals();

  const { base, quote, baseDecimals, quoteDecimals } = marketDecimals(
    market.symbol,
    decimalsBySymbol,
  );

  const baseBal = balances?.find((b) => b.symbol === base);
  const quoteBal = balances?.find((b) => b.symbol === quote);
  const availableRaw =
    side === "buy" ? (quoteBal?.available ?? 0) : (baseBal?.available ?? 0);
  const availableLabel =
    side === "buy"
      ? quoteBal
        ? `${displayAmount(quoteBal.available, quoteBal.decimals)} ${quote}`
        : `— ${quote}`
      : baseBal
        ? `${displayAmount(baseBal.available, baseBal.decimals)} ${base}`
        : `— ${base}`;

  const orderValueLabel = useMemo(() => {
    const priceRaw = parseHumanToRaw(price, quoteDecimals);
    const qtyRaw = parseHumanToRaw(quantity, baseDecimals);
    if (priceRaw === null || qtyRaw === null) return null;
    const notional =
      (BigInt(priceRaw) * BigInt(qtyRaw)) / BigInt(10) ** BigInt(baseDecimals);
    if (notional > BigInt(Number.MAX_SAFE_INTEGER)) return null;
    return formatPrice(Number(notional), quoteDecimals);
  }, [price, quantity, quoteDecimals, baseDecimals]);

  useEffect(() => {
    if (priceHint !== null) {
      setPrice(rawToHumanInput(priceHint, quoteDecimals));
    }
  }, [priceHint, priceHintNonce, quoteDecimals]);

  function applyPercent(nextPct: number) {
    setPct(nextPct);
    if (!balances || availableRaw <= 0) {
      setQuantity("");
      return;
    }

    if (side === "sell") {
      const qty = Math.floor((availableRaw * nextPct) / 100);
      const stepped =
        Math.floor(qty / market.quantity_step_size) * market.quantity_step_size;
      setQuantity(stepped > 0 ? rawToHumanInput(stepped, baseDecimals) : "");
      return;
    }

    const priceRaw = parseHumanToRaw(price, quoteDecimals);
    if (priceRaw === null || priceRaw <= 0) {
      setQuantity("");
      return;
    }

    const spend = Math.floor((availableRaw * nextPct) / 100);
    const qty = Number(
      (BigInt(spend) * BigInt(10) ** BigInt(baseDecimals)) / BigInt(priceRaw),
    );
    const stepped =
      Math.floor(qty / market.quantity_step_size) * market.quantity_step_size;
    setQuantity(stepped > 0 ? rawToHumanInput(stepped, baseDecimals) : "");
  }

  async function submit() {
    setError("");
    setOk("");
    const priceRaw = parseHumanToRaw(price, quoteDecimals);
    const qtyRaw = parseHumanToRaw(quantity, baseDecimals);
    if (priceRaw === null || qtyRaw === null) {
      setError("Enter a valid price and quantity.");
      return;
    }
    if (priceRaw % market.price_tick_size !== 0) {
      setError(
        `Price must be a multiple of tick ${formatPrice(market.price_tick_size, quoteDecimals)}.`,
      );
      return;
    }
    if (qtyRaw % market.quantity_step_size !== 0) {
      setError("Quantity must match the market step size.");
      return;
    }
    if (qtyRaw < market.min_order_quantity) {
      setError("Quantity is below the market minimum.");
      return;
    }

    setSubmitting(true);
    try {
      await createOrder({
        market_symbol: market.symbol,
        order_type: side,
        price: priceRaw,
        quantity: qtyRaw,
      });
      setOk("Order accepted — waiting for engine apply.");
      setQuantity("");
      setPct(0);
      onSubmitted?.();
    } catch (caught) {
      setError(
        caught instanceof Error ? caught.message : "Unable to place order.",
      );
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <div className="flex flex-col overflow-hidden rounded-lg border border-[#2c2533] bg-[#100d14]">
      <div className="grid grid-cols-2 gap-1 border-b border-[#2c2533] p-1.5">
        <button
          className={`rounded-lg py-2.5 text-sm font-semibold transition-colors ${
            side === "buy"
              ? "bg-[#14352c] text-[#74ddbd] shadow-[inset_0_0_0_1px_rgba(116,221,189,0.25)]"
              : "text-[#716878] hover:bg-[#17131d] hover:text-[#fff8f5]"
          }`}
          onClick={() => {
            setSide("buy");
            setPct(0);
          }}
          type="button"
        >
          Buy
        </button>
        <button
          className={`rounded-lg py-2.5 text-sm font-semibold transition-colors ${
            side === "sell"
              ? "bg-[#3a1c22] text-[#ff8175] shadow-[inset_0_0_0_1px_rgba(255,129,117,0.25)]"
              : "text-[#716878] hover:bg-[#17131d] hover:text-[#fff8f5]"
          }`}
          onClick={() => {
            setSide("sell");
            setPct(0);
          }}
          type="button"
        >
          Sell
        </button>
      </div>

      <div className="flex flex-1 flex-col gap-3.5 p-3.5">
        <div className="flex items-center justify-between text-xs">
          <span className="rounded-md bg-[#17131d] px-2 py-1 font-medium text-[#a198a5]">
            Limit
          </span>
          <span className="text-[#716878]">
            Available{" "}
            <span className="font-mono text-[#e6dfe7]">{availableLabel}</span>
          </span>
        </div>

        <AssetField
          label="Price"
          onChange={(value) => {
            setPrice(value);
            setPct(0);
          }}
          placeholder="0.00"
          symbol={quote}
          value={price}
        />

        <div className="space-y-2">
          <AssetField
            label="Quantity"
            onChange={(value) => {
              setQuantity(value);
              setPct(0);
            }}
            placeholder="0.00"
            symbol={base}
            value={quantity}
          />
          <div className="flex items-center gap-1">
            {PCT_STEPS.map((step) => (
              <button
                className={`h-7 flex-1 rounded-md text-[11px] font-medium transition-colors ${
                  pct === step
                    ? side === "buy"
                      ? "bg-[#14352c] text-[#74ddbd]"
                      : "bg-[#3a1c22] text-[#ff8175]"
                    : "bg-[#17131d] text-[#716878] hover:text-[#dcd4de]"
                }`}
                key={step}
                onClick={() => applyPercent(step)}
                type="button"
              >
                {step}%
              </button>
            ))}
          </div>
        </div>

        <AssetField
          label="Order value"
          placeholder="—"
          readOnly
          symbol={quote}
          value={orderValueLabel ?? ""}
        />

        {balances === null ? (
          <p className="rounded-xl border border-[#302839] bg-[#17131d] px-3 py-3 text-xs leading-5 text-[#817787]">
            <Link className="text-[#ff8175] underline" href="/login">
              Log in
            </Link>{" "}
            to place orders and see balances.
          </p>
        ) : null}

        {error ? <p className="text-xs text-[#ff9e96]">{error}</p> : null}
        {ok ? <p className="text-xs text-[#74ddbd]">{ok}</p> : null}

        <Button
          className={`mt-auto h-11 w-full rounded-xl text-[15px] ${
            side === "buy"
              ? "border-transparent bg-[#1f8f6f] text-white hover:bg-[#24a07d]"
              : "border-transparent bg-[#d14b4b] text-white hover:bg-[#e05555]"
          }`}
          disabled={
            submitting || balances === null || market.status !== "trading"
          }
          onClick={submit}
          tone="quiet"
        >
          {submitting
            ? "Submitting…"
            : `${side === "buy" ? "Buy" : "Sell"} ${base}`}
        </Button>
      </div>
    </div>
  );
}

function AssetField({
  label,
  symbol,
  value,
  onChange,
  placeholder,
  readOnly = false,
}: {
  label: string;
  symbol: string;
  value: string;
  onChange?: (value: string) => void;
  placeholder: string;
  readOnly?: boolean;
}) {
  return (
    <label className="block">
      <span className="mb-1.5 flex items-center justify-between text-[11px] text-[#716878]">
        <span>{label}</span>
        <span className="font-mono text-[10px] uppercase tracking-[0.08em] text-[#57505e]">
          {symbol}
        </span>
      </span>
      <div className="group relative flex h-12 items-center rounded-xl border border-[#302839] bg-[#17131d] transition-colors focus-within:border-[#ff6f61]/70">
        <input
          className="h-full w-full bg-transparent pr-12 pl-3.5 font-mono text-sm text-[#fff8f5] outline-none placeholder:text-[#4a4352] disabled:cursor-default"
          disabled={readOnly}
          inputMode="decimal"
          onChange={(event) => onChange?.(event.target.value)}
          placeholder={placeholder}
          readOnly={readOnly}
          value={value}
        />
        <span className="pointer-events-none absolute top-1/2 right-3 -translate-y-1/2">
          <AssetIcon size={22} symbol={symbol} />
        </span>
      </div>
    </label>
  );
}
