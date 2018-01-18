use libc::pid_t;
use proc_maps::MapRange;
use copy;
use address_finder::StackFrame;

pub fn is_maybe_thread_function(
    version: &str,
) -> Box<Fn(usize, pid_t, &MapRange, &Vec<MapRange>) -> bool> {
    let function = match version.as_ref() {
        "1.9.1" => self::ruby_1_9_1_0::is_maybe_thread,
        "1.9.2" => self::ruby_1_9_2_0::is_maybe_thread,
        "1.9.3" => self::ruby_1_9_3_0::is_maybe_thread,
        "2.0.0" => self::ruby_2_0_0_0::is_maybe_thread,
        "2.1.0" => self::ruby_2_1_0::is_maybe_thread,
        "2.1.1" => self::ruby_2_1_1::is_maybe_thread,
        "2.1.2" => self::ruby_2_1_2::is_maybe_thread,
        "2.1.3" => self::ruby_2_1_3::is_maybe_thread,
        "2.1.4" => self::ruby_2_1_4::is_maybe_thread,
        "2.1.5" => self::ruby_2_1_5::is_maybe_thread,
        "2.1.6" => self::ruby_2_1_6::is_maybe_thread,
        "2.1.7" => self::ruby_2_1_7::is_maybe_thread,
        "2.1.8" => self::ruby_2_1_8::is_maybe_thread,
        "2.1.9" => self::ruby_2_1_9::is_maybe_thread,
        "2.1.10" => self::ruby_2_1_10::is_maybe_thread,
        "2.2.0" => self::ruby_2_2_0::is_maybe_thread,
        "2.2.1" => self::ruby_2_2_1::is_maybe_thread,
        "2.2.2" => self::ruby_2_2_2::is_maybe_thread,
        "2.2.3" => self::ruby_2_2_3::is_maybe_thread,
        "2.2.4" => self::ruby_2_2_4::is_maybe_thread,
        "2.2.5" => self::ruby_2_2_5::is_maybe_thread,
        "2.2.6" => self::ruby_2_2_6::is_maybe_thread,
        "2.2.7" => self::ruby_2_2_7::is_maybe_thread,
        "2.2.8" => self::ruby_2_2_8::is_maybe_thread,
        "2.2.9" => self::ruby_2_2_9::is_maybe_thread,
        "2.3.0" => self::ruby_2_3_0::is_maybe_thread,
        "2.3.1" => self::ruby_2_3_1::is_maybe_thread,
        "2.3.2" => self::ruby_2_3_2::is_maybe_thread,
        "2.3.3" => self::ruby_2_3_3::is_maybe_thread,
        "2.3.4" => self::ruby_2_3_4::is_maybe_thread,
        "2.3.5" => self::ruby_2_3_5::is_maybe_thread,
        "2.3.6" => self::ruby_2_3_6::is_maybe_thread,
        "2.4.0" => self::ruby_2_4_0::is_maybe_thread,
        "2.4.1" => self::ruby_2_4_1::is_maybe_thread,
        "2.4.2" => self::ruby_2_4_2::is_maybe_thread,
        "2.4.3" => self::ruby_2_4_3::is_maybe_thread,
        "2.5.0" => self::ruby_2_5_0_rc1::is_maybe_thread,
        _ => panic!("oh no"),
    };
    Box::new(function)
}

