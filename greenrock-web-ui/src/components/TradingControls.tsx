import { useState, useEffect, useCallback } from "react";
import { fetchOpenOrders, fetchTradeHistory } from "../utils/core";
import type { Order, Trade } from "../types/core";

interface TradingControlsProps {
  symbol: string;
  onClose: () => void;
}

export function TradingControls({ symbol, onClose }: TradingControlsProps) {
  const [openOrders, setOpenOrders] = useState<Order[]>([]);
  const [tradeHistory, setTradeHistory] = useState<Trade[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<"orders" | "history">("orders");

  const loadData = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);

      const [orders, history] = await Promise.all([
        fetchOpenOrders(symbol),
        fetchTradeHistory(symbol),
      ]);

      setOpenOrders(orders);
      setTradeHistory(history);
    } catch (err) {
      setError(
        err instanceof Error ? err.message : "Failed to load trading data"
      );
      console.error("Failed to fetch trading data:", err);
    } finally {
      setLoading(false);
    }
  }, [symbol]);

  useEffect(() => {
    loadData();
  }, [loadData]);

  return (
    <div className="flex flex-col h-full bg-black text-white">
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b border-neutral-800">
        <div className="flex items-center space-x-3">
          <h2 className="text-lg font-medium">Trading Dashboard</h2>
          <span className="text-xs text-neutral-400 bg-neutral-900 px-2 py-1 rounded">
            {symbol}
          </span>
        </div>
        <div className="flex items-center space-x-2">
          <button
            onClick={loadData}
            disabled={loading}
            className="text-xs bg-blue-600 hover:bg-blue-700 disabled:bg-neutral-700 disabled:text-neutral-400 px-2 py-1 rounded transition-colors"
          >
            {loading ? "Refreshing..." : "Refresh"}
          </button>
          <button
            onClick={onClose}
            className="text-xs bg-rose-600 hover:bg-rose-700 px-2 py-1 rounded transition-colors"
          >
            Close
          </button>
        </div>
      </div>

      {/* Tabs */}
      <div className="flex border-b border-neutral-800">
        <button
          onClick={() => setActiveTab("orders")}
          className={`flex-1 px-4 py-2 text-sm font-medium transition-colors ${
            activeTab === "orders"
              ? "bg-neutral-900 text-white border-b-2 border-blue-500"
              : "text-neutral-400 hover:text-white hover:bg-neutral-900"
          }`}
        >
          Open Orders ({openOrders.length})
        </button>
        <button
          onClick={() => setActiveTab("history")}
          className={`flex-1 px-4 py-2 text-sm font-medium transition-colors ${
            activeTab === "history"
              ? "bg-neutral-900 text-white border-b-2 border-blue-500"
              : "text-neutral-400 hover:text-white hover:bg-neutral-900"
          }`}
        >
          Trade History ({tradeHistory.length})
        </button>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto p-4">
        {error ? (
          <div className="text-rose-400 text-center p-4">
            <div className="text-2xl mb-2">⚠️</div>
            <p className="font-medium">Error loading data</p>
            <p className="text-sm text-neutral-400 mt-1">{error}</p>
          </div>
        ) : loading ? (
          <div className="text-neutral-400 text-center p-4">
            <div className="text-2xl mb-2">⏳</div>
            <p>Loading trading data...</p>
          </div>
        ) : activeTab === "orders" ? (
          <div className="space-y-3">
            {openOrders.length === 0 ? (
              <div className="text-center text-neutral-500 p-8">
                <div className="text-4xl mb-3">📋</div>
                <p className="text-lg font-medium">No open orders</p>
                <p className="text-sm text-neutral-400 mt-1">
                  All orders have been filled or cancelled
                </p>
              </div>
            ) : (
              openOrders.map((order) => (
                <div
                  key={order.id}
                  className="bg-neutral-900 p-4 rounded-lg border border-neutral-800"
                >
                  <div className="flex justify-between items-start mb-3">
                    <div>
                      <span
                        className={`text-sm font-medium ${
                          order.side === "buy"
                            ? "text-emerald-400"
                            : "text-rose-400"
                        }`}
                      >
                        {order.side.toUpperCase()} {order.symbol}
                      </span>
                      <span className="ml-2 text-xs bg-neutral-800 px-2 py-1 rounded">
                        {order.type.toUpperCase()}
                      </span>
                    </div>
                    <span
                      className={`text-xs px-2 py-1 rounded ${
                        order.status === "open"
                          ? "bg-blue-600"
                          : "bg-neutral-700"
                      }`}
                    >
                      {order.status.toUpperCase()}
                    </span>
                  </div>
                  <div className="grid grid-cols-2 gap-3 text-sm">
                    <div>
                      <span className="text-neutral-400">Quantity: </span>
                      <span className="text-white font-mono">
                        {order.quantity.toFixed(6)}
                      </span>
                    </div>
                    {order.price && (
                      <div>
                        <span className="text-neutral-400">Price: </span>
                        <span className="text-white font-mono">
                          ${order.price.toLocaleString()}
                        </span>
                      </div>
                    )}
                  </div>
                  <div className="text-xs text-neutral-500 mt-3 pt-2 border-t border-neutral-800">
                    ID: {order.id} •{" "}
                    {new Date(order.timestamp).toLocaleString()}
                  </div>
                </div>
              ))
            )}
          </div>
        ) : (
          <div className="space-y-3">
            {tradeHistory.length === 0 ? (
              <div className="text-center text-neutral-500 p-8">
                <div className="text-4xl mb-3">📊</div>
                <p className="text-lg font-medium">No trade history</p>
                <p className="text-sm text-neutral-400 mt-1">
                  Recent trades will appear here
                </p>
              </div>
            ) : (
              tradeHistory.slice(0, 50).map((trade) => (
                <div
                  key={trade.id}
                  className="bg-neutral-900 p-4 rounded-lg border border-neutral-800"
                >
                  <div className="flex justify-between items-start mb-3">
                    <div>
                      <span
                        className={`text-sm font-medium ${
                          trade.side === "buy"
                            ? "text-emerald-400"
                            : "text-rose-400"
                        }`}
                      >
                        {trade.side.toUpperCase()} {trade.symbol}
                      </span>
                    </div>
                    <span className="text-sm text-white font-mono">
                      ${(trade.price * trade.quantity).toFixed(2)}
                    </span>
                  </div>
                  <div className="grid grid-cols-2 gap-3 text-sm">
                    <div>
                      <span className="text-neutral-400">Quantity: </span>
                      <span className="text-white font-mono">
                        {trade.quantity.toFixed(6)}
                      </span>
                    </div>
                    <div>
                      <span className="text-neutral-400">Price: </span>
                      <span className="text-white font-mono">
                        ${trade.price.toLocaleString()}
                      </span>
                    </div>
                  </div>
                  {trade.fee && (
                    <div className="text-xs text-neutral-400 mt-2">
                      <span>Fee: </span>
                      <span className="font-mono">${trade.fee.toFixed(6)}</span>
                    </div>
                  )}
                  <div className="text-xs text-neutral-500 mt-3 pt-2 border-t border-neutral-800">
                    ID: {trade.id} •{" "}
                    {new Date(trade.timestamp).toLocaleString()}
                  </div>
                </div>
              ))
            )}
          </div>
        )}
      </div>

      {/* Quick Actions */}
      <div className="p-4 border-t border-neutral-800">
        <div className="flex space-x-2">
          <button className="flex-1 bg-emerald-600 hover:bg-emerald-700 py-3 rounded text-sm font-medium transition-colors">
            Place Buy Order
          </button>
          <button className="flex-1 bg-rose-600 hover:bg-rose-700 py-3 rounded text-sm font-medium transition-colors">
            Place Sell Order
          </button>
        </div>
        <div className="text-xs text-neutral-500 text-center mt-3">
          Manual trading controls (coming soon)
        </div>
      </div>
    </div>
  );
}
