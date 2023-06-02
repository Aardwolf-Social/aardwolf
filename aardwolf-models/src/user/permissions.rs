use chrono::offset::Utc;
use diesel::{self, pg::PgConnection};
use failure::Fail;

use crate::{
    base_actor::{
        follow_request::{FollowRequest, NewFollowRequest},
        follower::{Follower, NewFollower},
        persona::{NewPersona, Persona},
        BaseActor, NewBaseActor,
    },
    base_post::{
        post::{
            comment::{Comment, NewComment},
            media_post::{MediaPost, NewMediaPost},
            {NewPost, Post},
        },
        {BasePost, NewBasePost},
    },
    file::{image::Image, File},
    generate_urls::GenerateUrls,
    sql_types::{FollowPolicy, Mime, Permission, PostVisibility, Role},
    user::UserLike,
};

#[derive(Clone, Debug, Fail)]
pub enum PermissionError {
    #[fail(display = "Failed to check user's permission")]
    Diesel,
    #[fail(display = "User doesn't have this permission")]
    Permission,
}

impl From<diesel::result::Error> for PermissionError {
    fn from(_: diesel::result::Error) -> Self {
        PermissionError::Diesel
    }
}

pub type PermissionResult<T> = Result<T, PermissionError>;

/// Define things a logged-in user is allowed to do.
///
/// The end-goal for this trait is to produce types like `PostCreator`, `UserFollower`, and
/// `InstanceConfigurator`. These types would *only* be producable through this trait, and would be
/// the only ways to perform the actions associated with the permission they came from.
///
/// This way, permission checking would be enforced by the compiler, since "making a post" or
/// "configuring the instance" would not be possible without calling these methods.
pub trait PermissionedUser: UserLike + Sized {
    fn can_post(
        &self,
        base_actor: BaseActor,
        conn: &mut PgConnection,
    ) -> PermissionResult<LocalPostCreator> {
        self.with_actor(base_actor).and_then(|actor| {
            self.has_permission(Permission::MakePost, conn)
                .map(|_| LocalPostCreator(actor))
        })
    }

    fn can_post_media(
        &self,
        base_actor: BaseActor,
        conn: &mut PgConnection,
    ) -> PermissionResult<LocalMediaPostCreator> {
        self.with_actor(base_actor).and_then(|actor| {
            self.has_permission(Permission::MakeMediaPost, conn)
                .map(|_| LocalMediaPostCreator(actor))
        })
    }

    fn can_post_comment(
        &self,
        base_actor: BaseActor,
        conn: &mut PgConnection,
    ) -> PermissionResult<LocalCommentCreator> {
        self.with_actor(base_actor).and_then(|actor| {
            self.has_permission(Permission::MakeComment, conn)
                .map(|_| LocalCommentCreator(actor))
        })
    }

    fn can_follow(
        &self,
        base_actor: BaseActor,
        conn: &mut PgConnection,
    ) -> PermissionResult<ActorFollower> {
        self.with_actor(base_actor).and_then(|actor| {
            self.has_permission(Permission::FollowUser, conn)
                .map(|_| ActorFollower(actor))
        })
    }

    fn can_make_persona(
        &self,
        conn: &mut PgConnection,
    ) -> PermissionResult<LocalPersonaCreator<Self>>
    where
        Self: Clone,
    {
        self.has_permission(Permission::MakePersona, conn)
            .map(|_| LocalPersonaCreator(self.clone()))
    }

    fn can_switch_persona(
        &self,
        persona: Persona,
        conn: &mut PgConnection,
    ) -> PermissionResult<PersonaSwitcher> {
        self.with_persona(persona, conn).and_then(|persona| {
            self.has_permission(Permission::SwitchPersona, conn)
                .map(|_| PersonaSwitcher(persona))
        })
    }

    fn can_delete_persona(
        &self,
        persona: Persona,
        conn: &mut PgConnection,
    ) -> PermissionResult<PersonaDeleter> {
        self.with_persona(persona, conn).and_then(|persona| {
            self.has_permission(Permission::DeletePersona, conn)
                .map(|_| PersonaDeleter(persona))
        })
    }

    fn can_manage_follow_requests(
        &self,
        base_actor: BaseActor,
        conn: &mut PgConnection,
    ) -> PermissionResult<FollowRequestManager> {
        self.with_actor(base_actor).and_then(|actor| {
            self.has_permission(Permission::ManageFollowRequest, conn)
                .map(|_| FollowRequestManager(actor))
        })
    }