pub fn get_stack_trace_function(
    version: &str,
) -> Box<Fn(usize, pid_t) -> Result<Vec<StackFrame>, copy::MemoryCopyError>> {
    let stack_trace_function = match version {
        "1.9.1" => self::ruby_1_9_1_0::get_stack_trace,
        "1.9.2" => self::ruby_1_9_2_0::get_stack_trace,
        "1.9.3" => self::ruby_1_9_3_0::get_stack_trace,
        "2.0.0" => self::ruby_2_0_0_0::get_stack_trace,
        "2.1.0" => self::ruby_2_1_0::get_stack_trace,
        "2.1.1" => self::ruby_2_1_1::get_stack_trace,
        "2.1.2" => self::ruby_2_1_2::get_stack_trace,
        "2.1.3" => self::ruby_2_1_3::get_stack_trace,
        "2.1.4" => self::ruby_2_1_4::get_stack_trace,
        "2.1.5" => self::ruby_2_1_5::get_stack_trace,
        "2.1.6" => self::ruby_2_1_6::get_stack_trace,
        "2.1.7" => self::ruby_2_1_7::get_stack_trace,
        "2.1.8" => self::ruby_2_1_8::get_stack_trace,
        "2.1.9" => self::ruby_2_1_9::get_stack_trace,
        "2.1.10" => self::ruby_2_1_10::get_stack_trace,
        "2.2.0" => self::ruby_2_2_0::get_stack_trace,
        "2.2.1" => self::ruby_2_2_1::get_stack_trace,
        "2.2.2" => self::ruby_2_2_2::get_stack_trace,
        "2.2.3" => self::ruby_2_2_3::get_stack_trace,
        "2.2.4" => self::ruby_2_2_4::get_stack_trace,
        "2.2.5" => self::ruby_2_2_5::get_stack_trace,
        "2.2.6" => self::ruby_2_2_6::get_stack_trace,
        "2.2.7" => self::ruby_2_2_7::get_stack_trace,
        "2.2.8" => self::ruby_2_2_8::get_stack_trace,
        "2.2.9" => self::ruby_2_2_9::get_stack_trace,
        "2.3.0" => self::ruby_2_3_0::get_stack_trace,
        "2.3.1" => self::ruby_2_3_1::get_stack_trace,
        "2.3.2" => self::ruby_2_3_2::get_stack_trace,
        "2.3.3" => self::ruby_2_3_3::get_stack_trace,
        "2.3.4" => self::ruby_2_3_4::get_stack_trace,
        "2.3.5" => self::ruby_2_3_5::get_stack_trace,
        "2.3.6" => self::ruby_2_3_6::get_stack_trace,
        "2.4.0" => self::ruby_2_4_0::get_stack_trace,
        "2.4.1" => self::ruby_2_4_1::get_stack_trace,
        "2.4.2" => self::ruby_2_4_2::get_stack_trace,
        "2.4.3" => self::ruby_2_4_3::get_stack_trace,
        "2.5.0" => self::ruby_2_5_0_rc1::get_stack_trace,
        _ => panic!("oh no"),
    };
    Box::new(stack_trace_function)
}

macro_rules! ruby_bindings_v_1_9_x(
    ($ruby_version:ident) => (
        pub mod $ruby_version {
            use std;
            use copy::*;
            use bindings::$ruby_version::*;
            use libc::pid_t;
            use copy::MemoryCopyError;

            get_stack_trace!(rb_thread_struct);
            get_ruby_string!();
            get_cfps!();
            get_label_and_path_1_9_0!();
            is_stack_base_1_9_0!();
        }
        ));

macro_rules! ruby_bindings(
    ($ruby_version:ident) => (
        mod $ruby_version {
            use std;
            use copy::*;
            use bindings::$ruby_version::*;
            use libc::pid_t;
            use copy::MemoryCopyError;


            // These 4 functions are the
            // core of how the program works. They're essentially a straight port of
            // this gdb script:
            // https://gist.github.com/csfrancis/11376304/raw/7a0450d11e64e3bb7c982b7ad2778f3603188c0f/gdb_ruby_backtrace.py
            // except without using gdb!!
            //
            // `get_cfps` corresponds to
            // (* const rb_thread_struct *(ruby_current_thread_address_location))->cfp
            //
            // `get_ruby_string` is doing ((Struct RString *) address) and then
            // trying one of two ways to get the actual Ruby string out depending
            // on how it's stored
            get_stack_trace!(rb_thread_struct);
            get_ruby_string!();
            get_cfps!();
            get_lineno_2_0_0!();
            get_label_and_path_2_0_0!();
            is_stack_base_1_9_0!();
        }
));

