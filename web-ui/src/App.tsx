import {
  // AreaSeries,
  createChart,
  ColorType,
  CandlestickSeries,
} from "lightweight-charts";
import type { Time, ISeriesApi } from "lightweight-charts";
import { useEffect, useRef, useState, useCallback } from "react";

type Candle = {
  time: Time;
  open: number;
  high: number;
  low: number;
  close: number;
};

type ApiCandle = {
  close: number;
  high: number;
  low: number;
  open: number;
  timestamp: number;
  ts: string;
  volume: number;
};

type Balance = {
  [symbol: string]: number;
};

type LatestSessionResponse = {
  session_id: string;
  candles: ApiCandle[];
  balance: Balance;
};

// Convert API candles to chart format
function convertApiCandlesToChart(apiCandles: ApiCandle[]): Candle[] {
  return apiCandles.map((candle) => ({
    time: Math.floor(candle.timestamp / 1000) as Time,
    open: candle.open,
    high: candle.high,
    low: candle.low,
    close: candle.close,
  }));
}

// Fetch latest session data from API
async function fetchLatestSession(): Promise<LatestSessionResponse> {
  const response = await fetch("http://localhost:4200/session");
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return response.json();
}

export const ChartComponent = (props: {
  colors: {
    backgroundColor: string;
    lineColor: string;
    textColor: string;
    areaTopColor: string;
    areaBottomColor: string;
  };
  candleData?: Candle[];
  onSeriesReady?: (series: ISeriesApi<"Candlestick">) => void;
}) => {
  const {
    colors: {
      backgroundColor = "black",
      lineColor = "#2962FF",
      textColor = "white",
      areaTopColor = "#2962FF",
      areaBottomColor = "rgba(41, 98, 255, 0.28)",
    } = {},
    candleData,
    onSeriesReady,
  } = props;

  const chartContainerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleResize = () => {
      chart.applyOptions({
        width: chartContainerRef.current?.clientWidth,
        height: chartContainerRef.current?.clientHeight || 400,
      });
    };

    const chart = createChart(chartContainerRef.current!, {
      layout: {
        background: { type: ColorType.Solid, color: "#000000" },
        textColor: "#ffffff",
      },
      grid: {
        vertLines: {
          color: "#1a1a1a",
        },
        horzLines: {
          color: "#1a1a1a",
        },
      },
      timeScale: {
        borderColor: "#333333",
        timeVisible: true,
      },
      rightPriceScale: {
        borderColor: "#333333",
      },
      width: chartContainerRef.current?.clientWidth,
      height: chartContainerRef.current?.clientHeight || 400,
    });

    const series = chart.addSeries(CandlestickSeries, {
      upColor: "#00ff88",
      downColor: "#ff4444",
      borderVisible: false,
      wickUpColor: "#00ff88",
      wickDownColor: "#ff4444",
    });

    // Use real data if available, otherwise fall back to generated data
    if (candleData && candleData.length > 0) {
      series.setData(candleData);
    }

    // Notify parent component that series is ready for real-time updates
    if (onSeriesReady) {
      onSeriesReady(series);
    }

    // const chartOptions = {
    //   layout: {
    //     textColor: "white",
    //     background: { type: "solid", color: "black" },
    //   },
    //   height: 200,
    // };

    chart.timeScale().fitContent();
    chart.timeScale().scrollToPosition(5, true);

    // Only simulate real-time data if using generated data
    // let intervalID: ReturnType<typeof setInterval> | null = null;
    // if (!candleData || candleData.length === 0) {
    //   const data = generateData(2500, 20, 1000);

    //   // simulate real-time data
    //   function* getNextRealtimeUpdate(realtimeData: Candle[]) {
    //     for (const dataPoint of realtimeData) {
    //       yield dataPoint;
    //     }
    //     return null;
    //   }
    //   const streamingDataProvider = getNextRealtimeUpdate(data.realtimeUpdates);

    //   intervalID = setInterval(() => {
    //     const update = streamingDataProvider.next();
    //     if (update.done) {
    //       if (intervalID) clearInterval(intervalID);
    //       return;
    //     }
    //     series.update(update.value);
    //   }, 100);
    // }

    window.addEventListener("resize", handleResize);

    return () => {
      window.removeEventListener("resize", handleResize);
      // if (intervalID) {
      //   clearInterval(intervalID);
      // }
      chart.remove();
    };
  }, [
    backgroundColor,
    lineColor,
    textColor,
    areaTopColor,
    areaBottomColor,
    candleData,
    onSeriesReady,
  ]);

  return <div ref={chartContainerRef} className="w-full h-full" />;
};

