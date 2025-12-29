use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Word::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Word::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Word::Semester).string().not_null())
                    .col(ColumnDef::new(Word::Unit).string().not_null())
                    .col(ColumnDef::new(Word::Word).string().null())
                    .col(ColumnDef::new(Word::Phrase).string().null())
                    .col(ColumnDef::new(Word::Sentence).string().null())
                    .col(ColumnDef::new(Word::Meaning).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Word::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Word {
    Table,
    Id,
    Semester,
    Unit,
    Word,
    Phrase,
    Sentence,
    Meaning,
}
