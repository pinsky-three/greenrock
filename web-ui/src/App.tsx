import type { ISeriesApi } from "lightweight-charts";
import { useEffect, useRef, useState, useCallback, useMemo } from "react";
import type {
  ApiCandle,
  Balance,
  OrderBook,
  // Portfolio,
} from "./types/core";
import { ChartComponent } from "./components/Chart";
import { ChatComponent } from "./components/Chat";
import { TradingControls } from "./components/TradingControls";
import { ErrorBoundary } from "./components/ErrorBoundary";
// Import react-icons
import {
  IoSearch,
  IoEllipsisVertical,
  IoTrendingUp,
  IoNotifications,
  IoSettings,
  IoRefresh,
  IoChatbubble,
  IoBarChart,
  IoWallet,
} from "react-icons/io5";
import { RiFullscreenLine, RiBarChartLine } from "react-icons/ri";
import { TbChartCandle, TbChartLine, TbChartArea } from "react-icons/tb";
import {
  convertApiCandlesToChart,
  createCandleStreamWebSocket,
  // createOrderBookStreamWebSocket,
  fetchBalance,
  fetchCandles,
  fetchOrderBook,
  // fetchPortfolio,
} from "./utils/core";

export default function App() {
  // New state for the updated API
  const [balance, setBalance] = useState<Balance | null>(null);
  // const [portfolio, setPortfolio] = useState<Portfolio | null>(null);
  const [orderBook, setOrderBook] = useState<OrderBook | null>(null);
  const [candles, setCandles] = useState<ApiCandle[]>([]);

  // UI state
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isCandleStreamConnected, setIsCandleStreamConnected] = useState(false);
  const [isOrderBookStreamConnected, setIsOrderBookStreamConnected] =
    useState(false);
  const [showChat, setShowChat] = useState(false);
  const [showTrading, setShowTrading] = useState(false);
  const [leftSidebarCollapsed, setLeftSidebarCollapsed] = useState(false);
  const [rightSidebarCollapsed, setRightSidebarCollapsed] = useState(false);
  const [candlestickSeries, setCandlestickSeries] =
    useState<ISeriesApi<"Candlestick"> | null>(null);

  // Trading configuration
  const [symbol] = useState("BTCUSDT");
  const [interval] = useState("1m");

  // Time range state - disabled for debugging
  // const [selectedTimeRange, setSelectedTimeRange] = useState<TimeRange>(() => {
  //   const presets = getTimeRangePresets();
  //   return { start: presets["1d"].start, end: presets["1d"].end };
  // });
  // const [timeRangePreset, setTimeRangePreset] = useState<string>("1d");

  // WebSocket refs
  const candleWsRef = useRef<WebSocket | null>(null);
  const orderBookWsRef = useRef<WebSocket | null>(null);
  const isMountedRef = useRef(true);
  const isLoadingDataRef = useRef(false);

  // Legacy state for backward compatibility (kept for potential future use)
  // const [sessionData, setSessionData] = useState<LatestSessionResponse | null>(
  //   null
  // );

  // Load data function using new API endpoints
  const loadData = useCallback(async () => {
    if (isLoadingDataRef.current) {
      // console.log("DEBUG: loadData already in progress, skipping");
      return;
    }

    // console.log("DEBUG: loadData called");
    isLoadingDataRef.current = true;

    try {
      setLoading(true);
      setError(null);

      // Fetch data from new endpoints in parallel
      const [balanceData, candlesData, orderBookData] = await Promise.all([
        fetchBalance(),
        // fetchPortfolio(),
        fetchCandles({ symbol, interval }),
        fetchOrderBook({ symbol, depth: 10 }),
      ]);

      // console.log("DEBUG: Fetched", candlesData.candles.length, "candles");
      setBalance(balanceData);
      // setPortfolio(portfolioData);
      setCandles(candlesData.candles);
      setOrderBook(orderBookData);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load data");
      console.error("Failed to fetch data:", err);
    } finally {
      setLoading(false);
      isLoadingDataRef.current = false;
    }
  }, [symbol, interval]);

  // Candle stream WebSocket connection management
  const connectCandleStream = useCallback(() => {
    if (
      candleWsRef.current?.readyState === WebSocket.OPEN ||
      candleWsRef.current?.readyState === WebSocket.CONNECTING
    ) {
      return; // Already connected or connecting
    }

    // Clean up any existing connection
    if (candleWsRef.current) {
      candleWsRef.current.close();
      candleWsRef.current = null;
    }

    try {
      const ws = createCandleStreamWebSocket();
      candleWsRef.current = ws;

      ws.onopen = () => {
        // console.log("Candle stream WebSocket connected");
        if (isMountedRef.current) {
          setIsCandleStreamConnected(true);
        }
      };

      ws.onmessage = (event) => {
        try {
          const apiCandle: ApiCandle = JSON.parse(event.data);
          // console.log("Received candle:", apiCandle);

          // Update candles state
          setCandles((prev) => {
            const newCandles = [...prev];
            // Replace the last candle if it has the same timestamp, otherwise add new one
            const lastIndex = newCandles.length - 1;
            if (
              lastIndex >= 0 &&
              newCandles[lastIndex].timestamp === apiCandle.timestamp
            ) {
              newCandles[lastIndex] = apiCandle;
            } else {
              newCandles.push(apiCandle);
            }
            return newCandles;
          });

          // Real-time updates will be handled through state changes
          // The Chart component will receive the updated candleData prop
        } catch (err) {
          console.error("Failed to parse candle WebSocket message:", err);
        }
      };

      ws.onerror = (error) => {
        console.error("Candle stream WebSocket error:", error);
      };

      ws.onclose = (event) => {
        // console.log(
        //   "Candle stream WebSocket disconnected:",
        //   event.code,
        //   event.reason
        // );

        if (isMountedRef.current) {
          setIsCandleStreamConnected(false);
        }

        // Clear the reference if this was the current connection
        if (candleWsRef.current === ws) {
          candleWsRef.current = null;
        }

        // Auto-reconnect if not a manual disconnection
        if (
          event.code !== 1000 &&
          event.code !== 1001 &&
          isMountedRef.current
        ) {
          setTimeout(() => {
            if (
              !candleWsRef.current &&
              candlestickSeries &&
              isMountedRef.current
            ) {
              connectCandleStream();
            }
          }, 3000);
        }
      };
    } catch (err) {
      console.error(
        "Failed to create candle stream WebSocket connection:",
        err
      );
    }
  }, [candlestickSeries]);

  // Order book stream WebSocket connection management
  const connectOrderBookStream = useCallback(() => {
    if (
      orderBookWsRef.current?.readyState === WebSocket.OPEN ||
      orderBookWsRef.current?.readyState === WebSocket.CONNECTING
    ) {
      return; // Already connected or connecting
    }

    // Clean up any existing connection
    if (orderBookWsRef.current) {
      orderBookWsRef.current.close();
      orderBookWsRef.current = null;
    }

    // try {
    //   const ws = createOrderBookStreamWebSocket();
    //   orderBookWsRef.current = ws;

    //   ws.onopen = () => {
    //     console.log("Order book stream WebSocket connected");
    //     if (isMountedRef.current) {
    //       setIsOrderBookStreamConnected(true);
    //       setError(null); // Clear any previous connection errors
    //     }
    //   };

    //   ws.onmessage = (event) => {
    //     try {
    //       const orderBookData: OrderBook = JSON.parse(event.data);
    //       console.log("Received order book:", orderBookData);

    //       // Validate order book data structure
    //       if (orderBookData && typeof orderBookData === "object") {
    //         // Ensure arrays exist and have proper structure
    //         const validatedOrderBook: OrderBook = {
    //           symbol: orderBookData.symbol || symbol,
    //           bids: Array.isArray(orderBookData.bids) ? orderBookData.bids : [],
    //           asks: Array.isArray(orderBookData.asks) ? orderBookData.asks : [],
    //           timestamp: orderBookData.timestamp || Date.now(),
    //         };
    //         setOrderBook(validatedOrderBook);
    //       }
    //     } catch (err) {
    //       console.error("Failed to parse order book WebSocket message:", err);
    //     }
    //   };

    //   ws.onerror = (error) => {
    //     console.error("Order book stream WebSocket error:", error);
    //     if (isMountedRef.current) {
    //       // Don't set error state immediately, wait for close event
    //     }
    //   };

    //   ws.onclose = (event) => {
    //     console.log(
    //       "Order book stream WebSocket disconnected:",
    //       event.code,
    //       event.reason
    //     );

    //     if (isMountedRef.current) {
    //       setIsOrderBookStreamConnected(false);

    //       // Only set error if it's not a clean close
    //       if (event.code !== 1000 && event.code !== 1001) {
    //         console.warn(
    //           "Order book stream disconnected unexpectedly, will retry"
    //         );
    //       }
    //     }

    //     // Clear the reference if this was the current connection
    //     if (orderBookWsRef.current === ws) {
    //       orderBookWsRef.current = null;
    //     }

    //     // Auto-reconnect if not a manual disconnection and component is mounted
    //     if (
    //       event.code !== 1000 &&
    //       event.code !== 1001 &&
    //       isMountedRef.current
    //     ) {
    //       setTimeout(() => {
    //         if (!orderBookWsRef.current && isMountedRef.current) {
    //           connectOrderBookStream();
    //         }
    //       }, 5000); // Longer delay for order book reconnection
    //     }
    //   };
    // } catch (err) {
    //   console.error(
    //     "Failed to create order book stream WebSocket connection:",
    //     err
    //   );
    //   if (isMountedRef.current) {
    //     setError("Failed to connect to order book stream");
    //   }
    // }
  }, []);

  // Disconnect WebSocket streams
  const disconnectWebSockets = useCallback(() => {
    if (candleWsRef.current) {
      candleWsRef.current.close(1000, "Manual disconnect");
      candleWsRef.current = null;
      setIsCandleStreamConnected(false);
    }
    if (orderBookWsRef.current) {
      orderBookWsRef.current.close(1000, "Manual disconnect");
      orderBookWsRef.current = null;
      setIsOrderBookStreamConnected(false);
    }
  }, []);

  // Fetch data on component mount and set up auto-refresh
  // useEffect(() => {
  //   loadData();

  //   // Auto-refresh every 30 minutes
  //   const interval = setInterval(loadData, 30 * 60_000);

  //   return () => clearInterval(interval);
  // }, [loadData]);

  useEffect(() => {
    console.log("DEBUG: Component mounted, calling loadData");
    loadData();
  }, [loadData]);

  // Handle WebSocket connections when chart series is ready
  useEffect(() => {
    // Add a small delay to ensure the backend is ready
    const connectTimer = setTimeout(() => {
      if (candlestickSeries && !candleWsRef.current) {
        connectCandleStream();
      }
      if (!orderBookWsRef.current) {
        connectOrderBookStream();
      }
    }, 1000); // 1 second delay

    return () => clearTimeout(connectTimer);
  }, [candlestickSeries, connectCandleStream, connectOrderBookStream]);

  // Cleanup WebSocket on component unmount
  useEffect(() => {
    return () => {
      isMountedRef.current = false;
      disconnectWebSockets();
    };
  }, [disconnectWebSockets]);

  // Handle series ready callback
  const handleSeriesReady = useCallback(
    (series: ISeriesApi<"Candlestick">) => {
      if (!candlestickSeries) {
        setCandlestickSeries(series);
      }
    },
    [candlestickSeries]
  );

  // Handle time range selection - disabled for debugging
  // const handleTimeRangeChange = useCallback((preset: string) => {
  //   const presets = getTimeRangePresets();
  //   const selectedPreset = presets[preset];
  //   if (selectedPreset) {
  //     setSelectedTimeRange({
  //       start: selectedPreset.start,
  //       end: selectedPreset.end,
  //     });
  //     setTimeRangePreset(preset);
  //   }
  // }, []);

  // Handle custom time range
  // const handleCustomTimeRange = useCallback((start: Date, end: Date) => {
  //   setSelectedTimeRange({ start, end });
  //   setTimeRangePreset("custom");
  // }, []);

  // Convert API candles to chart format - NO TIME FILTERING for debugging
  const chartData = useMemo(() => {
    if (candles.length === 0) return undefined;

    const convertedCandles = convertApiCandlesToChart(candles);

    // console.log(
    //   `DEBUG: ${candles.length} raw candles → ${convertedCandles.length} converted (NO FILTERING)`
    // );
    // console.log("First 3 converted candles:", convertedCandles.slice(0, 3));
    // console.log("Last 3 converted candles:", convertedCandles.slice(-3));

    return convertedCandles;
  }, [candles]);

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

  const portfolioValue = balance ? calculatePortfolioValue(balance) : 0;
  const isConnected = isCandleStreamConnected && isOrderBookStreamConnected;

  return (
    <ErrorBoundary>
      <div className="h-screen w-screen bg-neutral-950 text-white flex flex-col">
        {/* TradingView Style Header */}
        <header className="bg-neutral-950 h-12 border-b border-neutral-800 flex items-center px-3">
          {/* Left: Logo + Symbol Search */}
          <div className="flex items-center space-x-3">
            {/* Logo placeholder */}
            <div className="w-6 h-6 bg-emerald-500 rounded flex items-center justify-center text-black font-bold ">
              G
            </div>

            {/* Symbol Search */}
            <div className="flex items-center bg-neutral-950 rounded border-0 hover:border-neutral-600 px-3 py-1.5">
              <IoSearch className="text-neutral-400 mr-2" size={16} />
              <span className="text-white font-medium">{symbol}</span>
              {/* <span className="text-yellow-400 ml-2 text-xs font-medium">
                BINANCE
              </span> */}
              {/* <span className="text-neutral-500 mx-1">•</span>
              <span className="text-neutral-400 text-xs">1</span>
              <span className="text-neutral-500 mx-1">•</span>
              <span className="text-neutral-400 text-xs uppercase">Crypto</span> */}
            </div>

            {/* Price Info */}
            {candles.length > 0 && (
              <div className="flex items-center space-x-3 ml-4">
                <div className="text-white font-medium text-lg">
                  {candles[candles.length - 1]?.close?.toLocaleString(
                    undefined,
                    {
                      minimumFractionDigits: 2,
                      maximumFractionDigits: 2,
                    }
                  )}
                </div>
                <div className="text-emerald-400 text-sm font-medium">
                  +649.90 (+0.45%)
                </div>
                {/* <div className="text-neutral-400 text-xs">
                  H: 113,300.00 L: 113,293.80 C: 113,293.80
                </div> */}
              </div>
            )}
          </div>

          {/* Center: Timeframe + Chart Tools */}
          <div className="flex-1 flex justify-center items-center space-x-4 h-full py-1">
            {/* Static Timeframe Display */}
            <div className="flex items-center space-x-1 h-full">
              <div className="px-2 h-full text-xs bg-neutral-800 text-white rounded flex items-center">
                1m (DEBUG: All Data)
              </div>
            </div>

            {/* Chart Type Tools */}
            <div className="flex items-center space-x-1 border-l border-neutral-700 pl-4">
              <button
                className="p-1.5 text-neutral-400 hover:text-white hover:bg-neutral-800 rounded"
                title="Candles"
              >
                <TbChartCandle size={16} />
              </button>
              <button
                className="p-1.5 text-neutral-400 hover:text-white hover:bg-neutral-800 rounded"
                title="Line"
              >
                <TbChartLine size={16} />
              </button>
              <button
                className="p-1.5 text-neutral-400 hover:text-white hover:bg-neutral-800 rounded"
                title="Area"
              >
                <TbChartArea size={16} />
              </button>
            </div>

            {/* Indicators */}
            <div className="flex items-center space-x-1 border-l border-neutral-700 pl-4">
              <button className="px-2 py-1 text-xs text-neutral-400 hover:text-white hover:bg-neutral-800 rounded">
                <IoTrendingUp className="inline mr-1" size={14} />
                Indicators
              </button>
            </div>
          </div>

          {/* Right: Tools and Settings */}
          <div className="flex items-center space-x-1">
            {/* Layout Controls */}
            <button
              onClick={() => setLeftSidebarCollapsed(!leftSidebarCollapsed)}
              className={`p-1.5 rounded ${
                leftSidebarCollapsed
                  ? "text-neutral-400 hover:text-white hover:bg-neutral-800"
                  : "text-blue-400 bg-neutral-800"
              }`}
              title="Watchlist"
            >
              <IoWallet size={16} />
            </button>

            <button
              onClick={() => setRightSidebarCollapsed(!rightSidebarCollapsed)}
              className={`p-1.5 rounded ${
                rightSidebarCollapsed
                  ? "text-neutral-400 hover:text-white hover:bg-neutral-800"
                  : "text-blue-400 bg-neutral-800"
              }`}
              title="Order Book"
            >
              <RiBarChartLine size={16} />
            </button>

            {/* Divider */}
            <div className="h-6 w-px bg-neutral-700 mx-2"></div>

            {/* Action Tools */}
            <button
              onClick={() => setShowChat(!showChat)}
              className={`p-1.5 rounded ${
                showChat
                  ? "text-blue-400 bg-neutral-800"
                  : "text-neutral-400 hover:text-white hover:bg-neutral-800"
              }`}
              title="AI Assistant"
            >
              <IoChatbubble size={16} />
            </button>

            <button
              onClick={() => setShowTrading(!showTrading)}
              className={`p-1.5 rounded ${
                showTrading
                  ? "text-blue-400 bg-neutral-800"
                  : "text-neutral-400 hover:text-white hover:bg-neutral-800"
              }`}
              title="Trading Panel"
            >
              <IoBarChart size={16} />
            </button>

            {/* Status */}
            <div className="flex items-center space-x-1 mx-2">
              <div
                className={`w-2 h-2 rounded-full ${
                  isConnected ? "bg-green-400" : "bg-red-400"
                }`}
              ></div>
              <span className="text-xs text-neutral-400">
                {isConnected ? "Live" : "Offline"}
              </span>
            </div>

            {/* More Tools */}
            <button
              className="p-1.5 text-neutral-400 hover:text-white hover:bg-neutral-800 rounded"
              title="Alerts"
            >
              <IoNotifications size={16} />
            </button>

            <button
              onClick={loadData}
              disabled={loading}
              className="p-1.5 text-neutral-400 hover:text-white hover:bg-neutral-800 rounded disabled:opacity-50"
              title="Refresh"
            >
              <IoRefresh size={16} />
            </button>

            <button
              className="p-1.5 text-neutral-400 hover:text-white hover:bg-neutral-800 rounded"
              title="Fullscreen"
            >
              <RiFullscreenLine size={16} />
            </button>

            <button
              className="p-1.5 text-neutral-400 hover:text-white hover:bg-neutral-800 rounded"
              title="Settings"
            >
              <IoSettings size={16} />
            </button>

            <button
              className="p-1.5 text-neutral-400 hover:text-white hover:bg-neutral-800 rounded"
              title="More"
            >
              <IoEllipsisVertical size={16} />
            </button>
          </div>
        </header>

        {/* Main Content Area */}
        <div className="flex-1 flex relative">
          {/* Collapsible Left Sidebar */}
          {!showChat && !showTrading && !leftSidebarCollapsed && (
            <aside className="w-64 bg-neutral-950 border-r border-neutral-800 flex flex-col">
              {/* Tab Headers */}
              <div className="flex border-b border-neutral-800">
                <button className="flex-1 px-3 py-2 text-xs bg-neutral-800 text-white border-r border-neutral-700">
                  Portfolio
                </button>
                <button className="flex-1 px-3 py-2 text-xs text-neutral-400 hover:text-white hover:bg-neutral-800">
                  Watchlist
                </button>
              </div>

              {/* Portfolio Content */}
              <div className="flex-1 p-3 overflow-y-auto">
                {/* Account Summary */}
                <div className="mb-4 p-3 bg-neutral-900 rounded">
                  <div className="text-xs text-neutral-400 mb-1">
                    Total Balance
                  </div>
                  <div className="text-lg font-medium text-white">
                    $
                    {portfolioValue.toLocaleString(undefined, {
                      minimumFractionDigits: 2,
                    })}
                  </div>
                  <div className="text-xs text-emerald-400">+2.34% today</div>
                </div>

                {/* Holdings */}
                <div className="space-y-2">
                  <div className="text-xs font-medium text-neutral-300 mb-2">
                    Holdings
                  </div>
                  {balance ? (
                    Object.entries(balance)
                      .filter(([, value]) => value > 0)
                      .map(([sym, amount]) => (
                        <div
                          key={sym}
                          className="flex justify-between items-center p-2 bg-neutral-900 rounded text-xs"
                        >
                          <div className="flex items-center space-x-2">
                            <div className="w-6 h-6 bg-yellow-500 rounded-full flex items-center justify-center text-black font-bold text-xs">
                              {sym.charAt(0)}
                            </div>
                            <div>
                              <div className="text-white font-medium">
                                {sym}
                              </div>
                              <div className="text-neutral-400">
                                {amount.toFixed(sym === "USDT" ? 2 : 6)}
                              </div>
                            </div>
                          </div>
                          <div className="text-right">
                            <div className="text-white">
                              $
                              {(sym === "USDT"
                                ? amount
                                : amount * 50000
                              ).toFixed(2)}
                            </div>
                            <div className="text-emerald-400">+1.2%</div>
                          </div>
                        </div>
                      ))
                  ) : (
                    <div className="text-neutral-500 text-xs">Loading...</div>
                  )}
                </div>
              </div>
            </aside>
          )}

          {/* Center - Chart Area, Chat, or Trading */}
          <main className="flex-1 bg-neutral-950 flex flex-col relative">
            {showChat ? (
              <ErrorBoundary>
                <ChatComponent onClose={() => setShowChat(false)} />
              </ErrorBoundary>
            ) : showTrading ? (
              <ErrorBoundary>
                <TradingControls
                  symbol={symbol}
                  onClose={() => setShowTrading(false)}
                />
              </ErrorBoundary>
            ) : (
              <ErrorBoundary>
                {error ? (
                  <div className="flex items-center justify-center h-full">
                    <div className="text-rose-400">Error: {error}</div>
                  </div>
                ) : loading ? (
                  <div className="flex items-center justify-center h-full">
                    <div className="text-neutral-400">
                      Loading chart data...
                    </div>
                  </div>
                ) : (
                  <>
                    <ChartComponent
                      // colors={{
                      //   backgroundColor: "#000000",
                      //   lineColor: "#ffffff",
                      //   textColor: "#ffffff",
                      //   areaTopColor: "#000000",
                      //   areaBottomColor: "#000000",
                      // }}
                      candleData={chartData}
                      onSeriesReady={handleSeriesReady}
                      // autoFitContent={true}
                    />

                    {/* Chart Overlay Info */}
                    {/* <div className="absolute top-4 left-4 z-10 bg-neutral-950 bg-opacity-50 rounded p-2">
                      <div className="text-xs space-y-1">
                        <div className="text-neutral-400">
                          Volume:{" "}
                          {candles.length > 0
                            ? candles[
                                candles.length - 1
                              ]?.volume?.toLocaleString() || "N/A"
                            : "N/A"}
                        </div>
                        <div className="text-neutral-400">
                          Candles: {chartData?.length || 0}
                        </div>
                        {isConnected && (
                          <div className="text-emerald-400 text-xs">
                            ● Live Data
                          </div>
                        )}
                      </div>
                    </div> */}
                  </>
                )}
              </ErrorBoundary>
            )}
          </main>

          {/* Collapsible Right Sidebar */}
          {!showChat && !showTrading && !rightSidebarCollapsed && (
            <aside className="w-72 bg-neutral-950 border-l border-neutral-800 flex flex-col">
              {/* Tab Headers */}
              <div className="flex border-b border-neutral-800">
                <button className="flex-1 px-3 py-2 text-xs bg-neutral-800 text-white border-r border-neutral-700">
                  Order Book
                </button>
                <button className="flex-1 px-3 py-2 text-xs text-neutral-400 hover:text-white hover:bg-neutral-800 border-r border-neutral-700">
                  Trades
                </button>
                <button className="flex-1 px-3 py-2 text-xs text-neutral-400 hover:text-white hover:bg-neutral-800">
                  Info
                </button>
              </div>

              {/* Order Book Content */}
              <div className="flex-1 p-3 overflow-y-auto">
                <ErrorBoundary>
                  {orderBook ? (
                    <div className="space-y-4">
                      {/* Market Price */}
                      <div className="text-center p-2 bg-neutral-900 rounded">
                        <div className="text-lg font-mono text-white">
                          {orderBook.bids?.[0]?.price?.toLocaleString(
                            undefined,
                            {
                              minimumFractionDigits: 2,
                              maximumFractionDigits: 2,
                            }
                          ) || "N/A"}
                        </div>
                        <div className="text-xs text-neutral-400">
                          Market Price (USD)
                        </div>
                      </div>

                      {/* Asks */}
                      <div>
                        <div className="flex justify-between text-xs text-neutral-400 mb-2 font-mono">
                          <span>Price</span>
                          <span>Size</span>
                          <span>Total</span>
                        </div>
                        <div className="space-y-1">
                          {orderBook.asks && orderBook.asks.length > 0 ? (
                            orderBook.asks
                              .slice(0, 8)
                              .reverse()
                              .filter(
                                (ask) =>
                                  ask &&
                                  typeof ask.price === "number" &&
                                  typeof ask.quantity === "number"
                              )
                              .map((ask, i) => (
                                <div
                                  key={i}
                                  className="flex justify-between text-xs font-mono hover:bg-neutral-800 p-1 rounded"
                                >
                                  <span className="text-rose-400">
                                    {ask.price.toLocaleString(undefined, {
                                      minimumFractionDigits: 2,
                                      maximumFractionDigits: 2,
                                    })}
                                  </span>
                                  <span className="text-neutral-300">
                                    {ask.quantity.toFixed(4)}
                                  </span>
                                  <span className="text-neutral-400">
                                    {(ask.price * ask.quantity).toFixed(0)}
                                  </span>
                                </div>
                              ))
                          ) : (
                            <div className="text-neutral-500 text-xs text-center py-4">
                              No sell orders
                            </div>
                          )}
                        </div>
                      </div>

                      {/* Spread */}
                      <div className="text-center py-2">
                        <div className="text-xs text-neutral-400">
                          Spread:{" "}
                          {orderBook.asks?.length > 0 &&
                          orderBook.bids?.length > 0 &&
                          orderBook.asks[0]?.price &&
                          orderBook.bids[0]?.price
                            ? `$${(
                                orderBook.asks[0].price -
                                orderBook.bids[0].price
                              ).toFixed(2)}`
                            : "N/A"}
                        </div>
                      </div>

                      {/* Bids */}
                      <div>
                        <div className="space-y-1">
                          {orderBook.bids && orderBook.bids.length > 0 ? (
                            orderBook.bids
                              .slice(0, 8)
                              .filter(
                                (bid) =>
                                  bid &&
                                  typeof bid.price === "number" &&
                                  typeof bid.quantity === "number"
                              )
                              .map((bid, i) => (
                                <div
                                  key={i}
                                  className="flex justify-between text-xs font-mono hover:bg-neutral-800 p-1 rounded"
                                >
                                  <span className="text-emerald-400">
                                    {bid.price.toLocaleString(undefined, {
                                      minimumFractionDigits: 2,
                                      maximumFractionDigits: 2,
                                    })}
                                  </span>
                                  <span className="text-neutral-300">
                                    {bid.quantity.toFixed(4)}
                                  </span>
                                  <span className="text-neutral-400">
                                    {(bid.price * bid.quantity).toFixed(0)}
                                  </span>
                                </div>
                              ))
                          ) : (
                            <div className="text-neutral-500 text-xs text-center py-4">
                              No buy orders
                            </div>
                          )}
                        </div>
                      </div>

                      {/* Status */}
                      <div className="pt-3 border-t border-neutral-700 text-xs text-neutral-400 text-center">
                        {isOrderBookStreamConnected ? (
                          <span className="text-emerald-400">Live Updates</span>
                        ) : (
                          <span className="text-rose-400"> Disconnected</span>
                        )}
                      </div>
                    </div>
                  ) : (
                    <div className="text-neutral-500 text-xs text-center py-8">
                      Loading order book...
                    </div>
                  )}
                </ErrorBoundary>
              </div>
            </aside>
          )}
        </div>

        {/* Bottom Toolbar like TradingView */}
        <footer className="bg-neutral-950 h-10 border-t border-neutral-800 flex items-center justify-between px-4">
          {/* Left: Status and Stats */}
          <div className="flex items-center space-x-6 text-xs">
            <div className="flex items-center space-x-2">
              <span className="text-neutral-400">Portfolio:</span>
              <span className="text-white font-medium">
                $
                {portfolioValue.toLocaleString(undefined, {
                  minimumFractionDigits: 2,
                })}
              </span>
              <span className="text-emerald-400">+2.34%</span>
            </div>
            <div className="flex items-center space-x-2">
              <span className="text-neutral-400">Balance:</span>
              <span className="text-white">
                ${(balance?.USDT || 0).toFixed(2)}
              </span>
            </div>
            {candles.length > 0 && (
              <div className="flex items-center space-x-2">
                <span className="text-neutral-400">Vol:</span>
                <span className="text-white">
                  {(candles[candles.length - 1]?.volume || 0).toLocaleString()}
                </span>
              </div>
            )}
          </div>

          {/* Center: Data Status */}
          <div className="flex-1 flex justify-center">
            <div className="flex items-center space-x-4 text-xs">
              {chartData && (
                <span className="text-neutral-400">
                  {chartData.length} candles loaded
                </span>
              )}
              <div className="flex items-center space-x-1">
                <div
                  className={`w-1.5 h-1.5 rounded-full ${
                    isCandleStreamConnected ? "bg-green-400" : "bg-red-400"
                  }`}
                ></div>
                <span className="text-neutral-400">Price</span>
              </div>
              <div className="flex items-center space-x-1">
                <div
                  className={`w-1.5 h-1.5 rounded-full ${
                    isOrderBookStreamConnected ? "bg-green-400" : "bg-red-400"
                  }`}
                ></div>
                <span className="text-neutral-400">Book</span>
              </div>
            </div>
          </div>

          {/* Right: Quick Actions */}
          {/* <div className="flex items-center space-x-2">
            <div className="flex bg-neutral-900 rounded overflow-hidden">
              <button
                onClick={() => setShowTrading(true)}
                className="px-4 py-1 text-xs bg-green-600 hover:bg-green-700 text-white font-medium"
              >
                BUY
              </button>
              <button
                onClick={() => setShowTrading(true)}
                className="px-4 py-1 text-xs bg-red-600 hover:bg-red-700 text-white font-medium"
              >
                SELL
              </button>
            </div>
            <button
              onClick={() => setShowChat(true)}
              className="px-3 py-1 text-xs bg-purple-600 hover:bg-purple-700 text-white rounded font-medium"
            >
              AI
            </button>
          </div> */}
        </footer>
      </div>
    </ErrorBoundary>
  );
}
