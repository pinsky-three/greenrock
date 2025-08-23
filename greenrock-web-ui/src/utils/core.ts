import type { Time } from "lightweight-charts";
import type {
  ApiCandle,
  Balance,
  BinanceOrderResponse,
  BinanceTradeResponse,
  Candle,
  CandlesQuery,
  ChatRequest,
  ChatResponse,
  Order,
  OrderBook,
  OrderBookQuery,
  PauseResponse,
  Portfolio,
  TimeRangePresets,
  Trade,
} from "../types/core";

// API base URL configuration
// Supports environment override via VITE_API_BASE or defaults to same-origin
const getApiBase = (): string => {
  // Check for environment variable (Vite uses VITE_ prefix)
  if (import.meta.env?.VITE_API_BASE) {
    return import.meta.env.VITE_API_BASE as string;
  }

  // Fallback to same-origin with port 4200 for development
  // In production, this should be overridden via environment variable
  if (typeof window !== "undefined") {
    const { protocol, hostname } = window.location;
    return `${protocol}//${hostname}`;
  }

  // SSR fallback
  return "http://localhost:4200";
};

const API_BASE = getApiBase();

// Helper to create API URLs with proper encoding
const createApiUrl = (path: string, params?: URLSearchParams): string => {
  const url = new URL(path, API_BASE);
  if (params) {
    url.search = params.toString();
  }
  return url.toString();
};

// Helper to create WebSocket URLs (converts http/https to ws/wss)
const createWebSocketUrl = (path: string): string => {
  const url = new URL(path, API_BASE);
  url.protocol = url.protocol === "https:" ? "wss:" : "ws:";
  return url.toString();
};

// Convert API candles to chart format
export const convertApiCandlesToChart = (apiCandles: ApiCandle[]): Candle[] => {
  // console.log("Converting candles:", apiCandles.length, "samples:");
  // if (apiCandles.length > 0) {
  //   console.log("First candle:", apiCandles[0]);
  //   console.log("Last candle:", apiCandles[apiCandles.length - 1]);
  // }

  const converted = apiCandles.map((candle) => ({
    time: Math.floor(candle.timestamp / 1000) as Time,
    open: candle.open,
    high: candle.high,
    low: candle.low,
    close: candle.close,
    volume: candle.volume,
  }));

  // console.log(
  //   "Converted result:",
  //   converted.length,
  //   "first converted:",
  //   converted[0]
  // );
  return converted;
};

// Filter candles by time range
export const filterCandlesByTimeRange = (
  candles: Candle[],
  startTime: Date,
  endTime: Date
): Candle[] => {
  const startTimestamp = Math.floor(startTime.getTime() / 1000);
  const endTimestamp = Math.floor(endTime.getTime() / 1000);

  // console.log(
  //   `Filtering ${candles.length} candles between ${startTimestamp} and ${endTimestamp}`
  // );
  // if (candles.length > 0) {
  //   console.log(
  //     "First candle time:",
  //     candles[0].time,
  //     "Last candle time:",
  //     candles[candles.length - 1].time
  //   );
  // }

  const filtered = candles.filter((candle) => {
    let candleTime: number;
    if (typeof candle.time === "number") {
      candleTime = candle.time;
    } else if (typeof candle.time === "string") {
      candleTime = new Date(candle.time).getTime() / 1000;
    } else {
      // Handle BusinessDay or other Time types
      candleTime = 0; // Skip invalid entries
    }
    const isInRange =
      candleTime >= startTimestamp && candleTime <= endTimestamp;
    return isInRange;
  });

  console.log(`Filtered result: ${filtered.length} candles remain`);
  return filtered;
};

// Get predefined time ranges
export const getTimeRangePresets = (): TimeRangePresets => {
  const now = new Date();
  const presets = {
    "1h": {
      label: "1 Hour",
      start: new Date(now.getTime() - 60 * 60 * 1000),
      end: now,
    },
    "4h": {
      label: "4 Hours",
      start: new Date(now.getTime() - 4 * 60 * 60 * 1000),
      end: now,
    },
    "1d": {
      label: "1 Day",
      start: new Date(now.getTime() - 24 * 60 * 60 * 1000),
      end: now,
    },
    "3d": {
      label: "3 Days",
      start: new Date(now.getTime() - 3 * 24 * 60 * 60 * 1000),
      end: now,
    },
    "1w": {
      label: "1 Week",
      start: new Date(now.getTime() - 7 * 24 * 60 * 60 * 1000),
      end: now,
    },
    all: {
      label: "All Data",
      start: new Date(0), // Unix epoch
      end: now,
    },
    custom: {
      label: "Custom Range",
      start: new Date(now.getTime() - 24 * 60 * 60 * 1000), // Default to 1 day back
      end: now,
    },
  };
  return presets;
};

