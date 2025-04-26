//  @author Merve Gulmez (merve.gulmez@ericsson.com)
//  @version 0.1
//  @date 2023-08-31
//  @copyright © Ericsson AB 2023
//  SPDX-License-Identifier: BSD 3-Clause

#[macro_export]
#[cfg(feature = "bincode_v2")]
macro_rules! sdrad_push_function_args {
	// mutable reference passing
	($bytes:expr, $head:ident : &mut $var_type:ty) => {
		encode_into_std_write(&*$head, &mut $bytes, config::standard());
	};
	// mutable reference passing with tailing
	($bytes:expr, $head:ident : &mut $var_type:ty, $($tail:tt)+) => {
		encode_into_std_write(&*$head, &mut $bytes, config::standard());
		sdrad_push_function_args!($bytes, $($tail)+);
	};
	// reference value passing
	($bytes:expr, $head:ident : &$var_type:ty) => {
		encode_into_std_write($head, &mut $bytes, config::standard());
	};
	// reference value passing with tailing
	($bytes:expr, $head:ident : &$var_type:ty, $($tail:tt)+) => {
		encode_into_std_write($head, &mut $bytes, config::standard());
		sdrad_push_function_args!($bytes, $($tail)+);
	};

	($bytes:expr, $head:ident : $var_type:ty, $($tail:tt)*) => {
		encode_into_std_write(&$head, &mut $bytes, config::standard());
		sdrad_push_function_args!($bytes, $($tail)+);
	};
	// one argument passing
	($bytes:expr, $head:ident : $var_type:ty ) => {
		encode_into_std_write(&$head, &mut $bytes, config::standard());
	};
	($bytes:expr, mut $head:ident : $var_type:ty ) => {
		encode_into_std_write(&$head, &mut $bytes, config::standard());
	};
	($bytes:expr, mut $head:ident : $var_type:ty, $($tail:tt)+) => {
		encode_into_std_write($head, &mut $bytes, config::standard()).unwrap();
		sdrad_push_function_args!($bytes, $($tail)+);
	};
	($bytes:expr, ) => {};
}

#[macro_export]
#[cfg(feature = "bincode_v2")]
macro_rules! sdrad_store_changed_vars_global {
	/* [u8] */
	($bytes:expr, $head:ident : &mut &[$type:ty]) => {
		encode_into_std_write(&*$head, &mut $bytes, config::standard());
	};

	($bytes:expr, $head:ident : &mut &[$type:ty], $($tail:tt)+) => {
		encode_into_std_write(&*$head, &mut $bytes, config::standard());
		sdrad_store_changed_vars_global!($bytes, $($tail)+);
	};

	/* [u8] */
	($bytes:expr, $head:ident : &[$type:ty]) => {
	};

	($bytes:expr, $head:ident : &[$type:ty], $($tail:tt)+) => {
		sdrad_store_changed_vars_global!($bytes, $($tail)+);
	};

	// mutable reference passing
	($bytes:expr, $head:ident : &mut $var_type:ty) => {
		encode_into_std_write(&*$head, &mut $bytes, config::standard());
	};

	// mutable reference passing with tailing
	($bytes:expr, $head:ident : &mut $var_type:ty, $($tail:tt)+) => {
		encode_into_std_write(&*$head, &mut $bytes, config::standard());
		sdrad_store_changed_vars_global!($bytes, $($tail)+);
	};

	/*non mutable reference we don't need to do anything */
	($bytes:expr, $head:ident : &$var_type:ty) => {

	};
	/*check the other type*/
	($bytes:expr, $head:ident : &$var_type:ty, $($tail:tt)+) => {
		sdrad_store_changed_vars_global!($bytes, $($tail)+);
	};

	/*non mutable reference */
	($bytes:expr, $head:ident : $var_type:ty, $($tail:tt)+) => {
		sdrad_store_changed_vars_global!($bytes, $($tail)+);
	};

	($bytes:expr, $head:ident : $var_type:ty ) => {

	};

	($bytes:expr, mut $head:ident : $var_type:ty ) => {

	};

	($bytes:expr, mut $head:ident : $var_type:ty, $($tail:tt)+) => {
		sdrad_store_changed_vars_global!($bytes, $($tail)+);
	};

	($bytes:expr, ) => {};
}

