SET(CMAKE_SYSTEM_NAME Linux)
SET(CMAKE_SYSTEM_VERSION 1)
SET(CMAKE_C_COMPILER x86_64-linux-musl-cc)
SET(CMAKE_CXX_COMPILER x86_64-linux-musl-c++)
SET(CMAKE_FIND_ROOT_PATH /usr/local/opt/musl-cross/libexec/x86_64-linux-musl)
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