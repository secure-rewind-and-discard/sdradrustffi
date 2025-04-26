//  @author Merve Gulmez (merve.gulmez@ericsson.com)
//  @version 0.1
//  @date 2023-08-31
//  @copyright © Ericsson AB 2023
//  SPDX-License-Identifier: BSD 3-Clause

#[macro_export]
#[cfg(any(feature = "abomonation_v1", feature = "abomonation_v2"))]
macro_rules! sdrad_push_function_args {
	($buf:expr, $head:ident : &mut &[$type:ty]) => {
		encode(&*$head.to_vec(), &mut $buf);
	};

	($buf:expr, $head:ident : &mut &[$type:ty], $($tail:tt)+) => {
		encode(&*$head.to_vec(), &mut $buf);
		sdrad_push_function_args!($buf:expr, $($tail)+);
	};

	($buf:expr, $head:ident : &[$type:ty]) => {
		encode(&$head.to_vec(), &mut $buf);
	};

	($buf:expr, $head:ident : &[$type:ty], $($tail:tt)+) => {
		encode(&$head.to_vec(), &mut $buf);
		sdrad_push_function_args!($buf, $($tail)+);
	};
	/*str type*/
	($buf:expr, $head:ident:str) => {
		encode(&$head.to_string(), &mut $buf);
	};

	($buf:expr, $head:ident : &str, $($tail:tt)+) => {
		encode(&$head.as_bytes().to_vec(), &mut $buf);
		sdrad_push_function_args!($buf, $($tail)+);
	};

	// mutable reference passing
	($buf:expr, $head:ident : &mut $var_type:ty) => {
		encode(&*$head, &mut $buf);
	};
	// mutable reference passing with tailing
	($buf:expr, $head:ident : &mut $var_type:ty, $($tail:tt)+) => {
		encode(&*$head, &mut $buf);
		sdrad_push_function_args!($buf, $($tail)+);
	};
	// reference value passing
	($buf:expr, $head:ident : &$var_type:ty) => {
		encode($head, &mut $buf);
	};
	// reference value passing with tailing
	($buf:expr, $head:ident : &$var_type:ty, $($tail:tt)+) => {
		encode($head, &mut $buf);
		sdrad_push_function_args!($buf, $($tail)+);
	};

	($buf:expr, $head:ident : $var_type:ty, $($tail:tt)*) => {
		encode(&$head, &mut $buf);
		sdrad_push_function_args!($buf, $($tail)+);
	};
	// one argument passing
	($buf:expr, $head:ident : $var_type:ty ) => {
		encode(&$head, &mut $buf);
	};
	($buf:expr, mut $head:ident : $var_type:ty ) => {
		encode(&$head, &mut $buf);
	};
	($buf:expr, mut $head:ident : $var_type:ty, $($tail:tt)+) => {
		encode($head, &mut $buf);
		sdrad_push_function_args!($buf, $($tail)+);
	};
	($buf:expr, ) => {};
}

#[macro_export]
#[cfg(any(feature = "abomonation_v1", feature = "abomonation_v2"))]
macro_rules! sdrad_store_changed_vars_global {
	/* [u8] */
	($buf:expr, $head:ident : &mut &[$type:ty]) => {
		encode(&*$head.to_vec(), &mut $buf);
	};

	($buf:expr, $head:ident : &mut &[$type:ty], $($tail:tt)+) => {
		encode(&*$head.to_vec(), &mut $buf);
		sdrad_store_changed_vars_global!($buf, $($tail)+);
	};

	/* [u8] */
	($buf:expr, $head:ident : &[$type:ty]) => {

	};

	($buf:expr, $head:ident : &[$type:ty], $($tail:tt)+) => {
		sdrad_store_changed_vars_global!($buf, $($tail)+);
	};

	// mutable reference passing
	($buf:expr, $head:ident : &mut $var_type:ty) => {
		encode(&*$head, &mut $buf);
	};
	// mutable reference passing with tailing
	($buf:expr, $head:ident : &mut $var_type:ty, $($tail:tt)+) => {
		encode(&*$head, &mut $buf);
		sdrad_store_changed_vars_global!($buf, $($tail)+);
	};
	/*non mutable reference we don't need to do anything */
	($buf:expr, $head:ident : &$var_type:ty) => {

	};
	/*check the other type*/
	($buf:expr, $head:ident : &$var_type:ty, $($tail:tt)+) => {
		sdrad_store_changed_vars_global!($buf, $($tail)+);
	};

	/*non mutable reference */
	($buf:expr, $head:ident : $var_type:ty, $($tail:tt)+) => {
		sdrad_store_changed_vars_global!($buf, $($tail)+);
	};

	($buf:expr, $head:ident : $var_type:ty ) => {

	};

	($buf:expr, mut $head:ident : $var_type:ty ) => {

	};

	($buf:expr, mut $head:ident : $var_type:ty, $($tail:tt)+) => {
		sdrad_store_changed_vars_global!($buf, $($tail)+);
	};

	($buf:expr, ) => {};
}

