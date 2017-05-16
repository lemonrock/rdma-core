#@IgnoreInspection BashAddShebang
# This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
# Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


bindingsName='rdma-core'
rootIncludeFileName='rdma-core.h'
macosXHomebrewPackageNames='clang-format'
alpineLinuxPackageNames='rsync make gcc linux-headers libunwind-dev linux-grsec-dev'
clangAdditionalArguments=''
headersFolderPath="$homeFolder"/compile-rdma-core.conf.d/temporary/DESTDIR/usr/include
libFolderPath="$homeFolder"/compile-rdma-core.conf.d/temporary/DESTDIR/usr/lib
link='cxgb3 cxgb4 hfi1verbs hns i40iw ibumad ibverbs ipathverbs mlx4 mlx5 mthca nes ocrdma qedr rdmacm rxe vmw_pvrdma'
link_kind='static-nobundle'


bindgen_wrapper_generateStaticFunctions()
{
	local relativeHeaderFile="$1"
	local outputFileBaseName="$2"

	mkdir -m 0700 -p "$temporaryFolderPath"/includes

	clang-format -style="{BasedOnStyle: Chromium, IndentWidth: 4, ColumnLimit: 4000, BreakBeforeBraces: Allman}" "$headersFolderPath"/"$relativeHeaderFile" \
	| grep '^static inline ' \
	| sed \
		-e 's/^static inline //g' \
		-e 's/\*/ */g' \
		-e 's/(void)/()/g' \
		-e 's/, /,/g' \
		-e 's/)$//g' \
		-e 's/(/,/g' >"$temporaryFolderPath"/$outputFileBaseName.functions

	{
		printf '#include <%s>\n' "$relativeHeaderFile" >"$temporaryFolderPath"/includes/$outputFileBaseName.h

		cat <<-EOF
			#include <${relativeHeaderFile}>
			#include "${outputFileBaseName}.h"
		EOF

		local line
		while IFS= read -r line
		do
			local returnValueAndFunctionName="$(printf '%s\n' "$line" | cut -f1 -d ',')"
			local returnValue="$(printf '%s\n' "$returnValueAndFunctionName" | awk -F' ' '{$NF=""}1' | sed 's/ $//g')"
			local functionName="$(printf '%s\n' "$returnValueAndFunctionName" | awk -F' ' '{print $NF}')"

			local definitionOfFunction="$returnValue rust_${functionName}("

			local bodyOfFunction
			if [ "$returnValue" = 'void' ]; then
				bodyOfFunction="${functionName}("
			else
				bodyOfFunction="return ${functionName}("
			fi

			local numberOfArguments="$(printf '%s\n' "$line" | awk -F',' '{print NF-1}')"

			local argumentField
			local argumentIndex=0
			while [ $argumentIndex -lt $numberOfArguments ]
			do
				local cutIndex=$((2 + argumentIndex))

				local argumentValueAndArgumentName="$(printf '%s\n' "$line" | cut -f${cutIndex} -d ',')"
				local argumentValue="$(printf '%s\n' "$argumentValueAndArgumentName" | awk -F' ' '{$NF=""}1' | sed 's/ $//g')"
				local argumentName="$(printf '%s\n' "$argumentValueAndArgumentName" | awk -F' ' '{print $NF}')"

				if [ $argumentIndex -ne 0 ]; then
					definitionOfFunction="${definitionOfFunction}, "
					bodyOfFunction="${bodyOfFunction}, "
				fi
				definitionOfFunction="${definitionOfFunction}${argumentValue} ${argumentName}"
				bodyOfFunction="${bodyOfFunction}${argumentName}"

				argumentIndex=$((argumentIndex+1))
			done

			definitionOfFunction="${definitionOfFunction})"
			bodyOfFunction="${bodyOfFunction});"

			printf '\n%s;\n' "$definitionOfFunction" >>"$temporaryFolderPath"/includes/$outputFileBaseName.h

			printf '\n%s\n{\n\t%s\n}\n' "$definitionOfFunction" "$bodyOfFunction"

		done <"$temporaryFolderPath"/$outputFileBaseName.functions
	} >"$temporaryFolderPath"/includes/$outputFileBaseName.c
}

preprocess_before_headersFolderPath()
{
	bindgen_wrapper_generateStaticFunctions 'infiniband/verbs.h' 'infiniband-verbs-static-inline'
	bindgen_wrapper_generateStaticFunctions 'rdma/rdma_verbs.h' 'rdma-verbs-static-inline'
}

final_chance_to_tweak()
{
	sed -i -e 's/#\[derive(Debug, Default, Copy)\]/#[derive(Copy)]/g' "$outputFolderPath"/structs/ibv_values_ex.rs
}
