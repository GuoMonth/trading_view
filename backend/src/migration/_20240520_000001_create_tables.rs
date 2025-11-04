use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20240520_000001_create_tables"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
                    .col(ColumnDef::new(OhlcData::Code).string().not_null())
                    .col(ColumnDef::new(OhlcData::Timestamp).timestamp().not_null())
                    .col(ColumnDef::new(OhlcData::Open).double().not_null())
                    .col(ColumnDef::new(OhlcData::High).double().not_null())
                    .col(ColumnDef::new(OhlcData::Low).double().not_null())
                    .col(ColumnDef::new(OhlcData::Close).double().not_null())
                    .col(ColumnDef::new(OhlcData::CreatedAt).timestamp().default(Expr::current_timestamp()))
                    .index(
                        Index::create()
                            .name("idx-ohlc_data-code-timestamp")
                            .table(OhlcData::Table)
                            .col(OhlcData::Code)
                            .col(OhlcData::Timestamp)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除 ohlc_data 表
        manager
            .drop_table(Table::drop().table(OhlcData::Table).if_exists().to_owned())
            .await?;

        Ok(())
    }
}

// 定义表名和列名
#[derive(Iden)]
enum OhlcData {
    Table,
    Id,
    Code,
    Timestamp,
    Open,
    High,
    Low,
    Close,
    CreatedAt,
}