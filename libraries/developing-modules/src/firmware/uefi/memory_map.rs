use core::ffi::c_void;

type UefiStatus = u64;
type UefiHandle = *const c_void;

// These are types that will be filled in later, as they are unused for now
type TodoStruct = *const c_void;
type TodoFunction = *const c_void;
type PhysicalAddr = *const c_void;
type VirtualAddr = *const c_void;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UefiTableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc: u32,
    _reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UefiSystemTable {
    header: UefiTableHeader,
    vendor: *const u16,
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
#[derive(Copy, Clone, Debug)]
pub struct UefiBootServices {
    header: UefiTableHeader,
    raise_tpl: TodoFunction,
    restore_tpl: TodoFunction,
    allocate_pages: TodoFunction,
    free_pages: TodoFunction,
    get_memory_map: unsafe extern "efiapi" fn(
        map_size: *mut u32,
        memory_map: *mut UefiMemoryDescriptor,
        map_key: *mut u64,
        descriptor_size: *mut u64,
        descriptor_version: *mut u32,
    ) -> UefiStatus,
    allocate_pool: TodoFunction,
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
