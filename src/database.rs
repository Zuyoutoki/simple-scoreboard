use sqlx::SqlitePool;

struct Pool {
    pool: Option<SqlitePool>,
}
impl Pool {
    async fn get_pool(&mut self) -> Result<&SqlitePool, DatabaseError> {
        match &self.pool {
            Some(_) => Ok(self.pool.as_ref().unwrap()),
            None => {
                self.pool = Some(SqlitePool::connect("sqlite://db/db.sqlite3").await.unwrap());
                Ok(self.pool.as_ref().unwrap())
            }
        }
    }
}
static mut POOL: Pool = Pool { pool: None };

#[derive(Debug)]
pub enum DatabaseError {}

pub mod challenges {
    use crate::database::POOL;
    use sqlx::FromRow;

    #[derive(FromRow)]
    pub struct Challenge {
        pub id: Option<i64>,
        pub flag: Option<String>,
    }

    pub async fn list_by_flag(flag: &String) -> Option<Challenge> {
        let mut conn = unsafe { &mut POOL }
            .get_pool()
            .await
            .unwrap()
            .acquire()
            .await
            .unwrap();

        sqlx::query_as::<_, Challenge>(
            r#"
                SELECT id, flag
                FROM challenges
                WHERE flag = ?
                LIMIT 1;
            "#,
        )
        .bind(flag)
        .fetch_optional(&mut *conn)
        .await
        .unwrap()
    }
}

pub mod submissions {
    use sqlx::FromRow;

    use crate::database::POOL;

    #[derive(FromRow)]
    pub struct Submission {
        pub id: Option<i64>,
        pub user: Option<String>,
        pub challenge_id: Option<i64>,
        pub timestamp: Option<chrono::NaiveDateTime>,
    }

    pub async fn list() -> Vec<Submission> {
        let mut conn = unsafe { &mut POOL }
            .get_pool()
            .await
            .unwrap()
            .acquire()
            .await
            .unwrap();

        sqlx::query_as::<_, Submission>(
            r#"
                SELECT id, user, challenge_id, timestamp
                FROM submissions;
            "#,
        )
        .fetch_all(&mut *conn)
        .await
        .unwrap()
    }

    pub async fn list_by_username(user: &String) -> Vec<Submission> {
        let mut conn = unsafe { &mut POOL }
            .get_pool()
            .await
            .unwrap()
            .acquire()
            .await
            .unwrap();

        sqlx::query_as::<_, Submission>(
            r#"
                SELECT id, user, challenge_id, timestamp
                FROM submissions
                WHERE user = ?;
            "#,
        )
        .bind(user)
        .fetch_all(&mut *conn)
        .await
        .unwrap()
    }

    pub async fn create(user: &String, challenge_id: i64) -> Submission {
        let mut conn = unsafe { &mut POOL }
            .get_pool()
            .await
            .unwrap()
            .acquire()
            .await
            .unwrap();

        sqlx::query_as::<_, Submission>(
            r#"
                INSERT INTO submissions (user, challenge_id) VALUES (?, ?) 
                RETURNING id, user, challenge_id, timestamp;
            "#,
        )
        .bind(user)
        .bind(challenge_id)
        .fetch_one(&mut *conn)
        .await
        .unwrap()
    }
}
