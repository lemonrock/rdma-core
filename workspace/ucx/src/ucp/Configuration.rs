// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct Configuration
{
	handle: *mut ucp_config_t,
}

impl Drop for Configuration
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_config_release(self.handle) };
	}
}

impl PrintInformation for Configuration
{
	#[inline(always)]
	fn printInformationToStream(&self, stream: *mut FILE)
	{
		let printFlags = ucs_config_print_flags_t_UCS_CONFIG_PRINT_CONFIG | ucs_config_print_flags_t_UCS_CONFIG_PRINT_DOC | ucs_config_print_flags_t_UCS_CONFIG_PRINT_HEADER | ucs_config_print_flags_t_UCS_CONFIG_PRINT_HIDDEN;
		let title = CString::new("UCP Configuration").expect("Not a valid CStr");
		unsafe { ucp_config_print(self.handle, stream, title.as_ptr(), printFlags) };
	}
}

impl Configuration
{
	#[inline(always)]
	pub fn readFromEnvironmentVariables(environmentVariablePrefix: &str) -> Result<Self, ConfigurationReadError>
	{
		let mut config_p = unsafe { uninitialized() };
		
		let status = if environmentVariablePrefix.is_empty()
		{
			unsafe { ucp_config_read(null(), null(), &mut config_p) }
		}
		else
		{
			let prefix = CString::new(environmentVariablePrefix).expect("Not a valid CStr");
			unsafe { ucp_config_read(prefix.as_ptr(), null(), &mut config_p) }
		};
		
		if likely(status.isOk())
		{
			Ok
			(
				Configuration
				{
					handle: config_p,
				}
			)
		}
		else
		{
			use self::UcpFailure::*;
			use self::UcpPermanentFailureReason::*;
			use self::ConfigurationReadError::*;
			
			let failure = UcpFailure::convertError(status);
			
			match failure
			{
				Permanent(reason) => match reason
				{
					OutOfMemory => panic!("Out of memory"),
					NoTransportDeviceExists => Err(NoTransportDevicesExistThatAreSuitable),
					ElementDoesNotExist => Err(NoTransportDevicesExistThatAreSuitable),
					UnimplementedFunctionality => Err(FunctionalityNotImplementedOrSupported),
					UnsupportedSubSetOfFunctionality => Err(FunctionalityNotImplementedOrSupported),
					_ => panic!("Permanent failure to read configuration from environment variables because '{:?}'", reason),
				},
				_ => panic!("Inappropriate failure for UCP API '{}'", failure),
			}
		}
	}
	
	/// See the static `ucp_config_table` in ucp_context.c for potential values of name and value
	#[inline(always)]
	pub fn modify(&self, name: *const c_char, value: CString)
	{
		panic_on_error!(ucp_config_modify, self.handle, name, value.as_ptr());
		
		
		
		
		
			// better error handling
			// create a list of key names
		//xxxxx
		
		
		
		
		
		
		
	}
	
	/// applicationContextFeaturesIdeallySupported and contextWillBeSharedByMultipleWorkersFromDifferentThreads are programmer choices; how the code will be designed
	/// tagSenderMask and estimatedNumberOfEndPoints are configuration / per-invocation choices
	/// contextWillBeSharedByMultipleWorkersFromDifferentThreads should ideally be false
	#[inline(always)]
	pub fn initialiseApplicationContext(&self, applicationContextFeaturesIdeallySupported: &ApplicationContextFeaturesIdeallySupported, contextWillBeSharedByMultipleWorkersFromDifferentThreads: bool, tagSenderMask: u64, estimatedNumberOfEndPoints: usize) -> Result<ApplicationContext, ApplicationContextInitialisationError>
	{
		use ucp_params_field::*;
		
		let parameters = ucp_params_t
		{
			field_mask: UCP_PARAM_FIELD_FEATURES as u64 | UCP_PARAM_FIELD_REQUEST_SIZE as u64 | UCP_PARAM_FIELD_REQUEST_INIT as u64 | UCP_PARAM_FIELD_REQUEST_CLEANUP as u64 | UCP_PARAM_FIELD_TAG_SENDER_MASK as u64 | UCP_PARAM_FIELD_MT_WORKERS_SHARED as u64 | UCP_PARAM_FIELD_ESTIMATED_NUM_EPS as u64,
			features: applicationContextFeaturesIdeallySupported.as_u64(tagSenderMask),
			
			// Really of use to MPI
			request_size: 0, // reservedSpaceInNonBlockingRequests,
			request_init: None,
			request_cleanup: None,
			
			tag_sender_mask: tagSenderMask,
			mt_workers_shared: if contextWillBeSharedByMultipleWorkersFromDifferentThreads
			{
				1
			}
			else
			{
				0
			},
			estimated_num_eps: estimatedNumberOfEndPoints,
		};
		
		let mut context = unsafe { uninitialized() };
		
		let status = unsafe { ucp_init_version(UCP_API_MAJOR, UCP_API_MINOR, &parameters, self.handle, &mut context) };
		
		if likely(status.isOk())
		{
			Ok
			(
				ApplicationContext
				{
					handle: context,
				}
			)
		}
		else
		{
			use self::UcpFailure::*;
			use self::UcpPermanentFailureReason::*;
			use self::ApplicationContextInitialisationError::*;
			
			let failure = UcpFailure::convertError(status);
			
			match failure
			{
				Permanent(reason) => match reason
				{
					OutOfMemory => panic!("Out of memory"),
					UnimplementedFunctionality => Err(FunctionalityNotImplementedOrSupported),
					UnsupportedSubSetOfFunctionality => Err(FunctionalityNotImplementedOrSupported),
					_ => panic!("Permanent failure to initialise an application context because '{:?}'", reason)
				},
				_ => panic!("Inappropriate failure for UCP API '{}'", failure)
			}
		}
	}
}
