"use client";

import { createContext, useContext } from "react";

import type { DecimalsBySymbol } from "@/lib/trade-format";

const AssetDecimalsContext = createContext<DecimalsBySymbol | null>(null);

export function AssetDecimalsProvider({
  value,
  children,
}: {
  value: DecimalsBySymbol;
  children: React.ReactNode;
}) {
  return (
    <AssetDecimalsContext.Provider value={value}>
      {children}
    </AssetDecimalsContext.Provider>
  );
}

export function useAssetDecimals(): DecimalsBySymbol {
  const value = useContext(AssetDecimalsContext);
  if (!value) {
    throw new Error("useAssetDecimals requires AssetDecimalsProvider");
  }
  return value;
}