macro_rules! ruby_bindings_v2(
    ($ruby_version:ident) => (
        mod $ruby_version {
            use std;
            use copy::*;
            use bindings::$ruby_version::*;
            use libc::pid_t;
            use copy::MemoryCopyError;

            get_stack_trace!(rb_thread_struct);
            get_ruby_string!();
            get_cfps!();
            get_lineno_2_3_0!();
            get_label_and_path_2_3_0!();
            is_stack_base_1_9_0!();
        }
        ));

macro_rules! ruby_bindings_v2_5_x(
    ($ruby_version:ident) => (
        mod $ruby_version {
            use std;
            use copy::*;
            use bindings::$ruby_version::*;
            use libc::pid_t;
            use copy::MemoryCopyError;

            get_stack_trace!(rb_execution_context_struct);
            get_ruby_string!();
            get_cfps!();
            get_lineno_2_5_0!();
            get_label_and_path_2_5_0!();
            is_stack_base_2_5_0!();
            get_ruby_string_array_2_5_0!();
        }
        ));

macro_rules! get_stack_trace(
    ($thread_type:ident) => (

        use stack_trace::StackFrame;

        pub fn get_stack_trace(
            ruby_current_thread_address_location: usize,
            source_pid: pid_t,
            ) -> Result<Vec<StackFrame>, MemoryCopyError> {
            debug!(
                "current address location: {:x}",
                ruby_current_thread_address_location
                );
            let current_thread_addr: usize =
                copy_struct(ruby_current_thread_address_location, source_pid)?;
            debug!("{:x}", current_thread_addr);
            let thread: $thread_type = copy_struct(current_thread_addr, source_pid)?;
            debug!("thread: {:?}", thread);
            let mut trace = Vec::new();
            let cfps = get_cfps(thread.cfp as usize, stack_base(&thread) as usize, source_pid)?;
            for cfp in cfps.iter() {
                if cfp.iseq as usize == 0  || cfp.pc as usize == 0 {
                    debug!("huh."); // TODO: fixmeup
                    continue;
                }
                let iseq_struct: rb_iseq_struct = copy_struct(cfp.iseq as usize, source_pid)?;
                debug!("iseq_struct: {:?}", iseq_struct);
                let label_path  = get_label_and_path(&iseq_struct, &cfp, source_pid);
                match label_path {
                    Ok(call)  => trace.push(call),
                    Err(x) => {
                        // this is a heuristic: the intent of this is that it skips function calls into C extensions
                        if trace.len() > 0 {
                            debug!("guess that one didn't work; skipping");
                        } else {
                            return Err(x);
                        }
                    }
                }
            }
            Ok(trace)
        }

use proc_maps::{maps_contain_addr, MapRange};

pub fn is_maybe_thread(x: usize, pid: pid_t, heap_map: &MapRange, all_maps: &Vec<MapRange>) -> bool {
    if !heap_map.contains_addr(x) {
        return false;
    }

    let thread: $thread_type = match copy_struct(x, pid) {
        Ok(x) => x,
        _ => { return false; },
    };

    if !is_reasonable_thing(&thread, all_maps) {
        return false;
    }

    let stack_base = stack_base(&thread);
    let diff = stack_base - thread.cfp as i64;
    debug!("diff: {}", diff);
    if diff < 0 || diff > 3000000 {
        return false;
    }

    return true;
}
));

macro_rules! is_stack_base_1_9_0(
    () => (
        fn is_reasonable_thing(thread: &rb_thread_struct,  all_maps: &Vec<MapRange>) -> bool {
            maps_contain_addr(thread.vm as usize, all_maps) &&
                maps_contain_addr(thread.cfp as usize, all_maps) &&
                maps_contain_addr(thread.stack as usize, all_maps) &&
                maps_contain_addr(thread.self_ as usize, all_maps) &&
                thread.stack_size < 3000000 && thread.state >= 0
        }

        fn stack_base(thread: &rb_thread_struct) -> i64 {
            thread.stack as i64 + thread.stack_size as i64 * std::mem::size_of::<VALUE>() as i64 - 1 * std::mem::size_of::<rb_control_frame_t>() as i64
        }
        ));

