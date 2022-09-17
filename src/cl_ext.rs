// Copyright (c) 2022 Via Technology Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! FFI bindings for [cl_ext.h](https://github.com/KhronosGroup/OpenCL-Headers/blob/main/CL/cl_ext.h)  
//! cl_ext.h contains OpenCL extensions that don't have external (OpenGL, D3D) dependencies.  
//! OpenCL extensions are documented in the [OpenCL-Registry](https://github.com/KhronosGroup/OpenCL-Registry)

#![allow(non_camel_case_types, non_upper_case_globals)]

pub use super::cl::{
    cl_bitfield, cl_bool, cl_channel_type, cl_command_queue, cl_command_queue_properties,
    cl_command_type, cl_context, cl_device_id, cl_device_info, cl_event, cl_event_info,
    cl_image_desc, cl_image_format, cl_kernel, cl_kernel_exec_info, cl_kernel_info,
    cl_kernel_sub_group_info, cl_map_flags, cl_mem, cl_mem_flags, cl_mem_info,
    cl_mem_migration_flags, cl_mem_properties, cl_platform_id, cl_platform_info, cl_program,
    cl_program_info, cl_properties, cl_queue_properties, cl_sampler_properties,
};
use super::cl_platform::{cl_int, cl_uchar, cl_uint, cl_ulong};

#[allow(unused_imports)]
use libc::{c_char, c_void, intptr_t, size_t};

// cl_khr_command_buffer

pub type cl_device_command_buffer_capabilities_khr = cl_bitfield;
pub type cl_command_buffer_khr = *mut c_void;
pub type cl_sync_point_khr = cl_uint;
pub type cl_command_buffer_info_khr = cl_uint;
pub type cl_command_buffer_state_khr = cl_uint;
pub type cl_command_buffer_properties_khr = cl_properties;
pub type cl_command_buffer_flags_khr = cl_bitfield;
pub type cl_ndrange_kernel_command_properties_khr = cl_properties;
pub type cl_mutable_command_khr = *mut c_void;

// cl_device_info
pub const CL_DEVICE_COMMAND_BUFFER_CAPABILITIES_KHR: cl_device_info = 0x12A9;
pub const CL_DEVICE_COMMAND_BUFFER_REQUIRED_QUEUE_PROPERTIES_KHR: cl_device_info = 0x12AA;

// cl_device_command_buffer_capabilities_khr - bitfield
pub const CL_COMMAND_BUFFER_CAPABILITY_KERNEL_PRINTF_KHR:
    cl_device_command_buffer_capabilities_khr = 1 << 0;
pub const CL_COMMAND_BUFFER_CAPABILITY_DEVICE_SIDE_ENQUEUE_KHR:
    cl_device_command_buffer_capabilities_khr = 1 << 1;
pub const CL_COMMAND_BUFFER_CAPABILITY_SIMULTANEOUS_USE_KHR:
    cl_device_command_buffer_capabilities_khr = 1 << 2;
pub const CL_COMMAND_BUFFER_CAPABILITY_OUT_OF_ORDER_KHR: cl_device_command_buffer_capabilities_khr =
    1 << 3;

// cl_command_buffer_properties_khr
pub const CL_COMMAND_BUFFER_FLAGS_KHR: cl_command_buffer_properties_khr = 0x1293;

// cl_command_buffer_flags_khr
pub const CL_COMMAND_BUFFER_SIMULTANEOUS_USE_KHR: cl_command_buffer_flags_khr = 1 << 0;

// Error codes
pub const CL_INVALID_COMMAND_BUFFER_KHR: cl_int = -1138;
pub const CL_INVALID_SYNC_POINT_WAIT_LIST_KHR: cl_int = -1139;
pub const CL_INCOMPATIBLE_COMMAND_QUEUE_KHR: cl_int = -1140;

// cl_command_buffer_info_khr
pub const CL_COMMAND_BUFFER_QUEUES_KHR: cl_command_buffer_info_khr = 0x1294;
pub const CL_COMMAND_BUFFER_NUM_QUEUES_KHR: cl_command_buffer_info_khr = 0x1295;
pub const CL_COMMAND_BUFFER_REFERENCE_COUNT_KHR: cl_command_buffer_info_khr = 0x1296;
pub const CL_COMMAND_BUFFER_STATE_KHR: cl_command_buffer_info_khr = 0x1297;
pub const CL_COMMAND_BUFFER_PROPERTIES_ARRAY_KHR: cl_command_buffer_info_khr = 0x1298;

// cl_command_buffer_state_khr
pub const CL_COMMAND_BUFFER_STATE_RECORDING_KHR: cl_command_buffer_state_khr = 0;
pub const CL_COMMAND_BUFFER_STATE_EXECUTABLE_KHR: cl_command_buffer_state_khr = 1;
pub const CL_COMMAND_BUFFER_STATE_PENDING_KHR: cl_command_buffer_state_khr = 2;
pub const CL_COMMAND_BUFFER_STATE_INVALID_KHR: cl_command_buffer_state_khr = 3;

// cl_command_type
pub const CL_COMMAND_COMMAND_BUFFER_KHR: cl_command_type = 0x12A8;

pub type clCreateCommandBufferKHR_fn = Option<
    unsafe extern "C" fn(
        num_queues: cl_uint,
        queues: *const cl_command_queue,
        properties: *const cl_command_buffer_properties_khr,
        errcode_ret: *mut cl_int,
    ) -> cl_command_buffer_khr,
>;

pub type clFinalizeCommandBufferKHR_fn =
    Option<unsafe extern "C" fn(command_buffer: cl_command_buffer_khr) -> cl_int>;

pub type clRetainCommandBufferKHR_fn =
    Option<unsafe extern "C" fn(command_buffer: cl_command_buffer_khr) -> cl_int>;

pub type clReleaseCommandBufferKHR_fn =
    Option<unsafe extern "C" fn(command_buffer: cl_command_buffer_khr) -> cl_int>;

