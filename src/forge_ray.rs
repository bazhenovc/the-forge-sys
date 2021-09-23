/* automatically generated by rust-bindgen 0.59.1 */

use crate::forge_renderer::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AccelerationStructure {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RaytracingPipeline {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RaytracingShaderTable {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ParallelPrimitives {
    _unused: [u8; 0],
}
pub const AccelerationStructureBuildFlags_ACCELERATION_STRUCTURE_BUILD_FLAG_NONE: AccelerationStructureBuildFlags = 0;
pub const AccelerationStructureBuildFlags_ACCELERATION_STRUCTURE_BUILD_FLAG_ALLOW_UPDATE:
    AccelerationStructureBuildFlags = 1;
pub const AccelerationStructureBuildFlags_ACCELERATION_STRUCTURE_BUILD_FLAG_ALLOW_COMPACTION:
    AccelerationStructureBuildFlags = 2;
pub const AccelerationStructureBuildFlags_ACCELERATION_STRUCTURE_BUILD_FLAG_PREFER_FAST_TRACE:
    AccelerationStructureBuildFlags = 4;
pub const AccelerationStructureBuildFlags_ACCELERATION_STRUCTURE_BUILD_FLAG_PREFER_FAST_BUILD:
    AccelerationStructureBuildFlags = 8;
pub const AccelerationStructureBuildFlags_ACCELERATION_STRUCTURE_BUILD_FLAG_MINIMIZE_MEMORY:
    AccelerationStructureBuildFlags = 16;
pub const AccelerationStructureBuildFlags_ACCELERATION_STRUCTURE_BUILD_FLAG_PERFORM_UPDATE:
    AccelerationStructureBuildFlags = 32;
pub type AccelerationStructureBuildFlags = ::std::os::raw::c_int;
pub const AccelerationStructureGeometryFlags_ACCELERATION_STRUCTURE_GEOMETRY_FLAG_NONE:
    AccelerationStructureGeometryFlags = 0;
pub const AccelerationStructureGeometryFlags_ACCELERATION_STRUCTURE_GEOMETRY_FLAG_OPAQUE:
    AccelerationStructureGeometryFlags = 1;
pub const AccelerationStructureGeometryFlags_ACCELERATION_STRUCTURE_GEOMETRY_FLAG_NO_DUPLICATE_ANYHIT_INVOCATION:
    AccelerationStructureGeometryFlags = 2;
pub type AccelerationStructureGeometryFlags = ::std::os::raw::c_int;
pub const AccelerationStructureInstanceFlags_ACCELERATION_STRUCTURE_INSTANCE_FLAG_NONE:
    AccelerationStructureInstanceFlags = 0;
pub const AccelerationStructureInstanceFlags_ACCELERATION_STRUCTURE_INSTANCE_FLAG_TRIANGLE_CULL_DISABLE:
    AccelerationStructureInstanceFlags = 1;
pub const AccelerationStructureInstanceFlags_ACCELERATION_STRUCTURE_INSTANCE_FLAG_TRIANGLE_FRONT_COUNTERCLOCKWISE:
    AccelerationStructureInstanceFlags = 2;
pub const AccelerationStructureInstanceFlags_ACCELERATION_STRUCTURE_INSTANCE_FLAG_FORCE_OPAQUE:
    AccelerationStructureInstanceFlags = 4;
pub const AccelerationStructureInstanceFlags_ACCELERATION_STRUCTURE_INSTANCE_FLAG_FORCE_NON_OPAQUE:
    AccelerationStructureInstanceFlags = 8;
pub type AccelerationStructureInstanceFlags = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AccelerationStructureInstanceDesc {
    pub mAccelerationStructureIndex: u32,
    #[doc = " Row major affine transform for transforming the vertices in the geometry stored in pAccelerationStructure"]
    pub mTransform: [f32; 12usize],
    #[doc = " User defined instanced ID which can be queried in the shader"]
    pub mInstanceID: u32,
    pub mInstanceMask: u32,
    pub mInstanceContributionToHitGroupIndex: u32,
    pub mFlags: AccelerationStructureInstanceFlags,
}
#[test]
fn bindgen_test_layout_AccelerationStructureInstanceDesc() {
    assert_eq!(
        ::std::mem::size_of::<AccelerationStructureInstanceDesc>(),
        68usize,
        concat!("Size of: ", stringify!(AccelerationStructureInstanceDesc))
    );
    assert_eq!(
        ::std::mem::align_of::<AccelerationStructureInstanceDesc>(),
        4usize,
        concat!("Alignment of ", stringify!(AccelerationStructureInstanceDesc))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AccelerationStructureInstanceDesc>())).mAccelerationStructureIndex as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AccelerationStructureInstanceDesc),
            "::",
            stringify!(mAccelerationStructureIndex)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AccelerationStructureInstanceDesc>())).mTransform as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AccelerationStructureInstanceDesc),
            "::",
            stringify!(mTransform)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AccelerationStructureInstanceDesc>())).mInstanceID as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(AccelerationStructureInstanceDesc),
            "::",
            stringify!(mInstanceID)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AccelerationStructureInstanceDesc>())).mInstanceMask as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(AccelerationStructureInstanceDesc),
            "::",
            stringify!(mInstanceMask)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AccelerationStructureInstanceDesc>())).mInstanceContributionToHitGroupIndex
                as *const _ as usize
        },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(AccelerationStructureInstanceDesc),
            "::",
            stringify!(mInstanceContributionToHitGroupIndex)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AccelerationStructureInstanceDesc>())).mFlags as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(AccelerationStructureInstanceDesc),
            "::",
            stringify!(mFlags)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureGeometryDesc {
    pub pVertexArray: *mut ::std::os::raw::c_void,
    pub __bindgen_anon_1: AccelerationStructureGeometryDesc__bindgen_ty_1,
    pub mFlags: AccelerationStructureGeometryFlags,
    pub mVertexCount: u32,
    pub mIndexCount: u32,
    pub mIndexType: IndexType,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union AccelerationStructureGeometryDesc__bindgen_ty_1 {
    pub pIndices32: *mut u32,
    pub pIndices16: *mut u16,
}
#[test]
fn bindgen_test_layout_AccelerationStructureGeometryDesc__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<AccelerationStructureGeometryDesc__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(AccelerationStructureGeometryDesc__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<AccelerationStructureGeometryDesc__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(AccelerationStructureGeometryDesc__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AccelerationStructureGeometryDesc__bindgen_ty_1>())).pIndices32 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AccelerationStructureGeometryDesc__bindgen_ty_1),
            "::",
            stringify!(pIndices32)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AccelerationStructureGeometryDesc__bindgen_ty_1>())).pIndices16 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AccelerationStructureGeometryDesc__bindgen_ty_1),
            "::",
            stringify!(pIndices16)
        )
    );
}
#[test]
fn bindgen_test_layout_AccelerationStructureGeometryDesc() {
    assert_eq!(
        ::std::mem::size_of::<AccelerationStructureGeometryDesc>(),
        32usize,
        concat!("Size of: ", stringify!(AccelerationStructureGeometryDesc))
    );
    assert_eq!(
        ::std::mem::align_of::<AccelerationStructureGeometryDesc>(),
        8usize,
        concat!("Alignment of ", stringify!(AccelerationStructureGeometryDesc))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AccelerationStructureGeometryDesc>())).pVertexArray as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AccelerationStructureGeometryDesc),
            "::",
            stringify!(pVertexArray)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AccelerationStructureGeometryDesc>())).mFlags as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AccelerationStructureGeometryDesc),
            "::",
            stringify!(mFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AccelerationStructureGeometryDesc>())).mVertexCount as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(AccelerationStructureGeometryDesc),
            "::",
            stringify!(mVertexCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AccelerationStructureGeometryDesc>())).mIndexCount as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AccelerationStructureGeometryDesc),
            "::",
            stringify!(mIndexCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AccelerationStructureGeometryDesc>())).mIndexType as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(AccelerationStructureGeometryDesc),
            "::",
            stringify!(mIndexType)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AccelerationStructureDescBottom {
    pub mFlags: AccelerationStructureBuildFlags,
    #[doc = " Number of geometries / instances in thie acceleration structure"]
    pub mDescCount: u32,
    #[doc = " Array of geometries in the bottom level acceleration structure"]
    pub pGeometryDescs: *mut AccelerationStructureGeometryDesc,
}
#[test]
fn bindgen_test_layout_AccelerationStructureDescBottom() {
    assert_eq!(
        ::std::mem::size_of::<AccelerationStructureDescBottom>(),
        16usize,
        concat!("Size of: ", stringify!(AccelerationStructureDescBottom))
    );
    assert_eq!(
        ::std::mem::align_of::<AccelerationStructureDescBottom>(),
        8usize,
        concat!("Alignment of ", stringify!(AccelerationStructureDescBottom))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AccelerationStructureDescBottom>())).mFlags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AccelerationStructureDescBottom),
            "::",
            stringify!(mFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AccelerationStructureDescBottom>())).mDescCount as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AccelerationStructureDescBottom),
            "::",
            stringify!(mDescCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AccelerationStructureDescBottom>())).pGeometryDescs as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AccelerationStructureDescBottom),
            "::",
            stringify!(pGeometryDescs)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AccelerationStructureDescTop {
    pub mFlags: AccelerationStructureBuildFlags,
    pub mInstancesDescCount: u32,
    pub pInstanceDescs: *mut AccelerationStructureInstanceDesc,
    pub mBottomASDesc: *mut AccelerationStructureDescBottom,
    pub mIndexType: IndexType,
}
#[test]
fn bindgen_test_layout_AccelerationStructureDescTop() {
    assert_eq!(
        ::std::mem::size_of::<AccelerationStructureDescTop>(),
        32usize,
        concat!("Size of: ", stringify!(AccelerationStructureDescTop))
    );
    assert_eq!(
        ::std::mem::align_of::<AccelerationStructureDescTop>(),
        8usize,
        concat!("Alignment of ", stringify!(AccelerationStructureDescTop))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AccelerationStructureDescTop>())).mFlags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AccelerationStructureDescTop),
            "::",
            stringify!(mFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AccelerationStructureDescTop>())).mInstancesDescCount as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AccelerationStructureDescTop),
            "::",
            stringify!(mInstancesDescCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AccelerationStructureDescTop>())).pInstanceDescs as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AccelerationStructureDescTop),
            "::",
            stringify!(pInstanceDescs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AccelerationStructureDescTop>())).mBottomASDesc as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AccelerationStructureDescTop),
            "::",
            stringify!(mBottomASDesc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AccelerationStructureDescTop>())).mIndexType as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AccelerationStructureDescTop),
            "::",
            stringify!(mIndexType)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RaytracingHitGroup {
    pub pRootSignature: *mut RootSignature,
    pub pIntersectionShader: *mut Shader,
    pub pAnyHitShader: *mut Shader,
    pub pClosestHitShader: *mut Shader,
    pub pHitGroupName: *const ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_RaytracingHitGroup() {
    assert_eq!(
        ::std::mem::size_of::<RaytracingHitGroup>(),
        40usize,
        concat!("Size of: ", stringify!(RaytracingHitGroup))
    );
    assert_eq!(
        ::std::mem::align_of::<RaytracingHitGroup>(),
        8usize,
        concat!("Alignment of ", stringify!(RaytracingHitGroup))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RaytracingHitGroup>())).pRootSignature as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RaytracingHitGroup),
            "::",
            stringify!(pRootSignature)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RaytracingHitGroup>())).pIntersectionShader as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RaytracingHitGroup),
            "::",
            stringify!(pIntersectionShader)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RaytracingHitGroup>())).pAnyHitShader as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RaytracingHitGroup),
            "::",
            stringify!(pAnyHitShader)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RaytracingHitGroup>())).pClosestHitShader as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(RaytracingHitGroup),
            "::",
            stringify!(pClosestHitShader)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RaytracingHitGroup>())).pHitGroupName as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(RaytracingHitGroup),
            "::",
            stringify!(pHitGroupName)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RaytracingShaderTableDesc {
    pub pPipeline: *mut Pipeline,
    pub pGlobalRootSignature: *mut RootSignature,
    pub pRayGenShader: *const ::std::os::raw::c_char,
    pub pMissShaders: *mut *const ::std::os::raw::c_char,
    pub pHitGroups: *mut *const ::std::os::raw::c_char,
    pub mMissShaderCount: u32,
    pub mHitGroupCount: u32,
}
#[test]
fn bindgen_test_layout_RaytracingShaderTableDesc() {
    assert_eq!(
        ::std::mem::size_of::<RaytracingShaderTableDesc>(),
        48usize,
        concat!("Size of: ", stringify!(RaytracingShaderTableDesc))
    );
    assert_eq!(
        ::std::mem::align_of::<RaytracingShaderTableDesc>(),
        8usize,
        concat!("Alignment of ", stringify!(RaytracingShaderTableDesc))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RaytracingShaderTableDesc>())).pPipeline as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RaytracingShaderTableDesc),
            "::",
            stringify!(pPipeline)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RaytracingShaderTableDesc>())).pGlobalRootSignature as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RaytracingShaderTableDesc),
            "::",
            stringify!(pGlobalRootSignature)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RaytracingShaderTableDesc>())).pRayGenShader as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RaytracingShaderTableDesc),
            "::",
            stringify!(pRayGenShader)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RaytracingShaderTableDesc>())).pMissShaders as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(RaytracingShaderTableDesc),
            "::",
            stringify!(pMissShaders)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RaytracingShaderTableDesc>())).pHitGroups as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(RaytracingShaderTableDesc),
            "::",
            stringify!(pHitGroups)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RaytracingShaderTableDesc>())).mMissShaderCount as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(RaytracingShaderTableDesc),
            "::",
            stringify!(mMissShaderCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RaytracingShaderTableDesc>())).mHitGroupCount as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(RaytracingShaderTableDesc),
            "::",
            stringify!(mHitGroupCount)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RaytracingDispatchDesc {
    pub mWidth: u32,
    pub mHeight: u32,
    pub pShaderTable: *mut RaytracingShaderTable,
}
#[test]
fn bindgen_test_layout_RaytracingDispatchDesc() {
    assert_eq!(
        ::std::mem::size_of::<RaytracingDispatchDesc>(),
        16usize,
        concat!("Size of: ", stringify!(RaytracingDispatchDesc))
    );
    assert_eq!(
        ::std::mem::align_of::<RaytracingDispatchDesc>(),
        8usize,
        concat!("Alignment of ", stringify!(RaytracingDispatchDesc))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RaytracingDispatchDesc>())).mWidth as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RaytracingDispatchDesc),
            "::",
            stringify!(mWidth)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RaytracingDispatchDesc>())).mHeight as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(RaytracingDispatchDesc),
            "::",
            stringify!(mHeight)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RaytracingDispatchDesc>())).pShaderTable as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RaytracingDispatchDesc),
            "::",
            stringify!(pShaderTable)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RaytracingBuildASDesc {
    pub ppAccelerationStructures: *mut *mut AccelerationStructure,
    pub mCount: u32,
    pub mBottomASIndicesCount: u32,
    pub pBottomASIndices: *mut u32,
}
#[test]
fn bindgen_test_layout_RaytracingBuildASDesc() {
    assert_eq!(
        ::std::mem::size_of::<RaytracingBuildASDesc>(),
        24usize,
        concat!("Size of: ", stringify!(RaytracingBuildASDesc))
    );
    assert_eq!(
        ::std::mem::align_of::<RaytracingBuildASDesc>(),
        8usize,
        concat!("Alignment of ", stringify!(RaytracingBuildASDesc))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RaytracingBuildASDesc>())).ppAccelerationStructures as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RaytracingBuildASDesc),
            "::",
            stringify!(ppAccelerationStructures)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RaytracingBuildASDesc>())).mCount as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RaytracingBuildASDesc),
            "::",
            stringify!(mCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RaytracingBuildASDesc>())).mBottomASIndicesCount as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(RaytracingBuildASDesc),
            "::",
            stringify!(mBottomASIndicesCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RaytracingBuildASDesc>())).pBottomASIndices as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RaytracingBuildASDesc),
            "::",
            stringify!(pBottomASIndices)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Raytracing {
    _unused: [u8; 0],
}
pub type isRaytracingSupportedFn = ::std::option::Option<unsafe extern "C" fn(pRenderer: *mut Renderer) -> bool>;
extern "C" {
    #[link_name = "\u{1}?isRaytracingSupported@@3P6A_NPEAURenderer@@@ZEA"]
    pub static mut isRaytracingSupported: isRaytracingSupportedFn;
}
pub type initRaytracingFn =
    ::std::option::Option<unsafe extern "C" fn(pRenderer: *mut Renderer, ppRaytracing: *mut *mut Raytracing) -> bool>;