macro_rules! is_stack_base_2_5_0(
    () => (
        fn is_reasonable_thing(thread: &rb_execution_context_struct, all_maps: &Vec<MapRange>) -> bool {
            maps_contain_addr(thread.tag as usize, all_maps) &&
                maps_contain_addr(thread.cfp as usize, all_maps) &&
                maps_contain_addr(thread.vm_stack as usize, all_maps) &&
                thread.vm_stack_size < 3000000
        }

        fn stack_base(thread: &rb_execution_context_struct) -> i64 {
            thread.vm_stack as i64 + thread.vm_stack_size as i64 * std::mem::size_of::<VALUE>() as i64 - 1 * std::mem::size_of::<rb_control_frame_t>() as i64
        }
        ));

macro_rules! get_ruby_string_array_2_5_0(
    () => (
        fn get_ruby_string_array(addr: usize, string_class: usize, source_pid: pid_t) -> Result<String, MemoryCopyError> {
            // todo: we're doing an extra copy here for no reason
            let rstring: RString = copy_struct(addr, source_pid)?;
            if rstring.basic.klass as usize == string_class {
                return get_ruby_string(addr, source_pid);
            }
            // otherwise it's an RArray
            let rarray: RArray = copy_struct(addr, source_pid)?;
            debug!("blah: {}, array: {:?}", addr, unsafe { rarray.as_.ary });
            // TODO: this assumes that the array contents are stored inline and not on the heap
            // I think this will always be true but we should check instead
            // the reason I am not checking is that I don't know how to check yet
            let addr: usize = unsafe { rarray.as_.ary[1] as usize }; // 1 means get the absolute path, not the relative path
            get_ruby_string(addr, source_pid)
        }
        ));

macro_rules! get_ruby_string(
    () => (
        use std::ffi::CStr;

        fn get_ruby_string(addr: usize, source_pid: pid_t) -> Result<String, MemoryCopyError> {
            let vec = {
                let rstring: RString = copy_struct(addr, source_pid)?;
                let basic = rstring.basic;
                let is_array = basic.flags & 1 << 13 == 0;
                if is_array {
                    unsafe { CStr::from_ptr(rstring.as_.ary.as_ref().as_ptr() as *const i8) }
                    .to_bytes()
                        .to_vec()
                } else {
                    unsafe {
                        let addr = rstring.as_.heap.ptr as usize;
                        let len = rstring.as_.heap.len as usize;
                        copy_address_raw(addr as usize, len, source_pid)?
                    }
                }
            };
            Ok(String::from_utf8(vec).map_err(|x| {MemoryCopyError::InvalidStringError(x)})?)
        }
));

macro_rules! get_label_and_path_1_9_0(
    () => (
        fn get_label_and_path(
            iseq_struct: &rb_iseq_struct,
            cfp: &rb_control_frame_t,
            source_pid: pid_t,
            ) -> Result<StackFrame, MemoryCopyError> {
            Ok(StackFrame{
                name: get_ruby_string(iseq_struct.name as usize, source_pid)?,
                path: get_ruby_string(iseq_struct.filename as usize, source_pid)?,
                lineno: None,
            })
        }
        ));

macro_rules! get_lineno_2_0_0(
    () => (
        fn get_lineno(
            iseq_struct: &rb_iseq_struct,
            cfp: &rb_control_frame_t,
            source_pid: pid_t,
            ) -> Result<u32, MemoryCopyError> {
            let mut pos = cfp.pc as usize - iseq_struct.iseq_encoded as usize;
            if pos != 0 {
                pos -= 1;
            }
            let t_size = iseq_struct.line_info_size as usize;
            if t_size == 0 {
                Ok(0) //TODO: really?
            } else if t_size == 1 {
                let table: [iseq_line_info_entry; 1] = copy_struct(iseq_struct.line_info_table as usize, source_pid)?;
                Ok(table[0].line_no)
            } else {
                let table: Vec<iseq_line_info_entry> = copy_vec(iseq_struct.line_info_table as usize, t_size as usize, source_pid)?;
                for i in 0..t_size {
                    if pos == table[i].position as usize {
                        return Ok(table[i].line_no)
                    } else if table[i].position as usize > pos {
                        return Ok(table[i-1].line_no)
                    }
                }
                Ok(table[t_size-1].line_no)
            }
        }
));

