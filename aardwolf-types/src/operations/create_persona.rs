use aardwolf_models::{
    base_actor::{persona::Persona, BaseActor},
    generate_urls::GenerateUrls,
    user::{LocalPersonaCreator, PermissionedUser},
};
use diesel::pg::PgConnection;
use openssl::rsa::Rsa;

use crate::{
    forms::personas::{PersonaCreationFail, ValidatedPersonaCreationForm},
    traits::DbAction,
};

/// This operation creates a persona
pub struct CreatePersona<U, G>(
    pub LocalPersonaCreator<U>,
    pub ValidatedPersonaCreationForm,
    pub G,
)
where
    U: PermissionedUser,
    G: GenerateUrls;

impl<U, G> DbAction for CreatePersona<U, G>
where
    U: PermissionedUser,
    G: GenerateUrls,
{
    type Item = (BaseActor, Persona);
    type Error = PersonaCreationFail;

    fn db_action(
        self,
        conn: &mut PgConnection,
    ) -> Result<(BaseActor, Persona), PersonaCreationFail> {
        let key = Rsa::generate(2048)?;
        let priv_key = key.private_key_to_der()?;
        let pub_key = key.public_key_to_der_pkcs1()?;

        Ok(self.0.create_persona(
            self.1.display_name,
            self.1.follow_policy,
            self.1.default_visibility,
            self.1.is_searchable,
            None,
            self.1.shortname,
            priv_key,
            pub_key,
            self.2,
            conn,
        )?)
    }
}

#[cfg(test)]
mod tests {
    use aardwolf_models::{
        sql_types::{FollowPolicy, PostVisibility},
        user::PermissionedUser,
    };
    use aardwolf_test_helpers::models::{
        gen_string, make_verified_authenticated_user, with_connection, UrlGenerator,
    };

    use crate::{
        forms::personas::ValidatedPersonaCreationForm, operations::create_persona::CreatePersona,
        traits::DbAction,
    };

    #[test]
    fn create_persona_works() {
        with_connection(|conn| {
            let (user, _) = make_verified_authenticated_user(conn, &gen_string())?;
            let creator = user.can_make_persona(conn)?;

            let form = ValidatedPersonaCreationForm {
                display_name: "username".to_owned(),
                follow_policy: FollowPolicy::AutoAccept,
                default_visibility: PostVisibility::Public,
                shortname: "shortname".to_owned(),
                is_searchable: true,
            };

            let operation = CreatePersona(creator, form, UrlGenerator);

            assert!(operation.db_action(conn).is_ok());
            Ok(())
        })
    }
}
