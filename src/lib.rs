use std::{ffi::c_void, path::PathBuf, ptr::null_mut, slice};

use objc::{msg_send, runtime::Class, sel, sel_impl};
use objc_derive::selector_export;

pub type id = *mut objc::runtime::Object;

pub type NSInteger = i64;
pub type NSUInteger = u64;

pub trait GetObjcObject {
    fn objc_object(&self) -> id;
}

extern "C" {
    pub fn NSClassFromString(class_name: NSString) -> Class;
}

#[repr(transparent)]
#[derive(Clone)]
pub struct NSString(pub id);
impl std::ops::Deref for NSString {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for NSString {}
impl NSString {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(NSString), alloc) })
    }
}

impl NSString {
    pub fn from_str(val: &str) -> Self {
        let alloc = NSString::alloc();
        unsafe {
            let ret = alloc.init_with_bytes_length_encoding(
                val.as_ptr() as *const c_void,
                val.len() as usize,
                4,
            );
            ret
        }
    }

    /// A utility method for taking an `NSString` and bridging it to a Rust `&str`.
    pub fn to_str(&self) -> &str {
        unsafe {
            let bytes = self.utf8_string();
            let len = self.length_of_bytes_using_encoding(4);

            let bytes = slice::from_raw_parts(bytes, len as usize);

            std::str::from_utf8(bytes).unwrap()
        }
    }

    /// A utility method for taking an `NSString` and getting an owned `String` from it.
    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}

impl NSString {
    #[selector_export("initWithBytes:length:encoding:")]
    pub fn init_with_bytes_length_encoding(&self, bytes: *const c_void, length: usize, encoding: u64) -> NSString;

    #[selector_export("lengthOfBytesUsingEncoding:")]
    pub fn length_of_bytes_using_encoding(&self, encoding: u64) -> usize;

    #[selector_export("UTF8String")]
    pub fn utf8_string(&self) -> *const u8;
}

impl GetObjcObject for NSString {
    fn objc_object(&self) -> id {
        self.0
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct NSSet(pub id);
impl std::ops::Deref for NSSet {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for NSSet {}
impl NSSet {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(NSSet), alloc) })
    }
}

impl NSSet {
    #[selector_export(NSSet, "setWithArray:")]
    pub fn set_with_array(array: NSArray) -> NSSet;
}

impl GetObjcObject for NSSet {
    fn objc_object(&self) -> id {
        self.0
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct NSDictionary(pub id);
impl std::ops::Deref for NSDictionary {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for NSDictionary {}
impl NSDictionary {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(NSDictionary), alloc) })
    }
}

impl NSDictionary {
    #[selector_export("init")]
    pub fn init(&self) -> NSDictionary;
}

impl GetObjcObject for NSDictionary {
    fn objc_object(&self) -> id {
        self.0
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct NSArray(pub id);
impl std::ops::Deref for NSArray {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for NSArray {}
impl NSArray {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(NSArray), alloc) })
    }
}

impl NSArray {
    pub fn from_slice(slice: &[id]) -> Self {
        unsafe {
            let raw_ptr = slice.as_ptr();
            let ptr_value = raw_ptr as usize;
            let mut_ptr = ptr_value as *mut id;

            let ret = NSArray::array_with_objects_count(
                mut_ptr,
                slice.len() as u64,
            );
            ret
        }
    }

    pub fn map<T, F: Fn(id) -> T>(&self, transform: F) -> Vec<T> {
        unsafe {
            let count = self.count();

            // I don't know if it's worth trying to get in with NSFastEnumeration here. I'm content to
            // just rely on Rust, but someone is free to profile it if they want.
            (0..count)
                .map(|index| {
                    let item: id = self.object_at_index(index);
                    transform(item)
                })
                .collect()
        }
    }
}

impl NSArray {
    #[selector_export(NSArray, "arrayWithObjects:count:")]
    pub fn array_with_objects_count(objects: *mut id, count: u64) -> NSArray;

