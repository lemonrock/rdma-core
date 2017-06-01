#@IgnoreInspection BashAddShebang
# This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
# Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


bindingsName='openshmem-reference'
rootIncludeFileName='openshmem-reference.h'
macosXHomebrewPackageNames='clang-format'
alpineLinuxPackageNames='rsync make gcc linux-headers libunwind-dev linux-grsec-dev'
clangAdditionalArguments=''
headersFolderPath="$homeFolder"/lib/openshmem/src
libFolderPath="$homeFolder"/lib/openshmem/src
link=''
link_kind='static-nobundle'

final_chance_to_tweak()
{
	mv "$outputFolderPath"/structs/'__BindgenComplex<T>.rs' "$outputFolderPath"/structs/'__BindgenComplex.rs'
	sed -i -e 's/<T>//g' "$outputFolderPath"/structs.rs
}