#[macro_export]
#[cfg(any(feature = "abomonation_v1", feature = "abomonation_v2"))]
macro_rules! sdrad_restore_changed_vars_global {
	($buf:expr, $head:ident : &mut &[$type:ty]) => {
		let (a, remaining) = decode::<Vec<$type>>($buf).unwrap();
		*$head: Vec<$type> = &a[..];
	};

	($buf:expr, $head:ident : &mut &[$type:ty], $($tail:tt)+) => {
		let (a, remaining) = decode::<Vec<$type>>($buf).unwrap();
		*$head = &a[..];
		sdrad_restore_changed_vars_global!($buf:expr, $($tail)+);
	};

	($buf:ident, $head:ident : &mut $var_type:ty) => {
		let (a, r) =  decode::<$var_type>($buf).unwrap();
		*$head = a.clone();
	};

	($buf:ident, $head:ident : &mut $var_type:ty, $($tail:tt)+) => {
		let (a, r) =  decode::<$var_type>($buf).unwrap();
		*$head = a.clone();
		sdrad_restore_changed_vars_global!($buf, $($tail)+);
	};

	($buf:ident, $head:ident : &$var_type:ty) => {

	};
	($buf:ident, $head:ident : &$var_type:ty, $($tail:tt)+) => {
		sdrad_restore_changed_vars_global!($buf, $($tail)+);
	};

	($buf:ident, $head:ident : $var_type:ty, $($tail:tt)+) => {
		sdrad_restore_changed_vars_global!($buf, $($tail)+);
	};

	($buf:ident, mut $head:ident : $var_type:ty ) => {

	};
	($buf:ident, mut $head:ident : $var_type:ty, $($tail:tt)+) => {
		sdrad_restore_changed_vars_global!($buf, $($tail)+);
	};
	($buf:ident, $head:ident : $var_type:ty ) => {

	};
	($buf:ident, ) => {

	};
}

