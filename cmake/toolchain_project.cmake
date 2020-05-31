function(add_toolchain_project dir)
    set(CMAKE_INSTALL_DIR ${CMAKE_INSTALL_DIR}/wc65c816-snesdev-default-elf/tools)
    add_subdirectory(${dir})
endfunction()