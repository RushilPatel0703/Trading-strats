# Trade: project memory (keep brief; compress, don't append forever)

## Goal
Algo strategies combining technical, sentiment, and macro signals. Research → backtest → paper → live on IBKR.
Claude acts as expert algo-trading advisor; user supplies focus areas + reference GitHub repos over time.

## Decisions (why)
- **Broker:** Interactive Brokers (eventually). Paper first: IB Gateway port 4002.
- **Markets:** FX majors, gold (XAUUSD spot or MGC), S&P/Nasdaq via micro futures MES/MNQ (small size, liquid, nearly 24h).
- **Language:** Rust for hot paths (`crates/tcore`, PyO3 bindings when needed); Python for research, sentiment/NLP, orchestration.
  Rust over C++: memory safety, cargo, and NautilusTrader's core is Rust, so the stack stays one language.
- **Engine:** NautilusTrader (free, LGPL, Rust core): same strategy code for backtest and live, has an IB adapter, tick/bar level.
  Polars/Numba vectorised sweeps for fast idea screening before event-driven backtests.
- **Budget:** ~$20/mo. Free-first data; spend only where free data is inadequate (likely CME futures history or a news API).

## Data sources
- Price (free): Dukascopy tick/M1 (FX, XAUUSD, index CFDs back to ~2003); IB historical API once the account exists (FX data free).
- Futures history: Databento pay-as-you-go (sign-up credits) for MES/MNQ/MGC when needed.
- Sentiment/macro (free): GDELT, RSS (central banks, Reuters/FXStreet), Finnhub/AlphaVantage free tiers, Reddit,
  CFTC COT positioning (weekly), economic calendar (event risk is critical for FX/gold). Score locally with FinBERT.
- Storage: parquet in `data/catalog` (Nautilus ParquetDataCatalog). `data/` is gitignored.

## Layout
`src/trade/{data,sentiment,features,strategies,backtest,live,risk}` · `crates/tcore` · `configs/` · `research/` (notebooks)
`repos/` = cloned reference repos (gitignored) · `CandleStickPatterns/` = unrelated legacy MQL5, ignore.

## Conventions
- All timestamps UTC, ns precision. Bars labelled by close time. No lookahead: features use only data available at the timestamp.
- Backtests include spread, commission (IB tiered), slippage; report out-of-sample/walk-forward, not just in-sample.
- Secrets in `.env` only (template: `.env.example`).

## Open items
- Toolchain: uv + .venv (Py 3.12.16, nautilus 1.231.0, polars 2.1.0) done. rustup still to install.
- Confirm the user's jurisdiction allows FX/CFD/futures at IBKR (some, e.g. India under LRS, restrict these).
- Awaiting reference repos + first strategy focus.

## Log
- 2026-11-20: scaffolded the repo; chose Rust + Python, NautilusTrader, IBKR, and the free-first data plan.
- Remote: origin = github.com/RushilPatel0703/Trading-strats (was empty on 2026-11-20). Branch main.