    fn can_configure_instance(&self, conn: &mut PgConnection) -> PermissionResult<()> {
        self.has_permission(Permission::ConfigureInstance, conn)
    }

    fn can_ban_user(&self, conn: &mut PgConnection) -> PermissionResult<()> {
        self.has_permission(Permission::BanUser, conn)
    }

    fn can_block_instance(&self, conn: &mut PgConnection) -> PermissionResult<()> {
        self.has_permission(Permission::BlockInstance, conn)
    }

    fn can_grant_role(&self, conn: &mut PgConnection) -> PermissionResult<RoleGranter> {
        self.has_permission(Permission::GrantRole, conn)
            .map(|_| RoleGranter::new())
    }

    fn can_revoke_role(&self, conn: &mut PgConnection) -> PermissionResult<RoleRevoker> {
        self.has_permission(Permission::RevokeRole, conn)
            .map(|_| RoleRevoker::new())
    }

    fn with_actor(&self, base_actor: BaseActor) -> PermissionResult<BaseActor> {
        base_actor
            .local_user()
            .and_then(|id| {
                if id == self.id() {
                    Some(base_actor)
                } else {
                    None
                }
            })
            .ok_or(PermissionError::Permission)
    }

    fn with_persona(&self, persona: Persona, conn: &mut PgConnection) -> PermissionResult<Persona> {
        persona
            .belongs_to_user(self, conn)
            .map_err(|_| PermissionError::Permission)
            .and_then(|belongs| {
                if belongs {
                    Ok(persona)
                } else {
                    Err(PermissionError::Permission)
                }
            })
    }

    fn has_permission(
        &self,
        permission: Permission,
        conn: &mut PgConnection,
    ) -> PermissionResult<()> {
        use crate::schema::{permissions, role_permissions, roles, user_roles};
        use diesel::prelude::*;

        roles::dsl::roles
            .inner_join(user_roles::dsl::user_roles)
            .inner_join(role_permissions::dsl::role_permissions)
            .inner_join(
                permissions::dsl::permissions
                    .on(role_permissions::dsl::permission_id.eq(permissions::dsl::id)),
            )
            .filter(user_roles::dsl::user_id.eq(self.id()))
            .filter(permissions::dsl::name.eq(permission))
            .count()
            .get_result(conn)
            .map_err(From::from)
            .and_then(|count: i64| {
                if count > 0 {
                    Ok(())
                } else {
                    Err(PermissionError::Permission)
                }
            })
    }
}

pub struct RoleGranter(());

impl RoleGranter {
    pub(crate) fn new() -> RoleGranter {
        RoleGranter(())
    }

    pub fn grant_role<U: UserLike>(
        &self,
        user: &U,
        role: Role,
        conn: &mut PgConnection,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::{roles, user_roles};
        use diesel::prelude::*;

        if user.has_role(role, conn)? {
            return Ok(());
        }

        roles::table
            .filter(roles::dsl::name.eq(role))
            .select(roles::dsl::id)
            .get_result(conn)
            .and_then(|role_id: i32| {
                diesel::insert_into(user_roles::table)
                    .values((
                        user_roles::dsl::user_id.eq(user.id()),
                        user_roles::dsl::role_id.eq(role_id),
                        user_roles::dsl::created_at.eq(Utc::now()),
                    ))
                    .execute(conn)
                    .map(|_| ())
            })
    }
}

pub struct RoleRevoker(());

impl RoleRevoker {
    pub(crate) fn new() -> RoleRevoker {
        RoleRevoker(())
    }

    pub fn revoke_role<U: UserLike>(
        &self,
        user: &U,
        role: Role,
        conn: &mut PgConnection,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::{roles, user_roles};
        use diesel::prelude::*;

        if !user.has_role(role, conn)? {
            return Ok(());
        }

        roles::table
            .filter(roles::dsl::name.eq(role))
            .select(roles::dsl::id)
            .get_result(conn)
            .and_then(|role_id: i32| {
                let user_role = user_roles::table
                    .filter(user_roles::dsl::user_id.eq(user.id()))
                    .filter(user_roles::dsl::role_id.eq(role_id));

                diesel::delete(user_role).execute(conn)
            })
            .map(|_| ())
    }
}

