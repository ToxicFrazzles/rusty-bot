use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Guild::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Guild::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Guild::Snowflake).string_len(18).not_null().unique_key())
                    .col(ColumnDef::new(Guild::Name).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Snowflake).string_len(18).not_null().unique_key())
                    .col(ColumnDef::new(User::Blacklisted).boolean().default(false))
                    .to_owned(),
            ).await?;
        
        manager
            .create_table(
                Table::create()
                    .table(Member::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Member::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(Member::Guild)
                            .big_integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Member::User)
                            .big_integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Member::Blacklisted)
                            .boolean()
                            .default(false)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_c344773142054288ab28e10fa9e4d182")
                            .from(Member::Table, Member::Guild)
                            .to(Guild::Table, Guild::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_4a25b866e3c44e26965231f9f95bf357")
                            .from(Member::Table, Member::User)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Guild::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Guild {
    Table,
    Id,
    Snowflake,
    Name,
}


#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Snowflake,
    Blacklisted
}


#[derive(DeriveIden)]
enum Member {
    Table,
    Id,
    Guild,
    User,
    Blacklisted
}