use aardwolf_models::sql_types::{FollowPolicy, PostVisibility};
use gettext::Catalog;
use gettext_macros::i18n;

pub struct InputSelect<'a> {
    pub(crate) name: &'a str,
    pub(crate) label: String,
    pub(crate) selected: String,
    pub(crate) options: Vec<SelectOption<'a>>,
    pub(crate) error: Option<String>,
}

impl<'a> InputSelect<'a> {
    pub(crate) fn follow_policy_options(catalog: &Catalog) -> Vec<SelectOption> {
        vec![
            SelectOption {
                value: FollowPolicy::AutoAccept.as_str(),
                display: i18n!(catalog, "Automatically accept new followers"),
            },
            SelectOption {
                value: FollowPolicy::AutoReject.as_str(),
                display: i18n!(catalog, "Automatically reject new followers"),
            },
            SelectOption {
                value: FollowPolicy::ManualReview.as_str(),
                display: i18n!(catalog, "Manually review new followers"),
            },
        ]
    }

    pub(crate) fn visibility_options(catalog: &Catalog) -> Vec<SelectOption> {
        vec![
            SelectOption {
                value: PostVisibility::Public.as_str(),
                display: i18n!(catalog, "Visible to everyone"),
            },
            SelectOption {
                value: PostVisibility::FollowersOnly.as_str(),
                display: i18n!(catalog, "Visible to followers"),
            },
            SelectOption {
                value: PostVisibility::FriendsOnly.as_str(),
                display: i18n!(catalog, "Visible to mutuals"),
            },
            SelectOption {
                value: PostVisibility::ListedPeopleOnly.as_str(),
                display: i18n!(catalog, "Only visible to mentioned users"),
            },
        ]
    }
}

pub struct SelectOption<'a> {
    pub(crate) value: &'a str,
    pub(crate) display: String,
}