    #[selector_export("count")]
    pub fn count(&self) -> usize;

    #[selector_export("objectAtIndex:")]
    pub fn object_at_index(&self, index: usize) -> id;
}

impl GetObjcObject for NSArray {
    fn objc_object(&self) -> id {
        self.0
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct NSDecimalNumber(pub id);
impl std::ops::Deref for NSDecimalNumber {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for NSDecimalNumber {}
impl NSDecimalNumber {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(NSDecimalNumber), alloc) })
    }
}

impl NSDecimalNumber {
    #[selector_export("doubleValue")]
    pub fn double_value(&self) -> f64;
}

impl GetObjcObject for NSDecimalNumber {
    fn objc_object(&self) -> id {
        self.0
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct NSLocale(pub id);
impl std::ops::Deref for NSLocale {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for NSLocale {}
impl NSLocale {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(NSLocale), alloc) })
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct NSNumber(pub id);
impl std::ops::Deref for NSNumber {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for NSNumber {}
impl NSNumber {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(NSNumber), alloc) })
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct NSNumberFormatter(pub id);
impl std::ops::Deref for NSNumberFormatter {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for NSNumberFormatter {}
impl NSNumberFormatter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(NSNumberFormatter), alloc) })
    }
}

impl NSNumberFormatter {
    #[selector_export("setFormatterBehavior:")]
    pub fn set_formatter_behavior(&self, value: usize);

    #[selector_export("setNumberStyle:")]
    pub fn set_number_style(&self, value: usize);

    #[selector_export("setLocale:")]
    pub fn set_locale(&self, value: NSLocale);

    #[selector_export("stringFromNumber:")]
    pub fn string_from_number(&self, number: NSNumber) -> NSString;
}

impl GetObjcObject for NSNumberFormatter {
    fn objc_object(&self) -> id {
        self.0
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct NSURL(pub id);
impl std::ops::Deref for NSURL {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for NSURL {}
impl NSURL {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(NSURL), alloc) })
    }
}

impl NSURL {
    #[selector_export(NSURL, "URLWithString:")]
    pub fn url_with_string(url_string: NSString) -> NSURL;

    #[selector_export(NSURL, "fileURLWithPath:")]
    pub fn file_url_with_path(path: NSString) -> NSURL;
    
    #[selector_export("absoluteString")]
    pub fn absolute_string(&self) -> NSString;

    #[selector_export("path")]
    pub fn path(&self) -> NSString;

    #[selector_export("startAccessingSecurityScopedResource")]
    pub fn start_accessing_security_scoped_resource(&self) -> bool;
}

impl GetObjcObject for NSURL {
    fn objc_object(&self) -> id {
        self.0
    }
}


#[repr(transparent)]
#[derive(Clone)]
pub struct UTType(pub id);
impl std::ops::Deref for UTType {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for UTType {}
impl UTType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(UTType), alloc) })
    }
}

impl UTType {
    #[selector_export(UTType, "typeWithFilenameExtension:")]
    pub fn type_with_filename_extension(filenameExtension: NSString) -> UTType;
}

impl GetObjcObject for UTType {
    fn objc_object(&self) -> id {
        self.0
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct NSOperationQueue(pub id);
impl std::ops::Deref for NSOperationQueue {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for NSOperationQueue {}
impl NSOperationQueue {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(NSOperationQueue), alloc) })
    }
}

impl NSOperationQueue {
    #[selector_export(NSOperationQueue, "mainQueue")]
    pub fn main_queue() -> NSOperationQueue;

    #[selector_export("addOperationWithBlock:")]
    pub fn add_operation_with_block(&self, block: *const ::block::Block<(), ()>);
}

impl GetObjcObject for NSOperationQueue {
    fn objc_object(&self) -> id {
        self.0
    }
}


