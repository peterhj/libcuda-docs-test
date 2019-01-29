/* automatically generated by rust-bindgen */

pub const CUBLAS_STATUS_SUCCESS: cublasStatus_t = 0;
pub const CUBLAS_STATUS_NOT_INITIALIZED: cublasStatus_t = 1;
pub const CUBLAS_STATUS_ALLOC_FAILED: cublasStatus_t = 3;
pub const CUBLAS_STATUS_INVALID_VALUE: cublasStatus_t = 7;
pub const CUBLAS_STATUS_ARCH_MISMATCH: cublasStatus_t = 8;
pub const CUBLAS_STATUS_MAPPING_ERROR: cublasStatus_t = 11;
pub const CUBLAS_STATUS_EXECUTION_FAILED: cublasStatus_t = 13;
pub const CUBLAS_STATUS_INTERNAL_ERROR: cublasStatus_t = 14;
pub const CUBLAS_STATUS_NOT_SUPPORTED: cublasStatus_t = 15;
pub const CUBLAS_STATUS_LICENSE_ERROR: cublasStatus_t = 16;
pub type cublasStatus_t = u32;
pub const CUBLAS_OP_N: cublasOperation_t = 0;
pub const CUBLAS_OP_T: cublasOperation_t = 1;
pub const CUBLAS_OP_C: cublasOperation_t = 2;
pub type cublasOperation_t = u32;
pub const CUBLAS_POINTER_MODE_HOST: cublasPointerMode_t = 0;
pub const CUBLAS_POINTER_MODE_DEVICE: cublasPointerMode_t = 1;
pub type cublasPointerMode_t = u32;
pub const CUBLAS_ATOMICS_NOT_ALLOWED: cublasAtomicsMode_t = 0;
pub const CUBLAS_ATOMICS_ALLOWED: cublasAtomicsMode_t = 1;
pub type cublasAtomicsMode_t = u32;
pub const CUBLAS_GEMM_DFALT: cublasGemmAlgo_t = -1;
pub const CUBLAS_GEMM_ALGO0: cublasGemmAlgo_t = 0;
pub const CUBLAS_GEMM_ALGO1: cublasGemmAlgo_t = 1;
pub const CUBLAS_GEMM_ALGO2: cublasGemmAlgo_t = 2;
pub const CUBLAS_GEMM_ALGO3: cublasGemmAlgo_t = 3;
pub const CUBLAS_GEMM_ALGO4: cublasGemmAlgo_t = 4;
pub const CUBLAS_GEMM_ALGO5: cublasGemmAlgo_t = 5;
pub const CUBLAS_GEMM_ALGO6: cublasGemmAlgo_t = 6;
pub const CUBLAS_GEMM_ALGO7: cublasGemmAlgo_t = 7;
pub type cublasGemmAlgo_t = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cublasContext {
    _unused: [u8; 0],
}
pub type cublasHandle_t = *mut cublasContext;
extern "C" {
    pub fn cublasCreate_v2(handle: *mut cublasHandle_t) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasDestroy_v2(handle: cublasHandle_t) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasGetVersion_v2(
        handle: cublasHandle_t,
        version: *mut ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasGetProperty(
        type_: libraryPropertyType,
        value: *mut ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasSetStream_v2(handle: cublasHandle_t, streamId: cudaStream_t) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasGetStream_v2(
        handle: cublasHandle_t,
        streamId: *mut cudaStream_t,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasGetPointerMode_v2(
        handle: cublasHandle_t,
        mode: *mut cublasPointerMode_t,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasSetPointerMode_v2(
        handle: cublasHandle_t,
        mode: cublasPointerMode_t,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasGetAtomicsMode(
        handle: cublasHandle_t,
        mode: *mut cublasAtomicsMode_t,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasSetAtomicsMode(
        handle: cublasHandle_t,
        mode: cublasAtomicsMode_t,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasSnrm2_v2(
        handle: cublasHandle_t,
        n: ::std::os::raw::c_int,
        x: *const f32,
        incx: ::std::os::raw::c_int,
        result: *mut f32,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasSdot_v2(
        handle: cublasHandle_t,
        n: ::std::os::raw::c_int,
        x: *const f32,
        incx: ::std::os::raw::c_int,
        y: *const f32,
        incy: ::std::os::raw::c_int,
        result: *mut f32,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasSscal_v2(
        handle: cublasHandle_t,
        n: ::std::os::raw::c_int,
        alpha: *const f32,
        x: *mut f32,
        incx: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasSaxpy_v2(
        handle: cublasHandle_t,
        n: ::std::os::raw::c_int,
        alpha: *const f32,
        x: *const f32,
        incx: ::std::os::raw::c_int,
        y: *mut f32,
        incy: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasSgemv_v2(
        handle: cublasHandle_t,
        trans: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        x: *const f32,
        incx: ::std::os::raw::c_int,
        beta: *const f32,
        y: *mut f32,
        incy: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasSgemm_v2(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        B: *const f32,
        ldb: ::std::os::raw::c_int,
        beta: *const f32,
        C: *mut f32,
        ldc: ::std::os::raw::c_int,
    ) -> cublasStatus_t;
}
extern "C" {
    pub fn cublasGemmEx(
        handle: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const ::std::os::raw::c_void,
        A: *const ::std::os::raw::c_void,
        Atype: cudaDataType,
        lda: ::std::os::raw::c_int,
        B: *const ::std::os::raw::c_void,
        Btype: cudaDataType,
        ldb: ::std::os::raw::c_int,
        beta: *const ::std::os::raw::c_void,
        C: *mut ::std::os::raw::c_void,
        Ctype: cudaDataType,
        ldc: ::std::os::raw::c_int,
        computeType: cudaDataType,
        algo: cublasGemmAlgo_t,
    ) -> cublasStatus_t;
}
