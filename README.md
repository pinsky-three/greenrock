# 🌱 Greenrock

**AI-Powered Quantitative Trading & Financial Analysis Platform**

Greenrock is a sophisticated financial technology platform that combines artificial intelligence, advanced quantitative analysis, and real-time trading capabilities. Built with Rust for performance and Python for data science, it provides institutional-grade tools for algorithmic trading, portfolio management, and financial research.

---

## 🎯 Vision

Greenrock empowers traders, quants, and financial institutions with cutting-edge technology to:
- **Automate complex trading decisions** using AI-driven workflow orchestration
- **Analyze massive financial datasets** with high-performance data processing
- **Implement sophisticated trading strategies** with advanced technical indicators
- **Manage risk intelligently** through AI-assisted portfolio optimization
- **Research market patterns** using historical data spanning multiple years

---

## ✨ Key Features

### 🤖 **AI-Driven Trading Workflows**
- **Intelligent Task Orchestration**: Complex multi-step trading workflows with conditional logic
- **LLM Integration**: Natural language interaction with trading systems via OpenRouter API
- **Adaptive Decision Making**: AI agents that analyze market conditions and portfolio performance
- **Context-Aware Processing**: Maintains trading context across workflow executions

### 📊 **Advanced Technical Analysis**
- **20+ Technical Indicators**: MACD, RSI, EMA, SuperTrend, ATR, and custom clustering algorithms
- **Real-Time Calculations**: Sub-millisecond indicator computation on streaming data
- **SuperTrend Clustering**: Advanced K-means clustering for optimal parameter selection
- **Multi-Timeframe Analysis**: Comprehensive technical analysis across different intervals

### 🏦 **Binance Integration**
- **Real-Time WebSocket Streams**: Live market data with automatic reconnection
- **Complete Trading API**: Account management, order execution, and position tracking
- **Historical Data Access**: Years of OHLCV data for backtesting and research
- **Risk Management**: Balance monitoring and position sizing controls

### 💾 **High-Performance Data Processing**
- **Parquet Format Support**: Efficient storage and processing of large financial datasets
- **Polars Integration**: Lightning-fast dataframe operations for quantitative analysis
- **Memory-Efficient Ring Buffers**: Real-time data structures optimized for streaming analytics
- **Multi-Asset Support**: Cryptocurrency, forex, and traditional market data

### 🎛️ **Enterprise-Grade Architecture**
- **RESTful API**: HTTP endpoints for chat-based trading interaction and balance queries
- **PostgreSQL Persistence**: Reliable session storage and workflow state management
- **Concurrent Execution**: Parallel trading engines and web services
- **Production-Ready**: Comprehensive error handling, logging, and monitoring

---

## 🏗️ Architecture Overview

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Web API       │    │  AI Workflow    │    │  Trading Engine │
│   (Chat/REST)   │◄──►│   Orchestrator  │◄──►│   (Real-time)   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   PostgreSQL    │    │   Graph Flow    │    │   Binance API   │
│   (Sessions)    │    │   (Tasks)       │    │   (Market Data) │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Core Components

- **🧠 AI Workflow Engine**: Graph-based task orchestration with LLM integration
- **📈 Trading Strategies**: Modular strategy framework with pluggable algorithms  
- **🔌 Broker Abstraction**: Unified interface for multiple trading venues
- **📊 Technical Analysis**: High-performance indicator computation library
- **💾 Data Management**: Efficient time-series data storage and retrieval

---

## 🚀 Quick Start

### Prerequisites

- **Rust** 1.70+ (edition 2024)
- **Python** 3.12+ (for Jupyter analysis)
- **PostgreSQL** (for session persistence)
- **Binance API Keys** (for live trading)
- **OpenRouter API Key** (for AI features)

### Installation

1. **Clone and Setup**:
```bash
git clone <repository-url>
cd greenrock
cargo build --release
```

2. **Environment Configuration**:
```bash
# Create .env file
cat > .env << EOF
DATABASE_URL=postgresql://user:password@localhost/greenrock
BINANCE_API_KEY=your_binance_api_key
BINANCE_SECRET_KEY=your_binance_secret_key
OPENROUTER_API_KEY=your_openrouter_api_key
EOF
```

3. **Database Setup**:
```bash
createdb greenrock
# Run migrations if available
```

### Running the Platform

**Start the Full Platform**:
```bash
cargo run
```

This launches both:
- 🌐 **Web API Server** at `http://localhost:8000`
- 📊 **Real-Time Trading Engine** for BTCUSDT

**Available Endpoints**:
- `GET /health` - System health check
- `POST /chat` - AI-powered trading chat interface
- `GET /balance` - Account balance and positions

---

## 💼 Use Cases

### 🤖 **AI Trading Assistant**
```bash
curl -X POST http://localhost:8000/chat \
  -H "Content-Type: application/json" \
  -d '{"query": "What is my current portfolio allocation?", "session_id": "trader_001"}'
```

### 📊 **Real-Time Strategy Execution**
The platform continuously:
- Streams live market data from Binance
- Calculates technical indicators (MACD, SuperTrend, EMA)
- Executes trading signals based on strategy logic
- Manages risk through position sizing and stop-losses

### 🔬 **Quantitative Research**
Access to comprehensive datasets:
- **8+ years of Bitcoin data** (2017-2025) in high-performance Parquet format
- **Multiple timeframes** (1m, 5m, 1h, 1d intervals)
- **Jupyter integration** for interactive analysis
- **Custom indicator development** with Rust performance