#[repr(transparent)]
#[derive(Clone)]
pub struct NSUserDefaults(pub id);
impl std::ops::Deref for NSUserDefaults {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for NSUserDefaults {}
impl NSUserDefaults {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(NSUserDefaults), alloc) })
    }
}

impl NSUserDefaults {
    #[selector_export(NSUserDefaults, "standardUserDefaults")]
    pub fn standard_user_defaults() -> NSUserDefaults;

    #[selector_export("objectForKey:")]
    pub fn object_for_key(&self, key: NSString) -> id;

    #[selector_export("stringForKey:")]
    pub fn string_for_key(&self, key: NSString) -> NSString;

    #[selector_export("integerForKey:")]
    pub fn integer_for_key(&self, key: NSString) -> NSInteger;

    #[selector_export("setObject:forKey:")]
    pub fn set_object_for_key(&self, value: id, for_key: NSString);

    #[selector_export("setInteger:forKey:")]
    pub fn set_integer_for_key(&self, value: NSInteger, for_key: NSString);

    pub fn contains_key(&self, key: &str) -> bool {
        let key = NSString::from_str(key);
        let obj = self.object_for_key(key);

        if obj != null_mut() {
            true
        } else {
            false
        }
    }
}

impl GetObjcObject for NSUserDefaults {
    fn objc_object(&self) -> id {
        self.0
    }
}


#[repr(transparent)]
#[derive(Clone)]
pub struct NSError(pub id);
impl std::ops::Deref for NSError {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for NSError {}
impl NSError {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(NSError), alloc) })
    }
}

impl NSError {
    #[selector_export("code")]
    pub fn code(&self) -> NSInteger;

    #[selector_export("localizedDescription")]
    pub fn localized_description(&self) -> NSString;
}

impl GetObjcObject for NSError {
    fn objc_object(&self) -> id {
        self.0
    }
}



#[repr(transparent)]
#[derive(Clone)]
pub struct NSData(pub id);
impl std::ops::Deref for NSData {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for NSData {}
impl NSData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(NSData), alloc) })
    }
}

impl NSData {
    #[selector_export("initWithContentsOfURL:")]
    pub fn init_with_contents_of_url(&self, url: NSURL) -> NSData;

    #[selector_export("base64EncodedStringWithOptions:")]
    pub fn base64_encoded_string_with_options(&self, options: NSUInteger) -> NSString;
}

impl GetObjcObject for NSData {
    fn objc_object(&self) -> id {
        self.0
    }
}


#[repr(transparent)]
#[derive(Clone)]
pub struct NSBundle(pub id);
impl std::ops::Deref for NSBundle {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for NSBundle {}
impl NSBundle {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(NSBundle), alloc) })
    }
}

impl NSBundle {
    #[selector_export(NSBundle, "mainBundle")]
    pub fn main_bundle() -> NSBundle;

    #[selector_export("appStoreReceiptURL")]
    pub fn app_store_receipt_url(&self) -> NSURL;
}

impl GetObjcObject for NSBundle {
    fn objc_object(&self) -> id {
        self.0
    }
}


#[repr(transparent)]
#[derive(Clone)]
pub struct NSFileManager(pub id);
impl std::ops::Deref for NSFileManager {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl NSFileManager {
    #[selector_export(NSFileManager, "defaultManager")]
    pub fn default_manager() -> NSFileManager;

    #[selector_export("temporaryDirectory")]
    pub fn temporary_directory(&self) -> NSURL;

