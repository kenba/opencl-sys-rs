// Copyright (c) 2022-2023 Via Technology Ltd.
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

//! FFI bindings for [cl.h](https://github.com/KhronosGroup/OpenCL-Headers/blob/main/CL/cl.h)

#![allow(non_camel_case_types, non_upper_case_globals)]

use super::cl_platform::{cl_int, cl_uchar, cl_uint, cl_ulong};
#[allow(unused_imports)]
use libc::{c_char, c_uchar, c_void, intptr_t, size_t};

pub type cl_platform_id = *mut c_void;
pub type cl_device_id = *mut c_void;
pub type cl_context = *mut c_void;
pub type cl_command_queue = *mut c_void;
pub type cl_mem = *mut c_void;
pub type cl_program = *mut c_void;
pub type cl_kernel = *mut c_void;
pub type cl_event = *mut c_void;
pub type cl_sampler = *mut c_void;

pub type cl_bool = cl_uint; // WARNING!  Unlike cl_ types in cl_platform.h, cl_bool is not guaranteed to be the same size as the bool in kernels.
pub type cl_bitfield = cl_ulong;
pub type cl_properties = cl_ulong;
pub type cl_device_type = cl_bitfield;
pub type cl_platform_info = cl_uint;
pub type cl_device_info = cl_uint;
pub type cl_device_fp_config = cl_bitfield;
pub type cl_device_mem_cache_type = cl_uint;
pub type cl_device_local_mem_type = cl_uint;
pub type cl_device_exec_capabilities = cl_bitfield;
// #ifdef CL_VERSION_2_0
pub type cl_device_svm_capabilities = cl_bitfield;
// #endif
pub type cl_command_queue_properties = cl_bitfield;
// #ifdef CL_VERSION_1_2
pub type cl_device_partition_property = intptr_t;
pub type cl_device_affinity_domain = cl_bitfield;
// #endif

pub type cl_context_properties = intptr_t;
pub type cl_context_info = cl_uint;
// #ifdef CL_VERSION_2_0
pub type cl_queue_properties = cl_properties;
// #endif
pub type cl_command_queue_info = cl_uint;
pub type cl_channel_order = cl_uint;
pub type cl_channel_type = cl_uint;
pub type cl_mem_flags = cl_bitfield;
// #ifdef CL_VERSION_2_0
pub type cl_svm_mem_flags = cl_bitfield;
// #endif
pub type cl_mem_object_type = cl_uint;
pub type cl_mem_info = cl_uint;
// #ifdef CL_VERSION_1_2
pub type cl_mem_migration_flags = cl_bitfield;
// #endif
pub type cl_image_info = cl_uint;
// #ifdef CL_VERSION_1_1
pub type cl_buffer_create_type = cl_uint;
// #endif
pub type cl_addressing_mode = cl_uint;
pub type cl_filter_mode = cl_uint;
pub type cl_sampler_info = cl_uint;
pub type cl_map_flags = cl_bitfield;
// #ifdef CL_VERSION_2_0
pub type cl_pipe_properties = intptr_t;
pub type cl_pipe_info = cl_uint;
// #endif
pub type cl_program_info = cl_uint;
pub type cl_program_build_info = cl_uint;
// #ifdef CL_VERSION_1_2
pub type cl_program_binary_type = cl_uint;
// #endif
pub type cl_build_status = cl_int;
pub type cl_kernel_info = cl_uint;
// #ifdef CL_VERSION_1_2
pub type cl_kernel_arg_info = cl_uint;
pub type cl_kernel_arg_address_qualifier = cl_uint;
pub type cl_kernel_arg_access_qualifier = cl_uint;
pub type cl_kernel_arg_type_qualifier = cl_uint;
// #endif
pub type cl_kernel_work_group_info = cl_uint;
// #ifdef CL_VERSION_2_1
pub type cl_kernel_sub_group_info = cl_uint;
// #endif
pub type cl_event_info = cl_uint;
pub type cl_command_type = cl_uint;
pub type cl_profiling_info = cl_uint;
// #ifdef CL_VERSION_2_0
pub type cl_sampler_properties = cl_bitfield;
pub type cl_kernel_exec_info = cl_uint;
// #endif
// #ifdef CL_VERSION_3_0
pub type cl_device_atomic_capabilities = cl_bitfield;
pub type cl_device_device_enqueue_capabilities = cl_bitfield;
pub type cl_khronos_vendor_id = cl_uint;
pub type cl_mem_properties = cl_properties;
pub type cl_version = cl_uint;
// #endif

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_image_format {
    pub image_channel_order: cl_channel_order,
    pub image_channel_data_type: cl_channel_type,
}

// #ifdef CL_VERSION_1_2

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_image_desc {
    pub image_type: cl_mem_object_type,
    pub image_width: size_t,
    pub image_height: size_t,
    pub image_depth: size_t,
    pub image_array_size: size_t,
    pub image_row_pitch: size_t,
    pub image_slice_pitch: size_t,
    pub num_mip_levels: cl_uint,
    pub num_samples: cl_uint,
    // AKA `mem_object` in CL_VERSION_2_0+
    pub buffer: cl_mem,
}

// #endif

// #ifdef CL_VERSION_1_1

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_buffer_region {
    pub origin: size_t,
    pub size: size_t,
}

// #endif

// #ifdef CL_VERSION_3_0
pub const CL_NAME_VERSION_MAX_NAME_SIZE: usize = 64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cl_name_version {
    pub version: cl_version,
    pub name: [cl_uchar; CL_NAME_VERSION_MAX_NAME_SIZE],
}

// #endif

/******************************************************************************/

// Error Codes:
pub const CL_SUCCESS: cl_int = 0;
pub const CL_DEVICE_NOT_FOUND: cl_int = -1;
pub const CL_DEVICE_NOT_AVAILABLE: cl_int = -2;
pub const CL_COMPILER_NOT_AVAILABLE: cl_int = -3;
pub const CL_MEM_OBJECT_ALLOCATION_FAILURE: cl_int = -4;
pub const CL_OUT_OF_RESOURCES: cl_int = -5;
pub const CL_OUT_OF_HOST_MEMORY: cl_int = -6;
pub const CL_PROFILING_INFO_NOT_AVAILABLE: cl_int = -7;
pub const CL_MEM_COPY_OVERLAP: cl_int = -8;
pub const CL_IMAGE_FORMAT_MISMATCH: cl_int = -9;
pub const CL_IMAGE_FORMAT_NOT_SUPPORTED: cl_int = -10;
pub const CL_BUILD_PROGRAM_FAILURE: cl_int = -11;
pub const CL_MAP_FAILURE: cl_int = -12;
// #ifdef CL_VERSION_1_1
pub const CL_MISALIGNED_SUB_BUFFER_OFFSET: cl_int = -13;
pub const CL_EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST: cl_int = -14;
// #endif
// #ifdef CL_VERSION_1_2
pub const CL_COMPILE_PROGRAM_FAILURE: cl_int = -15;
pub const CL_LINKER_NOT_AVAILABLE: cl_int = -16;
pub const CL_LINK_PROGRAM_FAILURE: cl_int = -17;
pub const CL_DEVICE_PARTITION_FAILED: cl_int = -18;
pub const CL_KERNEL_ARG_INFO_NOT_AVAILABLE: cl_int = -19;
// #endif

pub const CL_INVALID_VALUE: cl_int = -30;
pub const CL_INVALID_DEVICE_TYPE: cl_int = -31;
pub const CL_INVALID_PLATFORM: cl_int = -32;
pub const CL_INVALID_DEVICE: cl_int = -33;
pub const CL_INVALID_CONTEXT: cl_int = -34;
pub const CL_INVALID_QUEUE_PROPERTIES: cl_int = -35;
pub const CL_INVALID_COMMAND_QUEUE: cl_int = -36;
pub const CL_INVALID_HOST_PTR: cl_int = -37;
pub const CL_INVALID_MEM_OBJECT: cl_int = -38;
pub const CL_INVALID_IMAGE_FORMAT_DESCRIPTOR: cl_int = -39;
pub const CL_INVALID_IMAGE_SIZE: cl_int = -40;
pub const CL_INVALID_SAMPLER: cl_int = -41;
pub const CL_INVALID_BINARY: cl_int = -42;
pub const CL_INVALID_BUILD_OPTIONS: cl_int = -43;
pub const CL_INVALID_PROGRAM: cl_int = -44;
pub const CL_INVALID_PROGRAM_EXECUTABLE: cl_int = -45;
pub const CL_INVALID_KERNEL_NAME: cl_int = -46;
pub const CL_INVALID_KERNEL_DEFINITION: cl_int = -47;
pub const CL_INVALID_KERNEL: cl_int = -48;
pub const CL_INVALID_ARG_INDEX: cl_int = -49;
pub const CL_INVALID_ARG_VALUE: cl_int = -50;
pub const CL_INVALID_ARG_SIZE: cl_int = -51;
pub const CL_INVALID_KERNEL_ARGS: cl_int = -52;
pub const CL_INVALID_WORK_DIMENSION: cl_int = -53;
pub const CL_INVALID_WORK_GROUP_SIZE: cl_int = -54;
pub const CL_INVALID_WORK_ITEM_SIZE: cl_int = -55;
pub const CL_INVALID_GLOBAL_OFFSET: cl_int = -56;
pub const CL_INVALID_EVENT_WAIT_LIST: cl_int = -57;
pub const CL_INVALID_EVENT: cl_int = -58;
pub const CL_INVALID_OPERATION: cl_int = -59;
pub const CL_INVALID_GL_OBJECT: cl_int = -60;
pub const CL_INVALID_BUFFER_SIZE: cl_int = -61;
pub const CL_INVALID_MIP_LEVEL: cl_int = -62;
pub const CL_INVALID_GLOBAL_WORK_SIZE: cl_int = -63;
// #ifdef CL_VERSION_1_1
pub const CL_INVALID_PROPERTY: cl_int = -64;
// #endif
// #ifdef CL_VERSION_1_2
pub const CL_INVALID_IMAGE_DESCRIPTOR: cl_int = -65;
pub const CL_INVALID_COMPILER_OPTIONS: cl_int = -66;
pub const CL_INVALID_LINKER_OPTIONS: cl_int = -67;
pub const CL_INVALID_DEVICE_PARTITION_COUNT: cl_int = -68;
// #endif
// #ifdef CL_VERSION_2_0
pub const CL_INVALID_PIPE_SIZE: cl_int = -69;
pub const CL_INVALID_DEVICE_QUEUE: cl_int = -70;
// #endif
// #ifdef CL_VERSION_2_2
pub const CL_INVALID_SPEC_ID: cl_int = -71;
pub const CL_MAX_SIZE_RESTRICTION_EXCEEDED: cl_int = -72;
// #endif