macro_rules! get_lineno_2_3_0(
    () => (
        fn get_lineno(
            iseq_struct: &rb_iseq_constant_body,
            cfp: &rb_control_frame_t,
            source_pid: pid_t,
            ) -> Result<u32, MemoryCopyError> {
            if iseq_struct.iseq_encoded as usize > cfp.pc as usize {
                return Err(MemoryCopyError::Other);
            }
            let mut pos = cfp.pc as usize - iseq_struct.iseq_encoded as usize; // TODO: investigate panic here
            if pos != 0 {
                pos -= 1;
            }
            let t_size = iseq_struct.line_info_size as usize;
            if t_size == 0 {
                Ok(0) //TODO: really?
            } else if t_size == 1 {
                let table: [iseq_line_info_entry; 1] = copy_struct(iseq_struct.line_info_table as usize, source_pid)?;
                Ok(table[0].line_no)
            } else {
                let table: Vec<iseq_line_info_entry> = copy_vec(iseq_struct.line_info_table as usize, t_size as usize, source_pid)?;
                for i in 0..t_size {
                    if pos == table[i].position as usize {
                        return Ok(table[i].line_no)
                    } else if table[i].position as usize > pos {
                        return Ok(table[i-1].line_no)
                    }
                }
                Ok(table[t_size-1].line_no)
            }
        }
));

macro_rules! get_lineno_2_5_0(
    () => (
        fn get_lineno(
            iseq_struct: &rb_iseq_constant_body,
            cfp: &rb_control_frame_t,
            source_pid: pid_t,
            ) -> Result<u32, MemoryCopyError> {
            let mut pos = cfp.pc as usize - iseq_struct.iseq_encoded as usize;
            if pos != 0 {
                pos -= 1;
            }
            let t_size = iseq_struct.insns_info_size as usize;
            if t_size == 0 {
                Ok(0) //TODO: really?
            } else if t_size == 1 {
                let table: [iseq_insn_info_entry; 1] = copy_struct(iseq_struct.insns_info as usize, source_pid)?;
                Ok(table[0].line_no as u32)
            } else {
                let table: Vec<iseq_insn_info_entry> = copy_vec(iseq_struct.insns_info as usize, t_size as usize, source_pid)?;
                for i in 0..t_size {
                    if pos == table[i].position as usize {
                        return Ok(table[i].line_no as u32)
                    } else if table[i].position as usize > pos {
                        return Ok(table[i-1].line_no as u32)
                    }
                }
                Ok(table[t_size-1].line_no as u32)
            }
        }
));

macro_rules! get_label_and_path_2_0_0(
    () => (
        fn get_label_and_path(
            iseq_struct: &rb_iseq_struct,
            cfp: &rb_control_frame_t,
            source_pid: pid_t,
            ) -> Result<StackFrame, MemoryCopyError> {
            Ok(StackFrame{
                name: get_ruby_string(iseq_struct.location.label as usize, source_pid)?,
                path: get_ruby_string(iseq_struct.location.path as usize, source_pid)?,
                lineno: Some(get_lineno(iseq_struct, cfp, source_pid)?),
            })
        }
        ));

macro_rules! get_label_and_path_2_3_0(
    () => (
        fn get_label_and_path(
            iseq_struct: &rb_iseq_struct,
            cfp: &rb_control_frame_t,
            source_pid: pid_t,
            ) -> Result<StackFrame, MemoryCopyError> {
            let body: rb_iseq_constant_body = copy_struct(iseq_struct.body as usize, source_pid)?;
            Ok(StackFrame{
                name: get_ruby_string(body.location.label as usize, source_pid)?,
                path: get_ruby_string(body.location.path as usize, source_pid)?,
                lineno: Some(get_lineno(&body, cfp, source_pid)?),
            })
        }
        ));

