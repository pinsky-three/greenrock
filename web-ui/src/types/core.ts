import type { Time } from "lightweight-charts";

export type Candle = {
  time: Time;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
};

export type ApiCandle = {
  close: number;
  high: number;
  low: number;
  open: number;
  timestamp: number;
  ts: string;
  volume: number;
};

export type Balance = {
  [symbol: string]: number;
};

// Chat API types
export type ChatRequest = {
  query: string;
  session_id?: string;
};

export type ChatResponse = {
  session_id: string;
  answer: string;
  status: string;
};

export type PauseResponse = {
  session_id: string;
  status: string;
  next_task: string;
  reason: string;
};

// Portfolio types
export type Portfolio = {
  [symbol: string]: number;
};

// Order Book types
export type OrderBookEntry = {
  price: number;
  quantity: number;
};

export type OrderBook = {
  symbol: string;
  bids: OrderBookEntry[];
  asks: OrderBookEntry[];
  timestamp: number;
};

// Trading types - API response format
export type BinanceTradeResponse = {
  id: number;
  price: string;
  qty: string;
  commission: string;
  commissionAsset: string;
  time: number;
  isBuyer: boolean;
  isMaker: boolean;
  isBestMatch: boolean;
};

export type BinanceOrderResponse = {
  symbol: string;
  orderId: number;
  orderListId: number;
  clientOrderId: string;
  price: string;
  origQty: string;
  executedQty: string;
  cummulativeQuoteQty: string;
  status: string;
  timeInForce: string;
  type: string;
  side: string;
  stopPrice: string;
  icebergQty: string;
  time: number;
  updateTime: number;
  isWorking: boolean;
  workingTime: number;
  origQuoteOrderQty: string;
  selfTradePreventionMode: string;
};

// Trading types - UI format
export type Order = {
  id: string;
  symbol: string;
  side: "buy" | "sell";
  type: "market" | "limit";
  quantity: number;
  price?: number;
  status: string;
  timestamp: number;
};

export type Trade = {
  id: string;
  symbol: string;
  side: "buy" | "sell";
  quantity: number;
  price: number;
  timestamp: number;
  fee?: number;
};

// API Query types
export type CandlesQuery = {
  symbol: string;
  interval: string;
  start?: string;
  end?: string;
};

export type OrderBookQuery = {
  symbol: string;
  depth: number;
};

// Legacy type for backward compatibility
export type LatestSessionResponse = {
  session_id: string;
  candles: ApiCandle[];
  balance: Balance;
};

export type TimeRange = {
  start: Date;
  end: Date;
};

export type TimeRangePreset = {
  label: string;
  start: Date;
  end: Date;
};

export type TimeRangePresets = {
  [key: string]: TimeRangePreset;
};