// cl_bool:
pub const CL_FALSE: cl_bool = 0;
pub const CL_TRUE: cl_bool = 1;
// #ifdef CL_VERSION_1_2
pub const CL_BLOCKING: cl_bool = CL_TRUE;
pub const CL_NON_BLOCKING: cl_bool = CL_FALSE;
// #endif

// cl_platform_info
pub const CL_PLATFORM_PROFILE: cl_platform_info = 0x0900;
pub const CL_PLATFORM_VERSION: cl_platform_info = 0x0901;
pub const CL_PLATFORM_NAME: cl_platform_info = 0x0902;
pub const CL_PLATFORM_VENDOR: cl_platform_info = 0x0903;
pub const CL_PLATFORM_EXTENSIONS: cl_platform_info = 0x0904;
// #ifdef CL_VERSION_2_0
pub const CL_PLATFORM_HOST_TIMER_RESOLUTION: cl_platform_info = 0x0905;
// #endif
// #ifdef CL_VERSION_3_0
pub const CL_PLATFORM_NUMERIC_VERSION: cl_platform_info = 0x0906;
pub const CL_PLATFORM_EXTENSIONS_WITH_VERSION: cl_platform_info = 0x0907;
// #endif

// cl_device_type - bitfield:
pub const CL_DEVICE_TYPE_DEFAULT: cl_device_type = 1 << 0;
pub const CL_DEVICE_TYPE_CPU: cl_device_type = 1 << 1;
pub const CL_DEVICE_TYPE_GPU: cl_device_type = 1 << 2;
pub const CL_DEVICE_TYPE_ACCELERATOR: cl_device_type = 1 << 3;
// #ifdef CL_VERSION_1_2
pub const CL_DEVICE_TYPE_CUSTOM: cl_device_type = 1 << 4;
pub const CL_DEVICE_TYPE_ALL: cl_device_type = 0xFFFF_FFFF;
// #endif

// cl_device_info:
pub const CL_DEVICE_TYPE: cl_device_info = 0x1000;
pub const CL_DEVICE_VENDOR_ID: cl_device_info = 0x1001;
pub const CL_DEVICE_MAX_COMPUTE_UNITS: cl_device_info = 0x1002;
pub const CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS: cl_device_info = 0x1003;
pub const CL_DEVICE_MAX_WORK_GROUP_SIZE: cl_device_info = 0x1004;
pub const CL_DEVICE_MAX_WORK_ITEM_SIZES: cl_device_info = 0x1005;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR: cl_device_info = 0x1006;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT: cl_device_info = 0x1007;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT: cl_device_info = 0x1008;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG: cl_device_info = 0x1009;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_FLOAT: cl_device_info = 0x100A;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE: cl_device_info = 0x100B;
pub const CL_DEVICE_MAX_CLOCK_FREQUENCY: cl_device_info = 0x100C;
pub const CL_DEVICE_ADDRESS_BITS: cl_device_info = 0x100D;
pub const CL_DEVICE_MAX_READ_IMAGE_ARGS: cl_device_info = 0x100E;
pub const CL_DEVICE_MAX_WRITE_IMAGE_ARGS: cl_device_info = 0x100F;
pub const CL_DEVICE_MAX_MEM_ALLOC_SIZE: cl_device_info = 0x1010;
pub const CL_DEVICE_IMAGE2D_MAX_WIDTH: cl_device_info = 0x1011;
pub const CL_DEVICE_IMAGE2D_MAX_HEIGHT: cl_device_info = 0x1012;
pub const CL_DEVICE_IMAGE3D_MAX_WIDTH: cl_device_info = 0x1013;
pub const CL_DEVICE_IMAGE3D_MAX_HEIGHT: cl_device_info = 0x1014;
pub const CL_DEVICE_IMAGE3D_MAX_DEPTH: cl_device_info = 0x1015;
pub const CL_DEVICE_IMAGE_SUPPORT: cl_device_info = 0x1016;
pub const CL_DEVICE_MAX_PARAMETER_SIZE: cl_device_info = 0x1017;
pub const CL_DEVICE_MAX_SAMPLERS: cl_device_info = 0x1018;
pub const CL_DEVICE_MEM_BASE_ADDR_ALIGN: cl_device_info = 0x1019;
pub const CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE: cl_device_info = 0x101A;
pub const CL_DEVICE_SINGLE_FP_CONFIG: cl_device_info = 0x101B;
pub const CL_DEVICE_GLOBAL_MEM_CACHE_TYPE: cl_device_info = 0x101C;
pub const CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE: cl_device_info = 0x101D;
pub const CL_DEVICE_GLOBAL_MEM_CACHE_SIZE: cl_device_info = 0x101E;
pub const CL_DEVICE_GLOBAL_MEM_SIZE: cl_device_info = 0x101F;
pub const CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE: cl_device_info = 0x1020;
pub const CL_DEVICE_MAX_CONSTANT_ARGS: cl_device_info = 0x1021;
pub const CL_DEVICE_LOCAL_MEM_TYPE: cl_device_info = 0x1022;
pub const CL_DEVICE_LOCAL_MEM_SIZE: cl_device_info = 0x1023;
pub const CL_DEVICE_ERROR_CORRECTION_SUPPORT: cl_device_info = 0x1024;
pub const CL_DEVICE_PROFILING_TIMER_RESOLUTION: cl_device_info = 0x1025;
pub const CL_DEVICE_ENDIAN_LITTLE: cl_device_info = 0x1026;
pub const CL_DEVICE_AVAILABLE: cl_device_info = 0x1027;
pub const CL_DEVICE_COMPILER_AVAILABLE: cl_device_info = 0x1028;
pub const CL_DEVICE_EXECUTION_CAPABILITIES: cl_device_info = 0x1029;
pub const CL_DEVICE_QUEUE_PROPERTIES: cl_device_info = 0x102A; // deprecated
// #ifdef CL_VERSION_2_0
pub const CL_DEVICE_QUEUE_ON_HOST_PROPERTIES: cl_device_info = 0x102A;
// #endif
pub const CL_DEVICE_NAME: cl_device_info = 0x102B;
pub const CL_DEVICE_VENDOR: cl_device_info = 0x102C;
pub const CL_DRIVER_VERSION: cl_device_info = 0x102D;
pub const CL_DEVICE_PROFILE: cl_device_info = 0x102E;
pub const CL_DEVICE_VERSION: cl_device_info = 0x102F;
pub const CL_DEVICE_EXTENSIONS: cl_device_info = 0x1030;
pub const CL_DEVICE_PLATFORM: cl_device_info = 0x1031;
// #ifdef CL_VERSION_1_2
// pub const CL_DEVICE_DOUBLE_FP_CONFIG: cl_device_info = 0x1032; defined in "cl_ext.h"
// #endif
// 0x1033 reserved for CL_DEVICE_HALF_FP_CONFIG which is already defined in "cl_ext.h"
// #ifdef CL_VERSION_1_1
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF: cl_device_info = 0x1034;
pub const CL_DEVICE_HOST_UNIFIED_MEMORY: cl_device_info = 0x1035; // deprecated
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR: cl_device_info = 0x1036;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT: cl_device_info = 0x1037;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_INT: cl_device_info = 0x1038;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG: cl_device_info = 0x1039;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT: cl_device_info = 0x103A;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE: cl_device_info = 0x103B;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF: cl_device_info = 0x103C;
pub const CL_DEVICE_OPENCL_C_VERSION: cl_device_info = 0x103D;
// #endif
// #ifdef CL_VERSION_1_2
pub const CL_DEVICE_LINKER_AVAILABLE: cl_device_info = 0x103E;
pub const CL_DEVICE_BUILT_IN_KERNELS: cl_device_info = 0x103F;
pub const CL_DEVICE_IMAGE_MAX_BUFFER_SIZE: cl_device_info = 0x1040;
pub const CL_DEVICE_IMAGE_MAX_ARRAY_SIZE: cl_device_info = 0x1041;
pub const CL_DEVICE_PARENT_DEVICE: cl_device_info = 0x1042;
pub const CL_DEVICE_PARTITION_MAX_SUB_DEVICES: cl_device_info = 0x1043;
pub const CL_DEVICE_PARTITION_PROPERTIES: cl_device_info = 0x1044;
pub const CL_DEVICE_PARTITION_AFFINITY_DOMAIN: cl_device_info = 0x1045;
pub const CL_DEVICE_PARTITION_TYPE: cl_device_info = 0x1046;
pub const CL_DEVICE_REFERENCE_COUNT: cl_device_info = 0x1047;
pub const CL_DEVICE_PREFERRED_INTEROP_USER_SYNC: cl_device_info = 0x1048;
pub const CL_DEVICE_PRINTF_BUFFER_SIZE: cl_device_info = 0x1049;
// #endif
// #ifdef CL_VERSION_2_0
pub const CL_DEVICE_IMAGE_PITCH_ALIGNMENT: cl_device_info = 0x104A;
pub const CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT: cl_device_info = 0x104B;
pub const CL_DEVICE_MAX_READ_WRITE_IMAGE_ARGS: cl_device_info = 0x104C;
pub const CL_DEVICE_MAX_GLOBAL_VARIABLE_SIZE: cl_device_info = 0x104D;
pub const CL_DEVICE_QUEUE_ON_DEVICE_PROPERTIES: cl_device_info = 0x104E;
pub const CL_DEVICE_QUEUE_ON_DEVICE_PREFERRED_SIZE: cl_device_info = 0x104F;
pub const CL_DEVICE_QUEUE_ON_DEVICE_MAX_SIZE: cl_device_info = 0x1050;
pub const CL_DEVICE_MAX_ON_DEVICE_QUEUES: cl_device_info = 0x1051;
pub const CL_DEVICE_MAX_ON_DEVICE_EVENTS: cl_device_info = 0x1052;
pub const CL_DEVICE_SVM_CAPABILITIES: cl_device_info = 0x1053;
pub const CL_DEVICE_GLOBAL_VARIABLE_PREFERRED_TOTAL_SIZE: cl_device_info = 0x1054;
pub const CL_DEVICE_MAX_PIPE_ARGS: cl_device_info = 0x1055;
pub const CL_DEVICE_PIPE_MAX_ACTIVE_RESERVATIONS: cl_device_info = 0x1056;
pub const CL_DEVICE_PIPE_MAX_PACKET_SIZE: cl_device_info = 0x1057;
pub const CL_DEVICE_PREFERRED_PLATFORM_ATOMIC_ALIGNMENT: cl_device_info = 0x1058;
pub const CL_DEVICE_PREFERRED_GLOBAL_ATOMIC_ALIGNMENT: cl_device_info = 0x1059;
pub const CL_DEVICE_PREFERRED_LOCAL_ATOMIC_ALIGNMENT: cl_device_info = 0x105A;
// #endif
// #ifdef CL_VERSION_2_1
pub const CL_DEVICE_IL_VERSION: cl_device_info = 0x105B;
pub const CL_DEVICE_MAX_NUM_SUB_GROUPS: cl_device_info = 0x105C;
pub const CL_DEVICE_SUB_GROUP_INDEPENDENT_FORWARD_PROGRESS: cl_device_info = 0x105D;
// #endif
// #ifdef CL_VERSION_3_0
pub const CL_DEVICE_NUMERIC_VERSION: cl_device_info = 0x105E;
pub const CL_DEVICE_EXTENSIONS_WITH_VERSION: cl_device_info = 0x1060;
pub const CL_DEVICE_ILS_WITH_VERSION: cl_device_info = 0x1061;
pub const CL_DEVICE_BUILT_IN_KERNELS_WITH_VERSION: cl_device_info = 0x1062;
pub const CL_DEVICE_ATOMIC_MEMORY_CAPABILITIES: cl_device_info = 0x1063;
pub const CL_DEVICE_ATOMIC_FENCE_CAPABILITIES: cl_device_info = 0x1064;
pub const CL_DEVICE_NON_UNIFORM_WORK_GROUP_SUPPORT: cl_device_info = 0x1065;
pub const CL_DEVICE_OPENCL_C_ALL_VERSIONS: cl_device_info = 0x1066;
pub const CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_MULTIPLE: cl_device_info = 0x1067;
pub const CL_DEVICE_WORK_GROUP_COLLECTIVE_FUNCTIONS_SUPPORT: cl_device_info = 0x1068;
pub const CL_DEVICE_GENERIC_ADDRESS_SPACE_SUPPORT: cl_device_info = 0x1069;
// 0x106A to 0x106E - Reserved for KHR extension, cl_khr_device_uuid
pub const CL_DEVICE_OPENCL_C_FEATURES: cl_device_info = 0x106F;
pub const CL_DEVICE_DEVICE_ENQUEUE_CAPABILITIES: cl_device_info = 0x1070;
pub const CL_DEVICE_PIPE_SUPPORT: cl_device_info = 0x1071;
pub const CL_DEVICE_LATEST_CONFORMANCE_VERSION_PASSED: cl_device_info = 0x1072;
// #endif

