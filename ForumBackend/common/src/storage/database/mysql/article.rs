// ==========================================
// 阶段一：模块引入与实体定义
// ==========================================
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "articles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub article_id: i64,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    pub cover_image: Option<String>,
    pub author_id: i64,
    pub category_id: Option<String>,
    pub status: i8,
    pub view_count: i64,
    pub like_count: i64,
    pub comment_count: i64,
    pub is_top: bool,
    pub published_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

// ==========================================
// 阶段二：关系特质与模型行为实现
// ==========================================
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
