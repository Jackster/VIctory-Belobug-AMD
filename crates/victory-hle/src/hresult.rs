//! The Windows `HRESULT` values the modules return, defined once.
//!
//! These used to be copied into every module that needed one, so a fix to
//! one copy left the others wrong. The values are the `winerror.h` ones,
//! written as `u32` bit patterns cast to `i32` because that is how a title
//! sees them. Constants that belong to one module's own protocol (for
//! example the `0x8AC8_1xxx` results of the audio coprocessor) stay there.

use crate::modules::xgameruntime::HRESULT;

/// `E_ABORT`: the operation was cancelled before it finished.
pub(crate) const E_ABORT: HRESULT = 0x8000_4004_u32 as i32;
/// `E_FAIL`: a generic failure.
pub(crate) const E_FAIL: HRESULT = 0x8000_4005_u32 as i32;
/// `E_ILLEGAL_METHOD_CALL`: the object's state does not allow the call.
pub(crate) const E_ILLEGAL_METHOD_CALL: HRESULT = 0x8000_000E_u32 as i32;
/// `E_INVALIDARG`: one or more arguments are not valid.
pub(crate) const E_INVALIDARG: HRESULT = 0x8007_0057_u32 as i32;
/// `E_NOINTERFACE`: the object does not implement the interface asked for.
pub(crate) const E_NOINTERFACE: HRESULT = 0x8000_4002_u32 as i32;
/// `E_NOT_SUFFICIENT_BUFFER`: the caller's buffer is too small for the result.
pub(crate) const E_NOT_SUFFICIENT_BUFFER: HRESULT = 0x8007_007A_u32 as i32;
/// `E_NOTIMPL`: the call is not implemented.
pub(crate) const E_NOTIMPL: HRESULT = 0x8000_4001_u32 as i32;
/// `E_OUTOFMEMORY`: not enough memory to complete the call.
pub(crate) const E_OUTOFMEMORY: HRESULT = 0x8007_000E_u32 as i32;
/// `E_PENDING`: the call is still in progress; ask again later.
pub(crate) const E_PENDING: HRESULT = 0x8000_000A_u32 as i32;
/// `E_POINTER`: an argument was a pointer that is not valid.
pub(crate) const E_POINTER: HRESULT = 0x8000_4003_u32 as i32;
