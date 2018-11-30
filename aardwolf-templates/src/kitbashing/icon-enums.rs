/// Presuming that all ForkAwesome icons follow the same prefix:
/// `fa fa-[name]
/// All values of [name] are as below
/// To add new icons simply find the desired names within ForkAwesome, 
/// and follow the syntax below:
/// `IconName::[SnakeCaseIconName] => write!(f, "actual-name"), // Normal icon`
/// `IconName::[SnakeCaseIconNameOutline] => write!(f, "actual-name-o"), // Outline icon`
///
/// Keeping them in alphabetical order would also be nice ;)


use std::fmt;
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
			IconName::VolumeOff => write!(f, "volume-off")
        }
    }
}
