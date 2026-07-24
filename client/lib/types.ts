export type User = {
  id: string;
  name: string;
  email: string;
  created_at: string;
  updated_at: string;
};

export type AuthResponse = {
  user: User;
  access_token: string;
  expires_in: number;
};

export type AssetBalance = {
  asset_id: string;
  symbol: string;
  name: string;
  decimals: number;
  available: number;
  locked: number;
  updated_at: string;
};

export type LedgerEntryType =
  "deposit" | "withdrawal" | "lock" | "unlock" | "trade" | "fee" | "adjustment";

export type LedgerEntry = {
  id: string;
  asset_id: string;
  asset_symbol: string;
  asset_decimals: number;
  entry_type: LedgerEntryType;
  available_delta: number;
  locked_delta: number;
  available_after: number;
  locked_after: number;
  reference_id: string | null;
  reference_type: string | null;
  command_id: string | null;
  created_at: string;
};

export type Session = {
  user: User;
  accessToken: string;
  expiresAt: number;
};

export type MarketStatus = "trading" | "halted" | "archived";

export type AssetStatus = "active" | "archived";

export type Asset = {
  id: string;
  symbol: string;
  name: string;
  decimals: number;
  status: AssetStatus;
  created_at: string;
};

export type Market = {
  id: string;
  symbol: string;
  name: string;
  base_asset_id: string;
  quote_asset_id: string;
  status: MarketStatus;
  price_tick_size: number;
  quantity_step_size: number;
  min_order_quantity: number;
  min_order_notional: number;
  created_at: string;
};

export type OrderType = "buy" | "sell";
export type OrderStatus =
  "active" | "partially_filled" | "filled" | "cancelled" | "rejected";

export type PriceLevel = {
  price: number;
  quantity: number;
  order_count: number;
};

export type OrderBookDepth = {
  market_symbol: string;
  bids: PriceLevel[];
  asks: PriceLevel[];
};

export type OrderBookLevelDelta = {
  side: "bid" | "ask";
  price: number;
  quantity: number;
  order_count: number;
};

export type OpenOrder = {
  id: string;
  user_id: string;
  market_symbol: string;
  order_type: OrderType;
  price: number;
  quantity: number;
  filled_quantity: number;
  remaining: number;
  status: OrderStatus;
  created_at: string;
};

export type TradeView = {
  id: string;
  market_id: string;
  market_symbol: string;
  maker_order_id: string;
  taker_order_id: string;
  price: number;
  quantity: number;
  buyer_user_id: string;
  seller_user_id: string;
  created_at: string;
};

export type CreateOrderResponse = {
  command_id: string;
  market_symbol: string;
  order_type: OrderType;
  price: number;
  quantity: number;
};

export type CancelOrderResponse = {
  command_id: string;
  order_id: string;
};

export type ExchangeEvent =
  | {
      type: "balance_updated";
      user_id: string;
      balances: AssetBalance[];
    }
  | {
      type: "order_book_updated";
      market_symbol: string;
      updates: OrderBookLevelDelta[];
    }
  | {
      type: "open_orders_updated";
      user_id: string;
      orders: OpenOrder[];
    }
  | {
      type: "trade_executed";
      trade: TradeView;
    };