### 🏦 **Portfolio Management**
- **Multi-asset portfolio tracking** across different venues
- **Risk-adjusted position sizing** based on volatility
- **Performance attribution** and drawdown analysis
- **Rebalancing automation** with custom triggers

---

## 🛠️ Development

### Project Structure

```
greenrock/
├── src/
│   ├── brokers/          # Trading venue integrations
│   ├── models/           # Data structures and analysis
│   ├── processor/        # AI workflow tasks
│   ├── runner/           # Trading execution engine
│   └── strategy/         # Trading strategy framework
├── analysis/             # Jupyter notebooks and datasets
├── processed_btc_data/   # Processed market data
└── templates/            # AI prompt templates
```

### Adding New Strategies

1. **Implement Strategy Trait**:
```rust
impl Strategy for MyStrategy {
    type State = MyStrategyState;
    
    fn tick(&self, ctx: &mut StrategyContext, timestamp: DateTime<Utc>, 
            state: &mut Self::State, symbol: String, 
            data_scope: Vec<Candle>, tick: Candle) -> StrategyAction {
        // Your strategy logic here
        StrategyAction::Pass
    }
}
```

2. **Register with Runner**:
```rust
let strategy = Box::new(MyStrategy::new());
let runner = Runner::new(broker, strategy);
```

### Custom AI Tasks

1. **Create Task Implementation**:
```rust
#[async_trait]
impl Task for MyAnalysisTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        // AI-powered analysis logic
        Ok(TaskResult::new(Some(result), NextAction::Continue))
    }
}
```

2. **Add to Workflow Graph**:
```rust
let graph = GraphBuilder::new("my_workflow")
    .add_task(Arc::new(MyAnalysisTask))
    .add_edge(entry_task_id, my_task_id)
    .build();
```

---

## 📚 Technology Stack

### **Core Runtime**
- **Rust** - High-performance systems programming
- **Tokio** - Async runtime for concurrent operations
- **Axum** - Modern web framework for APIs

### **AI & Workflows**
- **graph-flow** - Workflow orchestration engine  
- **rig-core** - LLM integration framework
- **OpenRouter** - Access to multiple AI models

### **Financial Data**
- **Polars** - High-performance dataframes
- **ta** - Technical analysis indicators
- **binance-rs** - Binance API client
- **Parquet** - Columnar data storage

### **Data Science**
- **Python 3.12+** - Analysis and research
- **JupyterLab** - Interactive development
- **nautilus-trader** - Institutional trading framework
- **ccxt** - Multi-exchange connectivity

---

## 🔧 Configuration

### Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `DATABASE_URL` | PostgreSQL connection string | ✅ |
| `BINANCE_API_KEY` | Binance API key | ✅ |
| `BINANCE_SECRET_KEY` | Binance secret key | ✅ |
| `OPENROUTER_API_KEY` | OpenRouter API key for AI | ✅ |

### Trading Configuration

**Strategy Parameters** (customizable in code):
- Technical indicator periods (MACD: 12,26,9 | EMA: 20 | SuperTrend: 10,3.0)
- Risk management settings (position size, stop-loss levels)
- Execution intervals and timeframes

**AI Model Settings**:
- Default model: `google/gemini-2.0-flash-001`
- Conversation history: 50 messages max
- Retry logic: 3 attempts with exponential backoff

---

## 📈 Performance & Scale

### **Real-Time Capabilities**
- **Sub-millisecond indicator calculation** on streaming data
- **WebSocket connection resilience** with automatic reconnection
- **Memory-efficient ring buffers** for continuous data processing
- **Concurrent trading engines** supporting multiple assets

### **Data Processing**
- **8+ years of historical data** readily accessible
- **Parquet format optimization** for analytical workloads  
- **Polars acceleration** for complex dataframe operations
- **Lazy evaluation** for memory-efficient large dataset analysis

### **Production Features**
- **Comprehensive error handling** with detailed logging
- **Session persistence** across system restarts
- **Graceful shutdown** handling for live trading positions
- **Monitoring endpoints** for system health

---

## 🤝 Contributing

We welcome contributions from the quantitative finance and AI community!

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/advanced-indicators`)
3. **Add comprehensive tests** for new functionality
4. **Update documentation** including this README
5. **Submit a pull request** with detailed description

### Areas for Contribution
- 📊 **New technical indicators** and analysis methods
- 🤖 **Additional AI workflow tasks** and decision logic  
- 🏦 **Multi-venue broker integrations** (FTX, Coinbase, etc.)
- 📈 **Advanced trading strategies** and risk management
- 🔬 **Research tools** and backtesting frameworks

---

## 📄 License

This project is open source under the MIT License. See LICENSE file for details.

## 🆘 Support

- **Documentation**: Check this README and inline code comments
- **Issues**: Open GitHub issues for bugs and feature requests  
- **Discussions**: Use GitHub Discussions for questions and ideas
- **Community**: Join our Discord for real-time collaboration

---

## 🏆 Acknowledgments

Greenrock builds upon the excellent work of:
- [graph-flow](https://crates.io/crates/graph-flow) - Workflow orchestration
- [rig-core](https://crates.io/crates/rig-core) - AI integration framework
- [Polars](https://pola.rs/) - High-performance dataframes
- [ta](https://crates.io/crates/ta) - Technical analysis library
- [Binance API](https://binance-docs.github.io/apidocs/) - Market data and trading

---

*🚀 Built with ❤️ for the future of quantitative finance*
