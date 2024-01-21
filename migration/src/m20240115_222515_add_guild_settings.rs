use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(Table::alter()
            .table(Guild::Table)
            .add_column_if_not_exists(
                ColumnDef::new(Guild::LeaveTimeout).big_integer().default(600)
            )
            .to_owned()
        ).await?;
        manager.alter_table(Table::alter()
            .table(Guild::Table)
            .add_column_if_not_exists(
                ColumnDef::new(Guild::SkipVotes).decimal().default(0.5)
            )
            .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(Table::alter()
            .table(Guild::Table)
            .drop_column(Guild::SkipVotes)
            .to_owned()
        ).await?;
        manager.alter_table(Table::alter()
            .table(Guild::Table)
            .drop_column(Guild::LeaveTimeout)
            .to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
enum Guild {
    Table,
    Id,
    Name,
    LeaveTimeout,
    SkipVotes,
}