// Transform Binance API responses to UI format
export const transformBinanceTradeToUI = (
  binanceTrade: BinanceTradeResponse,
  symbol: string
): Trade => {
  return {
    id: binanceTrade.id.toString(),
    symbol: symbol,
    side: binanceTrade.isBuyer ? "buy" : "sell",
    quantity: parseFloat(binanceTrade.qty),
    price: parseFloat(binanceTrade.price),
    timestamp: binanceTrade.time,
    fee: parseFloat(binanceTrade.commission),
  };
};

export const transformBinanceOrderToUI = (
  binanceOrder: BinanceOrderResponse
): Order => {
  return {
    id: binanceOrder.orderId.toString(),
    symbol: binanceOrder.symbol,
    side: binanceOrder.side.toLowerCase() as "buy" | "sell",
    type: binanceOrder.type.toLowerCase() as "market" | "limit",
    quantity: parseFloat(binanceOrder.origQty),
    price: binanceOrder.price ? parseFloat(binanceOrder.price) : undefined,
    status: binanceOrder.status.toLowerCase(),
    timestamp: binanceOrder.time,
  };
};

// API utility functions for new backend routes

// Chat API
export const sendChatMessage = async (
  request: ChatRequest
): Promise<ChatResponse | PauseResponse> => {
  const url = createApiUrl("/chat");
  const response = await fetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(request),
  });
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return response.json();
};

// Strategy API
export const fetchPortfolio = async (): Promise<Portfolio> => {
  const url = createApiUrl("/strategy/portfolio");
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return response.json();
};

// Broker API
export const fetchBalance = async (): Promise<Balance> => {
  const url = createApiUrl("/broker/balance");
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return response.json();
};

export const fetchOpenOrders = async (symbol: string): Promise<Order[]> => {
  const params = new URLSearchParams({ symbol });
  const url = createApiUrl("/broker/open_orders", params);
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  const binanceOrders: BinanceOrderResponse[] = await response.json();
  return binanceOrders.map(transformBinanceOrderToUI);
};

export const fetchTradeHistory = async (symbol: string): Promise<Trade[]> => {
  const params = new URLSearchParams({ symbol });
  const url = createApiUrl("/broker/trade_history", params);
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  const binanceTrades: BinanceTradeResponse[] = await response.json();
  return binanceTrades.map((trade) => transformBinanceTradeToUI(trade, symbol));
};

export const fetchCandles = async (
  query: CandlesQuery
): Promise<{ candles: ApiCandle[] }> => {
  const params = new URLSearchParams({
    symbol: query.symbol,
    interval: query.interval,
  });

  if (query.start) {
    params.append("start", query.start);
  }
  if (query.end) {
    params.append("end", query.end);
  }

  const url = createApiUrl("/broker/candles", params);
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return response.json();
};

export const fetchOrderBook = async (
  query: OrderBookQuery
): Promise<OrderBook> => {
  const params = new URLSearchParams({
    symbol: query.symbol,
    depth: query.depth.toString(),
  });

  const url = createApiUrl("/broker/order_book", params);
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return response.json();
};

// WebSocket connection utilities
export const createCandleStreamWebSocket = (): WebSocket => {
  const url = createWebSocketUrl("/broker/candle_stream");
  return new WebSocket(url);
};

export const createOrderBookStreamWebSocket = (): WebSocket => {
  const url = createWebSocketUrl("/broker/order_book_stream");
  return new WebSocket(url);
};

// // Legacy function for backward compatibility
// export const fetchLatestSession = async (): Promise<LatestSessionResponse> => {
//   // Use new endpoints to construct legacy response
//   try {
//     const [balance, candlesResponse] = await Promise.all([
//       fetchBalance(),
//       fetchCandles({ symbol: "BTCUSDT", interval: "1m" }),
//     ]);

//     return {
//       session_id: "legacy-session",
//       candles: candlesResponse.candles,
//       balance,
//     };
//   } catch (error) {
//     throw new Error(`Failed to fetch session data: ${error}`);
//   }
// };
