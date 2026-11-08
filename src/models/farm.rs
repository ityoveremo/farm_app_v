use serde::{Deserialize, Serialize};
use sqlx::postgres::PgTypeInfo;
use sqlx::{Postgres, Type};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FarmStatus {
    Pending,
    Approved,
    Rejected,
}

impl Type<Postgres> for FarmStatus {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("farm_status")
    }
}

impl<'r> sqlx::Decode<'r, Postgres> for FarmStatus {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        match value.as_str() {
            Ok(s) => match s {
                "pending" => Ok(FarmStatus::Pending),
                "approved" => Ok(FarmStatus::Approved),
                "rejected" => Ok(FarmStatus::Rejected),
                _ => Err(format!("Unknown farm status: {}", s).into()),
            },
            Err(e) => Err(e.into()),
        }
    }
}
