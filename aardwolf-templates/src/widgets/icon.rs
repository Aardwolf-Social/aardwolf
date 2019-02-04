use std::fmt;

impl Icon {
    pub fn angle_down() -> Self {
        Icon {
            name: IconName::AngleDown,
            size: IconSize::Normal,
        }
    }
    pub fn beer() -> Self {
        Icon {
            name: IconName::Beer,
            size: IconSize::Normal,
        }
    }
    pub fn bell() -> Self {
        Icon {
            name: IconName::Bell,
            size: IconSize::Normal,
        }
    }
    pub fn bookmark() -> Self {
        Icon {
            name: IconName::Bookmark,
            size: IconSize::Normal,
        }
    }
    pub fn calendar() -> Self {
        Icon {
            name: IconName::Calendar,
            size: IconSize::Normal,
        }
    }
    pub fn calendar_plus_o() -> Self {
        Icon {
            name: IconName::CalendarPlusOutline,
            size: IconSize::Normal,
        }
    }
    pub fn chevron_left() -> Self {
        Icon {
            name: IconName::ChevronLeft,
            size: IconSize::Normal,
        }
    }
    pub fn cloud() -> Self {
        Icon {
            name: IconName::Cloud,
            size: IconSize::Normal,
        }
    }
    pub fn cloud_download() -> Self {
        Icon {
            name: IconName::CloudDownload,
            size: IconSize::Normal,
        }
    }
    pub fn cloud_upload() -> Self {
        Icon {
            name: IconName::CloudUpload,
            size: IconSize::Normal,
        }
    }
    pub fn ellipsis_h() -> Self {
        Icon {
            name: IconName::EllipsisH,
            size: IconSize::Normal,
        }
    }
    pub fn envelope() -> Self {
        Icon {
            name: IconName::Envelope,
            size: IconSize::Normal,
        }
    }
    pub fn gears() -> Self {
        Icon {
            name: IconName::Gears,
            size: IconSize::Normal,
        }
    }
    pub fn github() -> Self {
        Icon {
            name: IconName::Github,
            size: IconSize::Normal,
        }
    }
    pub fn lightbulb_o() -> Self {
        Icon {
            name: IconName::LightbulbOutline,
            size: IconSize::Normal,
        }
    }
    pub fn list() -> Self {
        Icon {
            name: IconName::List,
            size: IconSize::Normal,
        }
    }
    pub fn logout() -> Self {
        Icon {
            name: IconName::Logout,
            size: IconSize::Normal,
        }
    }
    pub fn lok() -> Self {
        Icon {
            name: IconName::Lok,
            size: IconSize::Normal,
        }
    }
    pub fn newspaper() -> Self {
        Icon {
            name: IconName::Newspaper,
            size: IconSize::Normal,
        }
    }
    pub fn newspaper_o() -> Self {
        Icon {
            name: IconName::NewspaperOutline,
            size: IconSize::Normal,
        }
    }
    pub fn picture_o() -> Self {
        Icon {
            name: IconName::PictureOutline,
            size: IconSize::Normal,
        }
    }
    pub fn sliders() -> Self {
        Icon {
            name: IconName::Sliders,
            size: IconSize::Normal,
        }
    }
    pub fn star() -> Self {
        Icon {
            name: IconName::Star,
            size: IconSize::Normal,
        }
    }
    pub fn upload() -> Self {
        Icon {
            name: IconName::Upload,
            size: IconSize::Normal,
        }
    }
    pub fn user() -> Self {
        Icon {
            name: IconName::User,
            size: IconSize::Normal,
        }
    }
    pub fn user_circle() -> Self {
        Icon {
            name: IconName::UserCircle,
            size: IconSize::Normal,
        }
    }
    pub fn user_plus() -> Self {
        Icon {
            name: IconName::UserPlus,
            size: IconSize::Normal,
        }
    }
    pub fn users() -> Self {
        Icon {
            name: IconName::Users,
            size: IconSize::Normal,
        }
    }
    pub fn volume_off() -> Self {
        Icon {
            name: IconName::VolumeOff,
            size: IconSize::Normal,
        }
    }

    pub fn large(mut self) -> Self {
        self.size = IconSize::Large
    }

    //  What is supposed to go here?
}

/// Presuming that all ForkAwesome icons follow the same prefix:
/// `fa fa-[name]
/// All values of [name] are as below
/// To add new icons simply find the desired names within ForkAwesome,
/// and follow the syntax below:
/// `IconName::[SnakeCaseIconName] => write!(f, "actual-name"), // Normal icon`
/// `IconName::[SnakeCaseIconNameOutline] => write!(f, "actual-name-o"), // Outline icon`
///
/// Keeping them in alphabetical order would also be nice ;)

impl fmt::Display for IconName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IconName::AngleDown => write!(f, "angle-down"),
            IconName::Beer => write!(f, "beer"),
            IconName::Bell => write!(f, "bell"),
            IconName::Bookmark => write!(f, "bookmark"),
            IconName::Calendar => write!(f, "calendar"),
            IconName::CalendarPlusOutline => write!(f, "calendar-plus-o"),
            IconName::ChevronLeft => write!(f, "chevron-left"),
            IconName::Cloud => write!(f, "cloud"),
            IconName::CloudDownload => write!(f, "cloud-download"),
            IconName::CloudUpload => write!(f, "cloud-upload"),
            IconName::EllipsisH => write!(f, "ellipsis-h"),
            IconName::Envelope => write!(f, "envelope"),
            IconName::Gears => write!(f, "gears"),
            IconName::Github => write!(f, "github"),
            IconName::LightbulbOutline => write!(f, "lightbulb-o"),
            IconName::List => write!(f, "list"),
            IconName::Logout => write!(f, "logout"),
            IconName::Lok => write!(f, "lok"),
            IconName::Newspaper => write!(f, "newspaper"),
            IconName::NewspaperOutline => write!(f, "newspaper-o"),
            IconName::PictureOutline => write!(f, "picture-o"),
            IconName::Sliders => write!(f, "sliders"),
            IconName::Star => write!(f, "star"),
            IconName::Upload => write!(f, "upload"),
            IconName::User => write!(f, "user"),
            IconName::UserCircle => write!(f, "user-circle"),
            IconName::UserPlus => write!(f, "user-plus"),
            IconName::Users => write!(f, "users"),
            IconName::VolumeOff => write!(f, "volume-off"),
        }
    }
}
