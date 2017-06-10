// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait PrintInformation
{
	#[inline(always)]
	fn printInformationToStream(&self, stream: *mut FILE);
	
	#[inline(always)]
	fn printInformationToStandardError(&self)
	{
		self.printInformationToStream(unsafe { stderr } as *mut FILE)
	}
	
	#[inline(always)]
	fn printInformationToStandardOut(&self)
	{
		self.printInformationToStream(unsafe { stdout } as *mut FILE)
	}
	
	#[inline(always)]
	fn printInformationToString(&self) -> Result<String, PrintInformationToStringError>
	{
		use self::PrintInformationToStringError::*;
		
		let mut buffer = unsafe { uninitialized() };
		let mut size = unsafe { uninitialized() };
		let filePointer = unsafe { open_memstream(&mut buffer, &mut size) };
		if unlikely(filePointer.is_null())
		{
			return Err(CouldNotOpenMemoryStream(errno()));
		}
		
		let result = unsafe { fflush(filePointer) };
		debug_assert!(result == 0 || result == EOF, "result was not 0 or EOF but '{}'", result);
		if unlikely(result == EOF)
		{
			unsafe { free(buffer as *mut c_void) };
			return Err(CouldNotFlushMemoryStream(errno()))
		}
		
		let result = unsafe { fclose(filePointer) };
		debug_assert!(result == 0 || result == EOF, "result was not 0 or EOF but '{}'", result);
		if unlikely(result == EOF)
		{
			unsafe { free(buffer as *mut c_void) };
			return Err(CouldNotCloseMemoryStream(errno()))
		}
		
		let result = match unsafe { CStr::from_ptr(buffer) }.to_str()
		{
			Err(error) => Err(CouldNotConvertInformation(error)),
			Ok(string) => Ok(string.to_owned())
		};
		
		unsafe { free(buffer as *mut c_void) };
		result
	}
}
