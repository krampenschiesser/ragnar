use ragnar_lib::NativeEvent;
use crate::event::datatransfer::DataTransfer;
use crate::event::Event;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InputEvent {
    pub data: String,
    pub is_composing: bool,
    pub input_type: InputType,
    pub data_transfer: DataTransfer,
    pub event: Event,
}

impl NativeEvent for InputEvent {
    fn get_type() -> &'static str where Self: Sized {
        "html.inputevent"
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum InputType {
    InsertText,
    InsertReplacementText,
    InsertLineBreak,
    InsertParagraph,
    InsertOrderedList,
    InsertUnorderedList,
    InsertHorizontalRule,
    InsertFromYank,
    InsertFromDrop,
    InsertFromPaste,
    InsertFromPasteAsQuotation,
    InsertTranspose,
    InsertCompositionText,
    InsertLink,
    DeleteWordBackward,
    DeleteWordForward,
    DeleteSoftLineBackward,
    DeleteSoftLineForward,
    DeleteEntireSoftLine,
    DeleteHardLineBackward,
    DeleteHardLineForward,
    DeleteByDrag,
    DeleteByCut,
    DeleteContent,
    DeleteContentBackward,
    DeleteContentForward,
    HistoryUndo,
    HistoryRedo,
    FormatBold,
    FormatItalic,
    FormatUnderline,
    FormatStrikeThrough,
    FormatSuperscript,
    FormatSubscript,
    FormatJustifyFull,
    FormatJustifyCenter,
    FormatJustifyRight,
    FormatJustifyLeft,
    FormatIndent,
    FormatOutdent,
    FormatRemove,
    FormatSetBlockTextDirection,
    FormatSetInlineTextDirection,
    FormatBackColor,
    FormatFontColor,
    FormatFontName,
}