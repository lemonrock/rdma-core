// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


quick_error!
{
	#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub enum UcpFailure
	{
		InProgress
		{
			display("In progress")
		}

		/// Only relevant for received (tagged) messages, and, even then, can probably be avoided by using probe
		ReceivedTaggedMessageWasTruncated
		{
		}

		/// Only relevant for Worker arm
		CouldNotArmWorkerAsBusy
		{
		}
		
		/// Only relevant for non-blocking requests
		NonBlockingRequestCancelled
		{
		}

		Transient(transientFailureReason: UcpTransientFailureReason)
		{
			display("Transient failure '{:?}'", transientFailureReason)
		}
		
		RecoverableIfApplicationRestarted(recoverableIfApplicationRestartedFailureReason: UcpRecoverableIfApplicationRestartedFailureReason)
		{
			display("Recoverable if application restarted failure '{:?}'", recoverableIfApplicationRestartedFailureReason)
		}
		
		Permanent(permanentFailureReason: UcpPermanentFailureReason)
		{
			display("Permanent failure '{:?}'", permanentFailureReason)
		}
		
		Impossible(impossibleFailureReason: UcpImpossibleFailureReason)
		{
			display("Impossible failure '{:?}'", impossibleFailureReason)
		}
		
		Future(offset: u8)
		{
			display("offset was '{}'", offset)
		}
		
		Link(offset: u8)
		{
			display("offset was '{}'", offset)
		}
		
		EndPoint(offset: u8)
		{
			display("offset was '{}'", offset)
		}
		
		Unknown(offset: u8)
		{
			display("offset was '{}'", offset)
		}
	}
}

impl UcpFailure
{
	const FirstFutureError: ucs_status_t = ucs_status_t_UCS_ERR_UNSUPPORTED - 1;
	
	const LastFutureError: ucs_status_t = ucs_status_t_UCS_ERR_FIRST_LINK_FAILURE + 1;
	
	const FirstUnknownError: ucs_status_t = ucs_status_t_UCS_ERR_ENDPOINT_TIMEOUT - 1;
	
	#[inline(always)]
	pub fn as_ucs_status_t(&self) -> ucs_status_t
	{
		use self::UcpFailure::*;
		
		match *self
		{
			InProgress => ucs_status_t_UCS_INPROGRESS,
			ReceivedTaggedMessageWasTruncated => ucs_status_t_UCS_ERR_MESSAGE_TRUNCATED,
			CouldNotArmWorkerAsBusy => ucs_status_t_UCS_ERR_BUSY,
			NonBlockingRequestCancelled => ucs_status_t_UCS_ERR_CANCELED,
			Transient(reason) => reason as i8,
			RecoverableIfApplicationRestarted(reason) => reason as i8,
			Permanent(reason) => reason as i8,
			Impossible(reason) => reason as i8,
			Future(offset) => Self::FirstFutureError - (offset as i8),
			Link(offset) => ucs_status_t_UCS_ERR_FIRST_LINK_FAILURE - (offset as i8),
			EndPoint(offset) => ucs_status_t_UCS_ERR_FIRST_ENDPOINT_FAILURE - (offset as i8),
			Unknown(offset) => Self::FirstUnknownError - (offset as i8),
		}
	}
	
	#[inline(always)]
	pub fn convertError(status: ucs_status_t) -> Self
	{
		debug_assert!(status != ucs_status_t_UCS_OK, "status is OK");
		
		use self::UcpFailure::*;
		use self::UcpTransientFailureReason::*;
		use self::UcpRecoverableIfApplicationRestartedFailureReason::*;
		use self::UcpPermanentFailureReason::*;
		use self::UcpImpossibleFailureReason::*;
		
		match status
		{
			ucs_status_t_UCS_INPROGRESS => InProgress,
			ucs_status_t_UCS_ERR_NO_MESSAGE => Transient(NoPendingMessage),
			ucs_status_t_UCS_ERR_NO_RESOURCE => Transient(NoResource),
			ucs_status_t_UCS_ERR_IO_ERROR => RecoverableIfApplicationRestarted(UnderlyingEPollOrLibcIoOperationFailed),
			ucs_status_t_UCS_ERR_NO_MEMORY => Permanent(OutOfMemory),
			ucs_status_t_UCS_ERR_INVALID_PARAM => Permanent(InvalidParameter),
			ucs_status_t_UCS_ERR_UNREACHABLE => Transient(DestinationAddressIsUnreachable),
			ucs_status_t_UCS_ERR_INVALID_ADDR => Permanent(InvalidRemoteAddressOrTcpAddressIsNotIpV6OrCanNotPackIntoRemoteAddressBuffer),
			ucs_status_t_UCS_ERR_NOT_IMPLEMENTED => Permanent(UnimplementedFunctionality),
			ucs_status_t_UCS_ERR_MESSAGE_TRUNCATED => ReceivedTaggedMessageWasTruncated,
			ucs_status_t_UCS_ERR_NO_PROGRESS => Permanent(NoProgress),
			ucs_status_t_UCS_ERR_BUFFER_TOO_SMALL => Impossible(BufferTooSmall),
			ucs_status_t_UCS_ERR_NO_ELEM => Permanent(ElementDoesNotExist),
			ucs_status_t_UCS_ERR_SOME_CONNECTS_FAILED => Impossible(FailedToConnectToSomeOfTheRequestedEndPoints),
			ucs_status_t_UCS_ERR_NO_DEVICE => Permanent(NoTransportDeviceExists),
			ucs_status_t_UCS_ERR_BUSY => CouldNotArmWorkerAsBusy,
			ucs_status_t_UCS_ERR_CANCELED => NonBlockingRequestCancelled,
			ucs_status_t_UCS_ERR_SHMEM_SEGMENT => RecoverableIfApplicationRestarted(PosixOrSysVSharedMemoryError),
			ucs_status_t_UCS_ERR_ALREADY_EXISTS => Permanent(ElementAlreadyExists),
			ucs_status_t_UCS_ERR_OUT_OF_RANGE => Permanent(IndexOutOfRangeOrNameTooLong),
			ucs_status_t_UCS_ERR_TIMED_OUT => Impossible(OperationTimedOut),
			ucs_status_t_UCS_ERR_EXCEEDS_LIMIT => RecoverableIfApplicationRestarted(IsEmptyOrIsFull),
			ucs_status_t_UCS_ERR_UNSUPPORTED => Permanent(UnsupportedSubSetOfFunctionality),
			Self::LastFutureError ... Self::FirstFutureError => Future(-(status - Self::FirstFutureError) as u8),
			ucs_status_t_UCS_ERR_LAST_LINK_FAILURE ... ucs_status_t_UCS_ERR_FIRST_LINK_FAILURE => Link(-(status - ucs_status_t_UCS_ERR_FIRST_LINK_FAILURE) as u8),
			ucs_status_t_UCS_ERR_LAST_ENDPOINT_FAILURE ... ucs_status_t_UCS_ERR_FIRST_ENDPOINT_FAILURE => EndPoint(-(status - ucs_status_t_UCS_ERR_FIRST_ENDPOINT_FAILURE) as u8),
			ucs_status_t_UCS_ERR_ENDPOINT_TIMEOUT => Transient(EndPointTimeOut),
			ucs_status_t_UCS_ERR_LAST ... Self::FirstUnknownError => Unknown(-(status - Self::FirstUnknownError) as u8),
			_ => panic!("Unknown status, or non-error status '{}'", status),
		}
	}
}