// cl_device_fp_config - bitfield:
pub const CL_FP_DENORM: cl_device_fp_config = 1 << 0;
pub const CL_FP_INF_NAN: cl_device_fp_config = 1 << 1;
pub const CL_FP_ROUND_TO_NEAREST: cl_device_fp_config = 1 << 2;
pub const CL_FP_ROUND_TO_ZERO: cl_device_fp_config = 1 << 3;
pub const CL_FP_ROUND_TO_INF: cl_device_fp_config = 1 << 4;
pub const CL_FP_FMA: cl_device_fp_config = 1 << 5;
// #ifdef CL_VERSION_1_1
pub const CL_FP_SOFT_FLOAT: cl_device_fp_config = 1 << 6;
// #endif
// #ifdef CL_VERSION_1_2
pub const CL_FP_CORRECTLY_ROUNDED_DIVIDE_SQRT: cl_device_fp_config = 1 << 7;
// #endif

// cl_device_mem_cache_type
pub const CL_NONE: cl_device_mem_cache_type = 0x0;
pub const CL_READ_ONLY_CACHE: cl_device_mem_cache_type = 0x1;
pub const CL_READ_WRITE_CACHE: cl_device_mem_cache_type = 0x2;

// cl_device_local_mem_type
pub const CL_LOCAL: cl_device_local_mem_type = 0x1;
pub const CL_GLOBAL: cl_device_local_mem_type = 0x2;

// cl_device_exec_capabilities - bitfield:
pub const CL_EXEC_KERNEL: cl_device_exec_capabilities = 1 << 0;
pub const CL_EXEC_NATIVE_KERNEL: cl_device_exec_capabilities = 1 << 1;

// cl_command_queue_properties - bitfield:
pub const CL_QUEUE_OUT_OF_ORDER_EXEC_MODE_ENABLE: cl_command_queue_properties = 1 << 0;
pub const CL_QUEUE_PROFILING_ENABLE: cl_command_queue_properties = 1 << 1;
// #ifdef CL_VERSION_2_0
pub const CL_QUEUE_ON_DEVICE: cl_command_queue_properties = 1 << 2;
pub const CL_QUEUE_ON_DEVICE_DEFAULT: cl_command_queue_properties = 1 << 3;
// #endif

// cl_context_info
pub const CL_CONTEXT_REFERENCE_COUNT: cl_context_info = 0x1080;
pub const CL_CONTEXT_DEVICES: cl_context_info = 0x1081;
pub const CL_CONTEXT_PROPERTIES: cl_context_info = 0x1082;
// #ifdef CL_VERSION_1_1
pub const CL_CONTEXT_NUM_DEVICES: cl_context_info = 0x1083;
// #endif

// cl_context_properties:
pub const CL_CONTEXT_PLATFORM: cl_context_properties = 0x1084;
// #ifdef CL_VERSION_1_2
pub const CL_CONTEXT_INTEROP_USER_SYNC: cl_context_properties = 0x1085;
// #endif

// #ifdef CL_VERSION_1_2

// cl_device_partition_property
pub const CL_DEVICE_PARTITION_EQUALLY: cl_device_partition_property = 0x1086;
pub const CL_DEVICE_PARTITION_BY_COUNTS: cl_device_partition_property = 0x1087;
pub const CL_DEVICE_PARTITION_BY_COUNTS_LIST_END: cl_device_partition_property = 0x0;
pub const CL_DEVICE_PARTITION_BY_AFFINITY_DOMAIN: cl_device_partition_property = 0x1088;

// #endif

// #ifdef CL_VERSION_1_2

// cl_device_affinity_domain
pub const CL_DEVICE_AFFINITY_DOMAIN_NUMA: cl_device_affinity_domain = 1 << 0;
pub const CL_DEVICE_AFFINITY_DOMAIN_L4_CACHE: cl_device_affinity_domain = 1 << 1;
pub const CL_DEVICE_AFFINITY_DOMAIN_L3_CACHE: cl_device_affinity_domain = 1 << 2;
pub const CL_DEVICE_AFFINITY_DOMAIN_L2_CACHE: cl_device_affinity_domain = 1 << 3;
pub const CL_DEVICE_AFFINITY_DOMAIN_L1_CACHE: cl_device_affinity_domain = 1 << 4;
pub const CL_DEVICE_AFFINITY_DOMAIN_NEXT_PARTITIONABLE: cl_device_affinity_domain = 1 << 5;

// #endif

// #ifdef CL_VERSION_2_0

// cl_device_svm_capabilities
pub const CL_DEVICE_SVM_COARSE_GRAIN_BUFFER: cl_device_svm_capabilities = 1 << 0;
pub const CL_DEVICE_SVM_FINE_GRAIN_BUFFER: cl_device_svm_capabilities = 1 << 1;
pub const CL_DEVICE_SVM_FINE_GRAIN_SYSTEM: cl_device_svm_capabilities = 1 << 2;
pub const CL_DEVICE_SVM_ATOMICS: cl_device_svm_capabilities = 1 << 3;

// #endif

// cl_command_queue_info
pub const CL_QUEUE_CONTEXT: cl_command_queue_info = 0x1090;
pub const CL_QUEUE_DEVICE: cl_command_queue_info = 0x1091;
pub const CL_QUEUE_REFERENCE_COUNT: cl_command_queue_info = 0x1092;
pub const CL_QUEUE_PROPERTIES: cl_command_queue_info = 0x1093;
// #ifdef CL_VERSION_2_0
pub const CL_QUEUE_SIZE: cl_command_queue_info = 0x1094;
// #endif
// #ifdef CL_VERSION_2_1
pub const CL_QUEUE_DEVICE_DEFAULT: cl_command_queue_info = 0x1095;
// #endif
// #ifdef CL_VERSION_3_0
pub const CL_QUEUE_PROPERTIES_ARRAY: cl_command_queue_info = 0x1098;
// #endif

// cl_mem_flags and cl_svm_mem_flags - bitfield
pub const CL_MEM_READ_WRITE: cl_mem_flags = 1 << 0;
pub const CL_MEM_WRITE_ONLY: cl_mem_flags = 1 << 1;
pub const CL_MEM_READ_ONLY: cl_mem_flags = 1 << 2;
pub const CL_MEM_USE_HOST_PTR: cl_mem_flags = 1 << 3;
pub const CL_MEM_ALLOC_HOST_PTR: cl_mem_flags = 1 << 4;
pub const CL_MEM_COPY_HOST_PTR: cl_mem_flags = 1 << 5;
// reserved                      cl_mem_flags = 1 << 6;
// #ifdef CL_VERSION_1_2
pub const CL_MEM_HOST_WRITE_ONLY: cl_mem_flags = 1 << 7;
pub const CL_MEM_HOST_READ_ONLY: cl_mem_flags = 1 << 8;
pub const CL_MEM_HOST_NO_ACCESS: cl_mem_flags = 1 << 9;
// #endif
// #ifdef CL_VERSION_2_0
pub const CL_MEM_SVM_FINE_GRAIN_BUFFER: cl_svm_mem_flags = 1 << 10; // used by cl_svm_mem_flags only
pub const CL_MEM_SVM_ATOMICS: cl_svm_mem_flags = 1 << 11; // used by cl_svm_mem_flags only
pub const CL_MEM_KERNEL_READ_AND_WRITE: cl_svm_mem_flags = 1 << 12;
// #endif

// #ifdef CL_VERSION_1_2

// cl_mem_migration_flags - bitfield:
pub const CL_MIGRATE_MEM_OBJECT_HOST: cl_svm_mem_flags = 1 << 0;
pub const CL_MIGRATE_MEM_OBJECT_CONTENT_UNDEFINED: cl_svm_mem_flags = 1 << 1;
// #endif

