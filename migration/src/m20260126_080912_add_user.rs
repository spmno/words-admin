use sea_orm_migration::{prelude::*, schema::*};
use sea_orm::Expr;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table("users")
                    .if_not_exists()
                    .col(ColumnDef::new("id")
                        .uuid()
                        .not_null()
                        .default(Expr::cust("gen_random_uuid()"))
                        .primary_key()
                    )
                    .col(string("username"))
                    .col(string("phone").unique_key().not_null())
                    .col(string("password_hash"))
                    .col(string("password_salt"))
                    .col(string("roles"))
                    .col(timestamp("created_at").not_null().default(Expr::cust("now()")))
                    .col(timestamp("updated_at").not_null().default(Expr::cust("now()")))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table("users").to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Semester,
    Unit,
    Word,
    Phrase,
    Sentence,
    Meaning,
}
