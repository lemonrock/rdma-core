# This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
# Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


compile_library_name='rdma-core'

compile_library()
{
	compile_library_prepareCMakeToolChainFile()
	{
		CMAKE_TOOLCHAIN_FILE="$rootOutputFolderPath"/toolchain.cmake
		cat >"$CMAKE_TOOLCHAIN_FILE" <<-EOF
			SET(CMAKE_SYSTEM_NAME Linux)
			SET(CMAKE_SYSTEM_VERSION 1)
			SET(CMAKE_C_COMPILER ${compilerPrefix}cc)
			SET(CMAKE_CXX_COMPILER ${compilerPrefix}c++)
			SET(CMAKE_FIND_ROOT_PATH ${targetSysRootFolderPath})
			SET(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
			SET(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
			SET(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)

			SET(CMAKE_C_FLAGS_DEBUG "-Werror")
			SET(CMAKE_C_FLAGS_MINSIZEREL "-Werror")
			SET(CMAKE_C_FLAGS_RELWITHDEBINFO "-Werror")
			SET(CMAKE_C_FLAGS_RELEASE "-Werror")
			SET(CMAKE_CXX_FLAGS_DEBUG "-Werror")
			SET(CMAKE_CXX_FLAGS_MINSIZEREL "-Werror")
			SET(CMAKE_CXX_FLAGS_RELWITHDEBINFO "-Werror")
			SET(CMAKE_CXX_FLAGS_RELEASE "-Werror")
		
			# These, despite their names, are specific to the rdma-core build system.
			SET(CMAKE_C_FLAGS_DEBUG_INIT "-Werror")
			SET(CMAKE_C_FLAGS_MINSIZEREL_INIT "-Werror")
			SET(CMAKE_C_FLAGS_RELWITHDEBINFO_INIT "-Werror")
			SET(CMAKE_C_FLAGS_RELEASE_INIT "-Werror")
			SET(CMAKE_CXX_FLAGS_DEBUG_INIT "-Werror")
			SET(CMAKE_CXX_FLAGS_MINSIZEREL_INIT "-Werror")
			SET(CMAKE_CXX_FLAGS_RELWITHDEBINFO_INIT "-Werror")
			SET(CMAKE_CXX_FLAGS_RELEASE_INIT "-Werror")
		EOF
	}

	compile_library_build()
	{
		local buildFolderPath="$rootOutputFolderPath"/build
		mkdir -m 0700 "$buildFolderPath"
	
		local destDirFolderPath="$rootOutputFolderPath"/DESTDIR
		mkdir -m 0700 "$destDirFolderPath"
	
		cd "$buildFolderPath" 1>/dev/null 2>/dev/null
		
			cmake \
				-DCMAKE_TOOLCHAIN_FILE="$CMAKE_TOOLCHAIN_FILE" \
				-DCMAKE_BUILD_TYPE=Release \
				-DCMAKE_INSTALL_PREFIX='/usr' \
				-DIN_PLACE=0 \
				-DENABLE_STATIC=1 \
				-DENABLE_VALGRIND=0 \
				-DENABLE_RESOLVE_NEIGH=0 \
				-DNO_COMPAT_SYMS=1 \
				..
			make -j "$numberOfMakeJobs" DESTDIR="$destDirFolderPath" install
		
		cd - 1>/dev/null 2>/dev/null
	}
	
	local CMAKE_TOOLCHAIN_FILE
	compile_library_prepareCMakeToolChainFile
	
	compile_library_build
}

cargo_key_value_pairs()
{
	# compile.conf.d, bindgen-wrapper.conf.d, tools/bindgen-wrapper and lib/$compile_library_name are automatically added.
	
	cargo_key_value_pairs_link_lib 'static-nobundle' 'cxgb3'
	cargo_key_value_pairs_link_lib 'static-nobundle' 'cxgb4'
	cargo_key_value_pairs_link_lib 'static-nobundle' 'hfi1verbs'
	cargo_key_value_pairs_link_lib 'static-nobundle' 'hns'
	cargo_key_value_pairs_link_lib 'static-nobundle' 'i40iw'
	cargo_key_value_pairs_link_lib 'static-nobundle' 'ibcm'
	cargo_key_value_pairs_link_lib 'static-nobundle' 'ibumad'
	cargo_key_value_pairs_link_lib 'static-nobundle' 'ibverbs'
	cargo_key_value_pairs_link_lib 'static-nobundle' 'ipathverbs'
	cargo_key_value_pairs_link_lib 'static-nobundle' 'mlx4'
	cargo_key_value_pairs_link_lib 'static-nobundle' 'mlx5'
	cargo_key_value_pairs_link_lib 'static-nobundle' 'mthca'
	cargo_key_value_pairs_link_lib 'static-nobundle' 'nes'
	cargo_key_value_pairs_link_lib 'static-nobundle' 'ocrdma'
	cargo_key_value_pairs_link_lib 'static-nobundle' 'qedr'
	cargo_key_value_pairs_link_lib 'static-nobundle' 'rdmacm'
	cargo_key_value_pairs_link_lib 'static-nobundle' 'rxe'
	cargo_key_value_pairs_link_lib 'static-nobundle' 'vmw_pvrdma'
	
	# Search path
	cargo_key_value_pairs_search 'native' "$OUT_DIR"/root/usr/lib
	
	# Not used by us, but used by downstream ucx-sys crate's build.
	cargo_key_value_pairs_other 'root' "$OUT_DIR"/root/DESTDIR/usr
	cargo_key_value_pairs_other 'include' "$OUT_DIR"/root/DESTDIR/usr/include
	cargo_key_value_pairs_other 'libdir' "$OUT_DIR"/root/DESTDIR/usr/lib
}
