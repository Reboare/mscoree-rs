#![feature(trace_macros, macro_vis_matcher, macro_literal_matcher)]
// lib.rs - MIT License
//  MIT License
//  Copyright (c) 2018 Tyler Laing (ZerothLaw)
// 
//  Permission is hereby granted, free of charge, to any person obtaining a copy
//  of this software and associated documentation files (the "Software"), to deal
//  in the Software without restriction, including without limitation the rights
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
// 
//  The above copyright notice and this permission notice shall be included in all
//  copies or substantial portions of the Software.
// 
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//  SOFTWARE.

extern crate winapi;

extern crate mscorlib_safe;
extern crate mscorlib_sys;
extern crate mscoree_sys;

#[macro_use] mod macros;

pub mod host;
pub mod metahost;
pub mod wrappers;

/*
steps:
    get metahost
    get runtime info for desired version
    get clr runtime host
    start clr host
        register host control impl
    get host control
    get domain manager
    get app domain
    app domain load assembly
fn get_app_domain() { 
		use std::env;
		println!("CWD = {:?}", env::current_dir());
		let host = MetaHost::new();
		let runtime_info = host.get_runtime_info("v4.0.30319");
		let mut clr_host = match runtime_info.get_clr_host() {
			Ok(new_host) => new_host, 
			Err(hr) => { panic!("call failed with HRESULT: {:?}", hr); }
		};
		assert_eq!(clr_host.start(), true);
		let host = match clr_host.get_host_control() {
			Some(hst) => hst, 
			None => {
				panic!("get_host_control call failed");
			}
		};
		let domain_manager = host.get_domain_manager();
		let app_domain = domain_manager.app_domain("HappyFunTimes");
		match app_domain {
			Ok(apd) => {
				let asm = apd.load_assembly("TestAssembly, Version=1.0.0.0, Culture=neutral, PublicKeyToken=c97610437c81cba6, processorArchitecture=MSIL");
				match asm {
					Ok(mut asm) => {
						println!("Loaded assembly!"); 
						assert!(true);
						let types = asm.get_types();
						match types {
							Ok(mut types) => {
								let types: Vec<Type> = types.into_iter().map(|mut typ| {
									println!("{:?}", typ.name());
									typ.construct(&[]);
									typ
								}).collect();
								println!("types: {:?}", types)
							}, 
							Err(ex) => panic!(ex)
						}
					}, 
					Err(hr) => {println!("Assembly not loaded with HRESULT = {:?}", hr); assert!(false);}
				}
			}, 
			Err(hr) => {
				println!("AppDomain not loaded with HRESULT = {:?}", hr); 
				assert!(false);
			}
		}

	}*/