#[macro_export]
#[cfg(feature = "bincode_v2")]
macro_rules! sdrad_restore_changed_vars_global {

	($bytes:expr, $head:ident : &mut &[$type:ty]) => {
		*$head = decode_borrowed_from_slice(&$bytes[..], config::standard()).unwrap();
	};

	($bytes:expr, $head:ident : &mut &[$type:ty], $($tail:tt)+) => {
		*$head = decode_borrowed_from_slice(&$bytes[..], config::standard()).unwrap();
		sdrad_restore_changed_vars_global!($bytes:expr, $($tail)+);
	};

	($bytes:expr, $head:ident :  &[$type:ty]) => {

	};

	($bytes:expr, $head:ident :  &[$type:ty], $($tail:tt)+) => {
		sdrad_restore_changed_vars_global!($bytes:expr, $($tail)+);
	};

	($bytes:ident, $head:ident : &mut $var_type:ty) => {
		*$head = decode_borrowed_from_slice(&$bytes[..], config::standard()).unwrap();
	};

	($bytes:ident, $head:ident : &mut $var_type:ty, $($tail:tt)+) => {
		*$head = decode_borrowed_from_slice(&$bytes[..], config::standard()).unwrap();
		sdrad_restore_changed_vars_global!($bytes, $($tail)+);
	};

	($bytes:ident, $head:ident : &$var_type:ty) => {

	};

	($bytes:ident, $head:ident : &$var_type:ty, $($tail:tt)+) => {
		sdrad_restore_changed_vars_global!($bytes, $($tail)+);
	};

	($bytes:ident, $head:ident : $var_type:ty, $($tail:tt)+) => {
		sdrad_restore_changed_vars_global!($bytes, $($tail)+);
	};

	($bytes:ident, mut $head:ident : $var_type:ty ) => {

	};
	($bytes:ident, mut $head:ident : $var_type:ty, $($tail:tt)+) => {
		sdrad_restore_changed_vars_global!($bytes, $($tail)+);
	};
	($bytes:ident, $head:ident : $var_type:ty ) => {

	};
	($bytes:ident, ) => {

	};
}

#[macro_export]
#[cfg(feature = "bincode_v2")]
macro_rules! sdrad_pull_function_args {
	($bytes:expr, $head:ident : &mut [$type:ty]) => {
		let mut $head: Vec<$type> = decode_borrowed_from_slice(&$bytes[..], config::standard()).unwrap();
	};
	($bytes:expr, $head:ident : &mut [$type:ty],  $($tail:tt)*) => {
		let mut $head: Vec<$type> = decode_borrowed_from_slice(&$bytes[..], config::standard()).unwrap();
		sdrad_pull_function_args!($bytes, $($tail)+);
	};
	($bytes:ident, $head:ident : &[$type:ty]) => {   // slice type
		let $head : Vec<$type> = decode_borrowed_from_slice(&$bytes[..], config::standard()).unwrap();
	};
	($bytes:expr, $head:ident : &[$type:ty], $($tail:tt)+) => {
		let $head Vec<$type> = decode_borrowed_from_slice(&$bytes[..], config::standard()).unwrap();
		sdrad_pull_function_args!($bytes, $($tail)+);
	};

	// mutable reference value passing
	($bytes:expr, $head:ident : &mut $var_type:ty) => {
		let mut $head: $var_type = decode_borrowed_from_slice(&$bytes[..], config::standard()).unwrap();
	};

	// mutable reference value passing with tailing
	($bytes:expr, $head:ident : &mut $var_type:ty, $($tail:tt)+) => {
		let mut $head: $var_type =  decode_borrowed_from_slice(&$bytes[..], config::standard()).unwrap();
		sdrad_pull_function_args!($bytes, $($tail)+);
	};

	// reference value passing
	($bytes:expr, $head:ident : &$var_type:ty) => {
		let $head: $var_type = decode_borrowed_from_slice(&$bytes[..], config::standard()).unwrap();
	};


	($bytes:expr, $head:ident : &$var_type:ty, $($tail:tt)+) => {
		let $head: $var_type = decode_borrowed_from_slice(&$bytes[..], config::standard()).unwrap();
		sdrad_pull_function_args!($bytes, $($tail)+);
	};

	($bytes:expr, $head:ident : $var_type:ty, $($tail:tt)+) => {
		let $head: $var_type = decode_borrowed_from_slice(&$bytes[..], config::standard()).unwrap();
		sdrad_pull_function_args!($bytes, $($tail)+);
	};

	($bytes:expr, $head:ident : $var_type:ty ) => {
		let $head: $var_type = decode_borrowed_from_slice(&$bytes[..], config::standard()).unwrap();
	};

	($bytes:expr, mut $head:ident : $var_type:ty ) => {
		let mut $head: $var_type = decode_borrowed_from_slice(&$bytes[..], config::standard()).unwrap();
	};

	($bytes:expr, mut $head:ident : $var_type:ty, $($tail:tt)+) => {
		let mut $head: $var_type = decode_borrowed_from_slice(&$bytes[..], config::standard()).unwrap();
		sdrad_pull_function_args!($bytes, $($tail)+);
	};

	($bytes:expr, ) => {};
}

