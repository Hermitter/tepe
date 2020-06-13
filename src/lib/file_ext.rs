use std::collections::HashMap;
use std::ffi::OsStr;

/// Common types of file groups Telegram can send through a specific API.
#[derive(Debug, Eq, PartialEq)]
pub enum FileGroup {
    Animation,
    Video,
    Photo,
    Audio,
    /// The default file group
    Document,
}

// TODO: look at Telegram's docs to properly group these file extensions
lazy_static! {
    /// A collection of extensions belonging to a `FileGroup`.
    pub static ref FILE_EXT_HASHMAP: HashMap<&'static OsStr, FileGroup> = {
        let mut ext = HashMap::new();
        ext.insert(OsStr::new("gif"), FileGroup::Animation);

        ext.insert(OsStr::new("mp4"), FileGroup::Video);
        ext.insert(OsStr::new("mov"), FileGroup::Video);
        ext.insert(OsStr::new("avi"), FileGroup::Video);
        ext.insert(OsStr::new("flv"), FileGroup::Video);
        ext.insert(OsStr::new("mkv"), FileGroup::Video);
        ext.insert(OsStr::new("achd"), FileGroup::Video);

        ext.insert(OsStr::new("wav"), FileGroup::Audio);
        ext.insert(OsStr::new("mp3"), FileGroup::Audio);
        ext.insert(OsStr::new("m4a"), FileGroup::Audio);
        ext.insert(OsStr::new("wma"), FileGroup::Audio);
        ext.insert(OsStr::new("aac"), FileGroup::Audio);

        ext.insert(OsStr::new("jpg"), FileGroup::Photo);
        ext.insert(OsStr::new("png"), FileGroup::Photo);

        ext
    };
}
