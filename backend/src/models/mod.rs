//! Database models.

use super::schema::users;
use crate::json::{Employee, Person, Return, Student};
use diesel::{
    backend::RawValue,
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    prelude::*,
    serialize::{self, Output, ToSql},
    sql_types::{Integer, SqlType},
};
use serde::{
    de::{self, Deserializer, Visitor},
    Deserialize, Serialize,
};
use std::{fmt, time::SystemTime};

/// UCLouvainID unique identifier for UCLouvain members,
/// and can be aliased as FGS, NOMA, etc.
///
/// UCLouvainID is an 8-digits string, which can contain leading zeros.
/// `UCLouvainID::to_string` solve this problem by appending leading zeros if
/// needed, but uses one `u32` for storage, which is far better than a string.
///
/// ```rust
/// # use backend::models::UCLouvainID;
/// let id = UCLouvainID::new_unchecked(12345678);
/// assert_eq!("12345678", id.to_string());
///
/// // Accepts leading zeros;
/// let id = UCLouvainID::new_unchecked(00123456);
/// assert_eq!("00123456", id.to_string());
#[derive(Clone, Debug, PartialEq, Eq, FromSqlRow, AsExpression, Serialize)]
#[diesel(sql_type = Integer)]
pub struct UCLouvainID(u32);

/// Identifier for each UCLouvain member (employees and students).
pub type FGS = UCLouvainID;
/// Identifier for each UCLouvain student.
pub type NOMA = UCLouvainID;

impl UCLouvainID {
    /// Maximal value, as ids are always 8-digits long strings.
    pub const MAX: u32 = 99_999_999;

    #[inline(always)]
    /// Creates a new [`UCLouvainID`] from a given id,
    /// and never checks if `id` is greater than [`UCLouvainID::MAX`].
    pub fn new_unchecked(id: u32) -> Self {
        UCLouvainID(id)
    }
}

impl fmt::Display for UCLouvainID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:08}", self.0)
    }
}

impl std::convert::TryFrom<u64> for UCLouvainID {
    type Error = &'static str;

    #[inline(always)]
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value > (Self::MAX as u64) {
            Err("id cannot be greater than 99_999_999")
        } else {
            Ok(Self(value as u32))
        }
    }
}

// From: https://stackoverflow.com/a/72282900.
impl ToSql<Integer, Pg> for UCLouvainID {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        <i32 as ToSql<Integer, Pg>>::to_sql(&(self.0 as i32), &mut out.reborrow())
    }
}

impl FromSql<Integer, Pg> for UCLouvainID {
    fn from_sql(bytes: RawValue<Pg>) -> deserialize::Result<Self> {
        <i32 as FromSql<Integer, Pg>>::from_sql(bytes)
            .map(|id| UCLouvainID::new_unchecked(id as u32))
    }
}

struct UCLouvainIDVisitor;

impl<'de> Visitor<'de> for UCLouvainIDVisitor {
    type Value = UCLouvainID;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an unsigned integer between 0 and 99_999_999 included")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value.try_into().map_err(de::Error::custom)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value
            .parse()
            .map_err(de::Error::custom)
            .and_then(|id: u64| id.try_into().map_err(de::Error::custom))
    }
}

impl<'de> Deserialize<'de> for UCLouvainID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(UCLouvainIDVisitor)
    }
}

/// UCLouvain user, with minimal information.
#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: i32,
    pub fgs: FGS,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub created_at: SystemTime,
    pub last_seen_at: SystemTime,
}

impl User {
    pub fn new(fgs: FGS, firstname: String, lastname: String, email: String) -> Self {
        todo!()
    }
}

impl From<Employee> for User {
    fn from(employee: Employee) -> Self {
        let Person {
            matric_fgs,
            firstname,
            lastname,
            email,
        } = employee.person;

        Self::new(matric_fgs, firstname, lastname, email)
    }
}

impl From<Student> for User {
    fn from(student: Student) -> Self {
        let Return {
            matric_fgs,
            firstname,
            lastname,
            email,
        } = student.lire_dossier_etudiant_response._return;

        Self::new(matric_fgs, firstname, lastname, email)
    }
}
