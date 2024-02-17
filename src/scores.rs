use std::collections::HashMap;

use serde::Serialize;
use sqlx::FromRow;

use crate::database;

#[derive(FromRow, Serialize, Clone, Debug)]
pub struct Score {
    pub user: String,
    pub flags: Vec<i64>,
    pub timestamp: chrono::NaiveDateTime,
}

pub async fn list() -> Vec<Score> {
    let mut scores = HashMap::<String, Score>::new();
    let submissions = database::submissions::list().await;

    for submission in submissions {
        let user = &submission.user.unwrap();

        let score = match scores.get_mut(user) {
            Some(score) => score,
            None => {
                _ = scores.insert(
                    user.to_string(),
                    Score {
                        user: user.to_string(),
                        flags: Vec::<i64>::new().to_owned(),
                        timestamp: submission.timestamp.unwrap(),
                    },
                );

                scores.get_mut(user).unwrap()
            }
        };

        score.flags.push(submission.challenge_id.unwrap());
        match score.timestamp.cmp(&submission.timestamp.unwrap()) {
            std::cmp::Ordering::Less => score.timestamp = submission.timestamp.unwrap(),
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => (),
        }
    }

    let mut scores = scores.values().cloned().collect::<Vec<Score>>();
    scores.sort_by(|a, b| {
        match b
            .flags
            .iter()
            .sum::<i64>()
            .cmp(&a.flags.iter().sum::<i64>())
        {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => a.timestamp.cmp(&b.timestamp),
        }
    });

    scores
}
