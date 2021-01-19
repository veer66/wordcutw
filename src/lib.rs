use std::{ffi::{c_void, CStr}, mem::forget, os::raw::c_char, path::Path, ptr};

#[repr(C)]
#[derive(PartialEq, Clone, Debug)]
pub struct TextRange {
    pub s: usize,
    pub e: usize,
}

fn  wordcut_new_with_dict_path(path: &Path) -> *const c_void {
    match wordcut_engine::load_dict(path) {
        Ok(dict) => {
	    let wordcut = wordcut_engine::Wordcut::new(dict);
            let boxed_wordcut = Box::new(wordcut);
            Box::into_raw(boxed_wordcut) as *const c_void
        }
        Err(e) => {
            eprintln!("{}", e);
            return ptr::null();
        }
    }
}

pub extern "C" fn wordcut_new_with_dict(path: *const c_char) -> *const c_void {
    let path = unsafe { CStr::from_ptr(path) }.to_str().unwrap();
    let path = Path::new(path);
    wordcut_new_with_dict_path(path)
}

pub extern "C" fn wordcut_new_with_dict_from_default_dir(path: *const c_char) -> *const c_void {
    let path = unsafe { CStr::from_ptr(path) }.to_str().unwrap();
    let path = chamkho::cargo_dir().join(path);
    wordcut_new_with_dict_path(&path)
}

pub extern "C" fn delete_wordcut(wordcut: *const c_void) {
    let wordcut: *const wordcut_engine::Wordcut = wordcut as *const wordcut_engine::Wordcut;
    drop(wordcut);
}

pub extern "C" fn delete_text_ranges(text_ranges: *mut TextRange, range_count: usize) {
    unsafe {
	Vec::from_raw_parts(text_ranges, range_count, range_count)
    };
}

pub extern "C" fn wordcut_into_text_ranges(wordcut: *const c_void, text: *const c_char, range_count: *mut usize) -> *mut TextRange {
    let wordcut: *const wordcut_engine::Wordcut = wordcut as *const wordcut_engine::Wordcut;    
    let text = unsafe { CStr::from_ptr(text) }.to_str().unwrap();
    let text_ranges = unsafe { (*wordcut).segment(text) };
    let mut text_ranges: Vec<TextRange> = text_ranges.into_iter()
        .map(|r| TextRange {s: r.s, e: r.e})
        .collect();
    unsafe { *range_count = text_ranges.len(); };
    let p = text_ranges.as_mut_ptr();
    forget(text_ranges);
    return p
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use super::*;

    #[test]
    fn test_wordcut_into_text_ranges() {
	let text = CString::new("ลากา").unwrap().into_raw();
        let wordcut = wordcut_new_with_dict_from_default_dir(CString::new("data/thai.txt").unwrap().into_raw());
	let mut range_count = 0;
	let text_ranges = wordcut_into_text_ranges(wordcut, text, &mut range_count);
	assert_eq!(range_count, 2);		
	unsafe {
	    assert_eq!(*text_ranges, TextRange {s:0,e:2});
	    assert_eq!(*text_ranges.offset(1), TextRange {s:2,e:4});
	}
	delete_text_ranges(text_ranges, range_count);
        delete_wordcut(wordcut);
    }
}