#[macro_export]
#[cfg(any(feature = "abomonation_v1", feature = "abomonation_v2"))]
macro_rules! sdrad_pull_function_args {
	($buf:expr, $head:ident : &mut [$type:ty]) => {
		let (a, remaining) = decode::<Vec<$type>>($buf).unwrap();
		let mut $head = &a[..];
	};
	($buf:expr, $head:ident : &mut [$type:ty],  $($tail:tt)*) => {
		let (a, r) = decode::<Vec<$type>>($buf).unwrap();
		let mut $head: Vec<$type> = &a[..];
		sdrad_pull_function_args!(r, $($tail)+);
	};
	($buf:ident, $head:ident : &[$type:ty]) => {   // slice type
		let (a, r) =  decode::<Vec<$type>>($buf).unwrap();
		let $head = &a[..];
		//$buf = &mut r;
	};

	($buf:expr, $head:ident : &str, $($tail:tt)+) => {
		let (a, r) =  decode::<String>($buf).unwrap();
		let $head : &str = a;
		sdrad_pull_function_args!(r, $($tail)+);
	};

	($buf:expr, $head:ident : &[$type:ty], $($tail:tt)+) => {
		let (a, r) =  decode::<Vec<$type>>(&mut $buf).unwrap();
		let $head = &a[..];
		sdrad_pull_function_args!(r, $($tail)+);
	};

	// mutable reference value passing
	($buf:expr, $head:ident : &mut $var_type:ty) => {
		let (a, r) =  decode::<$var_type>($buf).unwrap();
		let mut $head: $var_type = a.clone();
	};

	// mutable reference value passing with tailing
	($buf:expr, $head:ident : &mut $var_type:ty, $($tail:tt)+) => {
		let (a, r) =  decode::<$var_type>( $buf).unwrap();
		let mut $head: $var_type = a;
		sdrad_pull_function_args!(r, $($tail)+);
	};

	// reference value passing
	($buf:expr, $head:ident : &$var_type:ty) => {
		let (a, r) =  decode::<$var_type>($buf).unwrap();
		let $head: $var_type = a.clone();
	};

	($buf:expr, $head:ident : &$var_type:ty, $($tail:tt)+) => {
		let (a, r) =  decode::<$var_type>( $buf).unwrap();
		let $head: $var_type = *a;
		sdrad_pull_function_args!(r, $($tail)+);
	};

	($buf:expr, $head:ident : $var_type:ty, $($tail:tt)+) => {
		let (a, r) = decode::<$var_type>( $buf).unwrap();
		let $head: $var_type = *a;
		sdrad_pull_function_args!(r, $($tail)+);
	};

	($buf:expr, $head:ident : $var_type:ty ) => {
		let (a, r) = decode::<$var_type>( $buf).unwrap();
		let $head: $var_type = *a;
	};

	($buf:expr, mut $head:ident : $var_type:ty ) => {
		let (a, r) = decode::<$var_type>( $buf).unwrap();
		let mut $head: $var_type = *a;
	};

	($buf:expr, mut $head:ident : $var_type:ty, $($tail:tt)+) => {
		let (a, r) = decode::<$var_type>( $buf).unwrap();
		let mut $head: $var_type = a;
		sdrad_pull_function_args!(r, $($tail)+);
	};

	($buf:expr, ) => {};
}

#[macro_export]
#[cfg(any(feature = "abomonation_v1", feature = "abomonation_v2"))]
macro_rules! sdrad_collect_ret {
    ($buf:expr, has_ret,  $rettype:ty) => {{
        let (a, r) = decode::<$rettype>($buf).unwrap();
        let retval: $rettype = *a.clone();
        retval
    }};

    ($buf:expr, no_ret,  $rettype:ty) => {{}};

    (no_ret) => {};
}

#[macro_export]
#[cfg(any(feature = "abomonation_v1", feature = "abomonation_v2"))]
macro_rules! sdrad_collect_ret_try {
	($udi:expr, $buf:ident, fn $f:ident($($x:tt)*) ->$rettype:ty) => {{
		if $buf.is_null() {
			let rsp :i64 = sdrad_get_stack_base_address($udi);
			$buf = rsp as *mut i64;
		}
		let mut sr = StackBufReader::new($buf as *mut c_void).unwrap();
		let ret_cap = sr.retrieve::<i64>();
		let ret_len = sr.retrieve::<i64>();
		let ret_address =  sr.retrieve::<i64>();
		let ret_address_ptr = ret_address as *mut i64;
		let mut rebuilt = Vec::from_raw_parts_in(ret_address_ptr as *mut u8, ret_len as usize, ret_cap as usize,
												SdradAllocator{ data_domain_id: $udi} );
		let mut bytes = &mut rebuilt;
		sdrad_restore_changed_vars_global!(bytes, $($x)*);
		let (a, r) = decode::<$rettype>(bytes).unwrap();
		let retval :$rettype = a.clone();
		retval
		// let (a, r) = decode::<$rettype>(bytes).unwrap();
		// let retval : $rettype = a;
		// retval
	}};

	($udi:expr, $buf:ident, fn $f:ident($($x:tt)*)) => {{}
	};

}