// cl_channel_order
pub const CL_R: cl_channel_order = 0x10B0;
pub const CL_A: cl_channel_order = 0x10B1;
pub const CL_RG: cl_channel_order = 0x10B2;
pub const CL_RA: cl_channel_order = 0x10B3;
pub const CL_RGB: cl_channel_order = 0x10B4;
pub const CL_RGBA: cl_channel_order = 0x10B5;
pub const CL_BGRA: cl_channel_order = 0x10B6;
pub const CL_ARGB: cl_channel_order = 0x10B7;
pub const CL_INTENSITY: cl_channel_order = 0x10B8;
pub const CL_LUMINANCE: cl_channel_order = 0x10B9;
// #ifdef CL_VERSION_1_1
pub const CL_Rx: cl_channel_order = 0x10BA;
pub const CL_RGx: cl_channel_order = 0x10BB;
pub const CL_RGBx: cl_channel_order = 0x10BC;
// #endif
// #ifdef CL_VERSION_2_0
pub const CL_DEPTH: cl_channel_order = 0x10BD;
pub const CL_sRGB: cl_channel_order = 0x10BF;
pub const CL_sRGBx: cl_channel_order = 0x10C0;
pub const CL_sRGBA: cl_channel_order = 0x10C1;
pub const CL_sBGRA: cl_channel_order = 0x10C2;
pub const CL_ABGR: cl_channel_order = 0x10C3;
// #endif

// cl_channel_type
pub const CL_SNORM_INT8: cl_channel_type = 0x10D0;
pub const CL_SNORM_INT16: cl_channel_type = 0x10D1;
pub const CL_UNORM_INT8: cl_channel_type = 0x10D2;
pub const CL_UNORM_INT16: cl_channel_type = 0x10D3;
pub const CL_UNORM_SHORT_565: cl_channel_type = 0x10D4;
pub const CL_UNORM_SHORT_555: cl_channel_type = 0x10D5;
pub const CL_UNORM_INT_101010: cl_channel_type = 0x10D6;
pub const CL_SIGNED_INT8: cl_channel_type = 0x10D7;
pub const CL_SIGNED_INT16: cl_channel_type = 0x10D8;
pub const CL_SIGNED_INT32: cl_channel_type = 0x10D9;
pub const CL_UNSIGNED_INT8: cl_channel_type = 0x10DA;
pub const CL_UNSIGNED_INT16: cl_channel_type = 0x10DB;
pub const CL_UNSIGNED_INT32: cl_channel_type = 0x10DC;
pub const CL_HALF_FLOAT: cl_channel_type = 0x10DD;
pub const CL_FLOAT: cl_channel_type = 0x10DE;
// #ifdef CL_VERSION_2_1
pub const CL_UNORM_INT_101010_2: cl_channel_type = 0x10E0;
// #endif

// cl_mem_object_type
pub const CL_MEM_OBJECT_BUFFER: cl_mem_object_type = 0x10F0;
pub const CL_MEM_OBJECT_IMAGE2D: cl_mem_object_type = 0x10F1;
pub const CL_MEM_OBJECT_IMAGE3D: cl_mem_object_type = 0x10F2;
// #ifdef CL_VERSION_1_2
pub const CL_MEM_OBJECT_IMAGE2D_ARRAY: cl_mem_object_type = 0x10F3;
pub const CL_MEM_OBJECT_IMAGE1D: cl_mem_object_type = 0x10F4;
pub const CL_MEM_OBJECT_IMAGE1D_ARRAY: cl_mem_object_type = 0x10F5;
pub const CL_MEM_OBJECT_IMAGE1D_BUFFER: cl_mem_object_type = 0x10F6;
// #endif
// #ifdef CL_VERSION_2_0
pub const CL_MEM_OBJECT_PIPE: cl_mem_object_type = 0x10F7;
// #endif

// cl_mem_info:
pub const CL_MEM_TYPE: cl_mem_info = 0x1100;
pub const CL_MEM_FLAGS: cl_mem_info = 0x1101;
pub const CL_MEM_SIZE: cl_mem_info = 0x1102;
pub const CL_MEM_HOST_PTR: cl_mem_info = 0x1103;
pub const CL_MEM_MAP_COUNT: cl_mem_info = 0x1104;
pub const CL_MEM_REFERENCE_COUNT: cl_mem_info = 0x1105;
pub const CL_MEM_CONTEXT: cl_mem_info = 0x1106;
// #ifdef CL_VERSION_1_1
pub const CL_MEM_ASSOCIATED_MEMOBJECT: cl_mem_info = 0x1107;
pub const CL_MEM_OFFSET: cl_mem_info = 0x1108;
// #endif
// #ifdef CL_VERSION_2_0
pub const CL_MEM_USES_SVM_POINTER: cl_mem_info = 0x1109;
// #endif
// #ifdef CL_VERSION_3_0
pub const CL_MEM_PROPERTIES: cl_mem_info = 0x110A;
// #endif

// cl_image_info
pub const CL_IMAGE_FORMAT: cl_image_info = 0x1110;
pub const CL_IMAGE_ELEMENT_SIZE: cl_image_info = 0x1111;
pub const CL_IMAGE_ROW_PITCH: cl_image_info = 0x1112;
pub const CL_IMAGE_SLICE_PITCH: cl_image_info = 0x1113;
pub const CL_IMAGE_WIDTH: cl_image_info = 0x1114;
pub const CL_IMAGE_HEIGHT: cl_image_info = 0x1115;
pub const CL_IMAGE_DEPTH: cl_image_info = 0x1116;
// #ifdef CL_VERSION_1_2
pub const CL_IMAGE_ARRAY_SIZE: cl_image_info = 0x1117;
pub const CL_IMAGE_BUFFER: cl_image_info = 0x1118;
pub const CL_IMAGE_NUM_MIP_LEVELS: cl_image_info = 0x1119;
pub const CL_IMAGE_NUM_SAMPLES: cl_image_info = 0x111A;
// #endif

// cl_pipe_info
// #ifdef CL_VERSION_2_0
pub const CL_PIPE_PACKET_SIZE: cl_pipe_info = 0x1120;
pub const CL_PIPE_MAX_PACKETS: cl_pipe_info = 0x1121;
// #endif
// #ifdef CL_VERSION_3_0
pub const CL_PIPE_PROPERTIES: cl_pipe_info = 0x1122;
// #endif

// cl_addressing_mode
pub const CL_ADDRESS_NONE: cl_addressing_mode = 0x1130;
pub const CL_ADDRESS_CLAMP_TO_EDGE: cl_addressing_mode = 0x1131;
pub const CL_ADDRESS_CLAMP: cl_addressing_mode = 0x1132;
pub const CL_ADDRESS_REPEAT: cl_addressing_mode = 0x1133;
// #ifdef CL_VERSION_1_1
pub const CL_ADDRESS_MIRRORED_REPEAT: cl_addressing_mode = 0x1134;
// #endif

// cl_filter_mode
pub const CL_FILTER_NEAREST: cl_filter_mode = 0x1140;
pub const CL_FILTER_LINEAR: cl_filter_mode = 0x1141;

// cl_sampler_info
pub const CL_SAMPLER_REFERENCE_COUNT: cl_sampler_info = 0x1150;
pub const CL_SAMPLER_CONTEXT: cl_sampler_info = 0x1151;
pub const CL_SAMPLER_NORMALIZED_COORDS: cl_sampler_info = 0x1152;
pub const CL_SAMPLER_ADDRESSING_MODE: cl_sampler_info = 0x1153;
pub const CL_SAMPLER_FILTER_MODE: cl_sampler_info = 0x1154;
// #ifdef CL_VERSION_2_0
// These enumerants are for the cl_khr_mipmap_image extension.
// They have since been added to cl_ext.h with an appropriate
// KHR suffix, but are left here for backwards compatibility.
pub const CL_SAMPLER_MIP_FILTER_MODE: cl_sampler_info = 0x1155;
pub const CL_SAMPLER_LOD_MIN: cl_sampler_info = 0x1156;
pub const CL_SAMPLER_LOD_MAX: cl_sampler_info = 0x1157;
// #endif
// #ifdef CL_VERSION_3_0
pub const CL_SAMPLER_PROPERTIES: cl_sampler_info = 0x1158;
// #endif

// cl_map_flags - bitfield
pub const CL_MAP_READ: cl_map_flags = 1 << 0;
pub const CL_MAP_WRITE: cl_map_flags = 1 << 1;
// #ifdef CL_VERSION_1_2
pub const CL_MAP_WRITE_INVALIDATE_REGION: cl_map_flags = 1 << 2;
// #endif

// cl_program_info
pub const CL_PROGRAM_REFERENCE_COUNT: cl_program_info = 0x1160;
pub const CL_PROGRAM_CONTEXT: cl_program_info = 0x1161;
pub const CL_PROGRAM_NUM_DEVICES: cl_program_info = 0x1162;
pub const CL_PROGRAM_DEVICES: cl_program_info = 0x1163;
pub const CL_PROGRAM_SOURCE: cl_program_info = 0x1164;
pub const CL_PROGRAM_BINARY_SIZES: cl_program_info = 0x1165;
pub const CL_PROGRAM_BINARIES: cl_program_info = 0x1166;
// #ifdef CL_VERSION_1_2
pub const CL_PROGRAM_NUM_KERNELS: cl_program_info = 0x1167;
pub const CL_PROGRAM_KERNEL_NAMES: cl_program_info = 0x1168;
// #endif
// #ifdef CL_VERSION_2_1
pub const CL_PROGRAM_IL: cl_program_info = 0x1169;
// #endif
// #ifdef CL_VERSION_2_2
pub const CL_PROGRAM_SCOPE_GLOBAL_CTORS_PRESENT: cl_program_info = 0x116A;
pub const CL_PROGRAM_SCOPE_GLOBAL_DTORS_PRESENT: cl_program_info = 0x116B;
// #endif

// cl_program_build_info
pub const CL_PROGRAM_BUILD_STATUS: cl_program_build_info = 0x1181;
pub const CL_PROGRAM_BUILD_OPTIONS: cl_program_build_info = 0x1182;
pub const CL_PROGRAM_BUILD_LOG: cl_program_build_info = 0x1183;
// #ifdef CL_VERSION_1_2
pub const CL_PROGRAM_BINARY_TYPE: cl_program_build_info = 0x1184;
// #endif
// #ifdef CL_VERSION_2_0
pub const CL_PROGRAM_BUILD_GLOBAL_VARIABLE_TOTAL_SIZE: cl_program_build_info = 0x1185;
// #endif

// #ifdef CL_VERSION_1_2

