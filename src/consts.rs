#[cfg(feature = "system-files")]
pub const RESOURCES_DIR: &str = "/usr/share/helper/";

#[cfg(feature = "system-files")]
pub const DOCUMENTATION_INFO_FILE: &str = "/usr/share/helper/documentation.toml";

#[cfg(feature = "system-files")]
pub const CONFIG_FILE: &str = "/etc/helper.conf";

#[cfg(feature = "local-files")]
pub const RESOURCES_DIR: &str = "./res/";

#[cfg(feature = "local-files")]
pub const DOCUMENTATION_INFO_FILE: &str = "./res/documentation.toml";

#[cfg(feature = "local-files")]
pub const CONFIG_FILE: &str = "./res/helper.conf";
