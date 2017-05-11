# This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
# Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


bindingsName='rdma-core'
rootIncludeFileName='infiniband/verbs.h'
macosXHomebrewPackageNames='clang-format'
alpineLinuxPackageNames='rsync make gcc linux-headers libunwind-dev linux-grsec-dev'
clangAdditionalArguments=''
headersFolderPath="$homeFolder"/compile-rdma-core.conf.d/temporary/usr/include
libFolderPath="$homeFolder"/compile-rdma-core.conf.d/temporary/usr/lib
link='ibverbs'
link_kind='static-nobundle'

final_chance_to_tweak()
{
	:
}
