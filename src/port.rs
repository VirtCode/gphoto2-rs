//! Camera port information
//!
//! Determine the port information used to connect to a device
//!
//! ## Getting port information
//! ```no_run
//! use gphoto2::{Context, Result};
//!
//! # fn main() -> Result<()> {
//! let context= Context::new()?;
//! let camera = context.autodetect_camera()?;
//!
//! let port_info = camera.port_info()?; // Returns a PortInfo struct
//!
//! # Ok(())
//! # }
//! ```

use crate::{
  helper::{as_ref, chars_to_string},
  try_gp_internal, Result,
};
use std::fmt;

/// Type of the port
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PortType {
  /// Serial port
  Serial,
  /// USB port
  Usb,
  /// Disk
  Disk,

  /// PTP/IP
  PTPIp,

  /// IP
  Ip,

  /// USB Disk direct
  UsbDiskDirect,

  /// USB SCSI
  UsbScsi,
}

/// Information about the port
///
/// ## Information
///  - [`name`](PortInfo::name): Name of the port
///  - [`path`](PortInfo::path): Path of the port
///  - [`port_type`](PortInfo::port_type): Type of the port
pub struct PortInfo {
  pub(crate) inner: libgphoto2_sys::GPPortInfo,
}

pub(crate) struct PortInfoList {
  pub(crate) inner: *mut libgphoto2_sys::GPPortInfoList,
}

impl Drop for PortInfoList {
  fn drop(&mut self) {
    unsafe {
      libgphoto2_sys::gp_port_info_list_free(self.inner);
    }
  }
}

impl From<libgphoto2_sys::GPPortInfo> for PortInfo {
  fn from(inner: libgphoto2_sys::GPPortInfo) -> Self {
    Self { inner }
  }
}

impl fmt::Debug for PortInfo {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("PortInfo")
      .field("name", &self.name())
      .field("path", &self.path())
      .field("port_type", &self.port_type())
      .finish()
  }
}

as_ref!(PortInfoList -> libgphoto2_sys::GPPortInfoList, *self.inner);

as_ref!(PortInfo -> libgphoto2_sys::GPPortInfo, self.inner);

impl PortType {
  fn new(port_type: libgphoto2_sys::GPPortType) -> Option<Self> {
    use libgphoto2_sys::GPPortType;

    match port_type {
      GPPortType::GP_PORT_NONE => None,
      GPPortType::GP_PORT_SERIAL => Some(Self::Serial),
      GPPortType::GP_PORT_USB => Some(Self::Usb),
      GPPortType::GP_PORT_DISK => Some(Self::Disk),
      GPPortType::GP_PORT_PTPIP => Some(Self::PTPIp),
      GPPortType::GP_PORT_IP => Some(Self::Ip),
      GPPortType::GP_PORT_USB_DISK_DIRECT => Some(Self::UsbDiskDirect),
      GPPortType::GP_PORT_USB_SCSI => Some(Self::UsbScsi),
    }
  }
}

impl PortInfo {
  /// Name of the port
  pub fn name(&self) -> String {
    try_gp_internal!(gp_port_info_get_name(self.inner, &out name).unwrap());

    chars_to_string(name)
  }

  /// Path of the port
  pub fn path(&self) -> String {
    try_gp_internal!(gp_port_info_get_path(self.inner, &out path).unwrap());

    chars_to_string(path)
  }

  /// [Port type](PortType)
  pub fn port_type(&self) -> Option<PortType> {
    try_gp_internal!(gp_port_info_get_type(self.inner, &out port_type).unwrap());

    PortType::new(port_type)
  }
}

impl PortInfoList {
  pub(crate) fn new() -> Result<Self> {
    try_gp_internal!(gp_port_info_list_new(&out port_info_list)?);
    try_gp_internal!(gp_port_info_list_load(port_info_list)?);

    Ok(Self { inner: port_info_list })
  }

  pub(crate) fn get_port_info(&self, p: i32) -> Result<PortInfo> {
    try_gp_internal!(gp_port_info_list_get_info(self.inner, p, &out port_info)?);

    Ok(port_info.into())
  }
}
