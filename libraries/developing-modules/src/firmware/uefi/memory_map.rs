// Much of this is inspired by the uefi-rs crate: https://github.com/rust-osdev/uefi-rs

use core::{mem, ptr, slice, ffi::c_void};

type UefiStatus = u64;
type UefiHandle = *const c_void;

const UEFI_ERROR_BIT: UefiStatus = 1 << (core::mem::size_of::<u64>() * 8 - 1);
const SUCCESS: UefiStatus = 0;
const BUFFER_TOO_SMALL: UefiStatus = UEFI_ERROR_BIT | 5;

// These are types that will be filled in later, as they are unused for now
type TodoStruct = *const c_void;
type TodoFunction = unsafe extern "efiapi" fn();
type PhysicalAddr = *const c_void;
type VirtualAddr = *const c_void;

#[repr(C)]
pub struct UefiTableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc: u32,
    _reserved: u32,
}

#[repr(C)]
pub struct UefiSystemTable {
    header: UefiTableHeader,
    vendor: *const u16,
    revision: u32,
    console_in_handle: UefiHandle,
    console_in: TodoStruct,
    console_out_handle: UefiHandle,
    console_out: TodoStruct,
    stderr_handle: UefiHandle,
    stderr: TodoStruct,
    runtime_services: TodoStruct,
    boot_services: *const UefiBootServices,
    table_entry_count: u64,
    config_table: TodoStruct,
}

impl UefiSystemTable {
    pub fn boot_services(&self) -> *const UefiBootServices {
        self.boot_services
    }
}

#[repr(C)]
pub struct UefiBootServices {
    header: UefiTableHeader,
    raise_tpl: TodoFunction,
    restore_tpl: TodoFunction,
    allocate_pages: TodoFunction,
    free_pages: TodoFunction,
    get_memory_map: unsafe extern "efiapi" fn(
        map_size: &mut u32,
        memory_map: *mut UefiMemoryDescriptor,
        map_key: &mut u64,
        descriptor_size: &mut u64,
        descriptor_version: &mut u32,
    ) -> UefiStatus,
    allocate_pool: unsafe extern "efiapi" fn(
        pool_type: UefiMemoryType,
        size: u64,
        buffer: *mut *mut c_void,
    ) -> UefiStatus,
    free_pool: TodoFunction,
    create_event: TodoFunction,
    set_timer: TodoFunction,
    wait_for_event: TodoFunction,
    signal_event: TodoFunction,
    close_event: TodoFunction,
    check_event: TodoFunction,
    install_protocol_interface: TodoFunction,
    reinstall_protocol_interface: TodoFunction,
    uninstall_protocol_interface: TodoFunction,
    handle_protocol: TodoFunction,
    register_protocol_notify: TodoFunction,
    locate_handle: TodoFunction,
    locate_device_path: TodoFunction,
    install_configuration_table: TodoFunction,
    load_image: TodoFunction,
    start_image: TodoFunction,
    exit: TodoFunction,
    unload_image: TodoFunction,
    exit_boot_services: TodoFunction,
    get_next_monotonic_count: TodoFunction,
    stall: TodoFunction,
    set_watchdog_timer: TodoFunction,
    connect_controller: TodoFunction,
    disconnect_controller: TodoFunction,
    open_protocol: TodoFunction,
    close_protocol: TodoFunction,
    open_protocol_information: TodoFunction,
    protocols_per_handle: TodoFunction,
    locate_handle_buffer: TodoFunction,
    locate_protocol: TodoFunction,
    install_multiple_protocol_interfaces: TodoFunction,
    uninstall_multiple_protocol_interfaces: TodoFunction,
    calculate_crc32: TodoFunction,
    copy_mem: TodoFunction,
    set_mem: TodoFunction,
    create_event_ex: TodoFunction,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UefiMemoryDescriptor {
    memory_type: u32,
    physical_start: PhysicalAddr,
    virtual_start: VirtualAddr,
    page_count: u64,
    attribute: u64,
}

#[repr(transparent)]
pub struct UefiMemoryType(pub u32);

const LOADER_DATA: UefiMemoryType = UefiMemoryType(2);

#[derive(Debug)]
pub struct UefiMemoryMap<'buf> {
    map_key: u64,
    map: &'buf mut [UefiMemoryDescriptor],
}

impl UefiBootServices {
    pub unsafe fn memory_map_size(&self) -> Result<usize, UefiStatus> {
        let mut map_size: u32 = 0;
        let mut map_key: u64 = 0;
        let mut descriptor_size: u64 = 0;
        let mut descriptor_version: u32 = 0;

        let status = (self.get_memory_map)(
            &mut map_size,
            ptr::null_mut(),
            &mut map_key,
            &mut descriptor_size,
            &mut descriptor_version,
        );

        if status != BUFFER_TOO_SMALL {
            return Err(status);
        }

        Ok(map_size as usize)
    }

    pub unsafe fn retrieve_memory_map<'buf>(&self, buf: &'buf mut [u8]) -> Result<UefiMemoryMap<'buf>, UefiStatus> {
        let mut map_size: u32 = 0;
        let mut map_key: u64 = 0;
        let mut descriptor_size: u64 = 0;
        let mut descriptor_version: u32 = 0;

        let status = (self.get_memory_map)(
            &mut map_size,
            ptr::null_mut(),
            &mut map_key,
            &mut descriptor_size,
            &mut descriptor_version,
        );

        if status != SUCCESS {
            return Err(status);
        }

        map_size += (mem::size_of::<UefiMemoryDescriptor>() * 2) as u32;

        let mut buffer: *mut c_void = ptr::null_mut();
        let status = (self.allocate_pool)(
            LOADER_DATA,
            map_size as u64,
            &mut buffer
        );

        if status != SUCCESS {
            return Err(status);
        }

        Ok(UefiMemoryMap {
            map_key,
            map: slice::from_raw_parts_mut(buffer as *mut UefiMemoryDescriptor, map_size as usize / mem::size_of::<UefiMemoryDescriptor>())
        })
    }
}
