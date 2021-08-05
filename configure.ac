AC_INIT([snes-dev])

AC_CONFIG_MACRO_DIRS([m4/])

AC_CANONICAL_BUILD
AC_CANONICAL_HOST
AC_CANONICAL_TARGET

if x$build_alias != x
    noncanonical_build=$build_alias
else
    noncaonical_build=$build
fi

if x$host_alias != x
    noncanonical_host=$host_alias
else
    noncaonical_host=$noncanonical_build
fi

if x$target_alias != x
    noncanonical_target=$target_alias
else
    noncanonical_target=$noncanonical_host
fi

case x$target in
    xw65-* )
        ;;
    * )
        AC_MSG_ERROR([Cannot build SNES-Dev for target $noncanonical_target])
        ;;
esac

AC_PROG_INSTALL

AC_ARG_WITH([sysroot],[
    Set the sysroot to for binutils and gcc, and also the installation prefix for libraries
])

AC_CONFIG_SUBDIRS([binutils-gdb])
AC_CONFIG_FILES([Makefile])
AC_OUTPUT