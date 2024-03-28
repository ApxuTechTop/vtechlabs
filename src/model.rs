use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct User {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,
}
impl From<InsertUser> for User {
    fn from(insert_user: InsertUser) -> Self {
        User {
            id: insert_user.id.unwrap(),
            name: insert_user.name.unwrap(),
            email: insert_user.email.unwrap(),
        }
    }
}