use aardwolf_models::{
    base_post::{post::Post, BasePost},
    generate_urls::GenerateUrls,
    user::LocalPostCreator,
};
use diesel::pg::PgConnection;

use crate::{
    forms::posts::ValidatedPostCreationForm,
    traits::DbAction,
    wrapper::{DbActionWrapper, Wrapped},
};

/// This operation creates a post
pub struct CreatePost<G>(pub LocalPostCreator, pub ValidatedPostCreationForm, pub G)
where
    G: GenerateUrls;

impl<G> Wrapped for CreatePost<G>
where
    G: GenerateUrls,
{
    type Wrapper = DbActionWrapper<Self, <Self as DbAction>::Item, <Self as DbAction>::Error>;
}

impl<G> DbAction for CreatePost<G>
where
    G: GenerateUrls,
{
    type Item = (BasePost, Post);
    type Error = CreatePostFail;

    fn db_action(self, conn: &PgConnection) -> Result<(BasePost, Post), CreatePostFail> {
        Ok(self.0.create_post(
            None,
            self.1.media_type,
            None,
            self.1.visibility,
            self.1.content,
            self.1.source,
            self.2,
            conn,
        )?)
    }
}

#[derive(Clone, Debug, Fail)]
pub enum CreatePostFail {
    #[fail(display = "Error in db")]
    Database,
}

impl From<diesel::result::Error> for CreatePostFail {
    fn from(_: diesel::result::Error) -> Self {
        CreatePostFail::Database
    }
}

#[cfg(test)]
mod tests {
    use aardwolf_models::{sql_types::PostVisibility, user::PermissionedUser};
    use aardwolf_test_helpers::models::{
        gen_string, make_verified_authenticated_user, user_with_base_actor, with_connection,
        UrlGenerator,
    };

    use crate::{
        forms::posts::ValidatedPostCreationForm, operations::create_post::CreatePost,
        traits::DbAction,
    };

    #[test]
    fn create_post_works() {
        with_connection(|conn| {
            make_verified_authenticated_user(conn, &gen_string()?, |user, _| {
                user_with_base_actor(conn, &user, |base_actor| {
                    let creator = user.can_post(base_actor, conn)?;

                    let form = ValidatedPostCreationForm {
                        media_type: "text/plain".parse()?,
                        visibility: PostVisibility::Public,
                        content: "<b>A Post</b>".to_owned(),
                        source: "**A Post**".to_owned(),
                    };

                    let operation = CreatePost(creator, form, UrlGenerator);

                    assert!(operation.db_action(conn).is_ok());
                    Ok(())
                })
            })
        })
    }
}
