use chrono::{offset::Utc, DateTime};
use diesel::{self, pg::PgConnection};

use crate::{
    base_actor::BaseActor, file::image::Image, schema::personas, sql_types::PostVisibility,
    user::UserLike,
};

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[diesel(table_name = personas)]
pub struct Persona {
    id: i32,
    default_visibility: PostVisibility,
    is_searchable: bool,
    avatar: Option<i32>, // foreign key to Image
    shortname: String,   // wtf is a SlugField
    base_actor: i32,     // foreign key to BaseActor
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Persona {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn default_visibility(&self) -> PostVisibility {
        self.default_visibility
    }

    pub fn is_searchable(&self) -> bool {
        self.is_searchable
    }

    pub fn avatar(&self) -> Option<i32> {
        self.avatar
    }

    pub fn shortname(&self) -> &str {
        &self.shortname
    }

    pub fn base_actor(&self) -> i32 {
        self.base_actor
    }

    pub fn by_id(id: i32, conn: &mut PgConnection) -> Result<Persona, diesel::result::Error> {
        use diesel::prelude::*;

        personas::table.find(id).first(conn)
    }

    pub fn belongs_to_user<U: UserLike>(
        &self,
        user: &U,
        conn: &mut PgConnection,
    ) -> Result<bool, diesel::result::Error> {
        use crate::schema::base_actors;
        use diesel::prelude::*;

        personas::table
            .inner_join(base_actors::table)
            .filter(personas::dsl::id.eq(self.id))
            .filter(base_actors::dsl::local_user.eq(user.id()))
            .select(personas::dsl::id)
            .get_result(conn)
            .map(|_: i32| true)
            .or_else(|e| match e {
                diesel::result::Error::NotFound => Ok(false),
                e => Err(e),
            })
    }

    pub fn delete(self, conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::prelude::*;

        diesel::delete(personas::table)
            .filter(personas::dsl::id.eq(self.id))
            .execute(conn)
            .map(|_| ())
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Insertable)]
#[diesel(table_name = personas)]
pub struct NewPersona {
    default_visibility: PostVisibility,
    is_searchable: bool,
    avatar: Option<i32>,
    shortname: String,
    base_actor: i32,
}

impl NewPersona {
    pub fn insert(self, conn: &mut PgConnection) -> Result<Persona, diesel::result::Error> {
        use diesel::prelude::*;

        diesel::insert_into(personas::table)
            .values(&self)
            .get_result(conn)
    }

    pub fn new(
        default_visibility: PostVisibility,
        is_searchable: bool,
        avatar: Option<&Image>,
        shortname: String,
        base_actor: &BaseActor,
    ) -> Self {
        NewPersona {
            default_visibility,
            is_searchable,
            avatar: avatar.map(|a| a.id()),
            shortname,
            base_actor: base_actor.id(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::*;

    #[test]
    fn create_persona() {
        with_connection(|conn| {
            let base_actor = make_base_actor(conn)?;
            let persona = make_persona(conn, &base_actor);

            assert!(persona.is_ok());

            Ok(())
        })
    }
}