pub struct LocalPostCreator(BaseActor);

impl LocalPostCreator {
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]
    pub fn create_post(
        &self,
        name: Option<String>,
        media_type: Mime,
        icon: Option<&Image>,
        visibility: PostVisibility,
        content: String,
        source: String,
        generate_id: impl GenerateUrls,
        conn: &mut PgConnection,
    ) -> Result<(BasePost, Post), diesel::result::Error> {
        use crate::schema::{base_posts, posts};
        use diesel::prelude::*;

        conn.transaction(|conn| {
            diesel::insert_into(base_posts::table)
                .values(&NewBasePost::local(
                    name,
                    media_type,
                    &self.0,
                    icon,
                    visibility,
                    generate_id,
                ))
                .get_result(conn)
                .and_then(|base_post: BasePost| {
                    diesel::insert_into(posts::table)
                        .values(&NewPost::new(content, Some(source), &base_post))
                        .get_result(conn)
                        .map(|post: Post| (base_post, post))
                })
        })
    }
}

pub struct LocalMediaPostCreator(BaseActor);

impl LocalMediaPostCreator {
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]
    pub fn make_media_post(
        &self,
        name: Option<String>,
        media_type: Mime,
        icon: Option<&Image>,
        visibility: PostVisibility,
        content: String,
        source: String,
        media: &File,
        generate_id: impl GenerateUrls,
        conn: &mut PgConnection,
    ) -> Result<(BasePost, Post, MediaPost), diesel::result::Error> {
        use crate::schema::media_posts;
        use diesel::prelude::*;

        conn.transaction(|conn| {
            LocalPostCreator(self.0.clone())
                .create_post(
                    name,
                    media_type,
                    icon,
                    visibility,
                    content,
                    source,
                    generate_id,
                    conn,
                )
                .and_then(|(base_post, post)| {
                    diesel::insert_into(media_posts::table)
                        .values(&NewMediaPost::new(media, &post))
                        .get_result(conn)
                        .map(|media_post: MediaPost| (base_post, post, media_post))
                })
        })
    }
}

pub struct LocalCommentCreator(BaseActor);

impl LocalCommentCreator {
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]
    pub fn make_comment(
        &self,
        name: Option<String>,
        media_type: Mime,
        icon: Option<&Image>,
        visibility: PostVisibility,
        content: String,
        source: String,
        conversation: &Post,
        parent: &Post,
        generate_id: impl GenerateUrls,
        conn: &mut PgConnection,
    ) -> Result<(BasePost, Post, Comment), CommentError> {
        use crate::schema::{base_posts, comments};
        use diesel::prelude::*;

        let conversation_base: BasePost = base_posts::table
            .filter(base_posts::dsl::id.eq(conversation.base_post()))
            .get_result(conn)?;

        if !conversation_base.is_visible_by(&self.0, conn)? {
            return Err(CommentError::Permission);
        }

        if parent.id() != conversation.id() {
            let parent_base: BasePost = base_posts::table
                .filter(base_posts::dsl::id.eq(parent.base_post()))
                .get_result(conn)?;

            if !parent_base.is_visible_by(&self.0, conn)? {
                return Err(CommentError::Permission);
            }
        }

        conn.transaction(|conn| {
            LocalPostCreator(self.0.clone())
                .create_post(
                    name,
                    media_type,
                    icon,
                    visibility,
                    content,
                    source,
                    generate_id,
                    conn,
                )
                .and_then(|(base_post, post)| {
                    diesel::insert_into(comments::table)
                        .values(NewComment::new(conversation, parent, &post))
                        .get_result(conn)
                        .map(|comment: Comment| (base_post, post, comment))
                })
        })
        .map_err(From::from)
    }
}

#[derive(Debug, Fail)]
pub enum CommentError {
    #[fail(display = "Error creating comment")]
    Diesel(diesel::result::Error),
    #[fail(display = "Not allowed to comment on provided post")]
    Permission,
}

impl From<diesel::result::Error> for CommentError {
    fn from(e: diesel::result::Error) -> Self {
        CommentError::Diesel(e)
    }
}

pub struct ActorFollower(BaseActor);

