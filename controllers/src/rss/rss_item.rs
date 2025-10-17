use anyhow::Result;
use rss::Channel;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, Set, ColumnTrait};
use models::rss::rss_item::{self, Column, Entity as RssItem};


pub async fn add_item(db: &DatabaseConnection, channel: Channel, channel_id: i32) -> Result<()> {
    let mut item_count = 0;
    for item in channel.items() {
        let item_clone = item.clone();
        let link = item_clone.link.clone().unwrap();
        
        let item_model = rss_item::ActiveModel {
            channel_id: Set(channel_id),
            title: Set(item_clone.title),
            link: Set(item_clone.link),
            description: Set(item_clone.description),
            author: Set(item_clone.author),
            comments: Set(item_clone.comments),
            pub_date: Set(item_clone.pub_date),
            content: Set(item_clone.content),
            extensions: Set(serde_json::json!(item.extensions)),
            ..Default::default()
        };
        
        let item_dupe = RssItem::find()
            .filter(Column::Link.eq(link))
            .one(db)
            .await?;
        
        if let None = item_dupe {
            let _last_id = RssItem::insert(item_model)
                .exec(db)
                .await?
                .last_insert_id;
            
            item_count += 1;
        }
    }
    tracing::info!("Added {} items to DB...", item_count);
    
    Ok(())
}