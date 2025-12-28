pub enum UserStatuses {
    Pending = 1,
    Actived = 2,
    Deleted = 3,
}

impl TryFrom<i32> for UserStatuses {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(UserStatuses::Pending),
            2 => Ok(UserStatuses::Actived),
            3 => Ok(UserStatuses::Deleted),
            _ => Err(()),
        }
    }
}
