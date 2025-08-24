# 🌱 Greenrock

**AI-Powered Quantitative Trading & Financial Analysis Platform**

> ⚠️ **EDUCATIONAL & RESEARCH PURPOSE ONLY**  
> This platform is designed for educational exploration of quantitative trading concepts and AI-driven financial analysis. **IT IS NOT FINANCIAL ADVICE** and should never be used for actual trading without proper risk management, thorough testing, and professional guidance. Trading financial instruments involves substantial risk of loss.

Greenrock is a sophisticated educational financial technology platform that demonstrates how artificial intelligence, advanced quantitative analysis, and real-time trading capabilities can be integrated. Built with Rust for performance and modern web technologies, it provides a comprehensive learning environment for understanding algorithmic trading, portfolio management, and financial research concepts.

## 🎓 Educational Focus

This platform serves as:
- **Learning Tool**: Understand quantitative trading strategies and AI integration
- **Research Environment**: Explore market data analysis and backtesting methodologies
- **Technology Demonstration**: See how modern systems architecture applies to financial technology
- **Development Foundation**: A starting point for building your own trading analysis tools

---

## 🎯 Educational Vision

Greenrock provides learners and researchers with modern technology to explore:
- **AI-driven workflow orchestration** for understanding automated decision-making systems
- **High-performance data processing** techniques for financial dataset analysis
- **Advanced technical indicators** and their implementation in trading strategies
- **Risk management concepts** through AI-assisted portfolio analysis
- **Market pattern research** using comprehensive historical datasets spanning multiple years

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

### 🎛️ **Production-Grade Architecture**
- **RESTful API**: HTTP endpoints for chat-based trading interaction and balance queries
- **Modern Web UI**: React-based interface with real-time charting and trading controls
- **PostgreSQL Persistence**: Reliable session storage and workflow state management
- **Concurrent Execution**: Parallel trading engines and web services
- **Docker Deployment**: Containerized application for easy deployment and scaling
- **Production-Ready**: Comprehensive error handling, logging, and monitoring

---

## 🏗️ Architecture Overview

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   React Web UI  │    │  AI Workflow    │    │  Trading Engine │
│  (Port 4200)    │◄──►│   Orchestrator  │◄──►│   (Real-time)   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   REST API      │    │   Graph Flow    │    │   Binance API   │
│ (Rust/Axum)    │    │   (Tasks)       │    │   (Market Data) │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │
         ▼
┌─────────────────┐
│   PostgreSQL    │
│   (Sessions)    │
└─────────────────┘
```

### Core Components

- **🌐 React Web Interface**: Modern trading dashboard with real-time charts and controls
- **🧠 AI Workflow Engine**: Graph-based task orchestration with LLM integration
- **📈 Trading Strategies**: Modular strategy framework with pluggable algorithms  
- **🔌 Broker Abstraction**: Unified interface for multiple trading venues (currently Binance)
- **📊 Technical Analysis**: High-performance indicator computation library
- **💾 Data Management**: Efficient time-series data storage and retrieval
- **🐳 Docker Integration**: Containerized deployment for consistent environments

---

## 🚀 Quick Start

> ⚠️ **Risk Warning**: This platform is for educational purposes only. Never use with real trading accounts without thorough testing and risk management.

### Option 1: Docker (Recommended)

**Prerequisites:**
- Docker installed on your system
- Basic understanding of environment variables

**1. Create Environment File:**
```bash
# Create .env file with your API keys
cat > .env << EOF
DATABASE_URL=postgresql://user:password@localhost/greenrock
BINANCE_API_KEY=your_binance_api_key_here
BINANCE_SECRET_KEY=your_binance_secret_key_here
OPENROUTER_API_KEY=your_openrouter_api_key_here
EOF
```

**2. Run with Docker:**
```bash
# Pull and run the latest image
docker run --env-file .env -p 4200:4200 pinsky/greenrock
```

**3. Access the Platform:**
- **Web Interface**: `http://localhost:4200`
- **Health Check**: `http://localhost:4200/health`

### Option 2: Local Development

**Prerequisites:**
- **Rust** 1.70+ (edition 2021)
- **Node.js** 20+ and **Bun** (for web UI)
- **PostgreSQL** (for session persistence)
- **Binance API Keys** (testnet recommended for learning)
- **OpenRouter API Key** (for AI features)

**1. Clone and Setup:**
```bash
git clone <repository-url>
cd greenrock
```

**2. Environment Configuration:**
```bash
# Create .env file (same as Docker option above)
cp .env.example .env
# Edit .env with your actual API keys
```

**3. Database Setup:**
```bash
createdb greenrock
# Initialize database tables (if migrations exist)
```

