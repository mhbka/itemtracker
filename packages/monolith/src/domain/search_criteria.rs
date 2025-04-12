use std::io::Write;
use diesel::{deserialize::{self, FromSql, FromSqlRow}, expression::AsExpression, pg::{Pg, PgValue}, serialize::{self, Output, ToSql}, sql_types::Jsonb};
use serde::{Serialize, Deserialize};

/// The search criteria used for all marketplaces within the gallery.
#[derive(Clone, Debug, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = Jsonb)]
pub struct SearchCriteria {
    pub keyword: String,
    pub exclude_keyword: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_price: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_price: Option<f32>,
}

// So that we can directly write to/pull from SQL
impl FromSql<Jsonb, Pg> for SearchCriteria {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let val: serde_json::Value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?;
        serde_json::from_value(val).map_err(|e| e.into())
    }
}

// ^^
impl ToSql<Jsonb, Pg> for SearchCriteria {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let json = serde_json::to_string(self)?;
        out.write_all(json.as_bytes())?;
        Ok(serialize::IsNull::No)
    }
}
