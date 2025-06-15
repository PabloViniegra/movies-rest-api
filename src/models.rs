// src/models.rs

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

pub mod director {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "directors")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub name: String,
    }
    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {}
    impl sea_orm::RelationTrait for Relation {
        fn def(&self) -> sea_orm::RelationDef {
            panic!("No Relation")
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod actor {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "actors")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub name: String,
    }
    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {}
    impl sea_orm::RelationTrait for Relation {
        fn def(&self) -> sea_orm::RelationDef {
            panic!("No Relation")
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod genre {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "genres")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub name: String,
    }
    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {}
    impl sea_orm::RelationTrait for Relation {
        fn def(&self) -> sea_orm::RelationDef {
            panic!("No Relation")
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod movie {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "movies")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub title: String,
        pub director_id: i32,
    }
    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {}
    impl sea_orm::RelationTrait for Relation {
        fn def(&self) -> sea_orm::RelationDef {
            panic!("No Relation")
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod movie_actor {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "movie_actors")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub movie_id: i32,
        #[sea_orm(primary_key)]
        pub actor_id: i32,
    }
    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {}
    impl sea_orm::RelationTrait for Relation {
        fn def(&self) -> sea_orm::RelationDef {
            panic!("No Relation")
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod movie_genre {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "movie_genres")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub movie_id: i32,
        #[sea_orm(primary_key)]
        pub genre_id: i32,
    }
    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {}
    impl sea_orm::RelationTrait for Relation {
        fn def(&self) -> sea_orm::RelationDef {
            panic!("No Relation")
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}
