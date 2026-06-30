// ==========================================
// 阶段一：模块引入与实体定义
// ==========================================
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i64,
    pub nick_name: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password_hash: String,
    pub proto_url: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

// ==========================================
// 阶段二：关系特质与模型行为实现
// ==========================================
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
