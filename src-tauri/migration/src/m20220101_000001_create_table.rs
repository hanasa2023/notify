use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(List::Table)
                    .if_not_exists()
                    .col(pk_auto(List::Id))
                    .col(string(List::Name).unique_key())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(pk_auto(Task::Id))
                    .col(string(Task::Name))
                    .col(string(Task::List))
                    .col(string(Task::Tag))
                    .col(boolean(Task::IsFlag))
                    .col(boolean(Task::IsFinish))
                    .col(timestamp(Task::Time))
                    .foreign_key(
                        ForeignKey::create()
                            .name("list-name")
                            .from(Task::Table, Task::List)
                            .to(List::Table, List::Name),
                    )
                    .to_owned(),
            )
            .await?;

        return Ok(());
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(List::Table).to_owned())
            .await?;

        return Ok(());
    }
}

#[derive(DeriveIden)]
enum Task {
    Table,
    Id,
    Name,
    List,
    Tag,
    IsFlag,
    IsFinish,
    Time,
}

#[derive(DeriveIden)]
enum List {
    Table,
    Id,
    Name,
}