extern "C" {
    #[link_name = "\u{1}?initRaytracing@@3P6A_NPEAURenderer@@PEAPEAURaytracing@@@ZEA"]
    pub static mut initRaytracing: initRaytracingFn;
}
pub type removeRaytracingFn =
    ::std::option::Option<unsafe extern "C" fn(pRenderer: *mut Renderer, pRaytracing: *mut Raytracing)>;
extern "C" {
    #[link_name = "\u{1}?removeRaytracing@@3P6AXPEAURenderer@@PEAURaytracing@@@ZEA"]
    pub static mut removeRaytracing: removeRaytracingFn;
}
pub type addAccelerationStructureFn = ::std::option::Option<
    unsafe extern "C" fn(
        pRaytracing: *mut Raytracing,
        pDesc: *const AccelerationStructureDescTop,
        ppAccelerationStructure: *mut *mut AccelerationStructure,
    ),
>;
extern "C" {
    #[link_name = "\u{1}?addAccelerationStructure@@3P6AXPEAURaytracing@@PEBUAccelerationStructureDescTop@@PEAPEAUAccelerationStructure@@@ZEA"]
    pub static mut addAccelerationStructure: addAccelerationStructureFn;
}
pub type removeAccelerationStructureFn = ::std::option::Option<
    unsafe extern "C" fn(pRaytracing: *mut Raytracing, pAccelerationStructure: *mut AccelerationStructure),
