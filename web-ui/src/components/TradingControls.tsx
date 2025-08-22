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
    <div className="flex flex-col h-full bg-gray-900 text-white">
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b border-gray-700">
        <div className="flex items-center space-x-3">
          <h2 className="text-lg font-medium">Trading Dashboard</h2>
          <span className="text-xs text-gray-400 bg-gray-800 px-2 py-1 rounded">
            {symbol}
          </span>
        </div>
        <div className="flex items-center space-x-2">
          <button
            onClick={loadData}
            disabled={loading}
            className="text-xs bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 px-2 py-1 rounded"
          >
            {loading ? "Refreshing..." : "Refresh"}
          </button>
          <button
            onClick={onClose}
            className="text-xs bg-red-600 hover:bg-red-700 px-2 py-1 rounded"
          >
            Close
          </button>
        </div>
      </div>

      {/* Tabs */}
      <div className="flex border-b border-gray-700">
        <button
          onClick={() => setActiveTab("orders")}
          className={`flex-1 px-4 py-2 text-sm font-medium ${
            activeTab === "orders"
              ? "bg-gray-800 text-white border-b-2 border-blue-500"
              : "text-gray-400 hover:text-white"
          }`}
        >
          Open Orders ({openOrders.length})
        </button>
        <button
          onClick={() => setActiveTab("history")}
          className={`flex-1 px-4 py-2 text-sm font-medium ${
            activeTab === "history"
              ? "bg-gray-800 text-white border-b-2 border-blue-500"
              : "text-gray-400 hover:text-white"
          }`}
        >
          Trade History ({tradeHistory.length})
        </button>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto p-4">
        {error ? (
          <div className="text-red-400 text-center">Error: {error}</div>
        ) : loading ? (
          <div className="text-gray-400 text-center">Loading...</div>
        ) : activeTab === "orders" ? (
          <div className="space-y-3">
            {openOrders.length === 0 ? (
              <div className="text-center text-gray-500">
                <div className="text-2xl mb-2">📋</div>
                <p>No open orders</p>
              </div>
            ) : (
              openOrders.map((order) => (
                <div key={order.id} className="bg-gray-800 p-3 rounded-lg">
                  <div className="flex justify-between items-start mb-2">
                    <div>
                      <span
                        className={`text-sm font-medium ${
                          order.side === "buy"
                            ? "text-green-400"
                            : "text-red-400"
                        }`}
                      >
                        {order.side.toUpperCase()} {order.symbol}
                      </span>
                      <span className="ml-2 text-xs bg-gray-700 px-2 py-1 rounded">
                        {order.type.toUpperCase()}
                      </span>
                    </div>
                    <span
                      className={`text-xs px-2 py-1 rounded ${
                        order.status === "open" ? "bg-blue-600" : "bg-gray-600"
                      }`}
                    >
                      {order.status.toUpperCase()}
                    </span>
                  </div>
                  <div className="grid grid-cols-2 gap-2 text-sm">
                    <div>
                      <span className="text-gray-400">Quantity: </span>
                      <span className="text-white">
                        {order.quantity.toFixed(6)}
                      </span>
                    </div>
                    {order.price && (
                      <div>
                        <span className="text-gray-400">Price: </span>
                        <span className="text-white">
                          ${order.price.toLocaleString()}
                        </span>
                      </div>
                    )}
                  </div>
                  <div className="text-xs text-gray-500 mt-2">
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
              <div className="text-center text-gray-500">
                <div className="text-2xl mb-2">📊</div>
                <p>No trade history</p>
              </div>
            ) : (
              tradeHistory.slice(0, 50).map((trade) => (
                <div key={trade.id} className="bg-gray-800 p-3 rounded-lg">
                  <div className="flex justify-between items-start mb-2">
                    <div>
                      <span
                        className={`text-sm font-medium ${
                          trade.side === "buy"
                            ? "text-green-400"
                            : "text-red-400"
                        }`}
                      >
                        {trade.side.toUpperCase()} {trade.symbol}
                      </span>
                    </div>
                    <span className="text-sm text-white">
                      ${(trade.price * trade.quantity).toFixed(2)}
                    </span>
                  </div>
                  <div className="grid grid-cols-2 gap-2 text-sm">
                    <div>
                      <span className="text-gray-400">Quantity: </span>
                      <span className="text-white">
                        {trade.quantity.toFixed(6)}
                      </span>
                    </div>
                    <div>
                      <span className="text-gray-400">Price: </span>
                      <span className="text-white">
                        ${trade.price.toLocaleString()}
                      </span>
                    </div>
                  </div>
                  {trade.fee && (
                    <div className="text-xs text-gray-400 mt-1">
                      Fee: ${trade.fee.toFixed(4)}
                    </div>
                  )}
                  <div className="text-xs text-gray-500 mt-2">
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
      <div className="p-4 border-t border-gray-700">
        <div className="flex space-x-2">
          <button className="flex-1 bg-green-600 hover:bg-green-700 py-2 rounded text-sm font-medium">
            Place Buy Order
          </button>
          <button className="flex-1 bg-red-600 hover:bg-red-700 py-2 rounded text-sm font-medium">
            Place Sell Order
          </button>
        </div>
        <div className="text-xs text-gray-500 text-center mt-2">
          Manual trading controls (coming soon)
        </div>
      </div>
    </div>
  );
}