// cl_program_binary_type
pub const CL_PROGRAM_BINARY_TYPE_NONE: cl_program_binary_type = 0x0;
pub const CL_PROGRAM_BINARY_TYPE_COMPILED_OBJECT: cl_program_binary_type = 0x1;
pub const CL_PROGRAM_BINARY_TYPE_LIBRARY: cl_program_binary_type = 0x2;
pub const CL_PROGRAM_BINARY_TYPE_EXECUTABLE: cl_program_binary_type = 0x4;

// #endif

// cl_build_status
pub const CL_BUILD_SUCCESS: cl_build_status = 0;
pub const CL_BUILD_NONE: cl_build_status = -1;
pub const CL_BUILD_ERROR: cl_build_status = -2;
pub const CL_BUILD_IN_PROGRESS: cl_build_status = -3;

// cl_kernel_info
pub const CL_KERNEL_FUNCTION_NAME: cl_kernel_info = 0x1190;
pub const CL_KERNEL_NUM_ARGS: cl_kernel_info = 0x1191;
pub const CL_KERNEL_REFERENCE_COUNT: cl_kernel_info = 0x1192;
pub const CL_KERNEL_CONTEXT: cl_kernel_info = 0x1193;
pub const CL_KERNEL_PROGRAM: cl_kernel_info = 0x1194;
// #ifdef CL_VERSION_1_2
pub const CL_KERNEL_ATTRIBUTES: cl_kernel_info = 0x1195;
// #endif

// #ifdef CL_VERSION_1_2

// cl_kernel_arg_info
pub const CL_KERNEL_ARG_ADDRESS_QUALIFIER: cl_kernel_arg_info = 0x1196;
pub const CL_KERNEL_ARG_ACCESS_QUALIFIER: cl_kernel_arg_info = 0x1197;
pub const CL_KERNEL_ARG_TYPE_NAME: cl_kernel_arg_info = 0x1198;
pub const CL_KERNEL_ARG_TYPE_QUALIFIER: cl_kernel_arg_info = 0x1199;
pub const CL_KERNEL_ARG_NAME: cl_kernel_arg_info = 0x119A;

// #endif

// #ifdef CL_VERSION_1_2

// cl_kernel_arg_address_qualifier
pub const CL_KERNEL_ARG_ADDRESS_GLOBAL: cl_kernel_arg_address_qualifier = 0x119B;
pub const CL_KERNEL_ARG_ADDRESS_LOCAL: cl_kernel_arg_address_qualifier = 0x119C;
pub const CL_KERNEL_ARG_ADDRESS_CONSTANT: cl_kernel_arg_address_qualifier = 0x119D;
pub const CL_KERNEL_ARG_ADDRESS_PRIVATE: cl_kernel_arg_address_qualifier = 0x119E;

// #endif

// #ifdef CL_VERSION_1_2

// cl_kernel_arg_access_qualifier
pub const CL_KERNEL_ARG_ACCESS_READ_ONLY: cl_kernel_arg_access_qualifier = 0x11A0;
pub const CL_KERNEL_ARG_ACCESS_WRITE_ONLY: cl_kernel_arg_access_qualifier = 0x11A1;
pub const CL_KERNEL_ARG_ACCESS_READ_WRITE: cl_kernel_arg_access_qualifier = 0x11A2;
pub const CL_KERNEL_ARG_ACCESS_NONE: cl_kernel_arg_access_qualifier = 0x11A3;

// #endif

// #ifdef CL_VERSION_1_2

// cl_kernel_arg_type_qualifier
pub const CL_KERNEL_ARG_TYPE_NONE: cl_kernel_arg_type_qualifier = 0;
pub const CL_KERNEL_ARG_TYPE_CONST: cl_kernel_arg_type_qualifier = 1 << 0;
pub const CL_KERNEL_ARG_TYPE_RESTRICT: cl_kernel_arg_type_qualifier = 1 << 1;
pub const CL_KERNEL_ARG_TYPE_VOLATILE: cl_kernel_arg_type_qualifier = 1 << 2;
// #ifdef CL_VERSION_2_0
pub const CL_KERNEL_ARG_TYPE_PIPE: cl_kernel_arg_type_qualifier = 1 << 3;
// #endif

// #endif

// cl_kernel_work_group_info
pub const CL_KERNEL_WORK_GROUP_SIZE: cl_kernel_work_group_info = 0x11B0;
pub const CL_KERNEL_COMPILE_WORK_GROUP_SIZE: cl_kernel_work_group_info = 0x11B1;
pub const CL_KERNEL_LOCAL_MEM_SIZE: cl_kernel_work_group_info = 0x11B2;
pub const CL_KERNEL_PREFERRED_WORK_GROUP_SIZE_MULTIPLE: cl_kernel_work_group_info = 0x11B3;
pub const CL_KERNEL_PRIVATE_MEM_SIZE: cl_kernel_work_group_info = 0x11B4;
// #ifdef CL_VERSION_1_2
pub const CL_KERNEL_GLOBAL_WORK_SIZE: cl_kernel_work_group_info = 0x11B5;
// #endif

// #ifdef CL_VERSION_2_1

// cl_kernel_sub_group_info
pub const CL_KERNEL_MAX_SUB_GROUP_SIZE_FOR_NDRANGE: cl_kernel_sub_group_info = 0x2033;
pub const CL_KERNEL_SUB_GROUP_COUNT_FOR_NDRANGE: cl_kernel_sub_group_info = 0x2034;
pub const CL_KERNEL_LOCAL_SIZE_FOR_SUB_GROUP_COUNT: cl_kernel_sub_group_info = 0x11B8;
pub const CL_KERNEL_MAX_NUM_SUB_GROUPS: cl_kernel_sub_group_info = 0x11B9;
pub const CL_KERNEL_COMPILE_NUM_SUB_GROUPS: cl_kernel_sub_group_info = 0x11BA;

// #endif

// #ifdef CL_VERSION_2_0

// cl_kernel_exec_info
pub const CL_KERNEL_EXEC_INFO_SVM_PTRS: cl_kernel_exec_info = 0x11B6;
pub const CL_KERNEL_EXEC_INFO_SVM_FINE_GRAIN_SYSTEM: cl_kernel_exec_info = 0x11B7;

// #endif

// cl_event_info
pub const CL_EVENT_COMMAND_QUEUE: cl_event_info = 0x11D0;
pub const CL_EVENT_COMMAND_TYPE: cl_event_info = 0x11D1;
pub const CL_EVENT_REFERENCE_COUNT: cl_event_info = 0x11D2;
pub const CL_EVENT_COMMAND_EXECUTION_STATUS: cl_event_info = 0x11D3;
// #ifdef CL_VERSION_1_1
pub const CL_EVENT_CONTEXT: cl_event_info = 0x11D4;
// #endif

// cl_command_type
pub const CL_COMMAND_NDRANGE_KERNEL: cl_command_type = 0x11F0;
pub const CL_COMMAND_TASK: cl_command_type = 0x11F1;
pub const CL_COMMAND_NATIVE_KERNEL: cl_command_type = 0x11F2;
pub const CL_COMMAND_READ_BUFFER: cl_command_type = 0x11F3;
pub const CL_COMMAND_WRITE_BUFFER: cl_command_type = 0x11F4;
pub const CL_COMMAND_COPY_BUFFER: cl_command_type = 0x11F5;
pub const CL_COMMAND_READ_IMAGE: cl_command_type = 0x11F6;
pub const CL_COMMAND_WRITE_IMAGE: cl_command_type = 0x11F7;
pub const CL_COMMAND_COPY_IMAGE: cl_command_type = 0x11F8;
pub const CL_COMMAND_COPY_IMAGE_TO_BUFFER: cl_command_type = 0x11F9;
pub const CL_COMMAND_COPY_BUFFER_TO_IMAGE: cl_command_type = 0x11FA;
pub const CL_COMMAND_MAP_BUFFER: cl_command_type = 0x11FB;
pub const CL_COMMAND_MAP_IMAGE: cl_command_type = 0x11FC;
pub const CL_COMMAND_UNMAP_MEM_OBJECT: cl_command_type = 0x11FD;
pub const CL_COMMAND_MARKER: cl_command_type = 0x11FE;
pub const CL_COMMAND_ACQUIRE_GL_OBJECTS: cl_command_type = 0x11FF;
pub const CL_COMMAND_RELEASE_GL_OBJECTS: cl_command_type = 0x1200;
// #ifdef CL_VERSION_1_1
pub const CL_COMMAND_READ_BUFFER_RECT: cl_command_type = 0x1201;
pub const CL_COMMAND_WRITE_BUFFER_RECT: cl_command_type = 0x1202;
pub const CL_COMMAND_COPY_BUFFER_RECT: cl_command_type = 0x1203;
pub const CL_COMMAND_USER: cl_command_type = 0x1204;
// #endif
// #ifdef CL_VERSION_1_2
pub const CL_COMMAND_BARRIER: cl_command_type = 0x1205;
pub const CL_COMMAND_MIGRATE_MEM_OBJECTS: cl_command_type = 0x1206;
pub const CL_COMMAND_FILL_BUFFER: cl_command_type = 0x1207;
pub const CL_COMMAND_FILL_IMAGE: cl_command_type = 0x1208;
// #endif
// #ifdef CL_VERSION_2_0
pub const CL_COMMAND_SVM_FREE: cl_command_type = 0x1209;
pub const CL_COMMAND_SVM_MEMCPY: cl_command_type = 0x120A;
pub const CL_COMMAND_SVM_MEMFILL: cl_command_type = 0x120B;
pub const CL_COMMAND_SVM_MAP: cl_command_type = 0x120C;
pub const CL_COMMAND_SVM_UNMAP: cl_command_type = 0x120D;
// #endif
// #ifdef CL_VERSION_3_0
pub const CL_COMMAND_SVM_MIGRATE_MEM: cl_command_type = 0x120E;
// #endif

// command execution status
pub const CL_COMPLETE: cl_int = 0x0;
pub const CL_RUNNING: cl_int = 0x1;
pub const CL_SUBMITTED: cl_int = 0x2;
pub const CL_QUEUED: cl_int = 0x3;

// cl_buffer_create_type
// #ifdef CL_VERSION_1_1
pub const CL_BUFFER_CREATE_TYPE_REGION: cl_buffer_create_type = 0x1220;
// #endif

// cl_profiling_info
pub const CL_PROFILING_COMMAND_QUEUED: cl_profiling_info = 0x1280;
pub const CL_PROFILING_COMMAND_SUBMIT: cl_profiling_info = 0x1281;
pub const CL_PROFILING_COMMAND_START: cl_profiling_info = 0x1282;
pub const CL_PROFILING_COMMAND_END: cl_profiling_info = 0x1283;
// #ifdef CL_VERSION_2_0
pub const CL_PROFILING_COMMAND_COMPLETE: cl_profiling_info = 0x1284;
// #endif

