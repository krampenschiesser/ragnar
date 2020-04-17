use ragnar_lib::NativeEvent;

pub mod mouse;
pub mod input;
pub mod datatransfer;
pub mod keyboard;

pub use mouse::*;
pub use input::*;
pub use keyboard::*;
pub use datatransfer::*;

use crate::global::file::File;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Event {
    pub bubbles: bool,
    pub cancelable: bool,
    pub composed: bool,
    pub timestamp: u64,
    pub is_trusted: bool,
    pub event_type: EventType,
    pub target_value: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TargetValue {
    String(String),
    File(File),
    None,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum EventPhase {
    None,
    CapturingPhase,
    AtTarget,
    BubblingPhase,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum EventType {
    Error,
    Abort,
    Load,
    BeforeUnload,
    Unload,
    Online,
    Offline,
    Focus,
    Blur,
    FocusIn,
    FocusOut,
    Close,
    PageHide,
    PageShow,
    PopState,
    AnimationStart,
    AnimationCancel,
    AnimationEnd,
    AnimationIteration,
    TransitionStart,
    TransitionCancel,
    TransitionEnd,
    TransitionRun,
    Reset,
    Submit,
    BeforePrint,
    AfterPrint,
    CompositionStart,
    CompositionUpdate,
    CompositionEnd,
    FullscreenChange,
    FullscreenError,
    Resize,
    Scroll,
    Cut,
    Copy,
    Paste,
    KeyDown,
    KeyPress,
    KeyUp,
    AuxClick,
    Click,
    ContextMenu,
    DblClick,
    MouseDown,
    MouseEnter,
    MouseLeave,
    MouseMove,
    MouseOver,
    MouseOut,
    MouseUp,
    PointerLockChange,
    PointerLockError,
    Select,
    Wheel,
    Drag,
    DragEnd,
    DragEnter,
    DragStart,
    DragLeave,
    DragOver,
    Drop,
    AudioProcess,
    CanPlay,
    CanPlayThrough,
    Complete,
    DurationChange,
    Emptied,
    Ended,
    LoadedData,
    LoadedMetaData,
    Pause,
    Play,
    Playing,
    RateChange,
    Seeked,
    Seeking,
    Stalled,
    Suspend,
    TimeUpdate,
    VolumeChange,
    Waiting,
    LoadEnd,
    LoadStart,
    Progress,
    Timeout,
    Change,
    Storage,
    Checking,
    Downloading,
    NoUpdate,
    Obsolete,
    UpdateReady,
    Broadcast,
    CheckboxStateChange,
    HashChange,
    Input,
    RadioStateChange,
    ReadyStateChange,
    ValueChange,
    Invalid,
    Open,
    Show,
}