#[macro_export]
#[cfg(feature = "bincode_v2")]
macro_rules! sdrad_collect_ret {
    ($bytes:expr, has_ret,  $rettype:ty) => {{
        let retval: $rettype = decode_borrowed_from_slice(&$bytes[..], config::standard()).unwrap();
        retval
    }};

    ($bytes:expr, no_ret,  $rettype:ty) => {{}};

    (no_ret) => {};
}

#[macro_export]
#[cfg(feature = "bincode_v2")]
macro_rules! sdrad_collect_ret_try {
	($udi:expr, $rsp:ident, fn $f:ident($($x:tt)*) ->$rettype:ty) => {{
		if $rsp.is_null() {
			let rsp :i64 = sdrad_get_stack_base_address($udi);
			$rsp = rsp as *mut i64;
		}
		let mut sr = StackBufReader::new($rsp as *mut c_void).unwrap();
		let ret_cap = sr.retrieve::<i64>();
		let ret_len = sr.retrieve::<i64>();
		let ret_address =  sr.retrieve::<i64>();
		let ret_address_ptr = ret_address as *mut i64;
		let mut rebuilt = Vec::from_raw_parts_in(ret_address_ptr as *mut u8, ret_len as usize, ret_cap as usize,
												SdradAllocator{ data_domain_id: $udi} );
		let mut bytes = &mut rebuilt;
		sdrad_restore_changed_vars_global!(&mut bytes, $($x)*);
		let retval :$rettype = decode_borrowed_from_slice(&bytes[..], config::standard()).unwrap();
		retval
	}};

	($udi:expr, $rsp:ident, fn $f:ident($($x:tt)*)) => {{}
	};

}

#[macro_export]
#[cfg(feature = "bincode_v2")]
macro_rules! sdrad_push_args {
	($udi:expr, $rsp_ptr:ident, fn $f:ident($($x:tt)+)) => {{
		let mut vec = Vec::new_in(SdradAllocatorFake{ data_domain_id: $udi});
		sdrad_push_function_args!(vec, $($x)*);
		let rsp :i64 = sdrad_get_stack_base_address($udi);
		$rsp_ptr = rsp as *mut i64;
		let mut sw = StackBufWriter::new($rsp_ptr as *mut c_void).unwrap();
		sw.put(vec.capacity() as i64);
		sw.put(vec.len() as i64);
		sw.put(vec.as_mut_ptr() as i64);
		sdrad_set_stack_offset($udi, 4*size_of::<i64>() as u64);
	}};
	($udi:expr, $rsp_ptr:ident, fn $f:ident()) => {{
	}};
}

// run the function inside nested domain
#[macro_export]
#[cfg(feature = "bincode_v2")]
macro_rules! sdrad_run_function {
	($udi:expr, $rsp_ptr:ident, fn $f:ident($($x:tt)*) -> $rettype:ty)  => {
		let retval : $rettype = paste!{sdrad_strip_types!([<__real_$f>]($($x)*))};
		let mut vec = Vec::new_in(SdradAllocatorFake { data_domain_id: $udi});
		encode_into_std_write(&retval, &mut vec,config::standard());
		sdrad_store_changed_vars_global!(&mut vec, $($x)*);
		let mut sw = StackBufWriter::new($rsp_ptr as *mut c_void).unwrap();
		sw.put(vec.capacity() as i64);
		sw.put(vec.len() as i64);
		sw.put(vec.as_mut_ptr() as i64);
	};
	// no ret value
	($udi:expr, $rsp_ptr:ident, fn $f:ident($($x:tt)*))  => {
		paste!{sdrad_strip_types!([<__real_$f>]($($x)*))};
	};
}

#[macro_export]
#[cfg(feature = "bincode_v2")]
macro_rules! sdrad_pull_args_run {
	($udi:expr, fn $f:ident($($x:tt)+) $(->$rettype:ty)? ) => {{
		let rsp :i64 = sdrad_get_stack_base_address($udi);
		let mut rsp_ptr = rsp as *mut i64;
		let mut sr = StackBufReader::new(rsp_ptr as *mut c_void).unwrap();
		let capacity = sr.retrieve::<i64>();
		let len = sr.retrieve::<i64>();
		let address =  sr.retrieve::<i64>();
		let address_ptr = address as *mut i64;
		let mut rebuilt = Vec::from_raw_parts_in(address_ptr as *mut u8, len as usize, capacity as usize, SdradAllocator { data_domain_id: $udi} );
		let mut bytes = &mut rebuilt;
		sdrad_pull_function_args!(bytes, $($x)*);
		sdrad_run_function!($udi, rsp_ptr, fn $f($($x)*) $(->$rettype)? );
	}};

	($udi:expr, fn $f:ident() $(->$rettype:ty)?) => {{
		let rsp :i64 = sdrad_get_stack_base_address($udi);
		let mut rsp_ptr = rsp as *mut i64;
		sdrad_run_function!($udi, rsp_ptr, fn $f() $(->$rettype)?);
	}};

}
