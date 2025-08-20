import {
  // AreaSeries,
  createChart,
  CandlestickSeries,
  ColorType,
} from "lightweight-charts";
import type { Time } from "lightweight-charts";
import { useEffect, useRef } from "react";

let randomFactor = 25 + Math.random() * 25;
const samplePoint = (i: number) =>
  i *
    (0.5 +
      Math.sin(i / 1) * 0.2 +
      Math.sin(i / 2) * 0.4 +
      Math.sin(i / randomFactor) * 0.8 +
      Math.sin(i / 50) * 0.5) +
  200 +
  i * 2;

type Candle = {
  time: Time;
  open: number;
  high: number;
  low: number;
  close: number;
};

function generateData(
  numberOfCandles = 500,
  updatesPerCandle = 5,
  startAt = 100
): { initialData: Candle[]; realtimeUpdates: Candle[] } {
  const createCandle = (val: number, time: number): Candle => ({
    time: time as Time,
    open: val,
    high: val,
    low: val,
    close: val,
  });

  const updateCandle = (candle: Candle, val: number): Candle => ({
    time: candle.time,
    close: val,
    open: candle.open,
    low: Math.min(candle.low, val),
    high: Math.max(candle.high, val),
  });

  randomFactor = 25 + Math.random() * 25;
  const date = new Date(Date.UTC(2018, 0, 1, 12, 0, 0, 0));
  const numberOfPoints = numberOfCandles * updatesPerCandle;
  const initialData: Candle[] = [];
  const realtimeUpdates: Candle[] = [];
  let lastCandle: Candle;
  let currentTime = date.getTime() / 1000;

  let previousValue = samplePoint(-1);
  for (let i = 0; i < numberOfPoints; ++i) {
    // Increment time for each data point to ensure unique timestamps
    if (i % updatesPerCandle === 0) {
      // New candle - increment by one day
      currentTime += 86400; // 24 * 60 * 60 seconds
    } else {
      // Update to existing candle - increment by small amount to maintain order
      currentTime += 60; // 1 minute
    }

    let value = samplePoint(i);
    const diff = (value - previousValue) * Math.random();
    value = previousValue + diff;
    previousValue = value;

    if (i % updatesPerCandle === 0) {
      const candle = createCandle(value, currentTime);
      lastCandle = candle;
      if (i < startAt) {
        initialData.push(candle);
      } else {
        realtimeUpdates.push(candle);
      }
    } else {
      const newCandle = updateCandle(lastCandle!, value);
      // Update the time for the candle update
      newCandle.time = currentTime as Time;
      lastCandle = newCandle;
      if (i >= startAt) {
        realtimeUpdates.push(newCandle);
      } else if ((i + 1) % updatesPerCandle === 0) {
        // Replace the last candle in initialData with the updated one
        initialData[initialData.length - 1] = newCandle;
      }
    }
  }

  return {
    initialData,
    realtimeUpdates,
  };
}

export const ChartComponent = (props: {
  colors: {
    backgroundColor: string;
    lineColor: string;
    textColor: string;
    areaTopColor: string;
    areaBottomColor: string;
  };
}) => {
  const {
    colors: {
      backgroundColor = "black",
      lineColor = "#2962FF",
      textColor = "white",
      areaTopColor = "#2962FF",
      areaBottomColor = "rgba(41, 98, 255, 0.28)",
    } = {},
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

    const data = generateData(2500, 20, 1000);

    series.setData(data.initialData);

    // const chartOptions = {
    //   layout: {
    //     textColor: "white",
    //     background: { type: "solid", color: "black" },
    //   },
    //   height: 200,
    // };

    chart.timeScale().fitContent();
    chart.timeScale().scrollToPosition(5, true);

    // simulate real-time data
    function* getNextRealtimeUpdate(realtimeData: Candle[]) {
      for (const dataPoint of realtimeData) {
        yield dataPoint;
      }
      return null;
    }
    const streamingDataProvider = getNextRealtimeUpdate(data.realtimeUpdates);

    const intervalID = setInterval(() => {
      const update = streamingDataProvider.next();
      if (update.done) {
        clearInterval(intervalID);
        return;
      }
      series.update(update.value);
    }, 100);

    window.addEventListener("resize", handleResize);

    return () => {
      window.removeEventListener("resize", handleResize);

      chart.remove();
    };
  }, [backgroundColor, lineColor, textColor, areaTopColor, areaBottomColor]);

  return <div ref={chartContainerRef} className="w-full h-full" />;
};

export default function App() {
  return (
    <div className="h-screen w-screen bg-black text-white flex flex-col">
      {/* Header */}
      <header className="bg-black p-3 border-b border-gray-800">
        <h1 className="text-xl font-medium text-center text-gray-200">
          Trading Dashboard
        </h1>
      </header>

      {/* Main Content Area */}
      <div className="flex-1 flex">
        {/* Left Sidebar */}
        <aside className="w-56 bg-black border-r border-gray-800 p-3">
          <h2 className="text-sm font-medium mb-3 text-gray-300">
            Market Overview
          </h2>
          <div className="space-y-2">
            <div className="bg-gray-900 p-2 rounded">
              <div className="text-xs text-gray-400">BTC/USD</div>
              <div className="text-green-400 font-medium text-sm">
                $43,250.00
              </div>
              <div className="text-xs text-green-400">+2.34%</div>
            </div>
            <div className="bg-gray-900 p-2 rounded">
              <div className="text-xs text-gray-400">ETH/USD</div>
              <div className="text-red-400 font-medium text-sm">$2,890.50</div>
              <div className="text-xs text-red-400">-1.22%</div>
            </div>
            <div className="bg-gray-900 p-2 rounded">
              <div className="text-xs text-gray-400">SOL/USD</div>
              <div className="text-green-400 font-medium text-sm">$98.75</div>
              <div className="text-xs text-green-400">+5.67%</div>
            </div>
          </div>
        </aside>

        {/* Center - Chart Area */}
        <main className="flex-1 bg-black">
          <ChartComponent
            colors={{
              backgroundColor: "#000000",
              lineColor: "#ffffff",
              textColor: "#ffffff",
              areaTopColor: "#000000",
              areaBottomColor: "#000000",
            }}
          />
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
                $15,847.32
              </span>
            </div>
            <div>
              <span className="text-xs text-gray-500">24h Change: </span>
              <span className="text-sm font-medium text-green-400">
                +$342.18 (+2.21%)
              </span>
            </div>
            <div>
              <span className="text-xs text-gray-500">Available Balance: </span>
              <span className="text-sm font-medium text-gray-200">
                $2,156.89
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