    #[selector_export("URLsForDirectory:inDomains:")]
    pub fn urls_for_directory_in_domains(&self, directory: NSSearchPathDirectory, domain_mask: NSSearchPathDomainMask) -> NSArray;
}

impl NSFileManager {
    pub fn get_documents_dir(&self) -> PathBuf {
        let paths = self.urls_for_directory_in_domains(NSSearchPathDirectory_NSDocumentDirectory, NSSearchPathDomainMask_NSUserDomainMask);
        let urls = paths.map(|val| {
            NSURL(val)
        });
        let path_string = urls[0].path().to_string();
        PathBuf::from(&path_string)
    }
}

impl GetObjcObject for NSFileManager {
    fn objc_object(&self) -> id {
        self.0
    }
}

pub const NSSearchPathDirectory_NSApplicationDirectory: NSSearchPathDirectory = 1;
pub const NSSearchPathDirectory_NSDemoApplicationDirectory: NSSearchPathDirectory = 2;
pub const NSSearchPathDirectory_NSDeveloperApplicationDirectory: NSSearchPathDirectory = 3;
pub const NSSearchPathDirectory_NSAdminApplicationDirectory: NSSearchPathDirectory = 4;
pub const NSSearchPathDirectory_NSLibraryDirectory: NSSearchPathDirectory = 5;
pub const NSSearchPathDirectory_NSDeveloperDirectory: NSSearchPathDirectory = 6;
pub const NSSearchPathDirectory_NSUserDirectory: NSSearchPathDirectory = 7;
pub const NSSearchPathDirectory_NSDocumentationDirectory: NSSearchPathDirectory = 8;
pub const NSSearchPathDirectory_NSDocumentDirectory: NSSearchPathDirectory = 9;
pub const NSSearchPathDirectory_NSCoreServiceDirectory: NSSearchPathDirectory = 10;
pub const NSSearchPathDirectory_NSAutosavedInformationDirectory: NSSearchPathDirectory = 11;
pub const NSSearchPathDirectory_NSDesktopDirectory: NSSearchPathDirectory = 12;
pub const NSSearchPathDirectory_NSCachesDirectory: NSSearchPathDirectory = 13;
pub const NSSearchPathDirectory_NSApplicationSupportDirectory: NSSearchPathDirectory = 14;
pub const NSSearchPathDirectory_NSDownloadsDirectory: NSSearchPathDirectory = 15;
pub const NSSearchPathDirectory_NSInputMethodsDirectory: NSSearchPathDirectory = 16;
pub const NSSearchPathDirectory_NSMoviesDirectory: NSSearchPathDirectory = 17;
pub const NSSearchPathDirectory_NSMusicDirectory: NSSearchPathDirectory = 18;
pub const NSSearchPathDirectory_NSPicturesDirectory: NSSearchPathDirectory = 19;
pub const NSSearchPathDirectory_NSPrinterDescriptionDirectory: NSSearchPathDirectory = 20;
pub const NSSearchPathDirectory_NSSharedPublicDirectory: NSSearchPathDirectory = 21;
pub const NSSearchPathDirectory_NSPreferencePanesDirectory: NSSearchPathDirectory = 22;
pub const NSSearchPathDirectory_NSApplicationScriptsDirectory: NSSearchPathDirectory = 23;
pub const NSSearchPathDirectory_NSItemReplacementDirectory: NSSearchPathDirectory = 99;
pub const NSSearchPathDirectory_NSAllApplicationsDirectory: NSSearchPathDirectory = 100;
pub const NSSearchPathDirectory_NSAllLibrariesDirectory: NSSearchPathDirectory = 101;
pub const NSSearchPathDirectory_NSTrashDirectory: NSSearchPathDirectory = 102;
pub type NSSearchPathDirectory = NSUInteger;
pub const NSSearchPathDomainMask_NSUserDomainMask: NSSearchPathDomainMask = 1;
pub const NSSearchPathDomainMask_NSLocalDomainMask: NSSearchPathDomainMask = 2;
pub const NSSearchPathDomainMask_NSNetworkDomainMask: NSSearchPathDomainMask = 4;
pub const NSSearchPathDomainMask_NSSystemDomainMask: NSSearchPathDomainMask = 8;
pub const NSSearchPathDomainMask_NSAllDomainsMask: NSSearchPathDomainMask = 65535;
pub type NSSearchPathDomainMask = NSUInteger;