pub type clEnqueueCommandBufferKHR_fn = Option<
    unsafe extern "C" fn(
        num_queues: cl_uint,
        queues: *mut cl_command_queue,
        command_buffer: cl_command_buffer_khr,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clCommandBarrierWithWaitListKHR_fn = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int,
>;

pub type clCommandCopyBufferKHR_fn = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        src_buffer: cl_mem,
        dst_buffer: cl_mem,
        src_offset: size_t,
        dst_offset: size_t,
        size: size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int,
>;

pub type clCommandCopyBufferRectKHR_fn = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        src_buffer: cl_mem,
        dst_buffer: cl_mem,
        src_origin: *const size_t,
        dst_origin: *const size_t,
        region: *const size_t,
        src_row_pitch: size_t,
        src_slice_pitch: size_t,
        dst_row_pitch: size_t,
        dst_slice_pitch: size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int,
>;

pub type clCommandCopyBufferToImageKHR_fn = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        src_buffer: cl_mem,
        dst_image: cl_mem,
        src_offset: size_t,
        dst_origin: *const size_t,
        region: *const size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int,
>;

pub type clCommandCopyImageKHR_fn = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        src_image: cl_mem,
        dst_image: cl_mem,
        src_origin: *const size_t,
        dst_origin: *const size_t,
        region: *const size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int,
>;

pub type clCommandCopyImageToBufferKHR_fn = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        src_image: cl_mem,
        dst_buffer: cl_mem,
        src_origin: *const size_t,
        region: *const size_t,
        dst_offset: size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int,
>;

pub type clCommandFillBufferKHR_fn = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        buffer: cl_mem,
        pattern: *const c_void,
        pattern_size: size_t,
        offset: size_t,
        size: size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int,
>;

pub type clCommandFillImageKHR_fn = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        image: cl_mem,
        fill_color: *const c_void,
        origin: *const size_t,
        region: *const size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int,
>;

pub type clCommandNDRangeKernelKHR_fn = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        properties: *const cl_ndrange_kernel_command_properties_khr,
        kernel: cl_kernel,
        work_dim: cl_uint,
        global_work_offset: *const size_t,
        global_work_size: *const size_t,
        local_work_size: *const size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int,
>;

pub type clGetCommandBufferInfoKHR_fn = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        param_name: cl_command_buffer_info_khr,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,
>;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_khr_command_buffer")]
extern "system" {

    pub fn clCreateCommandBufferKHR(
        num_queues: cl_uint,
        queues: *const cl_command_queue,
        properties: *const cl_command_buffer_properties_khr,
        errcode_ret: *mut cl_int,
    ) -> cl_command_buffer_khr;

    pub fn clFinalizeCommandBufferKHR(command_buffer: cl_command_buffer_khr) -> cl_int;

    pub fn clRetainCommandBufferKHR(command_buffer: cl_command_buffer_khr) -> cl_int;

    pub fn clReleaseCommandBufferKHR(command_buffer: cl_command_buffer_khr) -> cl_int;

    pub fn clEnqueueCommandBufferKHR(
        num_queues: cl_uint,
        queues: *mut cl_command_queue,
        command_buffer: cl_command_buffer_khr,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clCommandBarrierWithWaitListKHR(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int;

    pub fn clCommandCopyBufferKHR(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        src_buffer: cl_mem,
        dst_buffer: cl_mem,
        src_offset: size_t,
        dst_offset: size_t,
        size: size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int;

    pub fn clCommandCopyBufferRectKHR(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        src_buffer: cl_mem,
        dst_buffer: cl_mem,
        src_origin: *const size_t,
        dst_origin: *const size_t,
        region: *const size_t,
        src_row_pitch: size_t,
        src_slice_pitch: size_t,
        dst_row_pitch: size_t,
        dst_slice_pitch: size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int;

    pub fn clCommandCopyBufferToImageKHR(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        src_buffer: cl_mem,
        dst_image: cl_mem,
        src_offset: size_t,
        dst_origin: *const size_t,
        region: *const size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int;

    pub fn clCommandCopyImageKHR(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        src_image: cl_mem,
        dst_image: cl_mem,
        src_origin: *const size_t,
        dst_origin: *const size_t,
        region: *const size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int;

    pub fn clCommandCopyImageToBufferKHR(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        src_image: cl_mem,
        dst_buffer: cl_mem,
        src_origin: *const size_t,
        region: *const size_t,
        dst_offset: size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int;

    pub fn clCommandFillBufferKHR(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        buffer: cl_mem,
        pattern: *const c_void,
        pattern_size: size_t,
        offset: size_t,
        size: size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int;

    pub fn clCommandFillImageKHR(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        image: cl_mem,
        fill_color: *const c_void,
        origin: *const size_t,
        region: *const size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int;

    pub fn clCommandNDRangeKernelKHR(
        command_buffer: cl_command_buffer_khr,
        command_queue: cl_command_queue,
        properties: *const cl_ndrange_kernel_command_properties_khr,
        kernel: cl_kernel,
        work_dim: cl_uint,
        global_work_offset: *const size_t,
        global_work_size: *const size_t,
        local_work_size: *const size_t,
        num_sync_points_in_wait_list: cl_uint,
        sync_point_wait_list: *const cl_sync_point_khr,
        sync_point: *mut cl_sync_point_khr,
        mutable_handle: *mut cl_mutable_command_khr,
    ) -> cl_int;

    pub fn clGetCommandBufferInfoKHR(
        command_buffer: cl_command_buffer_khr,
        param_name: cl_command_buffer_info_khr,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

}

// cl_khr_command_buffer_mutable_dispatch

pub type cl_command_buffer_structure_type_khr = cl_uint;
pub type cl_mutable_dispatch_fields_khr = cl_bitfield;
pub type cl_mutable_command_info_khr = cl_uint;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct cl_mutable_dispatch_arg_khr {
    pub arg_index: cl_uint,
    pub arg_size: size_t,
    pub arg_value: *const c_void,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct cl_mutable_dispatch_exec_info_khr {
    pub param_name: cl_uint,
    pub param_value_size: size_t,
    pub param_value: *const c_void,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct cl_mutable_dispatch_config_khr {
    pub t_type: cl_command_buffer_structure_type_khr,
    pub next: *const c_void,
    pub command: cl_mutable_command_khr,
    pub num_args: cl_uint,
    pub num_svm_args: cl_uint,
    pub num_exec_infos: cl_uint,
    pub work_dim: cl_uint,
    pub arg_list: *const cl_mutable_dispatch_arg_khr,
    pub arg_svm_list: *const cl_mutable_dispatch_arg_khr,
    pub exec_info_list: *const cl_mutable_dispatch_exec_info_khr,
    pub global_work_offset: *const size_t,
    pub global_work_size: *const size_t,
    pub local_work_size: *const size_t,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct cl_mutable_base_config_khr {
    pub t_type: cl_command_buffer_structure_type_khr,
    pub next: *const c_void,
    pub num_mutable_dispatch: cl_uint,
    pub mutable_dispatch_list: *const cl_mutable_dispatch_config_khr,
}

pub const CL_COMMAND_BUFFER_MUTABLE_KHR: cl_command_buffer_flags_khr = 1 << 1;

pub const CL_INVALID_MUTABLE_COMMAND_KHR: cl_int = -1141;

pub const CL_DEVICE_MUTABLE_DISPATCH_CAPABILITIES_KHR: cl_device_info = 0x12B0;

pub const CL_MUTABLE_DISPATCH_UPDATABLE_FIELDS_KHR: cl_ndrange_kernel_command_properties_khr =
    0x12B1;

pub const CL_MUTABLE_DISPATCH_GLOBAL_OFFSET_KHR: cl_mutable_dispatch_fields_khr = 1 << 0;
pub const CL_MUTABLE_DISPATCH_GLOBAL_SIZE_KHR: cl_mutable_dispatch_fields_khr = 1 << 1;
pub const CL_MUTABLE_DISPATCH_LOCAL_SIZE_KHR: cl_mutable_dispatch_fields_khr = 1 << 2;
pub const CL_MUTABLE_DISPATCH_ARGUMENTS_KHR: cl_mutable_dispatch_fields_khr = 1 << 3;
pub const CL_MUTABLE_DISPATCH_EXEC_INFO_KHR: cl_mutable_dispatch_fields_khr = 1 << 4;

pub const CL_MUTABLE_COMMAND_COMMAND_QUEUE_KHR: cl_mutable_command_info_khr = 0x12A0;
pub const CL_MUTABLE_COMMAND_COMMAND_BUFFER_KHR: cl_mutable_command_info_khr = 0x12A1;
pub const CL_MUTABLE_COMMAND_COMMAND_TYPE_KHR: cl_mutable_command_info_khr = 0x12AD;
pub const CL_MUTABLE_DISPATCH_PROPERTIES_ARRAY_KHR: cl_mutable_command_info_khr = 0x12A2;
pub const CL_MUTABLE_DISPATCH_KERNEL_KHR: cl_mutable_command_info_khr = 0x12A3;
pub const CL_MUTABLE_DISPATCH_DIMENSIONS_KHR: cl_mutable_command_info_khr = 0x12A4;
pub const CL_MUTABLE_DISPATCH_GLOBAL_WORK_OFFSET_KHR: cl_mutable_command_info_khr = 0x12A5;
pub const CL_MUTABLE_DISPATCH_GLOBAL_WORK_SIZE_KHR: cl_mutable_command_info_khr = 0x12A6;
pub const CL_MUTABLE_DISPATCH_LOCAL_WORK_SIZE_KHR: cl_mutable_command_info_khr = 0x12A7;

pub const CL_STRUCTURE_TYPE_MUTABLE_BASE_CONFIG_KHR: cl_command_buffer_structure_type_khr = 0;
pub const CL_STRUCTURE_TYPE_MUTABLE_DISPATCH_CONFIG_KHR: cl_command_buffer_structure_type_khr = 1;

pub type clUpdateMutableCommandsKHR_fn = Option<
    unsafe extern "C" fn(
        command_buffer: cl_command_buffer_khr,
        mutable_config: *const cl_mutable_base_config_khr,
    ) -> cl_int,
>;

pub type clGetMutableCommandInfoKHR_fn = Option<
    unsafe extern "C" fn(
        command: cl_mutable_command_khr,
        param_name: cl_mutable_command_info_khr,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,
>;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_khr_command_buffer_mutable_dispatch")]
extern "system" {

    pub fn clUpdateMutableCommandsKHR(
        command_buffer: cl_command_buffer_khr,
        mutable_config: *const cl_mutable_base_config_khr,
    ) -> cl_int;

    pub fn clGetMutableCommandInfoKHR(
        command: cl_mutable_command_khr,
        param_name: cl_mutable_command_info_khr,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;
}

// cl_khr_fp64 extension

// #if CL_TARGET_OPENCL_VERSION <= 110
pub const CL_DEVICE_DOUBLE_FP_CONFIG: cl_device_info = 0x1032;
// #endif
// cl_khr_fp16 extension
pub const CL_DEVICE_HALF_FP_CONFIG: cl_device_info = 0x1033;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
extern "system" {

    /* Memory object destruction
     *
     * Apple extension for use to manage externally allocated buffers used with cl_mem objects with CL_MEM_USE_HOST_PTR
     *
     * Registers a user callback function that will be called when the memory object is deleted and its resources
     * freed. Each call to clSetMemObjectCallbackFn registers the specified user callback function on a callback
     * stack associated with memobj. The registered user callback functions are called in the reverse order in
     * which they were registered. The user callback functions are called and then the memory object is deleted
     * and its resources freed. This provides a mechanism for the application (and libraries) using memobj to be
     * notified when the memory referenced by host_ptr, specified when the memory object is created and used as
     * the storage bits for the memory object, can be reused or freed.
     *
     * The application may not call CL api's with the cl_mem object passed to the pfn_notify.
     *
     * Please check for the "cl_APPLE_SetMemObjectDestructor" extension using clGetDeviceInfo(CL_DEVICE_EXTENSIONS)
     * before using.
     */
    #[cfg(feature = "cl_apple_setmemobjectdestructor")]
    pub fn clSetMemObjectDestructorAPPLE(
        memobj: cl_mem,
        pfn_notify: Option<unsafe extern "C" fn(memobj: cl_mem, user_data: *mut c_void)>,
        user_data: *mut c_void,
    ) -> cl_int;

    /* Context Logging Functions
     *
     * The next three convenience functions are intended to be used as the pfn_notify parameter to clCreateContext().
     * Please check for the "cl_APPLE_ContextLoggingFunctions" extension using clGetDeviceInfo(CL_DEVICE_EXTENSIONS)
     * before using.
     *
     * clLogMessagesToSystemLog forwards on all log messages to the Apple System Logger
     */
    #[cfg(feature = "cl_apple_contextloggingfunctions")]
    pub fn clLogMessagesToSystemLogAPPLE(
        errstr: *const c_char,
        private_info: *const c_void,
        cb: size_t,
        user_data: *mut c_void,
    );

    // clLogMessagesToStdout sends all log messages to the file descriptor stdout
    #[cfg(feature = "cl_apple_contextloggingfunctions")]
    pub fn clLogMessagesToStdoutAPPLE(
        errstr: *const c_char,
        private_info: *const c_void,
        cb: size_t,
        user_data: *mut c_void,
    );

    // clLogMessagesToStderr sends all log messages to the file descriptor stderr
    #[cfg(feature = "cl_apple_contextloggingfunctions")]
    pub fn clLogMessagesToStderrAPPLE(
        errstr: *const c_char,
        private_info: *const c_void,
        cb: size_t,
        user_data: *mut c_void,
    );

}

// cl_khr_icd extension

pub const CL_PLATFORM_ICD_SUFFIX_KHR: cl_platform_info = 0x0920;

// Additional Error Codes
pub const CL_PLATFORM_NOT_FOUND_KHR: cl_int = -1001;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_khr_icd")]
extern "system" {
    pub fn clIcdGetPlatformIDsKHR(
        num_entries: cl_uint,
        platforms: *mut cl_platform_id,
        num_platforms: *mut cl_uint,
    ) -> cl_int;
}

pub type clIcdGetPlatformIDsKHR_fn = Option<
    unsafe extern "C" fn(
        num_entries: cl_uint,
        platforms: *mut cl_platform_id,
        num_platforms: *mut cl_uint,
    ) -> cl_int,
>;

// cl_khr_il_program extension

/// New property to clGetDeviceInfo for retrieving supported intermediate languages.
pub const CL_DEVICE_IL_VERSION_KHR: cl_device_info = 0x105B;

/// New property to clGetProgramInfo for retrieving for retrieving the IL of a program.
pub const CL_PROGRAM_IL_KHR: cl_program_info = 0x1169;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_khr_il_program")]
extern "system" {
    pub fn clCreateProgramWithILKHR(
        context: cl_context,
        il: *const c_void,
        length: size_t,
        errcode_ret: *mut cl_int,
    ) -> cl_program;
}

pub type clCreateProgramWithILKHR_fn = Option<
    unsafe extern "C" fn(
        context: cl_context,
        il: *const c_void,
        length: size_t,
        errcode_ret: *mut cl_int,
    ) -> cl_program,
>;

/* Extension: cl_khr_image2d_from_buffer
 *
 * This extension allows a 2D image to be created from a cl_mem buffer without
 * a copy. The type associated with a 2D image created from a buffer in an
 * OpenCL program is image2d_t. Both the sampler and sampler-less read_image
 * built-in functions are supported for 2D images and 2D images created from
 * a buffer.  Similarly, the write_image built-ins are also supported for 2D
 * images created from a buffer.
 *
 * When the 2D image from buffer is created, the client must specify the
 * width, height, image format (i.e. channel order and channel data type)
 * and optionally the row pitch.
 *
 * The pitch specified must be a multiple of
 * CL_DEVICE_IMAGE_PITCH_ALIGNMENT_KHR pixels.
 * The base address of the buffer must be aligned to
 * CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT_KHR pixels.
 */

pub const CL_DEVICE_IMAGE_PITCH_ALIGNMENT_KHR: cl_device_info = 0x104A;
pub const CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT_KHR: cl_device_info = 0x104B;

// cl_khr_initialize_memory extension
pub const CL_CONTEXT_MEMORY_INITIALIZE_KHR: cl_uint = 0x2030;

// cl_khr_terminate_context extension
pub const CL_CONTEXT_TERMINATED_KHR: cl_int = -1121;

pub const CL_DEVICE_TERMINATE_CAPABILITY_KHR: cl_uint = 0x2031;
pub const CL_CONTEXT_TERMINATE_KHR: cl_uint = 0x2032;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_khr_terminate_context")]
extern "system" {
    pub fn clTerminateContextKHR(context: cl_context) -> cl_int;
}

pub type clTerminateContextKHR_fn = Option<unsafe extern "C" fn(context: cl_context) -> cl_int>;

/*
 * Extension: cl_khr_spir
 *
 * This extension adds support to create an OpenCL program object from a
 * Standard Portable Intermediate Representation (SPIR) instance
 */

pub const CL_DEVICE_SPIR_VERSIONS: cl_uint = 0x40E0;
pub const CL_PROGRAM_BINARY_TYPE_INTERMEDIATE: cl_uint = 0x40E1;

// cl_khr_create_command_queue extension
pub type cl_queue_properties_khr = cl_properties;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_khr_create_command_queue")]
extern "system" {
    pub fn clCreateCommandQueueWithPropertiesKHR(
        context: cl_context,
        device: cl_device_id,
        properties: *const cl_queue_properties_khr,
        errcode_ret: *mut cl_int,
    ) -> cl_command_queue;
}

pub type clCreateCommandQueueWithPropertiesKHR_fn = Option<
    unsafe extern "C" fn(
        context: cl_context,
        device: cl_device_id,
        properties: *const cl_queue_properties_khr,
        errcode_ret: *mut cl_int,
    ) -> cl_command_queue,
>;

// cl_nv_device_attribute_query extension

pub type cl_nv_device_attribute_query = cl_uint;
pub const CL_DEVICE_COMPUTE_CAPABILITY_MAJOR_NV: cl_nv_device_attribute_query = 0x4000;
pub const CL_DEVICE_COMPUTE_CAPABILITY_MINOR_NV: cl_nv_device_attribute_query = 0x4001;
pub const CL_DEVICE_REGISTERS_PER_BLOCK_NV: cl_nv_device_attribute_query = 0x4002;
pub const CL_DEVICE_WARP_SIZE_NV: cl_nv_device_attribute_query = 0x4003;
pub const CL_DEVICE_GPU_OVERLAP_NV: cl_nv_device_attribute_query = 0x4004;
pub const CL_DEVICE_KERNEL_EXEC_TIMEOUT_NV: cl_nv_device_attribute_query = 0x4005;
pub const CL_DEVICE_INTEGRATED_MEMORY_NV: cl_nv_device_attribute_query = 0x4006;

// undocumented tokens for clGetDeviceInfo, see: https://anteru.net/blog/2014/associating-opencl-device-ids-with-gpus/
pub const CL_DEVICE_PCI_BUS_ID_NV: cl_nv_device_attribute_query = 0x4008;
pub const CL_DEVICE_PCI_SLOT_ID_NV: cl_nv_device_attribute_query = 0x4009;

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct cl_amd_device_topology {
    r#type: u32,
    unused: [u8; 17],
    pub bus: u8,
    pub device: u8,
    pub function: u8,
}

// cl_amd_device_attribute_query
pub type cl_amd_device_attribute_query = cl_uint;

pub const CL_DEVICE_PROFILING_TIMER_OFFSET_AMD: cl_amd_device_attribute_query = 0x4036;
pub const CL_DEVICE_TOPOLOGY_AMD: cl_amd_device_attribute_query = 0x4037;
pub const CL_DEVICE_BOARD_NAME_AMD: cl_amd_device_attribute_query = 0x4038;
pub const CL_DEVICE_GLOBAL_FREE_MEMORY_AMD: cl_amd_device_attribute_query = 0x4039;
pub const CL_DEVICE_SIMD_PER_COMPUTE_UNIT_AMD: cl_amd_device_attribute_query = 0x4040;
pub const CL_DEVICE_SIMD_WIDTH_AMD: cl_amd_device_attribute_query = 0x4041;
pub const CL_DEVICE_SIMD_INSTRUCTION_WIDTH_AMD: cl_amd_device_attribute_query = 0x4042;
pub const CL_DEVICE_WAVEFRONT_WIDTH_AMD: cl_amd_device_attribute_query = 0x4043;
pub const CL_DEVICE_GLOBAL_MEM_CHANNELS_AMD: cl_amd_device_attribute_query = 0x4044;
pub const CL_DEVICE_GLOBAL_MEM_CHANNEL_BANKS_AMD: cl_amd_device_attribute_query = 0x4045;
pub const CL_DEVICE_GLOBAL_MEM_CHANNEL_BANK_WIDTH_AMD: cl_amd_device_attribute_query = 0x4046;
pub const CL_DEVICE_LOCAL_MEM_SIZE_PER_COMPUTE_UNIT_AMD: cl_amd_device_attribute_query = 0x4047;
pub const CL_DEVICE_LOCAL_MEM_BANKS_AMD: cl_amd_device_attribute_query = 0x4048;
pub const CL_DEVICE_THREAD_TRACE_SUPPORTED_AMD: cl_amd_device_attribute_query = 0x4049;
pub const CL_DEVICE_GFXIP_MAJOR_AMD: cl_amd_device_attribute_query = 0x404A;
pub const CL_DEVICE_GFXIP_MINOR_AMD: cl_amd_device_attribute_query = 0x404B;
pub const CL_DEVICE_AVAILABLE_ASYNC_QUEUES_AMD: cl_amd_device_attribute_query = 0x404C;
pub const CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_AMD: cl_amd_device_attribute_query = 0x4030;
pub const CL_DEVICE_MAX_WORK_GROUP_SIZE_AMD: cl_amd_device_attribute_query = 0x4031;
pub const CL_DEVICE_PREFERRED_CONSTANT_BUFFER_SIZE_AMD: cl_amd_device_attribute_query = 0x4033;
pub const CL_DEVICE_PCIE_ID_AMD: cl_amd_device_attribute_query = 0x4034;

// cl_arm_printf extension
pub const CL_PRINTF_CALLBACK_ARM: cl_uint = 0x40B0;
pub const CL_PRINTF_BUFFERSIZE_ARM: cl_uint = 0x40B1;

// cl_ext_device_fission extension

// cl_device_partition_property_ext
pub type cl_device_partition_property_ext = cl_ulong;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_ext_device_fission")]
extern "system" {
    pub fn clReleaseDeviceEXT(device: cl_device_id) -> cl_int;

    pub fn clRetainDeviceEXT(device: cl_device_id) -> cl_int;

    pub fn clCreateSubDevicesEXT(
        in_device: cl_device_id,
        properties: *const cl_device_partition_property_ext,
        num_entries: cl_uint,
        out_devices: *mut cl_device_id,
        num_devices: *mut cl_uint,
    ) -> cl_int;
}

pub type clReleaseDeviceEXT_fn = Option<unsafe extern "C" fn(device: cl_device_id) -> cl_int>;

pub type clRetainDeviceEXT_fn = Option<unsafe extern "C" fn(device: cl_device_id) -> cl_int>;

pub type clCreateSubDevicesEXT_fn = Option<
    unsafe extern "C" fn(
        in_device: cl_device_id,
        properties: *const cl_device_partition_property_ext,
        num_entries: cl_uint,
        out_devices: *mut cl_device_id,
        num_devices: *mut cl_uint,
    ) -> cl_int,
>;

// cl_device_partition_property_ext
pub const CL_DEVICE_PARTITION_EQUALLY_EXT: cl_device_partition_property_ext = 0x4050;
pub const CL_DEVICE_PARTITION_BY_COUNTS_EXT: cl_device_partition_property_ext = 0x4051;
pub const CL_DEVICE_PARTITION_BY_NAMES_EXT: cl_device_partition_property_ext = 0x4052;
pub const CL_DEVICE_PARTITION_BY_AFFINITY_DOMAIN_EXT: cl_device_partition_property_ext = 0x4053;

// clDeviceGetInfo selectors
pub const CL_DEVICE_PARENT_DEVICE_EXT: cl_device_info = 0x4054;
pub const CL_DEVICE_PARTITION_TYPES_EXT: cl_device_info = 0x4055;
pub const CL_DEVICE_AFFINITY_DOMAINS_EXT: cl_device_info = 0x4056;
pub const CL_DEVICE_REFERENCE_COUNT_EXT: cl_device_info = 0x4057;
pub const CL_DEVICE_PARTITION_STYLE_EXT: cl_device_info = 0x4058;

// error codes
pub const CL_DEVICE_PARTITION_FAILED_EXT: cl_int = -1057;
pub const CL_INVALID_PARTITION_COUNT_EXT: cl_int = -1058;
pub const CL_INVALID_PARTITION_NAME_EXT: cl_int = -1059;

// CL_AFFINITY_DOMAINs
pub const CL_AFFINITY_DOMAIN_L1_CACHE_EXT: cl_uint = 0x1;
pub const CL_AFFINITY_DOMAIN_L2_CACHE_EXT: cl_uint = 0x2;
pub const CL_AFFINITY_DOMAIN_L3_CACHE_EXT: cl_uint = 0x3;
pub const CL_AFFINITY_DOMAIN_L4_CACHE_EXT: cl_uint = 0x4;
pub const CL_AFFINITY_DOMAIN_NUMA_EXT: cl_uint = 0x10;
pub const CL_AFFINITY_DOMAIN_NEXT_FISSIONABLE_EXT: cl_uint = 0x100;

// cl_device_partition_property_ext list terminators
pub const CL_PROPERTIES_LIST_END_EXT: cl_device_partition_property_ext = 0;
pub const CL_PARTITION_BY_COUNTS_LIST_END_EXT: cl_device_partition_property_ext = 0;
pub const CL_PARTITION_BY_NAMES_LIST_END_EXT: cl_device_partition_property_ext = 0xFFFFFFFF;

// cl_ext_migrate_memobject extension definitions

pub type cl_mem_migration_flags_ext = cl_bitfield;
pub const CL_MIGRATE_MEM_OBJECT_HOST_EXT: cl_mem_migration_flags_ext = 0x1;
pub const CL_COMMAND_MIGRATE_MEM_OBJECT_EXT: cl_mem_migration_flags_ext = 0x4040;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_ext_migrate_memobject")]
extern "system" {
    pub fn clEnqueueMigrateMemObjectEXT(
        command_queue: cl_command_queue,
        num_mem_objects: cl_uint,
        mem_objects: *const cl_mem,
        flags: cl_mem_migration_flags_ext,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;
}

pub type clEnqueueMigrateMemObjectEXT_fn = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_mem_objects: cl_uint,
        mem_objects: *const cl_mem,
        flags: cl_mem_migration_flags_ext,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

// cl_ext_cxx_for_opencl extension
pub const CL_DEVICE_CXX_FOR_OPENCL_NUMERIC_VERSION_EXT: cl_uint = 0x4230;

// cl_qcom_ext_host_ptr extension
pub type cl_qcom_ext_host_ptr = cl_uint;
pub const CL_MEM_EXT_HOST_PTR_QCOM: cl_qcom_ext_host_ptr = 1 << 29;

pub const CL_DEVICE_EXT_MEM_PADDING_IN_BYTES_QCOM: cl_qcom_ext_host_ptr = 0x40A0;
pub const CL_DEVICE_PAGE_SIZE_QCOM: cl_qcom_ext_host_ptr = 0x40A1;
pub const CL_IMAGE_ROW_ALIGNMENT_QCOM: cl_qcom_ext_host_ptr = 0x40A2;
pub const CL_IMAGE_SLICE_ALIGNMENT_QCOM: cl_qcom_ext_host_ptr = 0x40A3;
pub const CL_MEM_HOST_UNCACHED_QCOM: cl_qcom_ext_host_ptr = 0x40A4;
pub const CL_MEM_HOST_WRITEBACK_QCOM: cl_qcom_ext_host_ptr = 0x40A5;
pub const CL_MEM_HOST_WRITETHROUGH_QCOM: cl_qcom_ext_host_ptr = 0x40A6;
pub const CL_MEM_HOST_WRITE_COMBINING_QCOM: cl_qcom_ext_host_ptr = 0x40A7;

pub type cl_image_pitch_info_qcom = cl_uint;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_qcom_ext_host_ptr")]
extern "system" {
    pub fn clGetDeviceImageInfoQCOM(
        device: cl_device_id,
        image_width: size_t,
        image_height: size_t,
        image_format: *const cl_image_format,
        param_name: cl_image_pitch_info_qcom,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct cl_mem_ext_host_ptr {
    pub allocation_type: cl_uint,
    pub host_cache_policy: cl_uint,
}

// cl_qcom_ext_host_ptr_iocoherent extension
// Cache policy specifying io-coherence
pub const CL_MEM_HOST_IOCOHERENT_QCOM: cl_qcom_ext_host_ptr = 0x40A9;

// cl_qcom_ion_host_ptr extension
pub const CL_MEM_ION_HOST_PTR_QCOM: cl_qcom_ext_host_ptr = 0x40A8;

#[derive(Debug)]
#[repr(C)]
pub struct cl_mem_ion_host_ptr {
    pub ext_host_ptr: cl_mem_ext_host_ptr,
    pub ion_filedesc: cl_int,
    pub ion_hostptr: *mut c_void,
}

//cl_qcom_android_native_buffer_host_ptr extension
pub const CL_MEM_ANDROID_NATIVE_BUFFER_HOST_PTR_QCOM: cl_qcom_ext_host_ptr = 0x40C6;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_mem_android_native_buffer_host_ptr {
    pub ext_host_ptr: cl_mem_ext_host_ptr,
    pub anb_ptr: *mut c_void,
}

// cl_img_yuv_image extension
pub const CL_NV21_IMG: cl_channel_type = 0x40D0;
pub const CL_YV12_IMG: cl_channel_type = 0x40D1;

// cl_img_cached_allocations extension
pub const CL_MEM_USE_UNCACHED_CPU_MEMORY_IMG: cl_mem_flags = 1 << 26;
pub const CL_MEM_USE_CACHED_CPU_MEMORY_IMG: cl_mem_flags = 1 << 27;

// cl_img_use_gralloc_ptr extension
pub const CL_MEM_USE_GRALLOC_PTR_IMG: cl_mem_flags = 1 << 28;

// To be used by clGetEventInfo:
pub const CL_COMMAND_ACQUIRE_GRALLOC_OBJECTS_IMG: cl_event_info = 0x40D2;
pub const CL_COMMAND_RELEASE_GRALLOC_OBJECTS_IMG: cl_event_info = 0x40D3;

// Error codes from clEnqueueAcquireGrallocObjectsIMG and clEnqueueReleaseGrallocObjectsIMG
pub const CL_GRALLOC_RESOURCE_NOT_ACQUIRED_IMG: cl_int = 0x40D4;
pub const CL_INVALID_GRALLOC_OBJECT_IMG: cl_int = 0x40D5;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_img_use_gralloc_ptr")]
extern "system" {
    pub fn clEnqueueAcquireGrallocObjectsIMG(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueReleaseGrallocObjectsIMG(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;
}

// cl_img_generate_mipmap extension
pub type cl_mipmap_filter_mode_img = cl_uint;

// To be used by clEnqueueGenerateMipmapIMG
pub const CL_MIPMAP_FILTER_ANY_IMG: cl_mipmap_filter_mode_img = 0x0;
pub const CL_MIPMAP_FILTER_BOX_IMG: cl_mipmap_filter_mode_img = 0x1;

// To be used by clGetEventInfo
pub const CL_COMMAND_GENERATE_MIPMAP_IMG: cl_event_info = 0x40D6;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_img_generate_mipmap")]
extern "system" {
    pub fn clEnqueueGenerateMipmapIMG(
        command_queue: cl_command_queue,
        src_image: cl_mem,
        dst_image: cl_mem,
        mipmap_filter_mode: cl_mipmap_filter_mode_img,
        array_region: *const size_t,
        mip_region: *const size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;
}

// cl_img_mem_properties extension
// To be used by clCreateBufferWithProperties
pub const CL_MEM_ALLOC_FLAGS_IMG: cl_properties = 0x40D7;

// To be used wiith the CL_MEM_ALLOC_FLAGS_IMG property
pub type cl_mem_alloc_flags_img = cl_bitfield;

// To be used with cl_mem_alloc_flags_img
pub const CL_MEM_ALLOC_RELAX_REQUIREMENTS_IMG: cl_mem_alloc_flags_img = 1 << 0;

// cl_khr_subgroups extension
pub const CL_KERNEL_MAX_SUB_GROUP_SIZE_FOR_NDRANGE_KHR: cl_kernel_sub_group_info = 0x2033;
pub const CL_KERNEL_SUB_GROUP_COUNT_FOR_NDRANGE_KHR: cl_kernel_sub_group_info = 0x2034;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_khr_subgroups")]
extern "system" {
    pub fn clGetKernelSubGroupInfoKHR(
        in_kernel: cl_kernel,
        in_device: cl_device_id,
        param_name: cl_kernel_sub_group_info,
        input_value_size: size_t,
        input_value: *const c_void,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;
}

pub type clGetKernelSubGroupInfoKHR_fn = Option<
    unsafe extern "C" fn(
        in_kernel: cl_kernel,
        in_device: cl_device_id,
        param_name: cl_kernel_sub_group_info,
        input_value_size: size_t,
        input_value: *const c_void,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,
>;

// cl_khr_mipmap_image extension

// cl_sampler_properties
pub const CL_SAMPLER_MIP_FILTER_MODE_KHR: cl_sampler_properties = 0x1155;
pub const CL_SAMPLER_LOD_MIN_KHR: cl_sampler_properties = 0x1156;
pub const CL_SAMPLER_LOD_MAX_KHR: cl_sampler_properties = 0x1157;

// cl_khr_priority_hints extension
pub type cl_queue_priority_khr = cl_uint;

// cl_command_queue_properties
pub const CL_QUEUE_PRIORITY_KHR: cl_command_queue_properties = 0x1096;

// cl_queue_priority_khr
pub const CL_QUEUE_PRIORITY_HIGH_KHR: cl_queue_priority_khr = 1 << 0;
pub const CL_QUEUE_PRIORITY_MED_KHR: cl_queue_priority_khr = 1 << 1;
pub const CL_QUEUE_PRIORITY_LOW_KHR: cl_queue_priority_khr = 1 << 2;

// cl_khr_throttle_hints extension
pub type cl_queue_throttle_khr = cl_uint;

// cl_command_queue_properties
pub const CL_QUEUE_THROTTLE_KHR: cl_command_queue_properties = 0x1097;

// cl_queue_throttle_khr
pub const CL_QUEUE_THROTTLE_HIGH_KHR: cl_queue_throttle_khr = 1 << 0;
pub const CL_QUEUE_THROTTLE_MED_KHR: cl_queue_throttle_khr = 1 << 1;
pub const CL_QUEUE_THROTTLE_LOW_KHR: cl_queue_throttle_khr = 1 << 2;

// cl_khr_subgroup_named_barrier

// cl_device_info
pub const CL_DEVICE_MAX_NAMED_BARRIER_COUNT_KHR: cl_device_info = 0x2035;

// cl_khr_extended_versioning
pub type cl_version_khr = cl_uint;

pub const CL_VERSION_MAJOR_BITS_KHR: cl_version_khr = 10;
pub const CL_VERSION_MINOR_BITS_KHR: cl_version_khr = 10;
pub const CL_VERSION_PATCH_BITS_KHR: cl_version_khr = 12;

pub const CL_VERSION_MAJOR_MASK_KHR: cl_version_khr = (1 << CL_VERSION_MAJOR_BITS_KHR) - 1;
pub const CL_VERSION_MINOR_MASK_KHR: cl_version_khr = (1 << CL_VERSION_MINOR_BITS_KHR) - 1;
pub const CL_VERSION_PATCH_MASK_KHR: cl_version_khr = (1 << CL_VERSION_PATCH_BITS_KHR) - 1;

#[inline]
pub fn version_major_khr(version: cl_version_khr) -> cl_version_khr {
    version >> (CL_VERSION_MINOR_BITS_KHR + CL_VERSION_PATCH_BITS_KHR)
}

#[inline]
pub fn version_minor_khr(version: cl_version_khr) -> cl_version_khr {
    (version >> CL_VERSION_PATCH_BITS_KHR) & CL_VERSION_MINOR_MASK_KHR
}

#[inline]
pub fn version_patch_khr(version: cl_version_khr) -> cl_version_khr {
    version & CL_VERSION_PATCH_MASK_KHR
}

#[inline]
pub fn make_version_khr(
    major: cl_version_khr,
    minor: cl_version_khr,
    patch: cl_version_khr,
) -> cl_version_khr {
    ((major & CL_VERSION_MAJOR_MASK_KHR) << (CL_VERSION_MINOR_BITS_KHR + CL_VERSION_PATCH_BITS_KHR))
        | ((minor & CL_VERSION_MINOR_MASK_KHR) << CL_VERSION_PATCH_BITS_KHR)
        | (patch & CL_VERSION_PATCH_MASK_KHR)
}

pub const CL_NAME_VERSION_MAX_NAME_SIZE_KHR: usize = 64;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_name_version_khr {
    pub version: cl_version_khr,
    pub name: [cl_uchar; CL_NAME_VERSION_MAX_NAME_SIZE_KHR],
}

// cl_platform_info
pub const CL_PLATFORM_NUMERIC_VERSION_KHR: cl_platform_info = 0x0906;
pub const CL_PLATFORM_EXTENSIONS_WITH_VERSION_KHR: cl_platform_info = 0x0907;

// cl_device_info
pub const CL_DEVICE_NUMERIC_VERSION_KHR: cl_device_info = 0x105E;
pub const CL_DEVICE_OPENCL_C_NUMERIC_VERSION_KHR: cl_device_info = 0x105F;
pub const CL_DEVICE_EXTENSIONS_WITH_VERSION_KHR: cl_device_info = 0x1060;
pub const CL_DEVICE_ILS_WITH_VERSION_KHR: cl_device_info = 0x1061;
pub const CL_DEVICE_BUILT_IN_KERNELS_WITH_VERSION_KHR: cl_device_info = 0x1062;

// cl_khr_device_uuid extension

pub const CL_UUID_SIZE_KHR: usize = 16;
pub const CL_LUID_SIZE_KHR: usize = 8;

pub const CL_DEVICE_UUID_KHR: cl_device_info = 0x106A;
pub const CL_DRIVER_UUID_KHR: cl_device_info = 0x106B;
pub const CL_DEVICE_LUID_VALID_KHR: cl_device_info = 0x106C;
pub const CL_DEVICE_LUID_KHR: cl_device_info = 0x106D;
pub const CL_DEVICE_NODE_MASK_KHR: cl_device_info = 0x106E;

// cl_khr_pci_bus_info
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct cl_device_pci_bus_info_khr {
    pub pci_domain: cl_uint,
    pub pci_bus: cl_uint,
    pub pci_device: cl_uint,
    pub pci_function: cl_uint,
}

// cl_device_info
pub const CL_DEVICE_PCI_BUS_INFO_KHR: cl_device_info = 0x410F;

// cl_khr_suggested_local_work_size

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_khr_suggested_local_work_size")]
extern "system" {
    pub fn clGetKernelSuggestedLocalWorkSizeKHR(
        command_queue: cl_command_queue,
        kernel: cl_kernel,
        work_dim: cl_uint,
        global_work_offset: *const size_t,
        global_work_size: *const size_t,
        suggested_local_work_size: *mut size_t,
    ) -> cl_int;
}

pub type clGetKernelSuggestedLocalWorkSizeKHR_fn = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        kernel: cl_kernel,
        work_dim: cl_uint,
        global_work_offset: *const size_t,
        global_work_size: *const size_t,
        suggested_local_work_size: *mut size_t,
    ) -> cl_int,
>;

// cl_khr_integer_dot_product

pub type cl_device_integer_dot_product_capabilities_khr = cl_bitfield;
// cl_device_integer_dot_product_capabilities_khr
pub const CL_DEVICE_INTEGER_DOT_PRODUCT_INPUT_4x8BIT_PACKED_KHR:
    cl_device_integer_dot_product_capabilities_khr = 1 << 0;
pub const CL_DEVICE_INTEGER_DOT_PRODUCT_INPUT_4x8BIT_KHR:
    cl_device_integer_dot_product_capabilities_khr = 1 << 1;

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct cl_device_integer_dot_product_acceleration_properties_khr {
    pub signed_accelerated: cl_bool,
    pub unsigned_accelerated: cl_bool,
    pub mixed_signedness_accelerated: cl_bool,
    pub accumulating_saturating_signed_accelerated: cl_bool,
    pub accumulating_saturating_unsigned_accelerated: cl_bool,
    pub accumulating_saturating_mixed_signedness_accelerated: cl_bool,
}

// cl_device_info
pub const CL_DEVICE_INTEGER_DOT_PRODUCT_CAPABILITIES_KHR: cl_device_info = 0x1073;
pub const CL_DEVICE_INTEGER_DOT_PRODUCT_ACCELERATION_PROPERTIES_8BIT_KHR: cl_device_info = 0x1074;
pub const CL_DEVICE_INTEGER_DOT_PRODUCT_ACCELERATION_PROPERTIES_4x8BIT_PACKED_KHR: cl_device_info =
    0x1075;

// cl_khr_external_memory

pub type cl_external_memory_handle_type_khr = cl_uint;

// cl_platform_info
pub const CL_PLATFORM_EXTERNAL_MEMORY_IMPORT_HANDLE_TYPES_KHR: cl_platform_info = 0x2044;

// cl_device_info
pub const CL_DEVICE_EXTERNAL_MEMORY_IMPORT_HANDLE_TYPES_KHR: cl_device_info = 0x204F;

// cl_mem_properties
pub const CL_DEVICE_HANDLE_LIST_KHR: cl_ulong = 0x2051;
pub const CL_DEVICE_HANDLE_LIST_END_KHR: cl_ulong = 0;

// cl_command_type
pub const CL_COMMAND_ACQUIRE_EXTERNAL_MEM_OBJECTS_KHR: cl_command_type = 0x2047;
pub const CL_COMMAND_RELEASE_EXTERNAL_MEM_OBJECTS_KHR: cl_command_type = 0x2048;

pub type clEnqueueAcquireExternalMemObjectsKHR_fn = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_mem_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueReleaseExternalMemObjectsKHR_fn = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_mem_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_khr_external_memory")]
extern "system" {
    pub fn clEnqueueAcquireExternalMemObjectsKHR(
        command_queue: cl_command_queue,
        num_mem_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueReleaseExternalMemObjectsKHR(
        command_queue: cl_command_queue,
        num_mem_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;
}

// cl_khr_external_memory_dma_buf

// cl_external_memory_handle_type_khr
pub const CL_EXTERNAL_MEMORY_HANDLE_DMA_BUF_KHR: cl_external_memory_handle_type_khr = 0x2067;

// cl_khr_external_memory_dx

// cl_external_memory_handle_type_khr
pub const CL_EXTERNAL_MEMORY_HANDLE_D3D11_TEXTURE_KHR: cl_external_memory_handle_type_khr = 0x2063;
pub const CL_EXTERNAL_MEMORY_HANDLE_D3D11_TEXTURE_KMT_KHR: cl_external_memory_handle_type_khr =
    0x2064;
pub const CL_EXTERNAL_MEMORY_HANDLE_D3D12_HEAP_KHR: cl_external_memory_handle_type_khr = 0x2065;
pub const CL_EXTERNAL_MEMORY_HANDLE_D3D12_RESOURCE_KHR: cl_external_memory_handle_type_khr = 0x2066;

// cl_khr_external_memory_opaque_fd

// cl_external_memory_handle_type_khr
pub const CL_EXTERNAL_MEMORY_HANDLE_OPAQUE_FD_KHR: cl_external_memory_handle_type_khr = 0x2060;

// cl_khr_external_memory_win32

// cl_external_memory_handle_type_khr
pub const CL_EXTERNAL_MEMORY_HANDLE_OPAQUE_WIN32_KHR: cl_external_memory_handle_type_khr = 0x2061;
pub const CL_EXTERNAL_MEMORY_HANDLE_OPAQUE_WIN32_KMT_KHR: cl_external_memory_handle_type_khr =
    0x2062;

// cl_khr_external_semaphore
pub type cl_semaphore_khr = *mut c_void;
pub type cl_external_semaphore_handle_type_khr = cl_uint;
pub type cl_semaphore_properties_khr = cl_properties;

// cl_platform_info
pub const CL_PLATFORM_SEMAPHORE_IMPORT_HANDLE_TYPES_KHR: cl_platform_info = 0x2037;
pub const CL_PLATFORM_SEMAPHORE_EXPORT_HANDLE_TYPES_KHR: cl_platform_info = 0x2038;

// cl_device_info
pub const CL_DEVICE_SEMAPHORE_IMPORT_HANDLE_TYPES_KHR: cl_device_info = 0x204D;
pub const CL_DEVICE_SEMAPHORE_EXPORT_HANDLE_TYPES_KHR: cl_device_info = 0x204E;

// cl_semaphore_properties_khr
pub const CL_SEMAPHORE_EXPORT_HANDLE_TYPES_KHR: cl_semaphore_properties_khr = 0x203F;
pub const CL_SEMAPHORE_EXPORT_HANDLE_TYPES_LIST_END_KHR: cl_semaphore_properties_khr = 0;

pub type clGetSemaphoreHandleForTypeKHR_fn = Option<
    unsafe extern "C" fn(
        sema_object: cl_semaphore_khr,
        device: cl_device_id,
        handle_type: cl_external_semaphore_handle_type_khr,
        handle_size: size_t,
        handle_ptr: *mut c_void,
        handle_size_ret: *mut size_t,
    ) -> cl_int,
>;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_khr_external_semaphore")]
extern "system" {
    pub fn clGetSemaphoreHandleForTypeKHR(
        sema_object: cl_semaphore_khr,
        device: cl_device_id,
        handle_type: cl_external_semaphore_handle_type_khr,
        handle_size: size_t,
        handle_ptr: *mut c_void,
        handle_size_ret: *mut size_t,
    ) -> cl_int;
}

// cl_khr_external_semaphore_dx_fence

// cl_external_semaphore_handle_type_khr
pub const CL_SEMAPHORE_HANDLE_D3D12_FENCE_KHR: cl_external_semaphore_handle_type_khr = 0x2059;

// cl_khr_external_semaphore_opaque_fd
pub const CL_SEMAPHORE_HANDLE_OPAQUE_FD_KHR: cl_external_semaphore_handle_type_khr = 0x2055;

// cl_khr_external_semaphore_sync_fd
pub const CL_SEMAPHORE_HANDLE_SYNC_FD_KHR: cl_external_semaphore_handle_type_khr = 0x2058;

// cl_khr_external_semaphore_win32
pub const CL_SEMAPHORE_HANDLE_OPAQUE_WIN32_KHR: cl_external_semaphore_handle_type_khr = 0x2056;
pub const CL_SEMAPHORE_HANDLE_OPAQUE_WIN32_KMT_KHR: cl_external_semaphore_handle_type_khr = 0x2057;

// cl_khr_semaphore

// pub type cl_semaphore_properties_khr = cl_properties; defined above
pub type cl_semaphore_info_khr = cl_uint;
pub type cl_semaphore_type_khr = cl_uint;
pub type cl_semaphore_payload_khr = cl_ulong;

// cl_semaphore_type
pub const CL_SEMAPHORE_TYPE_BINARY_KHR: cl_semaphore_type_khr = 1;

// cl_platform_info
pub const CL_PLATFORM_SEMAPHORE_TYPES_KHR: cl_platform_info = 0x2036;

// cl_device_info
pub const CL_DEVICE_SEMAPHORE_TYPES_KHR: cl_device_info = 0x204C;

// cl_semaphore_info_khr
pub const CL_SEMAPHORE_CONTEXT_KHR: cl_semaphore_info_khr = 0x2039;
pub const CL_SEMAPHORE_REFERENCE_COUNT_KHR: cl_semaphore_info_khr = 0x203A;
pub const CL_SEMAPHORE_PROPERTIES_KHR: cl_semaphore_info_khr = 0x203B;
pub const CL_SEMAPHORE_PAYLOAD_KHR: cl_semaphore_info_khr = 0x203C;

// cl_semaphore_info_khr or cl_semaphore_properties_khr
pub const CL_SEMAPHORE_TYPE_KHR: cl_semaphore_info_khr = 0x203D;
/* enum CL_DEVICE_HANDLE_LIST_KHR */
/* enum CL_DEVICE_HANDLE_LIST_END_KHR */

// cl_command_type
pub const CL_COMMAND_SEMAPHORE_WAIT_KHR: cl_command_type = 0x2042;
pub const CL_COMMAND_SEMAPHORE_SIGNAL_KHR: cl_command_type = 0x2043;

// Error codes
pub const CL_INVALID_SEMAPHORE_KHR: cl_int = -1142;

pub type clCreateSemaphoreWithPropertiesKHR_fn = Option<
    unsafe extern "C" fn(
        context: cl_context,
        sema_props: *const cl_semaphore_properties_khr,
        errcode_ret: *mut cl_int,
    ) -> cl_semaphore_khr,
>;

pub type clEnqueueWaitSemaphoresKHR_fn = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_sema_objects: cl_uint,
        sema_objects: *const cl_semaphore_khr,
        sema_payload_list: *const cl_semaphore_payload_khr,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueSignalSemaphoresKHR_fn = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        num_sema_objects: cl_uint,
        sema_objects: *const cl_semaphore_khr,
        sema_payload_list: *const cl_semaphore_payload_khr,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clGetSemaphoreInfoKHR_fn = Option<
    unsafe extern "C" fn(
        sema_object: cl_semaphore_khr,
        param_name: cl_semaphore_info_khr,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,
>;

pub type clReleaseSemaphoreKHR_fn =
    Option<unsafe extern "C" fn(sema_object: cl_semaphore_khr) -> cl_int>;

pub type clRetainSemaphoreKHR_fn =
    Option<unsafe extern "C" fn(sema_object: cl_semaphore_khr) -> cl_int>;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_khr_semaphore")]
extern "system" {
    pub fn clCreateSemaphoreWithPropertiesKHR(
        context: cl_context,
        sema_props: *const cl_semaphore_properties_khr,
        errcode_ret: *mut cl_int,
    ) -> cl_semaphore_khr;

    pub fn clEnqueueWaitSemaphoresKHR(
        command_queue: cl_command_queue,
        num_sema_objects: cl_uint,
        sema_objects: *const cl_semaphore_khr,
        sema_payload_list: *const cl_semaphore_payload_khr,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueSignalSemaphoresKHR(
        command_queue: cl_command_queue,
        num_sema_objects: cl_uint,
        sema_objects: *const cl_semaphore_khr,
        sema_payload_list: *const cl_semaphore_payload_khr,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clGetSemaphoreInfoKHR(
        sema_object: cl_semaphore_khr,
        param_name: cl_semaphore_info_khr,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    pub fn clReleaseSemaphoreKHR(sema_object: cl_semaphore_khr) -> cl_int;

    pub fn clRetainSemaphoreKHR(sema_object: cl_semaphore_khr) -> cl_int;
}

// cl_arm_import_memory extension

pub type cl_import_properties_arm = intptr_t;

/// Default and valid properties name for cl_arm_import_memory
pub const CL_IMPORT_TYPE_ARM: cl_import_properties_arm = 0x40B2;

/// Host process memory type default value for CL_IMPORT_TYPE_ARM property
pub const CL_IMPORT_TYPE_HOST_ARM: cl_import_properties_arm = 0x40B3;

/// DMA BUF memory type value for CL_IMPORT_TYPE_ARM property
pub const CL_IMPORT_TYPE_DMA_BUF_ARM: cl_import_properties_arm = 0x40B4;

/// Protected memory property
pub const CL_IMPORT_TYPE_PROTECTED_ARM: cl_import_properties_arm = 0x40B5;

/// Android hardware buffer type value for CL_IMPORT_TYPE_ARM property
pub const CL_IMPORT_TYPE_ANDROID_HARDWARE_BUFFER_ARM: cl_import_properties_arm = 0x41E2;

/// Data consistency with host property
pub const CL_IMPORT_DMA_BUF_DATA_CONSISTENCY_WITH_HOST_ARM: cl_import_properties_arm = 0x41E3;

/// Index of plane in a multiplanar hardware buffer
pub const CL_IMPORT_ANDROID_HARDWARE_BUFFER_PLANE_INDEX_ARM: cl_import_properties_arm = 0x41EF;

/// Index of layer in a multilayer hardware buffer
pub const CL_IMPORT_ANDROID_HARDWARE_BUFFER_LAYER_INDEX_ARM: cl_import_properties_arm = 0x41F0;

/// Import memory size value to indicate a size for the whole buffer.
pub const CL_IMPORT_MEMORY_WHOLE_ALLOCATION_ARM: cl_import_properties_arm =
    cl_import_properties_arm::MAX;

/* This extension adds a new function that allows for direct memory import into
 * OpenCL via the clImportMemoryARM function.
 *
 * Memory imported through this interface will be mapped into the device's page
 * tables directly, providing zero copy access. It will never fall back to copy
 * operations and aliased buffers.
 *
 * Types of memory supported for import are specified as additional extension
 * strings.
 *
 * This extension produces cl_mem allocations which are compatible with all other
 * users of cl_mem in the standard API.
 *
 * This extension maps pages with the same properties as the normal buffer creation
 * function clCreateBuffer.
 */
#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_arm_import_memory")]
extern "system" {
    pub fn clImportMemoryARM(
        context: cl_context,
        flags: cl_mem_flags,
        properties: *const cl_import_properties_arm,
        memory: *mut c_void,
        size: size_t,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;
}

// cl_arm_shared_virtual_memory extension

// Used by clGetDeviceInfo
pub const CL_DEVICE_SVM_CAPABILITIES_ARM: cl_device_info = 0x40B6;

// Used by clGetMemObjectInfo
pub const CL_MEM_USES_SVM_POINTER_ARM: cl_mem_info = 0x40B7;

// Used by clSetKernelExecInfoARM:
pub type cl_kernel_exec_info_arm = cl_uint;
pub const CL_KERNEL_EXEC_INFO_SVM_PTRS_ARM: cl_kernel_exec_info_arm = 0x40B8;
pub const CL_KERNEL_EXEC_INFO_SVM_FINE_GRAIN_SYSTEM_ARM: cl_kernel_exec_info_arm = 0x40B9;

// To be used by clGetEventInfo:
pub const CL_COMMAND_SVM_FREE_ARM: cl_event_info = 0x40BA;
pub const CL_COMMAND_SVM_MEMCPY_ARM: cl_event_info = 0x40BB;
pub const CL_COMMAND_SVM_MEMFILL_ARM: cl_event_info = 0x40BC;
pub const CL_COMMAND_SVM_MAP_ARM: cl_event_info = 0x40BD;
pub const CL_COMMAND_SVM_UNMAP_ARM: cl_event_info = 0x40BF;

// Flag values returned by clGetDeviceInfo with CL_DEVICE_SVM_CAPABILITIES_ARM as the param_name.
pub type cl_device_svm_capabilities_arm = cl_bitfield;
pub const CL_DEVICE_SVM_COARSE_GRAIN_BUFFER_ARM: cl_device_svm_capabilities_arm = 1 << 0;
pub const CL_DEVICE_SVM_FINE_GRAIN_BUFFER_ARM: cl_device_svm_capabilities_arm = 1 << 1;
pub const CL_DEVICE_SVM_FINE_GRAIN_SYSTEM_ARM: cl_device_svm_capabilities_arm = 1 << 2;
pub const CL_DEVICE_SVM_ATOMICS_ARM: cl_device_svm_capabilities_arm = 1 << 3;

// Flag values used by clSVMAllocARM:
pub type cl_svm_mem_flags_arm = cl_bitfield;
pub const CL_MEM_SVM_FINE_GRAIN_BUFFER_ARM: cl_svm_mem_flags_arm = 1 << 10;
pub const CL_MEM_SVM_ATOMICS_ARM: cl_svm_mem_flags_arm = 1 << 11;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_arm_shared_virtual_memory")]
extern "system" {
    pub fn clSVMAllocARM(
        context: cl_context,
        flags: cl_svm_mem_flags_arm,
        size: size_t,
        alignment: cl_uint,
    ) -> *mut c_void;

    pub fn clSVMFreeARM(context: cl_context, svm_pointer: *mut c_void);

    pub fn clEnqueueSVMFreeARM(
        command_queue: cl_command_queue,
        num_svm_pointers: cl_uint,
        svm_pointers: *mut *mut c_void,
        pfn_free_func: Option<
            unsafe extern "C" fn(
                queue: cl_command_queue,
                num_svm_pointers: cl_uint,
                svm_pointers: *mut *mut c_void,
                user_data: *mut c_void,
            ),
        >,
        user_data: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueSVMMemcpyARM(
        command_queue: cl_command_queue,
        blocking_copy: cl_bool,
        dst_ptr: *mut c_void,
        src_ptr: *const c_void,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueSVMMemFillARM(
        command_queue: cl_command_queue,
        svm_ptr: *mut c_void,
        pattern: *const c_void,
        pattern_size: size_t,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueSVMMapARM(
        command_queue: cl_command_queue,
        blocking_map: cl_bool,
        flags: cl_map_flags,
        svm_ptr: *mut c_void,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueSVMUnmapARM(
        command_queue: cl_command_queue,
        svm_ptr: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clSetKernelArgSVMPointerARM(
        kernel: cl_kernel,
        arg_index: cl_uint,
        arg_value: *const c_void,
    ) -> cl_int;

    pub fn clSetKernelExecInfoARM(
        kernel: cl_kernel,
        param_name: cl_kernel_exec_info_arm,
        param_value_size: size_t,
        param_value: *const c_void,
    ) -> cl_int;
}

// cl_arm_get_core_id extension
// #ifdef CL_VERSION_1_2
pub const CL_DEVICE_COMPUTE_UNITS_BITFIELD_ARM: cl_device_info = 0x40BF;
// #endif

// cl_arm_job_slot_selection
// cl_device_info
pub const CL_DEVICE_JOB_SLOTS_ARM: cl_device_info = 0x41E0;
// cl_command_queue_properties
pub const CL_QUEUE_JOB_SLOT_ARM: cl_command_queue_properties = 0x41E1;

// cl_arm_scheduling_controls

// cl_device_info
pub const CL_DEVICE_SCHEDULING_CONTROLS_CAPABILITIES_ARM: cl_device_info = 0x41E4;

pub type cl_device_scheduling_controls_capabilities_arm = cl_bitfield;
pub const CL_DEVICE_SCHEDULING_KERNEL_BATCHING_ARM: cl_device_scheduling_controls_capabilities_arm =
    1 << 0;
pub const CL_DEVICE_SCHEDULING_WORKGROUP_BATCH_SIZE_ARM:
    cl_device_scheduling_controls_capabilities_arm = 1 << 1;
pub const CL_DEVICE_SCHEDULING_WORKGROUP_BATCH_SIZE_MODIFIER_ARM:
    cl_device_scheduling_controls_capabilities_arm = 1 << 2;
pub const CL_DEVICE_SCHEDULING_DEFERRED_FLUSH_ARM: cl_device_scheduling_controls_capabilities_arm =
    1 << 3;
pub const CL_DEVICE_SCHEDULING_REGISTER_ALLOCATION_ARM:
    cl_device_scheduling_controls_capabilities_arm = 1 << 4;
pub const CL_DEVICE_SCHEDULING_WARP_THROTTLING_ARM: cl_device_scheduling_controls_capabilities_arm =
    1 << 5;
pub const CL_DEVICE_SCHEDULING_COMPUTE_UNIT_BATCH_QUEUE_SIZE_ARM:
    cl_device_scheduling_controls_capabilities_arm = 1 << 6;

pub const CL_DEVICE_SUPPORTED_REGISTER_ALLOCATIONS_ARM: cl_device_info = 0x41EB;
pub const CL_DEVICE_MAX_WARP_COUNT_ARM: cl_device_info = 0x41EA;

pub const CL_KERNEL_MAX_WARP_COUNT_ARM: cl_kernel_info = 0x41E9;

pub const CL_KERNEL_EXEC_INFO_WORKGROUP_BATCH_SIZE_ARM: cl_kernel_exec_info = 0x41E5;
pub const CL_KERNEL_EXEC_INFO_WORKGROUP_BATCH_SIZE_MODIFIER_ARM: cl_kernel_exec_info = 0x41E6;
pub const CL_KERNEL_EXEC_INFO_WARP_COUNT_LIMIT_ARM: cl_kernel_exec_info = 0x41E8;
pub const CL_KERNEL_EXEC_INFO_COMPUTE_UNIT_MAX_QUEUED_BATCHES_ARM: cl_kernel_exec_info = 0x41F1;

pub const CL_QUEUE_KERNEL_BATCHING_ARM: cl_queue_properties = 0x41E7;
pub const CL_QUEUE_DEFERRED_FLUSH_ARM: cl_queue_properties = 0x41EC;

// cl_arm_controlled_kernel_termination

// Error code to indicate kernel terminated with failure
pub const CL_COMMAND_TERMINATED_ITSELF_WITH_FAILURE_ARM: cl_int = -1108;

// cl_device_info
pub const CL_DEVICE_CONTROLLED_TERMINATION_CAPABILITIES_ARM: cl_device_info = 0x41EE;

// Bit fields for controlled termination feature query
pub type cl_device_controlled_termination_capabilities_arm = cl_bitfield;
pub const CL_DEVICE_CONTROLLED_TERMINATION_SUCCESS_ARM:
    cl_device_controlled_termination_capabilities_arm = 1 << 0;
pub const CL_DEVICE_CONTROLLED_TERMINATION_FAILURE_ARM:
    cl_device_controlled_termination_capabilities_arm = 1 << 1;
pub const CL_DEVICE_CONTROLLED_TERMINATION_QUERY_ARM:
    cl_device_controlled_termination_capabilities_arm = 1 << 2;

// cl_event_info
pub const CL_EVENT_COMMAND_TERMINATION_REASON_ARM: cl_event_info = 0x41ED;

// Values returned for event termination reason query
pub type cl_command_termination_reason_arm = cl_uint;
pub const CL_COMMAND_TERMINATION_COMPLETION_ARM: cl_command_termination_reason_arm = 0;
pub const CL_COMMAND_TERMINATION_CONTROLLED_SUCCESS_ARM: cl_command_termination_reason_arm = 1;
pub const CL_COMMAND_TERMINATION_CONTROLLED_FAILURE_ARM: cl_command_termination_reason_arm = 2;
pub const CL_COMMAND_TERMINATION_ERROR_ARM: cl_command_termination_reason_arm = 3;

// cl_arm_protected_memory_allocation
pub const CL_MEM_PROTECTED_ALLOC_ARM: cl_bitfield = 1 << 36;

// cl_intel_exec_by_local_thread extension
pub const CL_QUEUE_THREAD_LOCAL_EXEC_ENABLE_INTEL: cl_bitfield = 1 << 31;

// cl_intel_device_attribute_query
pub type cl_device_feature_capabilities_intel = cl_bitfield;
pub const CL_DEVICE_FEATURE_FLAG_DP4A_INTEL: cl_device_feature_capabilities_intel = 1 << 0;
pub const CL_DEVICE_FEATURE_FLAG_DPAS_INTEL: cl_device_feature_capabilities_intel = 1 << 1;

// cl_device_info
pub const CL_DEVICE_IP_VERSION_INTEL: cl_device_info = 0x4250;
pub const CL_DEVICE_ID_INTEL: cl_device_info = 0x4251;
pub const CL_DEVICE_NUM_SLICES_INTEL: cl_device_info = 0x4252;
pub const CL_DEVICE_NUM_SUB_SLICES_PER_SLICE_INTEL: cl_device_info = 0x4253;
pub const CL_DEVICE_NUM_EUS_PER_SUB_SLICE_INTEL: cl_device_info = 0x4254;
pub const CL_DEVICE_NUM_THREADS_PER_EU_INTEL: cl_device_info = 0x4255;
pub const CL_DEVICE_FEATURE_CAPABILITIES_INTEL: cl_device_info = 0x4256;

// cl_intel_device_partition_by_names extension

pub const CL_DEVICE_PARTITION_BY_NAMES_INTEL: cl_device_info = 0x4052;
pub const CL_PARTITION_BY_NAMES_LIST_END_INTEL: cl_int = -1;

// cl_intel_accelerator extension
// cl_intel_motion_estimation extension
// cl_intel_advanced_motion_estimation extension

pub type cl_accelerator_intel = *mut c_void;
pub type cl_accelerator_type_intel = cl_uint;
pub type cl_accelerator_info_intel = cl_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct cl_motion_estimation_desc_intel {
    pub mb_block_type: cl_uint,
    pub subpixel_mode: cl_uint,
    pub sad_adjust_mode: cl_uint,
    pub search_path_type: cl_uint,
}

// error codes
pub const CL_INVALID_ACCELERATOR_INTEL: cl_int = -1094;
pub const CL_INVALID_ACCELERATOR_TYPE_INTEL: cl_int = -1095;
pub const CL_INVALID_ACCELERATOR_DESCRIPTOR_INTEL: cl_int = -1096;
pub const CL_ACCELERATOR_TYPE_NOT_SUPPORTED_INTEL: cl_int = -1097;

// cl_accelerator_type_intel
pub const CL_ACCELERATOR_TYPE_MOTION_ESTIMATION_INTEL: cl_accelerator_type_intel = 0x0;

// cl_accelerator_info_intel
pub const CL_ACCELERATOR_DESCRIPTOR_INTEL: cl_accelerator_info_intel = 0x4090;
pub const CL_ACCELERATOR_REFERENCE_COUNT_INTEL: cl_accelerator_info_intel = 0x4091;
pub const CL_ACCELERATOR_CONTEXT_INTEL: cl_accelerator_info_intel = 0x4092;
pub const CL_ACCELERATOR_TYPE_INTEL: cl_accelerator_info_intel = 0x4093;

// cl_motion_detect_desc_intel flags
pub type cl_motion_detect_desc_intel = cl_uint;
pub const CL_ME_MB_TYPE_16x16_INTEL: cl_motion_detect_desc_intel = 0x0;
pub const CL_ME_MB_TYPE_8x8_INTEL: cl_motion_detect_desc_intel = 0x1;
pub const CL_ME_MB_TYPE_4x4_INTEL: cl_motion_detect_desc_intel = 0x2;

pub const CL_ME_SUBPIXEL_MODE_INTEGER_INTEL: cl_motion_detect_desc_intel = 0x0;
pub const CL_ME_SUBPIXEL_MODE_HPEL_INTEL: cl_motion_detect_desc_intel = 0x1;
pub const CL_ME_SUBPIXEL_MODE_QPEL_INTEL: cl_motion_detect_desc_intel = 0x2;

pub const CL_ME_SAD_ADJUST_MODE_NONE_INTEL: cl_motion_detect_desc_intel = 0x0;
pub const CL_ME_SAD_ADJUST_MODE_HAAR_INTEL: cl_motion_detect_desc_intel = 0x1;

pub const CL_ME_SEARCH_PATH_RADIUS_2_2_INTEL: cl_motion_detect_desc_intel = 0x0;
pub const CL_ME_SEARCH_PATH_RADIUS_4_4_INTEL: cl_motion_detect_desc_intel = 0x1;
pub const CL_ME_SEARCH_PATH_RADIUS_16_12_INTEL: cl_motion_detect_desc_intel = 0x5;

pub const CL_ME_SKIP_BLOCK_TYPE_16x16_INTEL: cl_motion_detect_desc_intel = 0x0;
pub const CL_ME_CHROMA_INTRA_PREDICT_ENABLED_INTEL: cl_motion_detect_desc_intel = 0x1;
pub const CL_ME_LUMA_INTRA_PREDICT_ENABLED_INTEL: cl_motion_detect_desc_intel = 0x2;
pub const CL_ME_SKIP_BLOCK_TYPE_8x8_INTEL: cl_motion_detect_desc_intel = 0x4;

pub const CL_ME_FORWARD_INPUT_MODE_INTEL: cl_motion_detect_desc_intel = 0x1;
pub const CL_ME_BACKWARD_INPUT_MODE_INTEL: cl_motion_detect_desc_intel = 0x2;
pub const CL_ME_BIDIRECTION_INPUT_MODE_INTEL: cl_motion_detect_desc_intel = 0x3;

pub const CL_ME_BIDIR_WEIGHT_QUARTER_INTEL: cl_motion_detect_desc_intel = 16;
pub const CL_ME_BIDIR_WEIGHT_THIRD_INTEL: cl_motion_detect_desc_intel = 21;
pub const CL_ME_BIDIR_WEIGHT_HALF_INTEL: cl_motion_detect_desc_intel = 32;
pub const CL_ME_BIDIR_WEIGHT_TWO_THIRD_INTEL: cl_motion_detect_desc_intel = 43;
pub const CL_ME_BIDIR_WEIGHT_THREE_QUARTER_INTEL: cl_motion_detect_desc_intel = 48;

pub const CL_ME_COST_PENALTY_NONE_INTEL: cl_motion_detect_desc_intel = 0x0;
pub const CL_ME_COST_PENALTY_LOW_INTEL: cl_motion_detect_desc_intel = 0x1;
pub const CL_ME_COST_PENALTY_NORMAL_INTEL: cl_motion_detect_desc_intel = 0x2;
pub const CL_ME_COST_PENALTY_HIGH_INTEL: cl_motion_detect_desc_intel = 0x3;

pub const CL_ME_COST_PRECISION_QPEL_INTEL: cl_motion_detect_desc_intel = 0x0;
pub const CL_ME_COST_PRECISION_HPEL_INTEL: cl_motion_detect_desc_intel = 0x1;
pub const CL_ME_COST_PRECISION_PEL_INTEL: cl_motion_detect_desc_intel = 0x2;
pub const CL_ME_COST_PRECISION_DPEL_INTEL: cl_motion_detect_desc_intel = 0x3;

pub const CL_ME_LUMA_PREDICTOR_MODE_VERTICAL_INTEL: cl_motion_detect_desc_intel = 0x0;
pub const CL_ME_LUMA_PREDICTOR_MODE_HORIZONTAL_INTEL: cl_motion_detect_desc_intel = 0x1;
pub const CL_ME_LUMA_PREDICTOR_MODE_DC_INTEL: cl_motion_detect_desc_intel = 0x2;
pub const CL_ME_LUMA_PREDICTOR_MODE_DIAGONAL_DOWN_LEFT_INTEL: cl_motion_detect_desc_intel = 0x3;

pub const CL_ME_LUMA_PREDICTOR_MODE_DIAGONAL_DOWN_RIGHT_INTEL: cl_motion_detect_desc_intel = 0x4;
pub const CL_ME_LUMA_PREDICTOR_MODE_PLANE_INTEL: cl_motion_detect_desc_intel = 0x4;
pub const CL_ME_LUMA_PREDICTOR_MODE_VERTICAL_RIGHT_INTEL: cl_motion_detect_desc_intel = 0x5;
pub const CL_ME_LUMA_PREDICTOR_MODE_HORIZONTAL_DOWN_INTEL: cl_motion_detect_desc_intel = 0x6;
pub const CL_ME_LUMA_PREDICTOR_MODE_VERTICAL_LEFT_INTEL: cl_motion_detect_desc_intel = 0x7;
pub const CL_ME_LUMA_PREDICTOR_MODE_HORIZONTAL_UP_INTEL: cl_motion_detect_desc_intel = 0x8;

pub const CL_ME_CHROMA_PREDICTOR_MODE_DC_INTEL: cl_motion_detect_desc_intel = 0x0;
pub const CL_ME_CHROMA_PREDICTOR_MODE_HORIZONTAL_INTEL: cl_motion_detect_desc_intel = 0x1;
pub const CL_ME_CHROMA_PREDICTOR_MODE_VERTICAL_INTEL: cl_motion_detect_desc_intel = 0x2;
pub const CL_ME_CHROMA_PREDICTOR_MODE_PLANE_INTEL: cl_motion_detect_desc_intel = 0x3;

// cl_device_info
pub const CL_DEVICE_ME_VERSION_INTEL: cl_device_info = 0x407E;

pub const CL_ME_VERSION_LEGACY_INTEL: cl_uint = 0x0;
pub const CL_ME_VERSION_ADVANCED_VER_1_INTEL: cl_uint = 0x1;
pub const CL_ME_VERSION_ADVANCED_VER_2_INTEL: cl_uint = 0x2;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_intel_accelerator")]
extern "system" {
    pub fn clCreateAcceleratorINTEL(
        context: cl_context,
        accelerator_type: cl_accelerator_type_intel,
        descriptor_size: size_t,
        descriptor: *const c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_accelerator_intel;

    pub fn clGetAcceleratorInfoINTEL(
        accelerator: cl_accelerator_intel,
        param_name: cl_accelerator_info_intel,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    pub fn clRetainAcceleratorINTEL(accelerator: cl_accelerator_intel) -> cl_int;

    pub fn clReleaseAcceleratorINTEL(accelerator: cl_accelerator_intel) -> cl_int;
}

pub type clCreateAcceleratorINTEL_fn = Option<
    unsafe extern "C" fn(
        context: cl_context,
        accelerator_type: cl_accelerator_type_intel,
        descriptor_size: size_t,
        descriptor: *const c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_accelerator_intel,
>;

pub type clGetAcceleratorInfoINTEL_fn = Option<
    unsafe extern "C" fn(
        accelerator: cl_accelerator_intel,
        param_name: cl_accelerator_info_intel,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,
>;

pub type clRetainAcceleratorINTEL_fn =
    Option<unsafe extern "C" fn(accelerator: cl_accelerator_intel) -> cl_int>;

pub type clReleaseAcceleratorINTEL_fn =
    Option<unsafe extern "C" fn(accelerator: cl_accelerator_intel) -> cl_int>;

// cl_intel_simultaneous_sharing extension
pub const CL_DEVICE_SIMULTANEOUS_INTEROPS_INTEL: cl_uint = 0x4104;
pub const CL_DEVICE_NUM_SIMULTANEOUS_INTEROPS_INTEL: cl_uint = 0x4105;

// cl_intel_egl_image_yuv extension
pub const CL_EGL_YUV_PLANE_INTEL: cl_uint = 0x4107;

// cl_intel_packed_yuv extension
pub const CL_YUYV_INTEL: cl_uint = 0x4076;
pub const CL_UYVY_INTEL: cl_uint = 0x4077;
pub const CL_YVYU_INTEL: cl_uint = 0x4078;
pub const CL_VYUY_INTEL: cl_uint = 0x4079;

// cl_intel_required_subgroup_size extension
pub const CL_DEVICE_SUB_GROUP_SIZES_INTEL: cl_uint = 0x4108;
pub const CL_KERNEL_SPILL_MEM_SIZE_INTEL: cl_uint = 0x4109;
pub const CL_KERNEL_COMPILE_SUB_GROUP_SIZE_INTEL: cl_uint = 0x410A;

// cl_intel_driver_diagnostics extension
pub const CL_CONTEXT_SHOW_DIAGNOSTICS_INTEL: cl_uint = 0x4106;

pub type cl_diagnostics_verbose_level = cl_uint;
pub const CL_CONTEXT_DIAGNOSTICS_LEVEL_ALL_INTEL: cl_diagnostics_verbose_level = 0xff;
pub const CL_CONTEXT_DIAGNOSTICS_LEVEL_GOOD_INTEL: cl_diagnostics_verbose_level = 1;
pub const CL_CONTEXT_DIAGNOSTICS_LEVEL_BAD_INTEL: cl_diagnostics_verbose_level = 1 << 1;
pub const CL_CONTEXT_DIAGNOSTICS_LEVEL_NEUTRAL_INTEL: cl_diagnostics_verbose_level = 1 << 2;

// cl_intel_planar_yuv extension
pub const CL_NV12_INTEL: cl_uint = 0x410E;

pub const CL_MEM_NO_ACCESS_INTEL: cl_uint = 1 << 24;
pub const CL_MEM_ACCESS_FLAGS_UNRESTRICTED_INTEL: cl_uint = 1 << 25;

pub const CL_DEVICE_PLANAR_YUV_MAX_WIDTH_INTEL: cl_uint = 0x417E;
pub const CL_DEVICE_PLANAR_YUV_MAX_HEIGHT_INTEL: cl_uint = 0x417F;

// cl_intel_device_side_avc_motion_estimation extension
pub type cl_intel_avc_motion_estimation = cl_uint;

pub const CL_DEVICE_AVC_ME_VERSION_INTEL: cl_uint = 0x410B;
pub const CL_DEVICE_AVC_ME_SUPPORTS_TEXTURE_SAMPLER_USE_INTEL: cl_uint = 0x410C;
pub const CL_DEVICE_AVC_ME_SUPPORTS_PREEMPTION_INTEL: cl_uint = 0x410D;

pub const CL_AVC_ME_VERSION_0_INTEL: cl_intel_avc_motion_estimation = 0x0; // No support.
pub const CL_AVC_ME_VERSION_1_INTEL: cl_intel_avc_motion_estimation = 0x1; // First supported version.

pub const CL_AVC_ME_MAJOR_16x16_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_MAJOR_16x8_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_MAJOR_8x16_INTEL: cl_intel_avc_motion_estimation = 0x2;
pub const CL_AVC_ME_MAJOR_8x8_INTEL: cl_intel_avc_motion_estimation = 0x3;

pub const CL_AVC_ME_MINOR_8x8_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_MINOR_8x4_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_MINOR_4x8_INTEL: cl_intel_avc_motion_estimation = 0x2;
pub const CL_AVC_ME_MINOR_4x4_INTEL: cl_intel_avc_motion_estimation = 0x3;

pub const CL_AVC_ME_MAJOR_FORWARD_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_MAJOR_BACKWARD_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_MAJOR_BIDIRECTIONAL_INTEL: cl_intel_avc_motion_estimation = 0x2;

pub const CL_AVC_ME_PARTITION_MASK_ALL_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_PARTITION_MASK_16x16_INTEL: cl_intel_avc_motion_estimation = 0x7E;
pub const CL_AVC_ME_PARTITION_MASK_16x8_INTEL: cl_intel_avc_motion_estimation = 0x7D;
pub const CL_AVC_ME_PARTITION_MASK_8x16_INTEL: cl_intel_avc_motion_estimation = 0x7B;
pub const CL_AVC_ME_PARTITION_MASK_8x8_INTEL: cl_intel_avc_motion_estimation = 0x77;
pub const CL_AVC_ME_PARTITION_MASK_8x4_INTEL: cl_intel_avc_motion_estimation = 0x6F;
pub const CL_AVC_ME_PARTITION_MASK_4x8_INTEL: cl_intel_avc_motion_estimation = 0x5F;
pub const CL_AVC_ME_PARTITION_MASK_4x4_INTEL: cl_intel_avc_motion_estimation = 0x3F;

pub const CL_AVC_ME_SEARCH_WINDOW_EXHAUSTIVE_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_SEARCH_WINDOW_SMALL_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_SEARCH_WINDOW_TINY_INTEL: cl_intel_avc_motion_estimation = 0x2;
pub const CL_AVC_ME_SEARCH_WINDOW_EXTRA_TINY_INTEL: cl_intel_avc_motion_estimation = 0x3;
pub const CL_AVC_ME_SEARCH_WINDOW_DIAMOND_INTEL: cl_intel_avc_motion_estimation = 0x4;
pub const CL_AVC_ME_SEARCH_WINDOW_LARGE_DIAMOND_INTEL: cl_intel_avc_motion_estimation = 0x5;
pub const CL_AVC_ME_SEARCH_WINDOW_RESERVED0_INTEL: cl_intel_avc_motion_estimation = 0x6;
pub const CL_AVC_ME_SEARCH_WINDOW_RESERVED1_INTEL: cl_intel_avc_motion_estimation = 0x7;
pub const CL_AVC_ME_SEARCH_WINDOW_CUSTOM_INTEL: cl_intel_avc_motion_estimation = 0x8;
pub const CL_AVC_ME_SEARCH_WINDOW_16x12_RADIUS_INTEL: cl_intel_avc_motion_estimation = 0x9;
pub const CL_AVC_ME_SEARCH_WINDOW_4x4_RADIUS_INTEL: cl_intel_avc_motion_estimation = 0x2;
pub const CL_AVC_ME_SEARCH_WINDOW_2x2_RADIUS_INTEL: cl_intel_avc_motion_estimation = 0xa;

pub const CL_AVC_ME_SAD_ADJUST_MODE_NONE_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_SAD_ADJUST_MODE_HAAR_INTEL: cl_intel_avc_motion_estimation = 0x2;

pub const CL_AVC_ME_SUBPIXEL_MODE_INTEGER_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_SUBPIXEL_MODE_HPEL_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_SUBPIXEL_MODE_QPEL_INTEL: cl_intel_avc_motion_estimation = 0x3;

pub const CL_AVC_ME_COST_PRECISION_QPEL_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_COST_PRECISION_HPEL_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_COST_PRECISION_PEL_INTEL: cl_intel_avc_motion_estimation = 0x2;
pub const CL_AVC_ME_COST_PRECISION_DPEL_INTEL: cl_intel_avc_motion_estimation = 0x3;

pub const CL_AVC_ME_BIDIR_WEIGHT_QUARTER_INTEL: cl_intel_avc_motion_estimation = 0x10;
pub const CL_AVC_ME_BIDIR_WEIGHT_THIRD_INTEL: cl_intel_avc_motion_estimation = 0x15;
pub const CL_AVC_ME_BIDIR_WEIGHT_HALF_INTEL: cl_intel_avc_motion_estimation = 0x20;
pub const CL_AVC_ME_BIDIR_WEIGHT_TWO_THIRD_INTEL: cl_intel_avc_motion_estimation = 0x2B;
pub const CL_AVC_ME_BIDIR_WEIGHT_THREE_QUARTER_INTEL: cl_intel_avc_motion_estimation = 0x30;

pub const CL_AVC_ME_BORDER_REACHED_LEFT_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_BORDER_REACHED_RIGHT_INTEL: cl_intel_avc_motion_estimation = 0x2;
pub const CL_AVC_ME_BORDER_REACHED_TOP_INTEL: cl_intel_avc_motion_estimation = 0x4;
pub const CL_AVC_ME_BORDER_REACHED_BOTTOM_INTEL: cl_intel_avc_motion_estimation = 0x8;

pub const CL_AVC_ME_SKIP_BLOCK_PARTITION_16x16_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_SKIP_BLOCK_PARTITION_8x8_INTEL: cl_intel_avc_motion_estimation = 0x4000;

pub const CL_AVC_ME_SKIP_BLOCK_16x16_FORWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x1 << 24;
pub const CL_AVC_ME_SKIP_BLOCK_16x16_BACKWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x2 << 24;
pub const CL_AVC_ME_SKIP_BLOCK_16x16_DUAL_ENABLE_INTEL: cl_intel_avc_motion_estimation = 0x3 << 24;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_FORWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x55 << 24;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_BACKWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0xAA << 24;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_DUAL_ENABLE_INTEL: cl_intel_avc_motion_estimation = 0xFF << 24;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_0_FORWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x1 << 24;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_0_BACKWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x2 << 24;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_1_FORWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x1 << 26;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_1_BACKWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x2 << 26;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_2_FORWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x1 << 28;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_2_BACKWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x2 << 28;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_3_FORWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x1 << 30;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_3_BACKWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x2 << 30;

pub const CL_AVC_ME_BLOCK_BASED_SKIP_4x4_INTEL: cl_intel_avc_motion_estimation = 0x00;
pub const CL_AVC_ME_BLOCK_BASED_SKIP_8x8_INTEL: cl_intel_avc_motion_estimation = 0x80;

pub const CL_AVC_ME_INTRA_16x16_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_INTRA_8x8_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_INTRA_4x4_INTEL: cl_intel_avc_motion_estimation = 0x2;

pub const CL_AVC_ME_INTRA_LUMA_PARTITION_MASK_16x16_INTEL: cl_intel_avc_motion_estimation = 0x6;
pub const CL_AVC_ME_INTRA_LUMA_PARTITION_MASK_8x8_INTEL: cl_intel_avc_motion_estimation = 0x5;
pub const CL_AVC_ME_INTRA_LUMA_PARTITION_MASK_4x4_INTEL: cl_intel_avc_motion_estimation = 0x3;

pub const CL_AVC_ME_INTRA_NEIGHBOR_LEFT_MASK_ENABLE_INTEL: cl_intel_avc_motion_estimation = 0x60;
pub const CL_AVC_ME_INTRA_NEIGHBOR_UPPER_MASK_ENABLE_INTEL: cl_intel_avc_motion_estimation = 0x10;
pub const CL_AVC_ME_INTRA_NEIGHBOR_UPPER_RIGHT_MASK_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x8;
pub const CL_AVC_ME_INTRA_NEIGHBOR_UPPER_LEFT_MASK_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x4;

pub const CL_AVC_ME_LUMA_PREDICTOR_MODE_VERTICAL_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_LUMA_PREDICTOR_MODE_HORIZONTAL_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_LUMA_PREDICTOR_MODE_DC_INTEL: cl_intel_avc_motion_estimation = 0x2;
pub const CL_AVC_ME_LUMA_PREDICTOR_MODE_DIAGONAL_DOWN_LEFT_INTEL: cl_intel_avc_motion_estimation =
    0x3;
pub const CL_AVC_ME_LUMA_PREDICTOR_MODE_DIAGONAL_DOWN_RIGHT_INTEL: cl_intel_avc_motion_estimation =
    0x4;
pub const CL_AVC_ME_LUMA_PREDICTOR_MODE_PLANE_INTEL: cl_intel_avc_motion_estimation = 0x4;
pub const CL_AVC_ME_LUMA_PREDICTOR_MODE_VERTICAL_RIGHT_INTEL: cl_intel_avc_motion_estimation = 0x5;
pub const CL_AVC_ME_LUMA_PREDICTOR_MODE_HORIZONTAL_DOWN_INTEL: cl_intel_avc_motion_estimation = 0x6;
pub const CL_AVC_ME_LUMA_PREDICTOR_MODE_VERTICAL_LEFT_INTEL: cl_intel_avc_motion_estimation = 0x7;
pub const CL_AVC_ME_LUMA_PREDICTOR_MODE_HORIZONTAL_UP_INTEL: cl_intel_avc_motion_estimation = 0x8;
pub const CL_AVC_ME_CHROMA_PREDICTOR_MODE_DC_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_CHROMA_PREDICTOR_MODE_HORIZONTAL_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_CHROMA_PREDICTOR_MODE_VERTICAL_INTEL: cl_intel_avc_motion_estimation = 0x2;
pub const CL_AVC_ME_CHROMA_PREDICTOR_MODE_PLANE_INTEL: cl_intel_avc_motion_estimation = 0x3;

pub const CL_AVC_ME_FRAME_FORWARD_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_FRAME_BACKWARD_INTEL: cl_intel_avc_motion_estimation = 0x2;
pub const CL_AVC_ME_FRAME_DUAL_INTEL: cl_intel_avc_motion_estimation = 0x3;

pub const CL_AVC_ME_SLICE_TYPE_PRED_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_SLICE_TYPE_BPRED_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_SLICE_TYPE_INTRA_INTEL: cl_intel_avc_motion_estimation = 0x2;

pub const CL_AVC_ME_INTERLACED_SCAN_TOP_FIELD_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_INTERLACED_SCAN_BOTTOM_FIELD_INTEL: cl_intel_avc_motion_estimation = 0x1;

// cl_intel_unified_shared_memory extension
pub type cl_device_unified_shared_memory_capabilities_intel = cl_bitfield;
pub type cl_mem_properties_intel = cl_properties;
pub type cl_mem_alloc_flags_intel = cl_bitfield;
pub type cl_mem_info_intel = cl_uint;
pub type cl_unified_shared_memory_type_intel = cl_uint;
pub type cl_mem_advice_intel = cl_uint;

// cl_device_info
pub const CL_DEVICE_HOST_MEM_CAPABILITIES_INTEL: cl_device_info = 0x4190;
pub const CL_DEVICE_DEVICE_MEM_CAPABILITIES_INTEL: cl_device_info = 0x4191;
pub const CL_DEVICE_SINGLE_DEVICE_SHARED_MEM_CAPABILITIES_INTEL: cl_device_info = 0x4192;
pub const CL_DEVICE_CROSS_DEVICE_SHARED_MEM_CAPABILITIES_INTEL: cl_device_info = 0x4193;
pub const CL_DEVICE_SHARED_SYSTEM_MEM_CAPABILITIES_INTEL: cl_device_info = 0x4194;

// cl_device_unified_shared_memory_capabilities_intel - bitfield
pub const CL_UNIFIED_SHARED_MEMORY_ACCESS_INTEL:
    cl_device_unified_shared_memory_capabilities_intel = 1 << 0;
pub const CL_UNIFIED_SHARED_MEMORY_ATOMIC_ACCESS_INTEL:
    cl_device_unified_shared_memory_capabilities_intel = 1 << 1;
pub const CL_UNIFIED_SHARED_MEMORY_CONCURRENT_ACCESS_INTEL:
    cl_device_unified_shared_memory_capabilities_intel = 1 << 2;
pub const CL_UNIFIED_SHARED_MEMORY_CONCURRENT_ATOMIC_ACCESS_INTEL:
    cl_device_unified_shared_memory_capabilities_intel = 1 << 3;

pub const CL_MEM_ALLOC_FLAGS_INTEL: cl_mem_properties_intel = 0x4195;

// cl_mem_alloc_flags_intel - bitfield
pub const CL_MEM_ALLOC_WRITE_COMBINED_INTEL: cl_mem_alloc_flags_intel = 1 << 0;
pub const CL_MEM_ALLOC_INITIAL_PLACEMENT_DEVICE_INTEL: cl_mem_alloc_flags_intel = 1 << 1;
pub const CL_MEM_ALLOC_INITIAL_PLACEMENT_HOST_INTEL: cl_mem_alloc_flags_intel = 1 << 2;

// cl_mem_alloc_info_intel
pub const CL_MEM_ALLOC_TYPE_INTEL: cl_mem_info_intel = 0x419A;
pub const CL_MEM_ALLOC_BASE_PTR_INTEL: cl_mem_info_intel = 0x419B;
pub const CL_MEM_ALLOC_SIZE_INTEL: cl_mem_info_intel = 0x419C;
pub const CL_MEM_ALLOC_DEVICE_INTEL: cl_mem_info_intel = 0x419D;

// cl_unified_shared_memory_type_intel
pub const CL_MEM_TYPE_UNKNOWN_INTEL: cl_unified_shared_memory_type_intel = 0x4196;
pub const CL_MEM_TYPE_HOST_INTEL: cl_unified_shared_memory_type_intel = 0x4197;
pub const CL_MEM_TYPE_DEVICE_INTEL: cl_unified_shared_memory_type_intel = 0x4198;
pub const CL_MEM_TYPE_SHARED_INTEL: cl_unified_shared_memory_type_intel = 0x4199;

// cl_kernel_exec_info
pub const CL_KERNEL_EXEC_INFO_INDIRECT_HOST_ACCESS_INTEL: cl_kernel_exec_info = 0x4200;
pub const CL_KERNEL_EXEC_INFO_INDIRECT_DEVICE_ACCESS_INTEL: cl_kernel_exec_info = 0x4201;
pub const CL_KERNEL_EXEC_INFO_INDIRECT_SHARED_ACCESS_INTEL: cl_kernel_exec_info = 0x4202;
pub const CL_KERNEL_EXEC_INFO_USM_PTRS_INTEL: cl_kernel_exec_info = 0x4203;

// cl_command_type
pub const CL_COMMAND_MEMFILL_INTEL: cl_command_type = 0x4204;
pub const CL_COMMAND_MEMCPY_INTEL: cl_command_type = 0x4205;
pub const CL_COMMAND_MIGRATEMEM_INTEL: cl_command_type = 0x4206;
pub const CL_COMMAND_MEMADVISE_INTEL: cl_command_type = 0x4207;

pub type clHostMemAllocINTEL_fn = Option<
    unsafe extern "C" fn(
        context: cl_context,
        properties: *const cl_mem_properties_intel,
        size: size_t,
        alignment: cl_uint,
        errcode_ret: *mut cl_int,
    ) -> *mut c_void,
>;

pub type clDeviceMemAllocINTEL_fn = Option<
    unsafe extern "C" fn(
        context: cl_context,
        device: cl_device_id,
        properties: *const cl_mem_properties_intel,
        size: size_t,
        alignment: cl_uint,
        errcode_ret: *mut cl_int,
    ) -> *mut c_void,
>;

pub type clSharedMemAllocINTEL_fn = Option<
    unsafe extern "C" fn(
        context: cl_context,
        device: cl_device_id,
        properties: *const cl_mem_properties_intel,
        size: size_t,
        alignment: cl_uint,
        errcode_ret: *mut cl_int,
    ) -> *mut c_void,
>;

pub type clMemFreeINTEL_fn =
    Option<unsafe extern "C" fn(context: cl_context, ptr: *mut c_void) -> cl_int>;

pub type clMemBlockingFreeINTEL_fn =
    Option<unsafe extern "C" fn(context: cl_context, ptr: *mut c_void) -> cl_int>;

pub type clGetMemAllocInfoINTEL_fn = Option<
    unsafe extern "C" fn(
        context: cl_context,
        ptr: *const c_void,
        param_name: cl_mem_info_intel,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,
>;

pub type clSetKernelArgMemPointerINTEL_fn = Option<
    unsafe extern "C" fn(kernel: cl_kernel, arg_index: cl_uint, arg_value: *const c_void) -> cl_int,
>;

pub type clEnqueueMemFillINTEL_fn = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        dst_ptr: *mut c_void,
        pattern: *const c_void,
        pattern_size: size_t,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueMemcpyINTEL_fn = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        blocking: cl_bool,
        dst_ptr: *mut c_void,
        src_ptr: *const c_void,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueMemAdviseINTEL_fn = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        ptr: *const c_void,
        size: size_t,
        advice: cl_mem_advice_intel,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueMigrateMemINTEL_fn = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        ptr: *const c_void,
        size: size_t,
        flags: cl_mem_migration_flags,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

pub type clEnqueueMemsetINTEL_fn = Option<
    unsafe extern "C" fn(
        command_queue: cl_command_queue,
        dst_ptr: *mut c_void,
        value: cl_int,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int,
>;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_intel_unified_shared_memory")]
extern "system" {
    pub fn clHostMemAllocINTEL(
        context: cl_context,
        properties: *const cl_mem_properties_intel,
        size: size_t,
        alignment: cl_uint,
        errcode_ret: *mut cl_int,
    ) -> *mut c_void;

    pub fn clDeviceMemAllocINTEL(
        context: cl_context,
        device: cl_device_id,
        properties: *const cl_mem_properties_intel,
        size: size_t,
        alignment: cl_uint,
        errcode_ret: *mut cl_int,
    ) -> *mut c_void;

    pub fn clSharedMemAllocINTEL(
        context: cl_context,
        device: cl_device_id,
        properties: *const cl_mem_properties_intel,
        size: size_t,
        alignment: cl_uint,
        errcode_ret: *mut cl_int,
    ) -> *mut c_void;

    pub fn clMemFreeINTEL(context: cl_context, ptr: *mut c_void) -> cl_int;

    pub fn clMemBlockingFreeINTEL(context: cl_context, ptr: *mut c_void) -> cl_int;

    pub fn clGetMemAllocInfoINTEL(
        context: cl_context,
        ptr: *const c_void,
        param_name: cl_mem_info_intel,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    pub fn clSetKernelArgMemPointerINTEL(
        kernel: cl_kernel,
        arg_index: cl_uint,
        arg_value: *const c_void,
    ) -> cl_int;

    pub fn clEnqueueMemFillINTEL(
        command_queue: cl_command_queue,
        dst_ptr: *mut c_void,
        pattern: *const c_void,
        pattern_size: size_t,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueMemcpyINTEL(
        command_queue: cl_command_queue,
        blocking: cl_bool,
        dst_ptr: *mut c_void,
        src_ptr: *const c_void,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueMemAdviseINTEL(
        command_queue: cl_command_queue,
        ptr: *const c_void,
        size: size_t,
        advice: cl_mem_advice_intel,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueMigrateMemINTEL(
        command_queue: cl_command_queue,
        ptr: *const c_void,
        size: size_t,
        flags: cl_mem_migration_flags,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueMemsetINTEL(
        command_queue: cl_command_queue,
        dst_ptr: *mut c_void,
        value: cl_int,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;
}

// cl_intel_mem_alloc_buffer_location

// cl_mem_properties_intel
pub const CL_MEM_ALLOC_BUFFER_LOCATION_INTEL: cl_mem_properties_intel = 0x419E;

// cl_intel_create_buffer_with_properties extension

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_intel_create_buffer_with_properties")]
extern "system" {

    pub fn clCreateBufferWithPropertiesINTEL(
        context: cl_context,
        properties: *const cl_mem_properties_intel,
        flags: cl_mem_flags,
        size: size_t,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;
}

pub type clCreateBufferWithPropertiesINTEL_fn = Option<
    unsafe extern "C" fn(
        context: cl_context,
        properties: *const cl_mem_properties_intel,
        flags: cl_mem_flags,
        size: size_t,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,
>;

// cl_intel_mem_channel_property extension

pub const CL_MEM_CHANNEL_INTEL: cl_uint = 0x4213;

// cl_intel_mem_force_host_memory

// cl_mem_flags
pub const CL_MEM_FORCE_HOST_MEMORY_INTEL: cl_mem_flags = 1 << 20;

// cl_intel_command_queue_families

pub type cl_command_queue_capabilities_intel = cl_bitfield;

pub const CL_QUEUE_FAMILY_MAX_NAME_SIZE_INTEL: usize = 64;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_queue_family_properties_intel {
    pub properties: cl_command_queue_properties,
    pub capabilities: cl_command_queue_capabilities_intel,
    pub count: cl_uint,
    pub name: [cl_uchar; CL_QUEUE_FAMILY_MAX_NAME_SIZE_INTEL],
}

// cl_device_info
pub const CL_DEVICE_QUEUE_FAMILY_PROPERTIES_INTEL: cl_device_info = 0x418B;

// cl_queue_properties
pub const CL_QUEUE_FAMILY_INTEL: cl_queue_properties = 0x418C;
pub const CL_QUEUE_INDEX_INTEL: cl_queue_properties = 0x418D;

// cl_command_queue_capabilities_intel
pub const CL_QUEUE_DEFAULT_CAPABILITIES_INTEL: cl_command_queue_capabilities_intel = 0;
pub const CL_QUEUE_CAPABILITY_CREATE_SINGLE_QUEUE_EVENTS_INTEL:
    cl_command_queue_capabilities_intel = 1 << 0;
pub const CL_QUEUE_CAPABILITY_CREATE_CROSS_QUEUE_EVENTS_INTEL: cl_command_queue_capabilities_intel =
    1 << 1;
pub const CL_QUEUE_CAPABILITY_SINGLE_QUEUE_EVENT_WAIT_LIST_INTEL:
    cl_command_queue_capabilities_intel = 1 << 2;
pub const CL_QUEUE_CAPABILITY_CROSS_QUEUE_EVENT_WAIT_LIST_INTEL:
    cl_command_queue_capabilities_intel = 1 << 3;
pub const CL_QUEUE_CAPABILITY_TRANSFER_BUFFER_INTEL: cl_command_queue_capabilities_intel = 1 << 8;
pub const CL_QUEUE_CAPABILITY_TRANSFER_BUFFER_RECT_INTEL: cl_command_queue_capabilities_intel =
    1 << 9;
pub const CL_QUEUE_CAPABILITY_MAP_BUFFER_INTEL: cl_command_queue_capabilities_intel = 1 << 10;
pub const CL_QUEUE_CAPABILITY_FILL_BUFFER_INTEL: cl_command_queue_capabilities_intel = 1 << 11;
pub const CL_QUEUE_CAPABILITY_TRANSFER_IMAGE_INTEL: cl_command_queue_capabilities_intel = 1 << 12;
pub const CL_QUEUE_CAPABILITY_MAP_IMAGE_INTEL: cl_command_queue_capabilities_intel = 1 << 13;
pub const CL_QUEUE_CAPABILITY_FILL_IMAGE_INTEL: cl_command_queue_capabilities_intel = 1 << 14;
pub const CL_QUEUE_CAPABILITY_TRANSFER_BUFFER_IMAGE_INTEL: cl_command_queue_capabilities_intel =
    1 << 15;
pub const CL_QUEUE_CAPABILITY_TRANSFER_IMAGE_BUFFER_INTEL: cl_command_queue_capabilities_intel =
    1 << 16;
pub const CL_QUEUE_CAPABILITY_MARKER_INTEL: cl_command_queue_capabilities_intel = 1 << 24;
pub const CL_QUEUE_CAPABILITY_BARRIER_INTEL: cl_command_queue_capabilities_intel = 1 << 25;
pub const CL_QUEUE_CAPABILITY_KERNEL_INTEL: cl_command_queue_capabilities_intel = 1 << 26;

// cl_intel_queue_no_sync_operations
pub const CL_QUEUE_NO_SYNC_OPERATIONS_INTEL: cl_command_queue_properties = 1 << 29;

// cl_ext_image_requirements_info

pub type cl_image_requirements_info_ext = cl_uint;

pub const CL_IMAGE_REQUIREMENTS_ROW_PITCH_ALIGNMENT_EXT: cl_image_requirements_info_ext = 0x1290;
pub const CL_IMAGE_REQUIREMENTS_BASE_ADDRESS_ALIGNMENT_EXT: cl_image_requirements_info_ext = 0x1292;
pub const CL_IMAGE_REQUIREMENTS_SIZE_EXT: cl_image_requirements_info_ext = 0x12B2;
pub const CL_IMAGE_REQUIREMENTS_MAX_WIDTH_EXT: cl_image_requirements_info_ext = 0x12B3;
pub const CL_IMAGE_REQUIREMENTS_MAX_HEIGHT_EXT: cl_image_requirements_info_ext = 0x12B4;
pub const CL_IMAGE_REQUIREMENTS_MAX_DEPTH_EXT: cl_image_requirements_info_ext = 0x12B5;
pub const CL_IMAGE_REQUIREMENTS_MAX_ARRAY_SIZE_EXT: cl_image_requirements_info_ext = 0x12B6;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "cl_ext_image_requirements_info")]
extern "system" {

    pub fn clGetImageRequirementsInfoEXT(
        context: cl_context,
        properties: *const cl_mem_properties,
        flags: cl_mem_flags,
        image_format: *const cl_image_format,
        image_desc: *const cl_image_desc,
        param_name: cl_image_requirements_info_ext,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;
}

pub type clGetImageRequirementsInfoEXT_fn = Option<
    unsafe extern "C" fn(
        context: cl_context,
        properties: *const cl_mem_properties,
        flags: cl_mem_flags,
        image_format: *const cl_image_format,
        image_desc: *const cl_image_desc,
        param_name: cl_image_requirements_info_ext,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int,
>;

// cl_ext_image_from_buffer

pub const CL_IMAGE_REQUIREMENTS_SLICE_PITCH_ALIGNMENT_EXT: cl_image_requirements_info_ext = 0x1291;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(deref_nullptr)]
    fn bindgen_test_layout_cl_mem_ext_host_ptr() {
        assert_eq!(
            ::std::mem::size_of::<cl_mem_ext_host_ptr>(),
            8usize,
            concat!("Size of: ", stringify!(cl_mem_ext_host_ptr))
        );
        assert_eq!(
            ::std::mem::align_of::<cl_mem_ext_host_ptr>(),
            4usize,
            concat!("Alignment of ", stringify!(cl_mem_ext_host_ptr))
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_mem_ext_host_ptr>())).allocation_type as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_mem_ext_host_ptr),
                "::",
                stringify!(allocation_type)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_mem_ext_host_ptr>())).host_cache_policy as *const _
                    as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_mem_ext_host_ptr),
                "::",
                stringify!(host_cache_policy)
            )
        );
    }

    #[test]
    #[allow(deref_nullptr)]
    fn bindgen_test_layout_cl_mem_ion_host_ptr() {
        assert_eq!(
            ::std::mem::size_of::<cl_mem_ion_host_ptr>(),
            24usize,
            concat!("Size of: ", stringify!(cl_mem_ion_host_ptr))
        );
        assert_eq!(
            ::std::mem::align_of::<cl_mem_ion_host_ptr>(),
            8usize,
            concat!("Alignment of ", stringify!(cl_mem_ion_host_ptr))
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_mem_ion_host_ptr>())).ext_host_ptr as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_mem_ion_host_ptr),
                "::",
                stringify!(ext_host_ptr)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_mem_ion_host_ptr>())).ion_filedesc as *const _ as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_mem_ion_host_ptr),
                "::",
                stringify!(ion_filedesc)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_mem_ion_host_ptr>())).ion_hostptr as *const _ as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_mem_ion_host_ptr),
                "::",
                stringify!(ion_hostptr)
            )
        );
    }

    #[test]
    #[allow(deref_nullptr)]
    fn bindgen_test_layout_cl_mem_android_native_buffer_host_ptr() {
        assert_eq!(
            ::std::mem::size_of::<cl_mem_android_native_buffer_host_ptr>(),
            16usize,
            concat!(
                "Size of: ",
                stringify!(cl_mem_android_native_buffer_host_ptr)
            )
        );
        assert_eq!(
            ::std::mem::align_of::<cl_mem_android_native_buffer_host_ptr>(),
            8usize,
            concat!(
                "Alignment of ",
                stringify!(cl_mem_android_native_buffer_host_ptr)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_mem_android_native_buffer_host_ptr>())).ext_host_ptr
                    as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_mem_android_native_buffer_host_ptr),
                "::",
                stringify!(ext_host_ptr)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_mem_android_native_buffer_host_ptr>())).anb_ptr
                    as *const _ as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_mem_android_native_buffer_host_ptr),
                "::",
                stringify!(anb_ptr)
            )
        );
    }

    #[test]
    #[allow(deref_nullptr)]
    fn bindgen_test_layout_cl_name_version_khr() {
        assert_eq!(
            ::std::mem::size_of::<cl_name_version_khr>(),
            68usize,
            concat!("Size of: ", stringify!(cl_name_version_khr))
        );
        assert_eq!(
            ::std::mem::align_of::<cl_name_version_khr>(),
            4usize,
            concat!("Alignment of ", stringify!(cl_name_version_khr))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<cl_name_version_khr>())).version as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_name_version_khr),
                "::",
                stringify!(version)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<cl_name_version_khr>())).name as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_name_version_khr),
                "::",
                stringify!(name)
            )
        );
    }

    #[test]
    #[allow(deref_nullptr)]
    fn bindgen_test_layout_cl_device_pci_bus_info_khr() {
        assert_eq!(
            ::std::mem::size_of::<cl_device_pci_bus_info_khr>(),
            16usize,
            concat!("Size of: ", stringify!(cl_device_pci_bus_info_khr))
        );
        assert_eq!(
            ::std::mem::align_of::<cl_device_pci_bus_info_khr>(),
            4usize,
            concat!("Alignment of ", stringify!(cl_device_pci_bus_info_khr))
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_device_pci_bus_info_khr>())).pci_domain as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_device_pci_bus_info_khr),
                "::",
                stringify!(pci_domain)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_device_pci_bus_info_khr>())).pci_bus as *const _ as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_device_pci_bus_info_khr),
                "::",
                stringify!(pci_bus)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_device_pci_bus_info_khr>())).pci_device as *const _
                    as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_device_pci_bus_info_khr),
                "::",
                stringify!(pci_device)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_device_pci_bus_info_khr>())).pci_function as *const _
                    as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_device_pci_bus_info_khr),
                "::",
                stringify!(pci_function)
            )
        );
    }

    #[test]
    #[allow(deref_nullptr)]
    fn bindgen_test_layout_cl_device_integer_dot_product_acceleration_properties_khr() {
        assert_eq!(
            ::std::mem::size_of::<cl_device_integer_dot_product_acceleration_properties_khr>(),
            24usize,
            concat!(
                "Size of: ",
                stringify!(cl_device_integer_dot_product_acceleration_properties_khr)
            )
        );
        assert_eq!(
            ::std::mem::align_of::<cl_device_integer_dot_product_acceleration_properties_khr>(),
            4usize,
            concat!(
                "Alignment of ",
                stringify!(cl_device_integer_dot_product_acceleration_properties_khr)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_device_integer_dot_product_acceleration_properties_khr>(
                )))
                .signed_accelerated as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_device_integer_dot_product_acceleration_properties_khr),
                "::",
                stringify!(signed_accelerated)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_device_integer_dot_product_acceleration_properties_khr>(
                )))
                .unsigned_accelerated as *const _ as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_device_integer_dot_product_acceleration_properties_khr),
                "::",
                stringify!(unsigned_accelerated)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_device_integer_dot_product_acceleration_properties_khr>(
                )))
                .mixed_signedness_accelerated as *const _ as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_device_integer_dot_product_acceleration_properties_khr),
                "::",
                stringify!(mixed_signedness_accelerated)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_device_integer_dot_product_acceleration_properties_khr>(
                )))
                .accumulating_saturating_signed_accelerated as *const _ as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_device_integer_dot_product_acceleration_properties_khr),
                "::",
                stringify!(accumulating_saturating_signed_accelerated)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_device_integer_dot_product_acceleration_properties_khr>(
                )))
                .accumulating_saturating_unsigned_accelerated as *const _ as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_device_integer_dot_product_acceleration_properties_khr),
                "::",
                stringify!(accumulating_saturating_unsigned_accelerated)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_device_integer_dot_product_acceleration_properties_khr>(
                )))
                .accumulating_saturating_mixed_signedness_accelerated as *const _
                    as usize
            },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_device_integer_dot_product_acceleration_properties_khr),
                "::",
                stringify!(accumulating_saturating_mixed_signedness_accelerated)
            )
        );
    }

    #[test]
    #[allow(deref_nullptr)]
    fn bindgen_test_layout_cl_motion_estimation_desc_intel() {
        assert_eq!(
            ::std::mem::size_of::<cl_motion_estimation_desc_intel>(),
            16usize,
            concat!("Size of: ", stringify!(cl_motion_estimation_desc_intel))
        );
        assert_eq!(
            ::std::mem::align_of::<cl_motion_estimation_desc_intel>(),
            4usize,
            concat!("Alignment of ", stringify!(cl_motion_estimation_desc_intel))
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_motion_estimation_desc_intel>())).mb_block_type
                    as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_motion_estimation_desc_intel),
                "::",
                stringify!(mb_block_type)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_motion_estimation_desc_intel>())).subpixel_mode
                    as *const _ as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_motion_estimation_desc_intel),
                "::",
                stringify!(subpixel_mode)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_motion_estimation_desc_intel>())).sad_adjust_mode
                    as *const _ as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_motion_estimation_desc_intel),
                "::",
                stringify!(sad_adjust_mode)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_motion_estimation_desc_intel>())).search_path_type
                    as *const _ as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_motion_estimation_desc_intel),
                "::",
                stringify!(search_path_type)
            )
        );
    }

    #[test]
    #[allow(deref_nullptr)]
    fn bindgen_test_layout_cl_queue_family_properties_intel() {
        assert_eq!(
            ::std::mem::size_of::<cl_queue_family_properties_intel>(),
            88usize,
            concat!("Size of: ", stringify!(cl_queue_family_properties_intel))
        );
        assert_eq!(
            ::std::mem::align_of::<cl_queue_family_properties_intel>(),
            8usize,
            concat!(
                "Alignment of ",
                stringify!(cl_queue_family_properties_intel)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_queue_family_properties_intel>())).properties as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_queue_family_properties_intel),
                "::",
                stringify!(properties)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_queue_family_properties_intel>())).capabilities
                    as *const _ as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_queue_family_properties_intel),
                "::",
                stringify!(capabilities)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_queue_family_properties_intel>())).count as *const _
                    as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_queue_family_properties_intel),
                "::",
                stringify!(count)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<cl_queue_family_properties_intel>())).name as *const _
                    as usize
            },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_queue_family_properties_intel),
                "::",
                stringify!(name)
            )
        );
    }
}