impl ActorFollower {
    pub fn follow_actor(
        &self,
        target_actor: &BaseActor,
        conn: &mut PgConnection,
    ) -> Result<FollowRequest, FollowError> {
        use crate::schema::follow_requests;
        use diesel::prelude::*;

        match target_actor.follow_policy() {
            FollowPolicy::AutoAccept | FollowPolicy::ManualReview => {
                diesel::insert_into(follow_requests::table)
                    .values(&NewFollowRequest::new(&self.0, target_actor))
                    .get_result(conn)
                    .map_err(From::from)
            }
            FollowPolicy::AutoReject => Err(FollowError::Reject),
        }
    }
}

#[derive(Debug, Fail)]
pub enum FollowError {
    #[fail(display = "Error creating follow request")]
    Diesel(#[cause] diesel::result::Error),
    #[fail(display = "Target actor is not accepting follow requests")]
    Reject,
}

impl From<diesel::result::Error> for FollowError {
    fn from(e: diesel::result::Error) -> Self {
        FollowError::Diesel(e)
    }
}

pub struct FollowRequestManager(BaseActor);

impl FollowRequestManager {
    pub fn accept_follow_request(
        &self,
        follow_request: FollowRequest,
        conn: &mut PgConnection,
    ) -> Result<Follower, FollowRequestManagerError> {
        use crate::schema::followers;
        use diesel::prelude::*;

        if follow_request.requested_follow() != self.0.id() {
            return Err(FollowRequestManagerError::IdMismatch);
        }

        conn.transaction(|conn| {
            diesel::delete(&follow_request)
                .execute(conn)
                .and_then(|_| {
                    diesel::insert_into(followers::table)
                        .values(&NewFollower::from(follow_request))
                        .get_result(conn)
                })
                .map_err(From::from)
        })
    }

    pub fn reject_follow_request(
        &self,
        follow_request: &FollowRequest,
        conn: &mut PgConnection,
    ) -> Result<(), FollowRequestManagerError> {
        use diesel::prelude::*;

        if follow_request.requested_follow() != self.0.id() {
            return Err(FollowRequestManagerError::IdMismatch);
        }

        diesel::delete(follow_request)
            .execute(conn)
            .map(|_| ())
            .map_err(From::from)
    }
}

#[derive(Debug, Fail)]
pub enum FollowRequestManagerError {
    #[fail(display = "Error managing follow request")]
    Diesel(#[cause] diesel::result::Error),
    #[fail(display = "Cannot manage other actor's follow requests")]
    IdMismatch,
}

impl From<diesel::result::Error> for FollowRequestManagerError {
    fn from(e: diesel::result::Error) -> Self {
        FollowRequestManagerError::Diesel(e)
    }
}

pub struct LocalPersonaCreator<U: UserLike>(U);

impl<U: UserLike> LocalPersonaCreator<U> {
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]
    pub fn create_persona(
        &self,
        display_name: String,
        follow_policy: FollowPolicy,
        default_visibility: PostVisibility,
        is_searchable: bool,
        avatar: Option<&Image>,
        shortname: String,
        private_key_der: Vec<u8>,
        public_key_der: Vec<u8>,
        generate_id: impl GenerateUrls,
        conn: &mut PgConnection,
    ) -> Result<(BaseActor, Persona), diesel::result::Error> {
        use diesel::Connection;

        conn.transaction(|conn| {
            NewBaseActor::local(
                display_name,
                &self.0,
                follow_policy,
                private_key_der,
                public_key_der,
                generate_id,
            )
            .insert(conn)
            .and_then(|base_actor| {
                NewPersona::new(
                    default_visibility,
                    is_searchable,
                    avatar,
                    shortname,
                    &base_actor,
                )
                .insert(conn)
                .and_then(|persona| {
                    use crate::schema::users::dsl::{id, primary_persona, users};
                    use diesel::prelude::*;

                    if self.0.primary_persona().is_none() {
                        diesel::update(users.filter(id.eq(self.0.id())))
                            .set(primary_persona.eq(persona.id()))
                            .execute(conn)
                            .map(|_| (base_actor, persona))
                    } else {
                        Ok((base_actor, persona))
                    }
                })
            })
        })
    }
}

pub struct PersonaDeleter(Persona);

impl PersonaDeleter {
    pub fn delete_persona(self, conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
        self.0.delete(conn)
    }
}

pub struct PersonaSwitcher(Persona);

impl PersonaSwitcher {
    pub fn switch_persona(self) -> i32 {
        self.0.id()
    }
}
