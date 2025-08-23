import type { Time } from "lightweight-charts";
import type {
  ApiCandle,
  Balance,
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

// API utility functions for new backend routes

// Chat API
export const sendChatMessage = async (
  request: ChatRequest
): Promise<ChatResponse | PauseResponse> => {
  const response = await fetch("http://localhost:4200/chat", {
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
  const response = await fetch("http://localhost:4200/strategy/portfolio");
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return response.json();
};

// Broker API
export const fetchBalance = async (): Promise<Balance> => {
  const response = await fetch("http://localhost:4200/broker/balance");
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return response.json();
};

export const fetchOpenOrders = async (symbol: string): Promise<Order[]> => {
  const response = await fetch(
    `http://localhost:4200/broker/open_orders?${symbol}`
  );
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return response.json();
};

export const fetchTradeHistory = async (symbol: string): Promise<Trade[]> => {
  const response = await fetch(
    `http://localhost:4200/broker/trade_history?${symbol}`
  );
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return response.json();
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

  const response = await fetch(
    `http://localhost:4200/broker/candles?${params}`
  );
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

  const response = await fetch(
    `http://localhost:4200/broker/order_book?${params}`
  );
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return response.json();
};

// WebSocket connection utilities
export const createCandleStreamWebSocket = (): WebSocket => {
  return new WebSocket("ws://localhost:4200/broker/candle_stream");
};

export const createOrderBookStreamWebSocket = (): WebSocket => {
  return new WebSocket("ws://localhost:4200/broker/order_book_stream");
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