**4. Build and Run:**
```bash
# Build the Rust engine and web UI
cargo build --release --package greenrock-engine

# Start the platform
cargo run --package greenrock-engine
```

**Platform Access:**
- 🌐 **Web Dashboard**: `http://localhost:4200`
- 📊 **Real-Time Trading Engine**: Runs automatically for BTCUSDT
- 🤖 **AI Chat Interface**: Available in the web UI

### Available API Endpoints

- `GET /health` - System health check
- `POST /chat` - AI-powered trading chat interface
- `GET /broker/balance` - Account balance and positions
- `GET /broker/candles` - Historical candlestick data
- `GET /broker/order_book` - Current order book data
- `GET /strategy/portfolio` - Portfolio analysis

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
├── crates/
│   └── greenrock-engine/     # Main Rust trading engine
│       ├── src/
│       │   ├── brokers/      # Trading venue integrations  
│       │   ├── models/       # Data structures and analysis
│       │   ├── processor/    # AI workflow tasks
│       │   ├── runner/       # Trading execution engine
│       │   └── strategy/     # Trading strategy framework
│       └── templates/        # AI prompt templates
├── greenrock-web-ui/         # React web interface
│   ├── src/
│   │   ├── components/       # UI components (Chart, Chat, etc.)
│   │   ├── types/           # TypeScript type definitions
│   │   └── utils/           # API utilities and helpers
│   └── dist/                # Built web assets
├── analysis/                 # Jupyter notebooks and datasets
├── processed_btc_data/       # Processed market data
├── Dockerfile               # Multi-stage Docker build
└── .env                     # Environment configuration
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
- **Rust** - High-performance systems programming with Cargo workspace
- **Tokio** - Async runtime for concurrent operations
- **Axum** - Modern web framework for REST APIs

### **Frontend & UI**
- **React 19** - Modern web interface with hooks
- **TypeScript** - Type-safe frontend development
- **Vite** - Fast build tool and development server
- **Tailwind CSS** - Utility-first styling framework
- **Lightweight Charts** - Professional trading charts
- **Bun** - Fast JavaScript runtime and package manager

### **AI & Workflows**
- **graph-flow** - Workflow orchestration engine  
- **rig-core** - LLM integration framework
- **OpenRouter** - Access to multiple AI models (Gemini 2.0 Flash)

### **Financial Data**
- **Polars** - High-performance dataframes
- **ta** - Technical analysis indicators
- **binance-rs** - Binance API client with WebSocket support
- **Parquet** - Columnar data storage format

### **DevOps & Deployment**
- **Docker** - Multi-stage containerization
- **PostgreSQL** - Production database
- **WebSocket** - Real-time data streaming
- **CORS** - Cross-origin resource sharing

---

## 🔧 Configuration

### Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `DATABASE_URL` | PostgreSQL connection string | ✅ |
| `BINANCE_API_KEY` | Binance API key |  |
| `BINANCE_SECRET_KEY` | Binance secret key | |
| `OPENROUTER_API_KEY` | OpenRouter API key for AI |  |

### Trading Configuration

**Strategy Parameters** (customizable in code):
- Technical indicator periods (MACD: 12,26,9 | EMA: 20 | SuperTrend: 10,3.0)
- Risk management settings (position size, stop-loss levels)
- Execution intervals and timeframes

**Web Interface Configuration**:
- Frontend served on port 4200
- Real-time WebSocket connections for live data
- Auto-reconnection for market data streams
- Modern dark theme with professional trading colors [[memory:6996567]]

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
- 🏦 **Multi-venue broker integrations** (Coinbase, Kraken, etc.)
- 📈 **Advanced trading strategies** and risk management
- 🔬 **Research tools** and backtesting frameworks
- 🌐 **Web UI enhancements** and new dashboard features
- 🐳 **DevOps improvements** and deployment automation

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

## ⚠️ Disclaimer

**EDUCATIONAL PURPOSE ONLY**: Greenrock is designed as an educational platform to demonstrate modern financial technology concepts and AI integration. This software:

- **IS NOT FINANCIAL ADVICE**: Never use this platform for actual trading decisions
- **IS NOT INVESTMENT ADVICE**: All strategies and analysis are for learning purposes only
- **CARRIES SIGNIFICANT RISK**: Real trading involves substantial risk of financial loss
- **REQUIRES EXPERTISE**: Professional trading requires extensive knowledge and experience

**Before Any Real Trading:**
- Thoroughly test all strategies with paper trading
- Understand risk management principles
- Consult with qualified financial professionals
- Never risk more than you can afford to lose
- Ensure compliance with local financial regulations

**The developers and contributors assume no responsibility for any financial losses incurred through the use of this software.**