>;
extern "C" {
    #[link_name = "\u{1}?removeAccelerationStructure@@3P6AXPEAURaytracing@@PEAUAccelerationStructure@@@ZEA"]
    pub static mut removeAccelerationStructure: removeAccelerationStructureFn;
}
pub type removeAccelerationStructureScratchFn = ::std::option::Option<
    unsafe extern "C" fn(pRaytracing: *mut Raytracing, pAccelerationStructure: *mut AccelerationStructure),
>;
extern "C" {
    #[link_name = "\u{1}?removeAccelerationStructureScratch@@3P6AXPEAURaytracing@@PEAUAccelerationStructure@@@ZEA"]
    pub static mut removeAccelerationStructureScratch: removeAccelerationStructureScratchFn;
}
pub type addRaytracingShaderTableFn = ::std::option::Option<
    unsafe extern "C" fn(
        pRaytracing: *mut Raytracing,
        pDesc: *const RaytracingShaderTableDesc,
        ppTable: *mut *mut RaytracingShaderTable,
    ),
>;
extern "C" {
    #[link_name = "\u{1}?addRaytracingShaderTable@@3P6AXPEAURaytracing@@PEBURaytracingShaderTableDesc@@PEAPEAURaytracingShaderTable@@@ZEA"]
    pub static mut addRaytracingShaderTable: addRaytracingShaderTableFn;
}
pub type removeRaytracingShaderTableFn =
    ::std::option::Option<unsafe extern "C" fn(pRaytracing: *mut Raytracing, pTable: *mut RaytracingShaderTable)>;
