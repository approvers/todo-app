pub mod db_controller {
    use chrono::{serde::ts_seconds, DateTime, Utc};
    use mongodb::{bson::doc, sync::Client};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    // Task型
    pub struct Task {
        #[serde(with = "ts_seconds")]
        pub scheduled_date: DateTime<Utc>,
        #[serde(with = "ts_seconds")]
        pub register_date: DateTime<Utc>,
        pub title: String,
        pub memo: String,
        pub finish: bool,
    }

    //新規タスクの追加
    pub fn add_task(new_task: Task) -> Result<(), mongodb::error::Error> {
        let client = Client::with_uri_str("mongodb://localhost27017")?;
        let database = client.database("taskdb");
        let collection = database.collection::<Task>("task");

        let docs = vec![new_task];
        collection.insert_many(docs, None)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::db_controller::{add_task, Task};
    use chrono::prelude::Utc;

    #[test]
    fn add_new_task() {
        let new_task = Task {
            scheduled_date: Utc::now(),
            register_date: Utc::now(),
            title: String::from("LLP"),
            memo: String::from("Life Love Peace"),
            finish: false,
        };

        add_task(new_task).expect("Failed to add new Task");
        println!("add new Task");
    }
}