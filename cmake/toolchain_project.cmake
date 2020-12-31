include(ExternalProject)

function(add_toolchain_project dir)
    message(STATUS "Building ${dir} with local cross compiler toolchain")
    ExternalProject_Add(
            ${dir}
            SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/${dir}
            BINARY_DIR ${CMAKE_CURRENT_BINARY_DIR}/${dir}
            CMAKE_ARGS ${_SNESDEV_LOCAL_CMAKE_ARGS}
                -DSNESDEV_TOOLCHAIN_FILE=${SNESDEV_TOOLCHAIN_FILE}
                -DCMAKE_INSTALL_PREFIX=${SNESDEV_SYSROOT}
            DEPENDS ${_SNESDEV_LOCAL_PROJECT_DEPENDENCIES}
    )
endfunction()