export default function App() {
  const [sessionData, setSessionData] = useState<LatestSessionResponse | null>(
    null
  );
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isConnected, setIsConnected] = useState(false);
  const [candlestickSeries, setCandlestickSeries] =
    useState<ISeriesApi<"Candlestick"> | null>(null);
  const wsRef = useRef<WebSocket | null>(null);
  const isMountedRef = useRef(true);

  // Load data function
  const loadData = async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await fetchLatestSession();
      setSessionData(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load data");
      console.error("Failed to fetch session data:", err);
    } finally {
      setLoading(false);
    }
  };

  // WebSocket connection management
  const connectWebSocket = useCallback(() => {
    if (
      wsRef.current?.readyState === WebSocket.OPEN ||
      wsRef.current?.readyState === WebSocket.CONNECTING
    ) {
      return; // Already connected or connecting
    }

    // Clean up any existing connection
    if (wsRef.current) {
      wsRef.current.close();
      wsRef.current = null;
    }

    try {
      const ws = new WebSocket("ws://localhost:4200/session_stream");
      wsRef.current = ws;

      ws.onopen = () => {
        console.log("WebSocket connected");
        if (isMountedRef.current) {
          setIsConnected(true);
          setError(null);
        }
      };

      ws.onmessage = (event) => {
        try {
          const apiCandle: ApiCandle = JSON.parse(event.data);
          console.log("Received candle:", apiCandle);

          // Convert API candle to chart format
          const chartCandle: Candle = {
            time: Math.floor(apiCandle.timestamp / 1000) as Time,
            open: apiCandle.open,
            high: apiCandle.high,
            low: apiCandle.low,
            close: apiCandle.close,
          };

          // Update the chart with new candle data
          if (candlestickSeries) {
            candlestickSeries.update(chartCandle);
          }
        } catch (err) {
          console.error("Failed to parse WebSocket message:", err);
        }
      };

      ws.onerror = (error) => {
        console.error("WebSocket error:", error);
        if (isMountedRef.current) {
          setError("WebSocket connection error");
        }
      };

      ws.onclose = (event) => {
        console.log("WebSocket disconnected:", event.code, event.reason);

        if (isMountedRef.current) {
          setIsConnected(false);
        }

        // Clear the reference if this was the current connection
        if (wsRef.current === ws) {
          wsRef.current = null;
        }

        // Only attempt to reconnect if it wasn't a manual disconnection and component is still mounted
        if (
          event.code !== 1000 &&
          event.code !== 1001 &&
          isMountedRef.current
        ) {
          setTimeout(() => {
            // Double-check we still need to reconnect
            if (!wsRef.current && candlestickSeries && isMountedRef.current) {
              connectWebSocket();
            }
          }, 3000);
        }
      };
    } catch (err) {
      console.error("Failed to create WebSocket connection:", err);
      setError("Failed to establish WebSocket connection");
    }
  }, [candlestickSeries]);

  // Disconnect WebSocket
  const disconnectWebSocket = useCallback(() => {
    if (wsRef.current) {
      wsRef.current.close(1000, "Manual disconnect");
      wsRef.current = null;
      setIsConnected(false);
    }
  }, []);

  // Fetch data on component mount and set up auto-refresh
  useEffect(() => {
    loadData();

    // Auto-refresh every 30 seconds
    const interval = setInterval(loadData, 60000);

    return () => clearInterval(interval);
  }, []);

  // Handle WebSocket connection when chart series is ready
  useEffect(() => {
    if (candlestickSeries && !wsRef.current) {
      connectWebSocket();
    }
  }, [candlestickSeries, connectWebSocket]);

  // Cleanup WebSocket on component unmount
  useEffect(() => {
    return () => {
      isMountedRef.current = false;
      disconnectWebSocket();
    };
  }, [disconnectWebSocket]);

  // Handle series ready callback
  const handleSeriesReady = useCallback((series: ISeriesApi<"Candlestick">) => {
    setCandlestickSeries(series);
  }, []);

  // Convert API candles to chart format
  const chartData = sessionData
    ? convertApiCandlesToChart(sessionData.candles)
    : undefined;

  // Calculate portfolio value from balance
  const calculatePortfolioValue = (balance: Balance): number => {
    // For now, we'll just sum up the USDT value and estimate others
    // In a real app, you'd fetch current prices for each asset
    const usdtValue = balance.USDT || 0;
    const btcValue = (balance.BTC || 0) * 100000; // Rough estimate
    const ethValue = (balance.ETH || 0) * 3000; // Rough estimate
    const solValue = (balance.SOL || 0) * 100; // Rough estimate
    const adaValue = (balance.ADA || 0) * 1; // Rough estimate
    const xrpValue = (balance.XRP || 0) * 2; // Rough estimate

    return usdtValue + btcValue + ethValue + solValue + adaValue + xrpValue;
  };

  const portfolioValue = sessionData
    ? calculatePortfolioValue(sessionData.balance)
    : 0;

  return (
    <div className="h-screen w-screen bg-black text-white flex flex-col">
      {/* Header */}
      <header className="bg-black p-3 border-b border-gray-800 flex justify-between items-center">
        <h1 className="text-xl font-medium text-gray-200">
          Greenrock Dashboard
        </h1>
        <div className="flex items-center space-x-3">
          {sessionData && (
            <span className="text-xs text-gray-500">
              Last updated: {new Date().toLocaleTimeString()}
            </span>
          )}
          <div className="flex items-center space-x-2">
            <div
              className={`w-2 h-2 rounded-full ${
                isConnected ? "bg-green-400" : "bg-red-400"
              }`}
            ></div>
            <span className="text-xs text-gray-500">
              {isConnected ? "Live" : "Disconnected"}
            </span>
          </div>
          <button
            onClick={loadData}
            disabled={loading}
            className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 px-3 py-1.5 rounded text-xs font-medium"
          >
            {loading ? "Refreshing..." : "Refresh"}
          </button>
          <button
            onClick={() =>
              isConnected ? disconnectWebSocket() : connectWebSocket()
            }
            className={`px-3 py-1.5 rounded text-xs font-medium ${
              isConnected
                ? "bg-red-600 hover:bg-red-700"
                : "bg-green-600 hover:bg-green-700"
            }`}
          >
            {isConnected ? "Disconnect" : "Connect"}
          </button>
        </div>
      </header>

      {/* Main Content Area */}
      <div className="flex-1 flex">
        {/* Left Sidebar */}
        <aside className="w-56 bg-black border-r border-gray-800 p-3">
          <h2 className="text-sm font-medium mb-3 text-gray-300">
            Portfolio Balance
          </h2>
          <div className="space-y-2">
            {sessionData?.balance ? (
              Object.entries(sessionData.balance)
                .filter(([, value]) => value > 0)
                .map(([symbol, amount]) => (
                  <div key={symbol} className="bg-gray-900 p-2 rounded">
                    <div className="text-xs text-gray-400">{symbol}</div>
                    <div className="text-green-400 font-medium text-sm">
                      {amount.toFixed(symbol === "USDT" ? 2 : 6)}
                    </div>
                    <div className="text-xs text-gray-500">
                      {symbol === "USDT" ? "USD" : "Available"}
                    </div>
                  </div>
                ))
            ) : (
              <div className="text-gray-500 text-xs">Loading balance...</div>
            )}
          </div>
        </aside>

        {/* Center - Chart Area */}
        <main className="flex-1 bg-black">
          {error ? (
            <div className="flex items-center justify-center h-full">
              <div className="text-red-400">Error: {error}</div>
            </div>
          ) : loading ? (
            <div className="flex items-center justify-center h-full">
              <div className="text-gray-400">Loading chart data...</div>
            </div>
          ) : (
            <ChartComponent
              colors={{
                backgroundColor: "#000000",
                lineColor: "#ffffff",
                textColor: "#ffffff",
                areaTopColor: "#000000",
                areaBottomColor: "#000000",
              }}
              candleData={chartData}
              onSeriesReady={handleSeriesReady}
            />
          )}
        </main>

        {/* Right Sidebar */}
        <aside className="w-56 bg-black border-l border-gray-800 p-3">
          <h2 className="text-sm font-medium mb-3 text-gray-300">Order Book</h2>

          {/* Buy Orders */}
          <div className="mb-4">
            <h3 className="text-xs font-medium text-green-400 mb-2">
              Buy Orders
            </h3>
            <div className="space-y-1">
              {[
                { price: "43,245.00", amount: "0.125", total: "5,405.63" },
                { price: "43,240.00", amount: "0.250", total: "10,810.00" },
                { price: "43,235.00", amount: "0.089", total: "3,847.92" },
                { price: "43,230.00", amount: "0.456", total: "19,712.88" },
              ].map((order, i) => (
                <div key={i} className="flex justify-between text-xs">
                  <span className="text-green-400">{order.price}</span>
                  <span className="text-gray-400">{order.amount}</span>
                  <span className="text-gray-500">{order.total}</span>
                </div>
              ))}
            </div>
          </div>

          {/* Sell Orders */}
          <div>
            <h3 className="text-xs font-medium text-red-400 mb-2">
              Sell Orders
            </h3>
            <div className="space-y-1">
              {[
                { price: "43,255.00", amount: "0.234", total: "10,121.67" },
                { price: "43,260.00", amount: "0.178", total: "7,700.28" },
                { price: "43,265.00", amount: "0.345", total: "14,926.43" },
                { price: "43,270.00", amount: "0.567", total: "24,524.19" },
              ].map((order, i) => (
                <div key={i} className="flex justify-between text-xs">
                  <span className="text-red-400">{order.price}</span>
                  <span className="text-gray-400">{order.amount}</span>
                  <span className="text-gray-500">{order.total}</span>
                </div>
              ))}
            </div>
          </div>
        </aside>
      </div>

      {/* Bottom Panel */}
      <footer className="bg-black border-t border-gray-800 p-3">
        <div className="flex justify-between items-center">
          <div className="flex space-x-6">
            <div>
              <span className="text-xs text-gray-500">Portfolio Value: </span>
              <span className="text-sm font-medium text-green-400">
                ${portfolioValue.toFixed(2)}
              </span>
            </div>
            <div>
              <span className="text-xs text-gray-500">24h Change: </span>
              <span className="text-sm font-medium text-gray-400">N/A</span>
            </div>
            <div>
              <span className="text-xs text-gray-500">USDT Balance: </span>
              <span className="text-sm font-medium text-gray-200">
                ${(sessionData?.balance?.USDT || 0).toFixed(2)}
              </span>
            </div>
          </div>

          <div className="flex space-x-3">
            <button className="bg-green-600 hover:bg-green-700 px-3 py-1.5 rounded text-xs font-medium">
              Buy
            </button>
            <button className="bg-red-600 hover:bg-red-700 px-3 py-1.5 rounded text-xs font-medium">
              Sell
            </button>
            <button className="bg-gray-700 hover:bg-gray-600 px-3 py-1.5 rounded text-xs font-medium">
              Settings
            </button>
          </div>
        </div>
      </footer>
    </div>
  );
}
