use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Directores
        manager
            .create_table(
                Table::create()
                    .table(Directors::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Directors::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Directors::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Películas
        manager
            .create_table(
                Table::create()
                    .table(Movies::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Movies::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Movies::Title).string().not_null())
                    .col(ColumnDef::new(Movies::DirectorId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_movies_director")
                            .from(Movies::Table, Movies::DirectorId)
                            .to(Directors::Table, Directors::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Actores
        manager
            .create_table(
                Table::create()
                    .table(Actors::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Actors::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Actors::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Géneros
        manager
            .create_table(
                Table::create()
                    .table(Genres::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Genres::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Genres::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Relación película-actor (muchos a muchos)
        manager
            .create_table(
                Table::create()
                    .table(MovieActors::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(MovieActors::MovieId).integer().not_null())
                    .col(ColumnDef::new(MovieActors::ActorId).integer().not_null())
                    .primary_key(
                        Index::create()
                            .col(MovieActors::MovieId)
                            .col(MovieActors::ActorId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_movieactors_movie")
                            .from(MovieActors::Table, MovieActors::MovieId)
                            .to(Movies::Table, Movies::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_movieactors_actor")
                            .from(MovieActors::Table, MovieActors::ActorId)
                            .to(Actors::Table, Actors::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Relación película-género (muchos a muchos)
        manager
            .create_table(
                Table::create()
                    .table(MovieGenres::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(MovieGenres::MovieId).integer().not_null())
                    .col(ColumnDef::new(MovieGenres::GenreId).integer().not_null())
                    .primary_key(
                        Index::create()
                            .col(MovieGenres::MovieId)
                            .col(MovieGenres::GenreId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_moviegenres_movie")
                            .from(MovieGenres::Table, MovieGenres::MovieId)
                            .to(Movies::Table, Movies::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_moviegenres_genre")
                            .from(MovieGenres::Table, MovieGenres::GenreId)
                            .to(Genres::Table, Genres::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MovieGenres::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MovieActors::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Movies::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Actors::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Genres::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Directors::Table).to_owned())
            .await?;
        Ok(())
    }
}

// Enum con el nombre de las tablas y columnas (¡muy importante!)
#[derive(Iden)]
enum Directors {
    Table,
    Id,
    Name,
}
#[derive(Iden)]
enum Movies {
    Table,
    Id,
    Title,
    DirectorId,
}
#[derive(Iden)]
enum Actors {
    Table,
    Id,
    Name,
}
#[derive(Iden)]
enum Genres {
    Table,
    Id,
    Name,
}
#[derive(Iden)]
enum MovieActors {
    Table,
    MovieId,
    ActorId,
}
#[derive(Iden)]
enum MovieGenres {
    Table,
    MovieId,
    GenreId,
}