// cl_device_atomic_capabilities - bitfield
// #ifdef CL_VERSION_3_0
pub const CL_DEVICE_ATOMIC_ORDER_RELAXED: cl_device_atomic_capabilities = 1 << 0;
pub const CL_DEVICE_ATOMIC_ORDER_ACQ_REL: cl_device_atomic_capabilities = 1 << 1;
pub const CL_DEVICE_ATOMIC_ORDER_SEQ_CST: cl_device_atomic_capabilities = 1 << 2;
pub const CL_DEVICE_ATOMIC_SCOPE_WORK_ITEM: cl_device_atomic_capabilities = 1 << 3;
pub const CL_DEVICE_ATOMIC_SCOPE_WORK_GROUP: cl_device_atomic_capabilities = 1 << 4;
pub const CL_DEVICE_ATOMIC_SCOPE_DEVICE: cl_device_atomic_capabilities = 1 << 5;
pub const CL_DEVICE_ATOMIC_SCOPE_ALL_DEVICES: cl_device_atomic_capabilities = 1 << 6;
// #endif

// cl_device_device_enqueue_capabilities - bitfield
// #ifdef CL_VERSION_3_0
pub const CL_DEVICE_QUEUE_SUPPORTED: cl_device_device_enqueue_capabilities = 1 << 0;
pub const CL_DEVICE_QUEUE_REPLACEABLE_DEFAULT: cl_device_device_enqueue_capabilities = 1 << 1;
// #endif

// cl_khronos_vendor_id
pub const CL_KHRONOS_VENDOR_ID_CODEPLAY: cl_khronos_vendor_id = 0x10004;

// #ifdef CL_VERSION_3_0

// cl_version
pub const CL_VERSION_MAJOR_BITS: cl_version = 10;
pub const CL_VERSION_MINOR_BITS: cl_version = 10;
pub const CL_VERSION_PATCH_BITS: cl_version = 12;

pub const CL_VERSION_MAJOR_MASK: cl_version = (1 << CL_VERSION_MAJOR_BITS) - 1;
pub const CL_VERSION_MINOR_MASK: cl_version = (1 << CL_VERSION_MINOR_BITS) - 1;
pub const CL_VERSION_PATCH_MASK: cl_version = (1 << CL_VERSION_PATCH_BITS) - 1;

#[must_use]
pub const fn version_major(version: cl_version) -> cl_version {
    version >> (CL_VERSION_MINOR_BITS + CL_VERSION_PATCH_BITS)
}

#[must_use]
pub const fn version_minor(version: cl_version) -> cl_version {
    (version >> CL_VERSION_PATCH_BITS) & CL_VERSION_MINOR_MASK
}

#[must_use]
pub const fn version_patch(version: cl_version) -> cl_version {
    version & CL_VERSION_PATCH_MASK
}

#[must_use]
pub const fn make_version(major: cl_version, minor: cl_version, patch: cl_version) -> cl_version {
    ((major & CL_VERSION_MAJOR_MASK) << (CL_VERSION_MINOR_BITS + CL_VERSION_PATCH_BITS))
        | ((minor & CL_VERSION_MINOR_MASK) << CL_VERSION_PATCH_BITS)
        | (patch & CL_VERSION_PATCH_MASK)
}

