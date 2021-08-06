AC_INIT()

AC_CONFIG_MACRO_DIRS([m4/])

AC_CANONICAL_BUILD
AC_CANONICAL_HOST
AC_CANONICAL_TARGET

if test x$build_alias != x
then
    build_noncanonical=$build_alias
else
    build_noncanonical=$build
fi

AC_SUBST([build_noncanonical])

if test x$host_alias != x
then
    host_noncanonical=$host_alias
else
    host_noncanonical=$build_noncanonical
fi

AC_SUBST([host_noncanonical])

if test x$target_alias != x
then
    target_noncanonical=$target_alias
else
    target_noncanonical=$host_noncanonical
fi

AC_SUBST([target_noncanonical])

case x$target in
    xw65-* )
        ;;
    * )
        AC_MSG_ERROR([Cannot build SNES-Dev for target $target_noncanonical])
        ;;
esac

if test x${build_noncanonical} != x${host_noncanonical}
then
    # We are cross compiling, so make sure to use toolchains for the build target 
    toolchain_prefix="${build_noncanonical}-"
fi

AC_PROG_INSTALL
AC_PROG_LN_S
AC_PROG_AWK
AC_PROG_GREP
AC_PROG_SED
AC_PROG_MKDIR_P

AC_PROG_RANLIB
AC_PROG_CC
AC_PROG_CXX
AC_PATH_TOOL([AR],[ar])
LCRUST_PROG_RUSTC
LCRUST_RUSTC_VERSION

AX_PROG_CC_FOR_BUILD
AX_PROG_CXX_FOR_BUILD
LCRUST_PROG_RUSTC_FOR_BUILD
LCRUST_RUSTC_VERSION_FOR_BUILD

AC_PATH_PROGS([AR_FOR_BUILD],[ar ${build_noncanonical}-ar])


AC_ARG_VAR([CC_FOR_TARGET])
AC_ARG_VAR([CFLAGS_FOR_TARGET])
AC_ARG_VAR([CXX_FOR_TARGET])
AC_ARG_VAR([CXXFLAGS_FOR_TARGET])
AC_ARG_VAR([RUSTC_FOR_TARGET])
AC_ARG_VAR([RUSTFLAGS_FOR_TARGET])
AC_ARG_VAR([AS_FOR_TARGET])
AC_ARG_VAR([AR_FOR_TARGET])
AC_ARG_VAR([LD_FOR_TARGET])
AC_ARG_VAR([OBJCOPY_FOR_TARGET])

AC_MSG_CHECKING([for cc to use with $target_noncanonical])

if test "$CC_FOR_TARGET" \= ""
then
    AC_MSG_RESULT([just compiled])
    CC_FOR_TARGET="\${abs_builddir}/${toolchain_prefix}gcc/gcc/xgcc"
else
    AC_MSG_RESULT([$CC_FOR_TARGET])
fi

AC_MSG_CHECKING([for c++ to use with $target_noncanonical])

if test "$CXX_FOR_TARGET" \= ""
then
    AC_MSG_RESULT([just compiled])
    CXX_FOR_TARGET="\${abs_builddir}/${toolchain_prefix}gcc/gcc/xg++"
else
    AC_MSG_RESULT([$CXX_FOR_TARGET])
fi

AC_MSG_CHECKING([for as to use with $target_noncanonical])

if test "$AS_FOR_TARGET" \= ""
then
    AC_MSG_RESULT([just compiled])
    AS_FOR_TARGET="\${abs_builddir}/${toolchain_prefix}binutils-gdb/gas/as-new"
else
    AC_MSG_RESULT([$AS_FOR_TARGET])
fi

AC_MSG_CHECKING([for ar to use with $target_noncanonical])

if test "$AR_FOR_TARGET" \= ""
then
    AC_MSG_RESULT([just compiled])
    AR_FOR_TARGET="\${abs_builddir}/${toolchain_prefix}binutils-gdb/binutils/ar"
else
    AC_MSG_RESULT([$AR_FOR_TARGET])
fi

AC_MSG_CHECKING([for ld to use with $target_noncanonical])

if test "$LD_FOR_TARGET" \= ""
then
    AC_MSG_RESULT([just compiled])
    LD_FOR_TARGET="\${abs_builddir}/${toolchain_prefix}binutils-gdb/ld/ld-new"
else
    AC_MSG_RESULT([$LD_FOR_TARGET])
fi

AC_MSG_CHECKING([for objcopy to use with $target_noncanonical])

if test "$OBJCOPY_FOR_TARGET" \= ""
then
    AC_MSG_RESULT([just compiled])
    OBJCOPY_FOR_TARGET="\${abs_builddir}/${toolchain_prefix}binutils-gdb/binutils/objcopy"
else
    AC_MSG_RESULT([$OBJCOPY_FOR_TARGET])
fi

RUSTFLAGS_FOR_TARGET="$RUSTFLAGS_FOR_TARGET --target $target"

AC_MSG_CHECKING([where to install libraries for $taret_noncanonical])
AC_ARG_WITH([sysroot],[
    Set the sysroot to for binutils and gcc, and also the installation prefix for libraries
],[
    sysroot="$withval"
],[
    sysroot='${prefix}/'"$target_noncanonical"
])

AC_MSG_RESULT([$sysroot])
AC_SUBST([sysroot])




AC_ARG_ENABLE([float],[
    Enable Float support in qc and libc
],[
    float_support=$enableval
],[
    float_support=yes
])

AC_ARG_ENABLE([int128],[
    Enable int128 support in qc and gcc
],[
    int128_support=$enableval
],[
    int128_support=no
])

AC_ARG_ENABLE([binutils],[
    Build gnu binutils and use it's tools to build libraries
],[
    case x$enableval in
        xyes )
            host_dirs="$host_dirs binutils-gdb"
            if test x${build_noncanonical} != x${host_noncanonical}
            then
                build_dirs="$build_dirs ${build_noncanonical}-binutils-gdb"
            fi
            ;;
        xno )
            ;;
    esac
],[
    host_dirs="$host_dirs binutils-gdb"
    if test x${build_noncanonical} != x${host_noncanonical}
    then
        build_dirs="$build_dirs ${build_noncanonical}-binutils-gdb"
    fi
])

AC_ARG_ENABLE([gcc],[
    Build gcc and use it's tools to build libraries
],[
    case x$enableval in
        xyes )
            host_dirs="$host_dirs gcc"
            if test x${build_noncanonical} != x${host_noncanonical}
            then
                build_dirs="$build_dirs ${build_noncanonical}-gcc"
            fi
            ;;
        xno )
            ;;
    esac
],[
    
])


AC_ARG_ENABLE([qc],[
    Build qc extended arithmetic library
],[
    case x$enableval in
        xyes )
            target_dirs="$target_dirs qc"
            ;;
        xno )
            ;;
    esac
],[
    target_dirs="$target_dirs qc"
])

AC_SUBST([build_dirs])
AC_SUBST([host_dirs])
AC_SUBST([target_dirs])
AC_SUBST([int128_support])
AC_SUBST([float_support])


AC_CONFIG_FILES([Makefile])

AC_OUTPUT