macro_rules! get_label_and_path_2_5_0(
    () => (
        fn get_label_and_path(
            iseq_struct: &rb_iseq_struct,
            cfp: &rb_control_frame_t,
            source_pid: pid_t,
            ) -> Result<StackFrame, MemoryCopyError> {
            let body: rb_iseq_constant_body = copy_struct(iseq_struct.body as usize, source_pid)?;
            let rstring: RString = copy_struct(body.location.label as usize, source_pid)?;
            Ok(StackFrame{
                name: get_ruby_string(body.location.label as usize, source_pid)?,
                path:  get_ruby_string_array(body.location.pathobj as usize, rstring.basic.klass as usize, source_pid)?,
                lineno: Some(get_lineno(&body, cfp, source_pid)?),
            })
        }
        ));

macro_rules! get_cfps(
    () => (
        // Ruby stack grows down, starting at
        //   ruby_current_thread->stack + ruby_current_thread->stack_size - 1 * sizeof(rb_control_frame_t)
        // I don't know what the -1 is about. Also note that the stack_size is *not* in bytes! stack is a
        // VALUE*, and so stack_size is in units of sizeof(VALUE).
        //
        // The base of the call stack is therefore at
        //   stack + stack_size * sizeof(VALUE) - sizeof(rb_control_frame_t)
        // (with everything in bytes).
        fn get_cfps(cfp_address: usize, stack_base: usize, source_pid: pid_t) -> Result<Vec<rb_control_frame_t>, MemoryCopyError> {
            Ok(copy_vec(cfp_address, (stack_base as usize - cfp_address) as usize / std::mem::size_of::<rb_control_frame_t>(), source_pid)?)
        }
        ));

ruby_bindings_v_1_9_x!(ruby_1_9_1_0);
ruby_bindings_v_1_9_x!(ruby_1_9_2_0);
ruby_bindings_v_1_9_x!(ruby_1_9_3_0);
ruby_bindings!(ruby_2_0_0_0);
ruby_bindings!(ruby_2_1_0);
ruby_bindings!(ruby_2_1_1);
ruby_bindings!(ruby_2_1_2);
ruby_bindings!(ruby_2_1_3);
ruby_bindings!(ruby_2_1_4);
ruby_bindings!(ruby_2_1_5);
ruby_bindings!(ruby_2_1_6);
ruby_bindings!(ruby_2_1_7);
ruby_bindings!(ruby_2_1_8);
ruby_bindings!(ruby_2_1_9);
ruby_bindings!(ruby_2_1_10);
ruby_bindings!(ruby_2_2_0);
ruby_bindings!(ruby_2_2_1);
ruby_bindings!(ruby_2_2_2);
ruby_bindings!(ruby_2_2_3);
ruby_bindings!(ruby_2_2_4);
ruby_bindings!(ruby_2_2_5);
ruby_bindings!(ruby_2_2_6);
ruby_bindings!(ruby_2_2_7);
ruby_bindings!(ruby_2_2_8);
ruby_bindings!(ruby_2_2_9);
ruby_bindings_v2!(ruby_2_3_0);
ruby_bindings_v2!(ruby_2_3_1);
ruby_bindings_v2!(ruby_2_3_2);
ruby_bindings_v2!(ruby_2_3_3);
ruby_bindings_v2!(ruby_2_3_4);
ruby_bindings_v2!(ruby_2_3_5);
ruby_bindings_v2!(ruby_2_3_6);
ruby_bindings_v2!(ruby_2_4_0);
ruby_bindings_v2!(ruby_2_4_1);
ruby_bindings_v2!(ruby_2_4_2);
ruby_bindings_v2!(ruby_2_4_3);
ruby_bindings_v2_5_x!(ruby_2_5_0_rc1);