// #endif

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
#[cfg(feature = "static")]
unsafe extern "system" {
    // Platform API
    pub fn clGetPlatformIDs(
        num_entries: cl_uint,
        platforms: *mut cl_platform_id,
        num_platforms: *mut cl_uint,
    ) -> cl_int;

    pub fn clGetPlatformInfo(
        platform: cl_platform_id,
        param_name: cl_platform_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    // Device APIs
    pub fn clGetDeviceIDs(
        platform: cl_platform_id,
        device_type: cl_device_type,
        num_entries: cl_uint,
        devices: *mut cl_device_id,
        num_devices: *mut cl_uint,
    ) -> cl_int;

    pub fn clGetDeviceInfo(
        device: cl_device_id,
        param_name: cl_device_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn clCreateSubDevices(
        in_device: cl_device_id,
        properties: *const cl_device_partition_property,
        num_devices: cl_uint,
        out_devices: *mut cl_device_id,
        num_devices_ret: *mut cl_uint,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn clRetainDevice(device: cl_device_id) -> cl_int;

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn clReleaseDevice(device: cl_device_id) -> cl_int;

    #[cfg(feature = "CL_VERSION_2_1")]
    pub fn clSetDefaultDeviceCommandQueue(
        context: cl_context,
        device: cl_device_id,
        command_queue: cl_command_queue,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_2_1")]
    pub fn clGetDeviceAndHostTimer(
        device: cl_device_id,
        device_timestamp: *mut cl_ulong,
        host_timestamp: *mut cl_ulong,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_2_1")]
    pub fn clGetHostTimer(device: cl_device_id, host_timestamp: *mut cl_ulong) -> cl_int;

    // Context APIs
    pub fn clCreateContext(
        properties: *const cl_context_properties,
        num_devices: cl_uint,
        devices: *const cl_device_id,
        pfn_notify: Option<
            unsafe extern "C" fn(
                errinfo: *const c_char,
                private_info: *const c_void,
                cb: size_t,
                user_data: *mut c_void,
            ),
        >,
        user_data: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_context;

    pub fn clCreateContextFromType(
        properties: *const cl_context_properties,
        device_type: cl_device_type,
        pfn_notify: Option<
            unsafe extern "C" fn(
                errinfo: *const c_char,
                private_info: *const c_void,
                cb: size_t,
                user_data: *mut c_void,
            ),
        >,
        user_data: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_context;

    pub fn clRetainContext(context: cl_context) -> cl_int;

    pub fn clReleaseContext(context: cl_context) -> cl_int;

    pub fn clGetContextInfo(
        context: cl_context,
        param_name: cl_context_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_3_0")]
    pub fn clSetContextDestructorCallback(
        context: cl_context,
        pfn_notify: Option<unsafe extern "C" fn(context: cl_context, user_data: *mut c_void)>,
        user_data: *mut c_void,
    ) -> cl_int;

    // Command Queue APIs

    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn clCreateCommandQueueWithProperties(
        context: cl_context,
        device: cl_device_id,
        properties: *const cl_queue_properties,
        errcode_ret: *mut cl_int,
    ) -> cl_command_queue;

    pub fn clRetainCommandQueue(command_queue: cl_command_queue) -> cl_int;

    pub fn clReleaseCommandQueue(command_queue: cl_command_queue) -> cl_int;

    pub fn clGetCommandQueueInfo(
        command_queue: cl_command_queue,
        param_name: cl_command_queue_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    // Memory Object APIs
    pub fn clCreateBuffer(
        context: cl_context,
        flags: cl_mem_flags,
        size: size_t,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;

    pub fn clCreateSubBuffer(
        buffer: cl_mem,
        flags: cl_mem_flags,
        buffer_create_type: cl_buffer_create_type,
        buffer_create_info: *const c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn clCreateImage(
        context: cl_context,
        flags: cl_mem_flags,
        image_format: *const cl_image_format,
        image_desc: *const cl_image_desc,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;

    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn clCreatePipe(
        context: cl_context,
        flags: cl_mem_flags,
        pipe_packet_size: cl_uint,
        pipe_max_packets: cl_uint,
        properties: *const cl_pipe_properties,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;

    #[cfg(feature = "CL_VERSION_3_0")]
    pub fn clCreateBufferWithProperties(
        context: cl_context,
        properties: *const cl_mem_properties,
        flags: cl_mem_flags,
        size: size_t,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;

    #[cfg(feature = "CL_VERSION_3_0")]
    pub fn clCreateImageWithProperties(
        context: cl_context,
        properties: *const cl_mem_properties,
        flags: cl_mem_flags,
        image_format: *const cl_image_format,
        image_desc: *const cl_image_desc,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;

    pub fn clRetainMemObject(memobj: cl_mem) -> cl_int;

    pub fn clReleaseMemObject(memobj: cl_mem) -> cl_int;

    pub fn clGetSupportedImageFormats(
        context: cl_context,
        flags: cl_mem_flags,
        image_type: cl_mem_object_type,
        num_entries: cl_uint,
        image_formats: *mut cl_image_format,
        num_image_formats: *mut cl_uint,
    ) -> cl_int;

    pub fn clGetMemObjectInfo(
        memobj: cl_mem,
        param_name: cl_mem_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    pub fn clGetImageInfo(
        image: cl_mem,
        param_name: cl_image_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn clGetPipeInfo(
        pipe: cl_mem,
        param_name: cl_pipe_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn clSetMemObjectDestructorCallback(
        memobj: cl_mem,
        pfn_notify: Option<unsafe extern "C" fn(memobj: cl_mem, user_data: *mut c_void)>,
        user_data: *mut c_void,
    ) -> cl_int;

    // SVM Allocation APIs

    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn clSVMAlloc(
        context: cl_context,
        flags: cl_svm_mem_flags,
        size: size_t,
        alignment: cl_uint,
    ) -> *mut c_void;

    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn clSVMFree(context: cl_context, svm_pointer: *mut c_void);

    // Sampler APIs

    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn clCreateSamplerWithProperties(
        context: cl_context,
        normalized_coords: *const cl_sampler_properties,
        errcode_ret: *mut cl_int,
    ) -> cl_sampler;

    pub fn clRetainSampler(sampler: cl_sampler) -> cl_int;

    pub fn clReleaseSampler(sampler: cl_sampler) -> cl_int;

    pub fn clGetSamplerInfo(
        sampler: cl_sampler,
        param_name: cl_sampler_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    // Program Object APIs
    pub fn clCreateProgramWithSource(
        context: cl_context,
        count: cl_uint,
        strings: *const *const c_char,
        lengths: *const size_t,
        errcode_ret: *mut cl_int,
    ) -> cl_program;

    pub fn clCreateProgramWithBinary(
        context: cl_context,
        num_devices: cl_uint,
        device_list: *const cl_device_id,
        lengths: *const size_t,
        binaries: *const *const c_uchar,
        binary_status: *mut cl_int,
        errcode_ret: *mut cl_int,
    ) -> cl_program;

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn clCreateProgramWithBuiltInKernels(
        context: cl_context,
        num_devices: cl_uint,
        device_list: *const cl_device_id,
        kernel_names: *const c_char,
        errcode_ret: *mut cl_int,
    ) -> cl_program;

    #[cfg(feature = "CL_VERSION_2_1")]
    pub fn clCreateProgramWithIL(
        context: cl_context,
        il: *const c_void,
        length: size_t,
        errcode_ret: *mut cl_int,
    ) -> cl_program;

    pub fn clRetainProgram(program: cl_program) -> cl_int;

    pub fn clReleaseProgram(program: cl_program) -> cl_int;

    pub fn clBuildProgram(
        program: cl_program,
        num_devices: cl_uint,
        device_list: *const cl_device_id,
        options: *const c_char,
        pfn_notify: Option<unsafe extern "C" fn(program: cl_program, user_data: *mut c_void)>,
        user_data: *mut c_void,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn clCompileProgram(
        program: cl_program,
        num_devices: cl_uint,
        device_list: *const cl_device_id,
        options: *const c_char,
        num_input_headers: cl_uint,
        input_headers: *const cl_program,
        header_include_names: *const *const c_char,
        pfn_notify: Option<unsafe extern "C" fn(program: cl_program, user_data: *mut c_void)>,
        user_data: *mut c_void,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn clLinkProgram(
        context: cl_context,
        num_devices: cl_uint,
        device_list: *const cl_device_id,
        options: *const c_char,
        num_input_programs: cl_uint,
        input_programs: *const cl_program,
        pfn_notify: Option<unsafe extern "C" fn(program: cl_program, user_data: *mut c_void)>,
        user_data: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_program;

    #[cfg(feature = "CL_VERSION_2_2")]
    #[cfg_attr(
        feature = "CL_VERSION_3_0",
        deprecated(since = "0.1.0", note = "From CL_VERSION_3_0")
    )]
    pub fn clSetProgramReleaseCallback(
        program: cl_program,
        pfn_notify: Option<unsafe extern "C" fn(program: cl_program, user_data: *mut c_void)>,
        user_data: *mut c_void,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_2_2")]
    pub fn clSetProgramSpecializationConstant(
        program: cl_program,
        spec_id: cl_uint,
        spec_size: size_t,
        spec_value: *const c_void,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn clUnloadPlatformCompiler(platform: cl_platform_id) -> cl_int;

    pub fn clGetProgramInfo(
        program: cl_program,
        param_name: cl_program_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    pub fn clGetProgramBuildInfo(
        program: cl_program,
        device: cl_device_id,
        param_name: cl_program_build_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    // Kernel Object APIs
    pub fn clCreateKernel(
        program: cl_program,
        kernel_name: *const c_char,
        errcode_ret: *mut cl_int,
    ) -> cl_kernel;

    pub fn clCreateKernelsInProgram(
        program: cl_program,
        num_kernels: cl_uint,
        kernels: *mut cl_kernel,
        num_kernels_ret: *mut cl_uint,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_2_1")]
    pub fn clCloneKernel(source_kernel: cl_kernel, errcode_ret: *mut cl_int) -> cl_kernel;

    pub fn clRetainKernel(kernel: cl_kernel) -> cl_int;

    pub fn clReleaseKernel(kernel: cl_kernel) -> cl_int;

    pub fn clSetKernelArg(
        kernel: cl_kernel,
        arg_index: cl_uint,
        arg_size: size_t,
        arg_value: *const c_void,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn clSetKernelArgSVMPointer(
        kernel: cl_kernel,
        arg_index: cl_uint,
        arg_value: *const c_void,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn clSetKernelExecInfo(
        kernel: cl_kernel,
        param_name: cl_kernel_exec_info,
        param_value_size: size_t,
        param_value: *const c_void,
    ) -> cl_int;

    pub fn clGetKernelInfo(
        kernel: cl_kernel,
        param_name: cl_kernel_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn clGetKernelArgInfo(
        kernel: cl_kernel,
        arg_indx: cl_uint,
        param_name: cl_kernel_arg_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    pub fn clGetKernelWorkGroupInfo(
        kernel: cl_kernel,
        device: cl_device_id,
        param_name: cl_kernel_work_group_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_2_1")]
    pub fn clGetKernelSubGroupInfo(
        kernel: cl_kernel,
        device: cl_device_id,
        param_name: cl_kernel_sub_group_info,
        input_value_size: size_t,
        input_value: *const c_void,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    // Event Object APIs
    pub fn clWaitForEvents(num_events: cl_uint, event_list: *const cl_event) -> cl_int;

    pub fn clGetEventInfo(
        event: cl_event,
        param_name: cl_event_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn clCreateUserEvent(context: cl_context, errcode_ret: *mut cl_int) -> cl_event;

    pub fn clRetainEvent(event: cl_event) -> cl_int;

    pub fn clReleaseEvent(event: cl_event) -> cl_int;

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn clSetUserEventStatus(event: cl_event, execution_status: cl_int) -> cl_int;

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn clSetEventCallback(
        event: cl_event,
        command_exec_callback_type: cl_int,
        pfn_notify: Option<
            unsafe extern "C" fn(
                event: cl_event,
                event_command_status: cl_int,
                user_data: *mut c_void,
            ),
        >,
        user_data: *mut c_void,
    ) -> cl_int;

    // Profiling APIs
    pub fn clGetEventProfilingInfo(
        event: cl_event,
        param_name: cl_profiling_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    // Flush and Finish APIs
    pub fn clFlush(command_queue: cl_command_queue) -> cl_int;

    pub fn clFinish(command_queue: cl_command_queue) -> cl_int;

    // Enqueued Commands APIs
    pub fn clEnqueueReadBuffer(
        command_queue: cl_command_queue,
        buffer: cl_mem,
        blocking_read: cl_bool,
        offset: size_t,
        cb: size_t,
        ptr: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn clEnqueueReadBufferRect(
        command_queue: cl_command_queue,
        buffer: cl_mem,
        blocking_read: cl_bool,
        buffer_origin: *const size_t,
        host_origin: *const size_t,
        region: *const size_t,
        buffer_row_pitch: size_t,
        buffer_slc_pitch: size_t,
        host_row_pitch: size_t,
        host_slc_pitch: size_t,
        ptr: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueWriteBuffer(
        command_queue: cl_command_queue,
        buffer: cl_mem,
        blocking_write: cl_bool,
        offset: size_t,
        cb: size_t,
        ptr: *const c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn clEnqueueWriteBufferRect(
        command_queue: cl_command_queue,
        buffer: cl_mem,
        blocking_write: cl_bool,
        buffer_origin: *const size_t,
        host_origin: *const size_t,
        region: *const size_t,
        buffer_row_pitch: size_t,
        buffer_slc_pitch: size_t,
        host_row_pitch: size_t,
        host_slc_pitch: size_t,
        ptr: *const c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn clEnqueueFillBuffer(
        command_queue: cl_command_queue,
        buffer: cl_mem,
        pattern: *const c_void,
        pattern_size: size_t,
        offset: size_t,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueCopyBuffer(
        command_queue: cl_command_queue,
        src_buffer: cl_mem,
        dst_buffer: cl_mem,
        src_offset: size_t,
        dst_offset: size_t,
        cb: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_1_1")]
    pub fn clEnqueueCopyBufferRect(
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
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueReadImage(
        command_queue: cl_command_queue,
        image: cl_mem,
        blocking_read: cl_bool,
        origin: *const size_t,
        region: *const size_t,
        row_pitch: size_t,
        slice_pitch: size_t,
        ptr: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueWriteImage(
        command_queue: cl_command_queue,
        image: cl_mem,
        blocking_write: cl_bool,
        origin: *const size_t,
        region: *const size_t,
        input_row_pitch: size_t,
        input_slice_pitch: size_t,
        ptr: *const c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn clEnqueueFillImage(
        command_queue: cl_command_queue,
        image: cl_mem,
        fill_color: *const c_void,
        origin: *const size_t,
        region: *const size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueCopyImage(
        command_queue: cl_command_queue,
        src_image: cl_mem,
        dst_image: cl_mem,
        src_origin: *const size_t,
        dst_origin: *const size_t,
        region: *const size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueCopyImageToBuffer(
        command_queue: cl_command_queue,
        src_image: cl_mem,
        dst_buffer: cl_mem,
        src_origin: *const size_t,
        region: *const size_t,
        dst_offset: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueCopyBufferToImage(
        command_queue: cl_command_queue,
        src_buffer: cl_mem,
        dst_image: cl_mem,
        src_offset: size_t,
        dst_origin: *const size_t,
        region: *const size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueMapBuffer(
        command_queue: cl_command_queue,
        buffer: cl_mem,
        blocking_map: cl_bool,
        map_flags: cl_map_flags,
        offset: size_t,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
        errcode_ret: *mut cl_int,
    ) -> *mut c_void;

    pub fn clEnqueueMapImage(
        command_queue: cl_command_queue,
        image: cl_mem,
        blocking_map: cl_bool,
        map_flags: cl_map_flags,
        origin: *const size_t,
        region: *const size_t,
        image_row_pitch: *mut size_t,
        image_slice_pitch: *mut size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
        errcode_ret: *mut cl_int,
    ) -> *mut c_void;

    pub fn clEnqueueUnmapMemObject(
        command_queue: cl_command_queue,
        memobj: cl_mem,
        mapped_ptr: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn clEnqueueMigrateMemObjects(
        command_queue: cl_command_queue,
        num_mem_objects: cl_uint,
        mem_objects: *const cl_mem,
        flags: cl_mem_migration_flags,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueNDRangeKernel(
        command_queue: cl_command_queue,
        kernel: cl_kernel,
        work_dim: cl_uint,
        global_work_offset: *const size_t,
        global_work_dims: *const size_t,
        local_work_dims: *const size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueNativeKernel(
        command_queue: cl_command_queue,
        user_func: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
        args: *mut c_void,
        cb_args: size_t,
        num_mem_objects: cl_uint,
        mem_list: *const cl_mem,
        args_mem_loc: *const *const c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn clEnqueueMarkerWithWaitList(
        command_queue: cl_command_queue,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn clEnqueueBarrierWithWaitList(
        command_queue: cl_command_queue,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn clEnqueueSVMFree(
        command_queue: cl_command_queue,
        num_svm_pointers: cl_uint,
        svm_pointers: *const *const c_void,
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

    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn clEnqueueSVMMemcpy(
        command_queue: cl_command_queue,
        blocking_copy: cl_bool,
        dst_ptr: *mut c_void,
        src_ptr: *const c_void,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn clEnqueueSVMMemFill(
        command_queue: cl_command_queue,
        svm_ptr: *mut c_void,
        pattern: *const c_void,
        pattern_size: size_t,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn clEnqueueSVMMap(
        command_queue: cl_command_queue,
        blocking_map: cl_bool,
        flags: cl_map_flags,
        svm_ptr: *mut c_void,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_2_0")]
    pub fn clEnqueueSVMUnmap(
        command_queue: cl_command_queue,
        svm_ptr: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_2_1")]
    pub fn clEnqueueSVMMigrateMem(
        command_queue: cl_command_queue,
        num_svm_pointers: cl_uint,
        svm_pointers: *const *const c_void,
        sizes: *const size_t,
        flags: cl_mem_migration_flags,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn clGetExtensionFunctionAddressForPlatform(
        platform: cl_platform_id,
        func_name: *const c_char,
    ) -> *mut c_void;

    // Deprecated OpenCL 1.1 APIs

    #[cfg_attr(
        any(
            feature = "CL_VERSION_1_2",
            feature = "CL_VERSION_2_0",
            feature = "CL_VERSION_2_1",
            feature = "CL_VERSION_2_2",
            feature = "CL_VERSION_3_0"
        ),
        deprecated(since = "0.1.0", note = "From CL_VERSION_1_2 use clCreateImage")
    )]
    pub fn clCreateImage2D(
        context: cl_context,
        flags: cl_mem_flags,
        image_format: *mut cl_image_format,
        image_width: size_t,
        image_depth: size_t,
        image_row_pitch: size_t,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;

    #[cfg_attr(
        any(
            feature = "CL_VERSION_1_2",
            feature = "CL_VERSION_2_0",
            feature = "CL_VERSION_2_1",
            feature = "CL_VERSION_2_2",
            feature = "CL_VERSION_3_0"
        ),
        deprecated(since = "0.1.0", note = "From CL_VERSION_1_2 use clCreateImage")
    )]
    pub fn clCreateImage3D(
        context: cl_context,
        flags: cl_mem_flags,
        image_format: *mut cl_image_format,
        image_width: size_t,
        image_height: size_t,
        image_depth: size_t,
        image_row_pitch: size_t,
        image_slice_pitch: size_t,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;

    #[cfg_attr(
        any(
            feature = "CL_VERSION_1_2",
            feature = "CL_VERSION_2_0",
            feature = "CL_VERSION_2_1",
            feature = "CL_VERSION_2_2",
            feature = "CL_VERSION_3_0"
        ),
        deprecated(
            since = "0.1.0",
            note = "From CL_VERSION_1_2 use clEnqueueMarkerWithWaitList"
        )
    )]
    pub fn clEnqueueMarker(command_queue: cl_command_queue, event: *mut cl_event) -> cl_int;

    #[deprecated(since = "0.1.0", note = "From CL_VERSION_1_2")]
    pub fn clEnqueueWaitForEvents(
        command_queue: cl_command_queue,
        num_events: cl_uint,
        event_list: *mut cl_event,
    ) -> cl_int;

    #[cfg_attr(
        any(
            feature = "CL_VERSION_1_2",
            feature = "CL_VERSION_2_0",
            feature = "CL_VERSION_2_1",
            feature = "CL_VERSION_2_2",
            feature = "CL_VERSION_3_0"
        ),
        deprecated(
            since = "0.1.0",
            note = "From CL_VERSION_1_2 use clEnqueueBarrierWithWaitList"
        )
    )]
    pub fn clEnqueueBarrier(command_queue: cl_command_queue) -> cl_int;

    #[cfg_attr(
        any(
            feature = "CL_VERSION_1_2",
            feature = "CL_VERSION_2_0",
            feature = "CL_VERSION_2_1",
            feature = "CL_VERSION_2_2",
            feature = "CL_VERSION_3_0"
        ),
        deprecated(
            since = "0.1.0",
            note = "From CL_VERSION_1_2 use clUnloadPlatformCompiler"
        )
    )]
    pub fn clUnloadCompiler() -> cl_int;

    #[cfg_attr(
        any(
            feature = "CL_VERSION_1_2",
            feature = "CL_VERSION_2_0",
            feature = "CL_VERSION_2_1",
            feature = "CL_VERSION_2_2",
            feature = "CL_VERSION_3_0"
        ),
        deprecated(
            since = "0.1.0",
            note = "From CL_VERSION_1_2 use clGetExtensionFunctionAddressForPlatform"
        )
    )]
    pub fn clGetExtensionFunctionAddress(func_name: *const c_char);

    // Deprecated OpenCL 2.0 APIs

    #[cfg_attr(
        any(
            feature = "CL_VERSION_2_0",
            feature = "CL_VERSION_2_1",
            feature = "CL_VERSION_2_2",
            feature = "CL_VERSION_3_0"
        ),
        deprecated(
            since = "0.1.0",
            note = "From CL_VERSION_2_0 use clCreateCommandQueueWithProperties"
        )
    )]
    pub fn clCreateCommandQueue(
        context: cl_context,
        device: cl_device_id,
        properties: cl_command_queue_properties,
        errcode_ret: *mut cl_int,
    ) -> cl_command_queue;

    #[cfg_attr(
        any(
            feature = "CL_VERSION_2_0",
            feature = "CL_VERSION_2_1",
            feature = "CL_VERSION_2_2",
            feature = "CL_VERSION_3_0"
        ),
        deprecated(
            since = "0.1.0",
            note = "From CL_VERSION_2_0 use clCreateSamplerWithProperties"
        )
    )]
    pub fn clCreateSampler(
        context: cl_context,
        normalize_coords: cl_bool,
        addressing_mode: cl_addressing_mode,
        filter_mode: cl_filter_mode,
        errcode_ret: *mut cl_int,
    ) -> cl_sampler;

    // Deprecated 1.2
    #[cfg_attr(
        any(
            feature = "CL_VERSION_2_0",
            feature = "CL_VERSION_2_1",
            feature = "CL_VERSION_2_2",
            feature = "CL_VERSION_3_0"
        ),
        deprecated(
            since = "0.1.0",
            note = "From CL_VERSION_2_0 use clEnqueueNDRangeKernel"
        )
    )]
    pub fn clEnqueueTask(
        command_queue: cl_command_queue,
        kernel: cl_kernel,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(deref_nullptr)]
    fn bindgen_test_layout_cl_image_format() {
        assert_eq!(
            ::core::mem::size_of::<cl_image_format>(),
            8usize,
            concat!("Size of: ", stringify!(cl_image_format))
        );
        assert_eq!(
            ::core::mem::align_of::<cl_image_format>(),
            4usize,
            concat!("Alignment of ", stringify!(cl_image_format))
        );
        assert_eq!(
            ::core::mem::offset_of!(cl_image_format, image_channel_order),
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_image_format),
                "::",
                stringify!(image_channel_order)
            )
        );
        assert_eq!(
            ::core::mem::offset_of!(cl_image_format, image_channel_data_type),
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_image_format),
                "::",
                stringify!(image_channel_data_type)
            )
        );
    }

    #[test]
    #[allow(deref_nullptr)]
    fn bindgen_test_layout_cl_image_desc() {
        assert_eq!(
            ::core::mem::size_of::<cl_image_desc>(),
            72usize,
            concat!("Size of: ", stringify!(cl_image_desc))
        );
        assert_eq!(
            ::core::mem::align_of::<cl_image_desc>(),
            8usize,
            concat!("Alignment of ", stringify!(cl_image_desc))
        );
        assert_eq!(
            ::core::mem::offset_of!(cl_image_desc, image_type),
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_image_desc),
                "::",
                stringify!(image_type)
            )
        );
        assert_eq!(
            ::core::mem::offset_of!(cl_image_desc, image_width),
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_image_desc),
                "::",
                stringify!(image_width)
            )
        );
        assert_eq!(
            ::core::mem::offset_of!(cl_image_desc, image_height),
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_image_desc),
                "::",
                stringify!(image_height)
            )
        );
        assert_eq!(
            ::core::mem::offset_of!(cl_image_desc, image_depth),
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_image_desc),
                "::",
                stringify!(image_depth)
            )
        );
        assert_eq!(
            ::core::mem::offset_of!(cl_image_desc, image_array_size),
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_image_desc),
                "::",
                stringify!(image_array_size)
            )
        );
        assert_eq!(
            ::core::mem::offset_of!(cl_image_desc, image_row_pitch),
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_image_desc),
                "::",
                stringify!(image_row_pitch)
            )
        );
        assert_eq!(
            ::core::mem::offset_of!(cl_image_desc, image_slice_pitch),
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_image_desc),
                "::",
                stringify!(image_slice_pitch)
            )
        );
        assert_eq!(
            ::core::mem::offset_of!(cl_image_desc, num_mip_levels),
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_image_desc),
                "::",
                stringify!(num_mip_levels)
            )
        );
        assert_eq!(
            ::core::mem::offset_of!(cl_image_desc, num_samples),
            60usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_image_desc),
                "::",
                stringify!(num_samples)
            )
        );
    }

    #[test]
    #[allow(deref_nullptr)]
    fn bindgen_test_layout_cl_buffer_region() {
        assert_eq!(
            ::core::mem::size_of::<cl_buffer_region>(),
            16usize,
            concat!("Size of: ", stringify!(cl_buffer_region))
        );
        assert_eq!(
            ::core::mem::align_of::<cl_buffer_region>(),
            8usize,
            concat!("Alignment of ", stringify!(cl_buffer_region))
        );
        assert_eq!(
            ::core::mem::offset_of!(cl_buffer_region, origin),
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_buffer_region),
                "::",
                stringify!(origin)
            )
        );
        assert_eq!(
            ::core::mem::offset_of!(cl_buffer_region, size),
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_buffer_region),
                "::",
                stringify!(size)
            )
        );
    }

    #[test]
    #[allow(deref_nullptr)]
    fn bindgen_test_layout_cl_name_version() {
        assert_eq!(
            ::core::mem::size_of::<cl_name_version>(),
            68usize,
            concat!("Size of: ", stringify!(cl_name_version))
        );
        assert_eq!(
            ::core::mem::align_of::<cl_name_version>(),
            4usize,
            concat!("Alignment of ", stringify!(cl_name_version))
        );
        assert_eq!(
            ::core::mem::offset_of!(cl_name_version, version),
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_name_version),
                "::",
                stringify!(version)
            )
        );
        assert_eq!(
            ::core::mem::offset_of!(cl_name_version, name),
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(cl_name_version),
                "::",
                stringify!(name)
            )
        );
    }
}
