
    use serde::{Serialize, Deserialize};
    use surrealdb::sql::Thing;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Out {
        pub id: Thing,
        pub name: String
    }
    #[derive(Debug, Deserialize, Serialize)]
    pub struct In {
        //pub id: Thing,
        pub name: String
    }
