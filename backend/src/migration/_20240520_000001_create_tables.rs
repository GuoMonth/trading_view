use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建 symbol 表
        manager
            .create_table(
                Table::create()
                    .table(Symbol::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Symbol::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Symbol::Name).string().not_null())
                    .col(ColumnDef::new(Symbol::Exchange).string().not_null())
                    .col(ColumnDef::new(Symbol::Type).string().not_null())
                    .col(ColumnDef::new(Symbol::CreatedAt).timestamp().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Symbol::UpdatedAt).timestamp().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        // 创建 ohlc_data 表
        manager
            .create_table(
                Table::create()
                    .table(OhlcData::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OhlcData::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OhlcData::SymbolId).integer().not_null())
                    .col(ColumnDef::new(OhlcData::Timestamp).timestamp().not_null())
                    .col(ColumnDef::new(OhlcData::Open).double().not_null())
                    .col(ColumnDef::new(OhlcData::High).double().not_null())
                    .col(ColumnDef::new(OhlcData::Low).double().not_null())
                    .col(ColumnDef::new(OhlcData::Close).double().not_null())
                    .col(ColumnDef::new(OhlcData::Volume).double().null())
                    .col(ColumnDef::new(OhlcData::CreatedAt).timestamp().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(OhlcData::UpdatedAt).timestamp().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-ohlc_data-symbol_id")
                            .from(OhlcData::Table, OhlcData::SymbolId)
                            .to(Symbol::Table, Symbol::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx-ohlc_data-symbol_id-timestamp")
                            .table(OhlcData::Table)
                            .col(OhlcData::SymbolId)
                            .col(OhlcData::Timestamp)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建 indicator_data 表
        manager
            .create_table(
                Table::create()
                    .table(IndicatorData::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IndicatorData::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IndicatorData::SymbolId).integer().not_null())
                    .col(ColumnDef::new(IndicatorData::Timestamp).timestamp().not_null())
                    .col(ColumnDef::new(IndicatorData::MaShort).double().null())
                    .col(ColumnDef::new(IndicatorData::MaMedium).double().null())
                    .col(ColumnDef::new(IndicatorData::MaLong).double().null())
                    .col(ColumnDef::new(IndicatorData::Rsi).double().null())
                    .col(ColumnDef::new(IndicatorData::Macd).double().null())
                    .col(ColumnDef::new(IndicatorData::MacdSignal).double().null())
                    .col(ColumnDef::new(IndicatorData::MacdHistogram).double().null())
                    .col(ColumnDef::new(IndicatorData::BollingerMiddle).double().null())
                    .col(ColumnDef::new(IndicatorData::BollingerUpper).double().null())
                    .col(ColumnDef::new(IndicatorData::BollingerLower).double().null())
                    .col(ColumnDef::new(IndicatorData::CreatedAt).timestamp().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(IndicatorData::UpdatedAt).timestamp().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-indicator_data-symbol_id")
                            .from(IndicatorData::Table, IndicatorData::SymbolId)
                            .to(Symbol::Table, Symbol::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx-indicator_data-symbol_id-timestamp")
                            .table(IndicatorData::Table)
                            .col(IndicatorData::SymbolId)
                            .col(IndicatorData::Timestamp)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建 trading_signal 表
        manager
            .create_table(
                Table::create()
                    .table(TradingSignal::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TradingSignal::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TradingSignal::SymbolId).integer().not_null())
                    .col(ColumnDef::new(TradingSignal::Timestamp).timestamp().not_null())
                    .col(ColumnDef::new(TradingSignal::SignalType).string().not_null())
                    .col(ColumnDef::new(TradingSignal::Source).string().not_null())
                    .col(ColumnDef::new(TradingSignal::Price).double().not_null())
                    .col(ColumnDef::new(TradingSignal::CreatedAt).timestamp().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(TradingSignal::UpdatedAt).timestamp().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-trading_signal-symbol_id")
                            .from(TradingSignal::Table, TradingSignal::SymbolId)
                            .to(Symbol::Table, Symbol::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx-trading_signal-symbol_id-timestamp")
                            .table(TradingSignal::Table)
                            .col(TradingSignal::SymbolId)
                            .col(TradingSignal::Timestamp)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建 backtest_result 表
        manager
            .create_table(
                Table::create()
                    .table(BacktestResult::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BacktestResult::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BacktestResult::SymbolId).integer().not_null())
                    .col(ColumnDef::new(BacktestResult::Timeframe).string().not_null())
                    .col(ColumnDef::new(BacktestResult::StartDate).timestamp().not_null())
                    .col(ColumnDef::new(BacktestResult::EndDate).timestamp().not_null())
                    .col(ColumnDef::new(BacktestResult::InitialCapital).double().not_null())
                    .col(ColumnDef::new(BacktestResult::FinalCapital).double().not_null())
                    .col(ColumnDef::new(BacktestResult::TotalReturn).double().not_null())
                    .col(ColumnDef::new(BacktestResult::AnnualReturn).double().not_null())
                    .col(ColumnDef::new(BacktestResult::MaxDrawdown).double().not_null())
                    .col(ColumnDef::new(BacktestResult::SharpeRatio).double().not_null())
                    .col(ColumnDef::new(BacktestResult::TradeCount).integer().not_null())
                    .col(ColumnDef::new(BacktestResult::WinningTrades).integer().not_null())
                    .col(ColumnDef::new(BacktestResult::LosingTrades).integer().not_null())
                    .col(ColumnDef::new(BacktestResult::WinRate).double().not_null())
                    .col(ColumnDef::new(BacktestResult::AverageWin).double().not_null())
                    .col(ColumnDef::new(BacktestResult::AverageLoss).double().not_null())
                    .col(ColumnDef::new(BacktestResult::ProfitFactor).double().not_null())
                    .col(ColumnDef::new(BacktestResult::CreatedAt).timestamp().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(BacktestResult::UpdatedAt).timestamp().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-backtest_result-symbol_id")
                            .from(BacktestResult::Table, BacktestResult::SymbolId)
                            .to(Symbol::Table, Symbol::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建 trade_record 表
        manager
            .create_table(
                Table::create()
                    .table(TradeRecord::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TradeRecord::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TradeRecord::BacktestResultId).string().not_null())
                    .col(ColumnDef::new(TradeRecord::Timestamp).timestamp().not_null())
                    .col(ColumnDef::new(TradeRecord::TradeType).string().not_null())
                    .col(ColumnDef::new(TradeRecord::Price).double().not_null())
                    .col(ColumnDef::new(TradeRecord::Quantity).double().not_null())
                    .col(ColumnDef::new(TradeRecord::Amount).double().not_null())
                    .col(ColumnDef::new(TradeRecord::Fee).double().not_null())
                    .col(ColumnDef::new(TradeRecord::RemainingCapital).double().not_null())
                    .col(ColumnDef::new(TradeRecord::Position).double().not_null())
                    .col(ColumnDef::new(TradeRecord::CreatedAt).timestamp().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(TradeRecord::UpdatedAt).timestamp().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-trade_record-backtest_result_id")
                            .from(TradeRecord::Table, TradeRecord::BacktestResultId)
                            .to(BacktestResult::Table, BacktestResult::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx-trade_record-backtest_result_id-timestamp")
                            .table(TradeRecord::Table)
                            .col(TradeRecord::BacktestResultId)
                            .col(TradeRecord::Timestamp)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除 trade_record 表
        manager
            .drop_table(Table::drop().table(TradeRecord::Table).if_exists().to_owned())
            .await?;

        // 删除 backtest_result 表
        manager
            .drop_table(Table::drop().table(BacktestResult::Table).if_exists().to_owned())
            .await?;

        // 删除 trading_signal 表
        manager
            .drop_table(Table::drop().table(TradingSignal::Table).if_exists().to_owned())
            .await?;

        // 删除 indicator_data 表
        manager
            .drop_table(Table::drop().table(IndicatorData::Table).if_exists().to_owned())
            .await?;

        // 删除 ohlc_data 表
        manager
            .drop_table(Table::drop().table(OhlcData::Table).if_exists().to_owned())
            .await?;

        // 删除 symbol 表
        manager
            .drop_table(Table::drop().table(Symbol::Table).if_exists().to_owned())
            .await?;

        Ok(())
    }
}

// 定义表名和列名
#[derive(Iden)]
enum Symbol {
    Table,
    Id,
    Name,
    Exchange,
    Type,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum OhlcData {
    Table,
    Id,
    SymbolId,
    Timestamp,
    Open,
    High,
    Low,
    Close,
    Volume,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum IndicatorData {
    Table,
    Id,
    SymbolId,
    Timestamp,
    MaShort,
    MaMedium,
    MaLong,
    Rsi,
    Macd,
    MacdSignal,
    MacdHistogram,
    BollingerMiddle,
    BollingerUpper,
    BollingerLower,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum TradingSignal {
    Table,
    Id,
    SymbolId,
    Timestamp,
    SignalType,
    Source,
    Price,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum BacktestResult {
    Table,
    Id,
    SymbolId,
    Timeframe,
    StartDate,
    EndDate,
    InitialCapital,
    FinalCapital,
    TotalReturn,
    AnnualReturn,
    MaxDrawdown,
    SharpeRatio,
    TradeCount,
    WinningTrades,
    LosingTrades,
    WinRate,
    AverageWin,
    AverageLoss,
    ProfitFactor,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum TradeRecord {
    Table,
    Id,
    BacktestResultId,
    Timestamp,
    TradeType,
    Price,
    Quantity,
    Amount,
    Fee,
    RemainingCapital,
    Position,
    CreatedAt,
    UpdatedAt,
}