use borderbook::{ Orderbook, Order };
use std::ffi::{ CStr };
use std::os::raw::{ c_char };

pub struct OrderbookType;

#[no_mangle]
pub unsafe extern "C" fn borderbook_new() -> *mut OrderbookType {
    Box::into_raw(Box::new(Orderbook::<String>::new())) as *mut OrderbookType
}


#[no_mangle]
pub extern "C" fn borderbook_free(ptr: *mut OrderbookType) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr as *mut Orderbook<String>); }
}


#[no_mangle]
pub extern "C" fn borderbook_insert(ptr: *mut OrderbookType, key: *const c_char, bid: bool, volume: f64, price: f64) {
    let orderbook = unsafe {
        assert!(!ptr.is_null());
        &mut *(ptr as *mut Orderbook<String>)
    };

    let key = unsafe {
        assert!(!key.is_null());
        CStr::from_ptr(key)
    };

    let key = key.to_str().unwrap().to_owned();

    let order = Order { volume, price };

    if bid {
        orderbook.insert_bid(key, order);
    }
    else {
        orderbook.insert_ask(key, order);
    }
}

#[no_mangle]
pub extern "C" fn borderbook_print(ptr: *mut OrderbookType) {
    let orderbook = unsafe {
        assert!(!ptr.is_null());
        &mut *(ptr as *mut Orderbook<String>)
    };

    print!("{:?}", orderbook);
}