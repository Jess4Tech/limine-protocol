use core::ffi::CStr;

use super::uuid::UUID;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq)]
/// File types
pub enum FileType {
    /// Generic file type
    Generic = 0,
    /// Optical drive
    Optical = 1,
    /// Remote server
    TFTP = 2,
    /// Unknown
    Unknown,
}

impl From<u32> for FileType {
    fn from(v: u32) -> Self {
        match v {
            0 => Self::Generic,
            1 => Self::Optical,
            2 => Self::TFTP,
            _ => Self::Unknown,
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
/// File structure
pub struct File<'a> {
    /// Revision of the File structure
    pub revision: u64,
    /// The address of the file
    pub address: *const u8,
    /// The size of the file
    pub size: u64,
    /// The path of the file
    pub path: &'a CStr,
    /// A command line associated with the file
    pub cmdline: &'a CStr,
    /// The kind of media the file is on
    pub media_type: FileType,
    /// Unused
    unused: u32,
    /// IP of the TFTP server, if one was used
    pub tftp_ip: u32,
    /// Port of the TFTP server, if one was used
    pub tftp_port: u32,
    /// !-based partition index of the volume from which the file was loaded
    pub partition_index: u32,
    /// If non-zero, the ID of the disk from the MBR
    pub mbr_disk_id: u32,
    /// The UUID of the disk from which the file was loaded from GPT, if it is non-zero
    pub gpt_disk_uuid: UUID,
    /// The UUID of the partition from which the file was loaded from GPT, if it is non-zero
    pub gpt_part_uuid: UUID,
    /// The UUID of the filesystem frmo which the file was laoded from GPT, if it is non-zero,
    pub part_uuid: UUID,
}