#[macro_export]
#[cfg(any(feature = "abomonation_v1", feature = "abomonation_v2"))]
macro_rules! sdrad_push_args {
	($udi:expr, $buf_ptr:ident, fn $f:ident($($x:tt)+)) => {{
		let mut vec = Vec::new_in(SdradAllocatorFake{ data_domain_id: $udi});
		sdrad_push_function_args!(vec, $($x)*);
		let rsp :i64 = sdrad_get_stack_base_address($udi);
		$buf_ptr = rsp as *mut i64;
		let mut sw = StackBufWriter::new($buf_ptr as *mut c_void).unwrap();
		sw.put(vec.capacity() as i64);
		sw.put(vec.len() as i64);
		sw.put(vec.as_mut_ptr() as i64);
		sdrad_set_stack_offset($udi, 4*size_of::<i64>() as u64);
	}};
	($udi:expr, $buf_ptr:ident, fn $f:ident()) => {{
	}};
}

// run the function inside nested domain
#[macro_export]
#[cfg(any(feature = "abomonation_v1", feature = "abomonation_v2"))]
macro_rules! sdrad_run_function {
	($udi:expr, $rsp_ptr:expr, fn $f:ident() -> $rettype:ty)  => {
		let retval : $rettype = paste!{sdrad_strip_types!([<__real_$f>]())};
		let mut vec = Vec::new_in(SdradAllocatorFake { data_domain_id: $udi});
		encode(&retval, &mut vec);
		let mut sw = StackBufWriter::new($rsp_ptr as *mut c_void).unwrap();
		sw.put(vec.capacity() as i64);
		sw.put(vec.len() as i64);
		sw.put(vec.as_mut_ptr() as i64);
	};

	// ret value
	($udi:expr, $rsp_ptr:expr, fn $f:ident($($x:tt)*) -> $rettype:ty)  => {
		let retval : $rettype = paste!{sdrad_strip_types!([<__real_$f>]($($x)*))};
		let mut vec = Vec::new_in(SdradAllocatorFake { data_domain_id: $udi});
		encode(&retval, &mut vec);
		sdrad_store_changed_vars_global!(&mut vec, $($x)*);
		let mut sw = StackBufWriter::new($rsp_ptr as *mut c_void).unwrap();
		sw.put(vec.capacity() as i64);
		sw.put(vec.len() as i64);
		sw.put(vec.as_mut_ptr() as i64);
	};
	// no ret value
	($udi:expr, $rsp_ptr:expr, fn $f:ident($($x:tt)*))  => {
		paste!{sdrad_strip_types!([<__real_$f>]($($x)*))};
		let mut vec = Vec::new_in(SdradAllocatorFake { data_domain_id: $udi});
		vec.push(1); // todo
		sdrad_store_changed_vars_global!(&mut vec, $($x)*);
		if vec.capacity() > 0 {
			let mut sw = StackBufWriter::new($rsp_ptr as *mut c_void).unwrap();
			sw.put(vec.capacity() as i64);
			sw.put(vec.len() as i64);
			sw.put(vec.as_mut_ptr() as i64);
		}
	};
	// no ret value
	($udi:expr, fn $f:ident($($x:tt)*))  => {
		paste!{sdrad_strip_types!([<__real_$f>]($($x)*))};
	};

}

#[macro_export]
#[cfg(any(feature = "abomonation_v1", feature = "abomonation_v2"))]
macro_rules! sdrad_pull_args_run {
	($udi:expr, fn $f:ident()) => {{
		sdrad_run_function!($udi, fn $f());
	}};

	($udi:expr, fn $f:ident() $(->$rettype:ty)?) => {{
		let rsp :i64 = sdrad_get_stack_base_address($udi);
		let mut rsp_ptr = rsp as *mut i64;
		sdrad_run_function!($udi, rsp_ptr, fn $f() $(->$rettype)?);
	}};


	($udi:expr, fn $f:ident($($x:tt)*) $(->$rettype:ty)? ) => {{
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

}
