use crate::database;

#[derive(Debug)]
pub enum FlagError {
    InvalidFlag,
    AlreadySubmitted,
}

pub async fn submit(
    flag: &String,
    user: &String,
) -> Result<database::submissions::Submission, FlagError> {
    let challenge = database::challenges::list_by_flag(flag).await;
    match challenge {
        None => Err(FlagError::InvalidFlag),
        Some(challenge) => {
            if database::submissions::list_by_username(user)
                .await
                .iter()
                .filter(|submission| submission.challenge_id == challenge.id)
                .collect::<Vec<&database::submissions::Submission>>()
                .len()
                > 0
            {
                Err(FlagError::AlreadySubmitted)
            } else {
                Ok(database::submissions::create(user, challenge.id.unwrap()).await)
            }
        }
    }
}
