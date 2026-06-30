// ==========================================
// 阶段一：模块引入与实体定义
// ==========================================
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "comments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub article_id: i64,
    pub user_id: i64,
    pub content: String,
    pub is_reply: bool,
    pub root_comment_id: Option<i64>,
    pub reply_to_user_id: Option<i64>,
    pub like_count: i32,
    pub created_at: DateTime,
    pub is_deleted: bool,
}

// ==========================================
// 阶段二：关系特质与模型行为实现
// ==========================================
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
