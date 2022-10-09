pub enum PLUGIN_RESULT {
    PLUGIN_CONTINUE = 0,
    PLUGIN_OVERRIDE = 1,
    PLUGIN_STOP = 2,
}

pub enum EQueryCvarValueStatus {
    eQueryCvarValueStatus_ValueIntact = 0,
    eQueryCvarValueStatus_CvarNotFound = 1,
    eQueryCvarValueStatus_NotACvar = 2,
    eQueryCvarValueStatus_CvarProtected = 3,
}

pub type QueryCvarCookie_t = u32;
pub const InvalidQueryCvarCookie: i32 = -1;

pub const INTERFACEVERSION_ISERVERPLUGINCALLBACKS_VERSION_1: &'static str = "ISERVERPLUGINCALLBACKS001";
pub const INTERFACEVERSION_ISERVERPLUGINCALLBACKS_VERSION_2: &'static str = "ISERVERPLUGINCALLBACKS002";
pub const INTERFACEVERSION_ISERVERPLUGINCALLBACKS: &'static str = "ISERVERPLUGINCALLBACKS003";

pub const INTERFACEVERSION_ISERVERPLUGINHELPERS: &'static str = "ISERVERPLUGINHELPERS001";

pub enum DIALOG_TYPE {
    DIALOG_MSG = 0,
    DIALOG_MENU = 1,
    DIALOG_TEXT = 2,
    DIALOG_ENTRY = 3,
    DIALOG_ASKCONNECT = 4,
}