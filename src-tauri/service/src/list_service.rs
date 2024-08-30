pub struct ListService;
use ::entity::{list, list::Entity as List};
use ::entity::{task, task::Entity as Task};
use sea_orm::*;

impl ListService {
    async fn add_list(db: &DbConn, list_name: &str) -> Result<String, DbErr> {
        let new_list = list::ActiveModel {
            name: Set(String::from(list_name)),
            ..Default::default()
        };

        let list = List::insert(new_list).exec(db).await?;

        Ok(format!(
            "Successully add list by id {}",
            list.last_insert_id
        ))
    }

    // TODO： 在删除list前先删除所有list中的task
    async fn delete_list(db: &DbConn, list_name: &str) -> Result<String, DbErr> {
        let list_id = List::find()
            .filter(list::Column::Name.eq(list_name))
            .all(db)
            .await?;

        List::delete_by_id(list_id[0].id);

        Ok(format!("Successfully delete list by name {list_name}"))
    }
}
