/// Implements `From` and `Into` for enums with `#[repr(usize)]`. Useful for interfacing with C
/// enums.
macro_rules! impl_clike {
    ($t:ty, $($c:ty) +) => {
        $(
            impl From<$c> for $t {
                fn from(v: $c) -> $t {
                    use std::mem;
                    unsafe { mem::transmute(v as usize) }
                }
            }

            impl Into<$c> for $t {
                fn into(self) -> $c {
                    self as $c
                }
            }
        )*
    };

    ($t:ty) => {
        impl_clike!($t, i8 i16 i32 i64 isize u8 u16 u32 u64 usize);
    }
}

#[derive(Debug)]
#[repr(usize)]
pub enum Opcode {
    Initialize,
    Shutdown,
    ChangePreset,
    GetCurrentPresetNum,
    SetCurrentPresetName,
    GetCurrentPresetName,
    GetParameterLabel,
    GetParameterDisplay,
    GetParameterName,
    _GetVu,
    SetSampleRate,
    SetBlockSize,
    StateChanged,
    EditorGetRect, // Get rect, son.
    EditorOpen,
    EditorClose,
    _EditorDraw,
    _EditorMouse,
    _EditorKey,
    EditorIdle,
    _EditorTop,
    _EditorSleep,
    _EditorIdentify,
    GetData,
    SetData,
    ProcessEvents,
    CanBeAutomated,
    StringToParameter,
    _GetNumCategories,
    GetPresetName,
    _CopyPreset,
    _ConnectIn,
    _ConnectOut,
    GetInputInfo,
    GetOutputInfo,
    GetCategory,
    _GetCurrentPosition,
    _GetDestinationBuffer,
    OfflineNotify,
    OfflinePrepare,
    OfflineRun,
    ProcessVarIo,
    SetSpeakerArrangement,
    _SetBlocksizeAndSampleRate,
    SoftBypass,
    GetEffectName,
    _GetErrorText,
    GetVendorName,
    GetProductName,
    GetVendorVersion,
    VendorSpecific,
    CanDo,
    GetTailSize,
    _Idle,
    _GetIcon,
    _SetVewPosition,
    GetParamInfo,
    _KeysRequired,
    GetApiVersion,
    EditorKeyDown,
    EditorKeyUp,
    EditorSetKnobMode,
    GetMidiProgramName,
    GetCurrentMidiProgram,
    GetMidiProgramCategory,
    HasMidiProgramsChanged,
    GetMidiKeyName,
    BeginSetPreset,
    EndSetPreset,
    GetSpeakerArrangement,
    ShellGetNextPlugin,
    StartProcess,
    StopProcess,
    SetTotalSampleToProcess,
    SetPanLaw,
    BeginLoadBank,
    BeginLoadPreset,
    SetPrecision,
    GetNumMidiInputs,
    GetNumMidiOutputs,
}
impl_clike!(Opcode);