extern "C" {
    #[link_name = "\u{1}?removeRaytracingShaderTable@@3P6AXPEAURaytracing@@PEAURaytracingShaderTable@@@ZEA"]
    pub static mut removeRaytracingShaderTable: removeRaytracingShaderTableFn;
}
pub type cmdBuildAccelerationStructureFn = ::std::option::Option<
    unsafe extern "C" fn(pCmd: *mut Cmd, pRaytracing: *mut Raytracing, pDesc: *mut RaytracingBuildASDesc),
>;
extern "C" {
    #[link_name = "\u{1}?cmdBuildAccelerationStructure@@3P6AXPEAUCmd@@PEAURaytracing@@PEAURaytracingBuildASDesc@@@ZEA"]
    pub static mut cmdBuildAccelerationStructure: cmdBuildAccelerationStructureFn;
}
pub type cmdDispatchRaysFn = ::std::option::Option<
    unsafe extern "C" fn(pCmd: *mut Cmd, pRaytracing: *mut Raytracing, pDesc: *const RaytracingDispatchDesc),
>;
extern "C" {
    #[link_name = "\u{1}?cmdDispatchRays@@3P6AXPEAUCmd@@PEAURaytracing@@PEBURaytracingDispatchDesc@@@ZEA"]
    pub static mut cmdDispatchRays: cmdDispatchRaysFn;
}
