import { useState, useCallback, useRef, useEffect } from "react";
import { sendChatMessage } from "../utils/core";
import type { ChatRequest, ChatResponse, PauseResponse } from "../types/core";

interface ChatMessage {
  id: string;
  role: "user" | "assistant";
  content: string;
  timestamp: Date;
  status?: "completed" | "paused" | "error";
  sessionId?: string;
}

interface ChatComponentProps {
  onClose: () => void;
}

export function ChatComponent({ onClose }: ChatComponentProps) {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [inputValue, setInputValue] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [currentSessionId, setCurrentSessionId] = useState<string | null>(null);
  const [isMobile, setIsMobile] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const checkIsMobile = () => {
      setIsMobile(window.innerWidth < 768);
    };

    checkIsMobile();
    window.addEventListener("resize", checkIsMobile);

    return () => window.removeEventListener("resize", checkIsMobile);
  }, []);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleSendMessage = useCallback(async () => {
    if (!inputValue.trim() || isLoading) return;

    const userMessage: ChatMessage = {
      id: Date.now().toString(),
      role: "user",
      content: inputValue,
      timestamp: new Date(),
    };

    setMessages((prev) => [...prev, userMessage]);
    setInputValue("");
    setIsLoading(true);

    try {
      const request: ChatRequest = {
        query: inputValue,
        session_id: currentSessionId || undefined,
      };

      const response = await sendChatMessage(request);

      let assistantMessage: ChatMessage;

      if ("answer" in response) {
        // ChatResponse
        const chatResponse = response as ChatResponse;
        assistantMessage = {
          id: (Date.now() + 1).toString(),
          role: "assistant",
          content: chatResponse.answer,
          timestamp: new Date(),
          status: chatResponse.status as "completed",
          sessionId: chatResponse.session_id,
        };
        setCurrentSessionId(chatResponse.session_id);
      } else {
        // PauseResponse
        const pauseResponse = response as PauseResponse;
        assistantMessage = {
          id: (Date.now() + 1).toString(),
          role: "assistant",
          content: `Task paused: ${pauseResponse.reason}\nNext task: ${pauseResponse.next_task}`,
          timestamp: new Date(),
          status: "paused",
          sessionId: pauseResponse.session_id,
        };
        setCurrentSessionId(pauseResponse.session_id);
      }

      setMessages((prev) => [...prev, assistantMessage]);
    } catch (error) {
      const errorMessage: ChatMessage = {
        id: (Date.now() + 1).toString(),
        role: "assistant",
        content: `Error: ${
          error instanceof Error ? error.message : "Unknown error"
        }`,
        timestamp: new Date(),
        status: "error",
      };
      setMessages((prev) => [...prev, errorMessage]);
    } finally {
      setIsLoading(false);
    }
  }, [inputValue, isLoading, currentSessionId]);

  const handleKeyPress = useCallback(
    (event: React.KeyboardEvent) => {
      if (event.key === "Enter" && !event.shiftKey) {
        event.preventDefault();
        handleSendMessage();
      }
    },
    [handleSendMessage]
  );

  const clearChat = useCallback(() => {
    setMessages([]);
    setCurrentSessionId(null);
  }, []);

  return (
    <div className="flex flex-col h-full bg-neutral-900 text-white">
      {/* Header */}
      <div className="flex items-center justify-between p-3 md:p-4 border-b border-gray-700">
        <div className="flex items-center space-x-2 md:space-x-3">
          <h2 className="text-base md:text-lg font-medium">Greenrock Agent</h2>
          {currentSessionId && (
            <span className="hidden sm:inline text-xs text-neutral-400 bg-neutral-800 px-2 py-1 rounded">
              Session: {currentSessionId.slice(0, 8)}...
            </span>
          )}
        </div>
        <div className="flex items-center space-x-1 md:space-x-2">
          <button
            onClick={clearChat}
            className="text-xs bg-neutral-700 hover:bg-neutral-600 px-2 py-1 rounded"
          >
            Clear
          </button>
          <button
            onClick={onClose}
            className="text-xs bg-rose-600 hover:bg-rose-700 px-2 py-1 rounded"
          >
            Close
          </button>
        </div>
      </div>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto p-3 md:p-4 space-y-3 md:space-y-4">
        {messages.length === 0 ? (
          <div className="text-center text-neutral-500 mt-6 md:mt-8">
            <div className="text-xl md:text-2xl mb-2">🤖</div>
            <p className="text-sm md:text-base">
              Start a conversation with the AI assistant
            </p>
            <p className="text-xs md:text-sm mt-1">
              Ask about trading strategies, market analysis, or portfolio
              management
            </p>
          </div>
        ) : (
          messages.map((message) => (
            <div
              key={message.id}
              className={`flex ${
                message.role === "user" ? "justify-end" : "justify-start"
              }`}
            >
              <div
                className={`max-w-[85%] sm:max-w-xs lg:max-w-md px-3 md:px-4 py-2 rounded-lg ${
                  message.role === "user"
                    ? "bg-blue-600 text-white"
                    : message.status === "error"
                    ? "bg-rose-600 text-white"
                    : message.status === "paused"
                    ? "bg-yellow-600 text-white"
                    : "bg-neutral-700 text-white"
                }`}
              >
                <div className="text-xs md:text-sm whitespace-pre-wrap">
                  {message.content}
                </div>
                <div className="text-xs opacity-75 mt-1">
                  {message.timestamp.toLocaleTimeString()}
                  {message.status && (
                    <span className="ml-2 capitalize">({message.status})</span>
                  )}
                </div>
              </div>
            </div>
          ))
        )}
        {isLoading && (
          <div className="flex justify-start">
            <div className="bg-neutral-700 text-white px-4 py-2 rounded-lg">
              <div className="flex items-center space-x-2">
                <div className="animate-pulse flex space-x-1">
                  <div className="w-2 h-2 bg-neutral-400 rounded-full animate-bounce"></div>
                  <div
                    className="w-2 h-2 bg-neutral-400 rounded-full animate-bounce"
                    style={{ animationDelay: "0.1s" }}
                  ></div>
                  <div
                    className="w-2 h-2 bg-neutral-400 rounded-full animate-bounce"
                    style={{ animationDelay: "0.2s" }}
                  ></div>
                </div>
                <span className="text-sm">AI is thinking...</span>
              </div>
            </div>
          </div>
        )}
        <div ref={messagesEndRef} />
      </div>

      {/* Input */}
      <div className="p-3 md:p-4 border-t border-gray-700">
        <div className="flex space-x-2">
          <textarea
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            onKeyPress={handleKeyPress}
            placeholder="Ask about trading strategies, market analysis, or portfolio management..."
            className="flex-1 bg-neutral-800 text-white px-3 py-2 rounded-lg border border-gray-600 focus:border-blue-500 focus:outline-none resize-none text-sm md:text-base"
            rows={isMobile ? 1 : 2}
            disabled={isLoading}
          />
          <button
            onClick={handleSendMessage}
            disabled={!inputValue.trim() || isLoading}
            className="bg-blue-600 hover:bg-blue-700 disabled:bg-neutral-600 px-3 md:px-4 py-2 rounded-lg font-medium text-sm md:text-base"
          >
            Send
          </button>
        </div>
        <div className="text-xs text-neutral-500 mt-2">
          Press Enter to send{!isMobile ? ", Shift+Enter for new line" : ""}
        </div>
      </div>
    </div>
  );
}
