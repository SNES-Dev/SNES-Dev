[+ AutoGen5 template -*- Mode: Makefile -*-
in
+]

# Overridable Programs

INSTALL := @INSTALL@
INSTALL_PROGRAM := @INSTALL_PROGRAM@
INSTALL_SCRIPT := @INSTALL_SCRIPT@
INSTALL_DATA := @INSTALL_DATA@

LN_S := @LN_S@
MKDIR_P := @MKDIR_P@
GREP := @GREP@
AWK := @AWK@
SED := @SED@

RUSTC := @RUSTC@
RUSTFLAGS := @RUSTFLAGS@
CC := @CC@
CFLAGS := @CFLAGS@
CXX := @CXX@
CXXFLAGS := @CXXFLAGS@
AR := @AR@
RANLIB := @RANLIB@

SHELL = @SHELL@

PWD_COMMAND = $${PWDCMD-pwd}

RUSTC_FOR_BUILD := @RUSTC_FOR_BUILD@
RUSTFLAGS_FOR_BUILD := @RUSTFLAGS_FOR_BUILD@
CC_FOR_BUILD := @CC_FOR_BUILD@
CFLAGS_FOR_BUILD := @CFLAGS_FOR_BUILD@
CXX_FOR_BUILD := @CXX_FOR_BUILD@
CXXFLAGS_FOR_BUILD := @CXXFLAGS_FOR_BUILD@
AR_FOR_BUILD := @AR_FOR_BUILD@

RUSTC_FOR_TARGET = @RUSTC_FOR_TARGET@
RUSTFLAGS_FOR_TARGET = @RUSTFLAGS_FOR_TARGET@
CC_FOR_TARGET = @CC_FOR_TARGET@
CFLAGS_FOR_TARGET = @CFLAGS_FOR_TARGET@
CXX_FOR_TARGET = @CXX_FOR_TARGET@
CXXFLAGS_FOR_TARGET = @CXXFLAGS_FOR_TARGET@
AS_FOR_TARGET = @AS_FOR_TARGET@
AR_FOR_TARGET = @AR_FOR_TARGET@
LD_FOR_TARGET = @LD_FOR_TARGET@
OBJCOPY_FOR_TARGET = @OBJCOPY_FOR_TARGET@


# Target Variables
build_alias=@build_noncanonical@
build=@build@
host_alias=@host_noncanonical@
host=@host@
target_alias=@target_noncanonical@
target=@target@

# Build and Source Directories

srcdir=@srcdir@
builddir=@builddir@

abs_builddir=@abs_builddir@
abs_srcdir=@abs_srcdir@


# Installation Directories

prefix := @prefix@
exec_prefix := @exec_prefix@
bindir := @bindir@
libdir := @libdir@
sbindir := @sbindir@
libexecdir := @libexecdir@
includedir := @includedir@
datarootdir := @datarootdir@
datadir := @datadir@
docdir := @docdir@
mandir := @mandir@
infodir := @infodir@
dvidir := @dvidir@
htmldir := @htmldir@
pdfdir := @pdfdir@
localedir := @localedir@
sysconfdir := @sysconfdir@
localstatedir := @localstatedir@
runstatedir := @runstatedir@
sharedstatedir := @sharedstatedir@

sysroot := @sysroot@
target_libdir := ${sysroot}/lib
target_includedir := ${sysroot}/include


# Package Variables

BUILD_DIRS = @build_dirs@
HOST_DIRS = @host_dirs@
TARGET_DIRS = @target_dirs@

languages = @languages@

BASE_DEFS = INSTALL="$(INSTALL)"; export INSTALL; \
			INSTALL_PROGRAM="$(INSTALL_PROGRAM)"; export INSTALL_PROGRAM; \
			LN_S="$(LN_S)"; export LN_S; \
			MKDIR_P="$(MKDIR_P)"; export MKDIR_P; \
			GREP="$(GREP)"; export GREP; \
			AWK="$(AWK)"; export AWK; \
			SED="$(SED)"; export SED; 

BUILD_DEFS = $(BASE_DEFS) \
			CC_FOR_BUILD="$(CC_FOR_BUILD)"; export CC_FOR_BUILD; \
			CFLAGS_FOR_BUILD="$(CFLAGS_FOR_BUILD)"; export CFLAGS_FOR_BUILD; \
			CC="$(CC_FOR_BUILD)"; export CC; \
			CFLAGS="$(CFLAGS_FOR_BUILD)"; export CFLAGS; \
			CXX_FOR_BUILD="$(CXX_FOR_BUILD)"; export CX_FOR_BUILD; \
			CXXFLAGS_FOR_BUILD="$(CFLAGS_FOR_BUILD)"; export CXXFLAGS_FOR_BUILD; \
			CXX="$(CXX_FOR_BUILD)"; export CXX; \
			CXXFLAGS="$(CXXFLAGS_FOR_BUILD)"; export CXXFLAGS; \
			RUSTC_FOR_BUILD="$(RUSTC_FOR_BUILD)"; export RUSTC_FOR_BUILD; \
			RUSTFLAGS_FOR_BUILD="$(RUSTFLAGS_FOR_BUILD)"; export RUSTFLAGS_FOR_BUILD; \
			RUSTC="$(RUSTC_FOR_BUILD)"; export RUSTC; \
			RUSTFLAGS="$(RUSTFLAGS_FOR_BUILD)"; export RUSTFLAGS; \
			AR="$(AR_FOR_BUILD)"; export AR; \
			prefix="$(prefix)"; export prefix; \
			exec_prefix="$(exec_prefix)"; export exec_prefix; \
			bindir="$(bindir)"; export bindir; \
			sbindir="$(sbindir)"; export sbindir; \
			libexec="$(libexecdir)"; export libexecdir; \
			libdir="$(libdir)"; export libdir; \
			includedir="$(includedir)"; export includedir; \
			datarootdir=$(datarootdir); export datarootdir; \
			datadir="$(datadir)"; export datadir; \
			docdir="$(docdir)"; export docdir; \
			mandir="$(mandir)"; export mandir; \
			infodir="$(infodir)"; export infodir; \
			htmldir="$(htmldir)"; export htmldir; \
			dvidir="$(dvidir)"; export dvidir; \
			pdfdir="$(pdfdir)"; export pdfdir; \
			localedir="$(localedir)"; export localedir; \
			sysconfdir="$(sysconfdir)"; export sysconfdir; \
			sharedstatedir="$(sharedstatedir)"; export sharedstatedir; \
			localstatedir="$(localstatedir)"; export localstatedir


BUILD_CONFIGURE_OPTS = --prefix $(prefix) --exec-prefix $(exec_prefix) \
						--bindir $(bindir) --sbindir $(sbindir) \
						--libexecdir $(libexecdir) --libdir $(libdir) \
						--includedir $(includedir) --datarootdir $(datarootdir) \
						--datadir $(datadir) --docdir $(docdir) \
						--dvidir $(dvidir) --htmldir $(htmldir) \
						--pdfdir $(pdfdir) --mandir $(mandir) \
						--infodir $(infodir) --localedir $(localedir) \
						--sysconfdir $(sysconfdir) --sharedstatedir $(sharedstatedir) \
						--localstatedir $(localstatedir) 

ifneq (,@build_alias@)
	HOST_CONFIGURE_OPTS += --build $(build_alias) --host $(build_alias)
endif

ifneq (,@target_alias@)
	HOST_CONFIGURE_OPTIONS += --target $(target_alias)
endif

HOST_DEFS =	$(BASE_DEFS) \
			CC_FOR_BUILD="$(CC_FOR_BUILD)"; export CC_FOR_BUILD; \
			CFLAGS_FOR_BUILD="$(CFLAGS_FOR_BUILD)"; export CFLAGS_FOR_BUILD; \
			CC="$(CC)"; export CC; \
			CFLAGS="$(CFLAGS)"; export CFLAGS; \
			CXX_FOR_BUILD="$(CXX_FOR_BUILD)"; export CX_FOR_BUILD; \
			CXXFLAGS_FOR_BUILD="$(CFLAGS_FOR_BUILD)"; export CXXFLAGS_FOR_BUILD; \
			CXX="$(CXX)"; export CXX; \
			CXXFLAGS="$(CXXFLAGS)"; export CXXFLAGS; \
			RUSTC_FOR_BUILD="$(RUSTC_FOR_BUILD)"; export RUSTC_FOR_BUILD; \
			RUSTFLAGS_FOR_BUILD="$(RUSTFLAGS_FOR_BUILD)"; export RUSTFLAGS_FOR_BUILD; \
			RUSTC="$(RUSTC)"; export RUSTC; \
			RUSTFLAGS="$(RUSTFLAGS)"; export RUSTFLAGS; \
			AR="$(AR)"; export AR; \
			prefix="$(prefix)"; export prefix; \
			exec_prefix="$(exec_prefix)"; export exec_prefix; \
			bindir="$(bindir)"; export bindir; \
			sbindir="$(sbindir)"; export sbindir; \
			libexec="$(libexecdir)"; export libexecdir; \
			libdir="$(libdir)"; export libdir; \
			includedir="$(includedir)"; export includedir; \
			datarootdir=$(datarootdir); export datarootdir; \
			datadir="$(datadir)"; export datadir; \
			docdir="$(docdir)"; export docdir; \
			mandir="$(mandir)"; export mandir; \
			infodir="$(infodir)"; export infodir; \
			htmldir="$(htmldir)"; export htmldir; \
			dvidir="$(dvidir)"; export dvidir; \
			pdfdir="$(pdfdir)"; export pdfdir; \
			localedir="$(localedir)"; export localedir; \
			sysconfdir="$(sysconfdir)"; export sysconfdir; \
			sharedstatedir="$(sharedstatedir)"; export sharedstatedir; \
			localstatedir="$(localstatedir)"; export localstatedir

HOST_CONFIGURE_OPTS = --prefix $(prefix) --exec-prefix $(exec_prefix) \
						--bindir $(bindir) --sbindir $(sbindir) \
						--libexecdir $(libexecdir) --libdir $(libdir) \
						--includedir $(includedir) --datarootdir $(datarootdir) \
						--datadir $(datadir) --docdir $(docdir) \
						--dvidir $(dvidir) --htmldir $(htmldir) \
						--pdfdir $(pdfdir) --mandir $(mandir) \
						--infodir $(infodir) --localedir $(localedir) \
						--sysconfdir $(sysconfdir) --sharedstatedir $(sharedstatedir) \
						--localstatedir $(localstatedir) 

ifneq (,@build_alias@)
	HOST_CONFIGURE_OPTS += --build $(build_alias)
endif 

ifneq (,@host_alias@)
	HOST_CONFIGURE_OPTS += --host $(host_alias)
endif 

ifneq (,@target_alias@)
	HOST_CONFIGURE_OPTS += --target $(target_alias)
endif 

TARGET_DEFS = $(BASE_DEFS) \
				CC_FOR_BUILD="$(CC)"; export CC_FOR_BUILD; \
				CFLAGS_FOR_BUILD="$(CFLAGS)"; export CFLAGS_FOR_BUILD; \
				CC="$(CC_FOR_TARGET)"; export CC; \
				CFLAGS="$(CFLAGS_FOR_TARGET)"; export CFLAGS; \
				CXX_FOR_BUILD="$(CXX)"; export CXX_FOR_BUILD; \
				CXXFLAGS_FOR_BUILD="$(CXXFLAGS)"; export CXXFLAGS_FOR_BUILD; \
				CXX="$(CXX_FOR_TARGET)"; export CXX; \
				CFLAGS="$(CXXFLAGS_FOR_TARGET)"; export CXXFLAGS; \
				RUSTC_FOR_BUILD="$(RUSTC)"; export RUSTC_FOR_BUILD; \
				RUSTFLAGS_FOR_BUILD="$(CFLAGS)"; export RUSTFLAGS_FOR_BUILD; \
				RUSTC="$(RUSTC_FOR_TARGET)"; export RUSTC; \
				RUSTFLAGS="$(CFLAGS_FOR_TARGET)"; export RUSTFLAGS; \
				AR="$(AR_FOR_TARGET)"; export AR; \
				AS="$(AS_FOR_TARGET)"; export AS; \
				OBJCOPY="$(OBJCOPY_FOR_TARGET)"; export OBJCOPY; \
				LD="$(LD_FOR_TARGET)"; export LD; \
				prefix="$(sysroot)"; export prefix; \
				exec_prefix="$(sysroot)"; export exec_prefix; \
				libdir="$(target_libdir)"; export libdir; \
				includedir="$(target_includedir)"; export includedir

TARGET_CONFIGURE_OPTS = --prefix $(sysroot) --exec-prefix $(sysroot) --libdir $(target_libdir) --includedir $(target_includedir)

ifneq (,@host_alias@)
	TARGET_CONFIGURE_OPTS += --build ${host_alias}
endif

ifneq (,@target_alias@)
	TARGET_CONFIGURE_OPTS += --host ${target_alias}
endif


# Global Targets

all: stage2

.PHONY: all stage0 stage1 stage2 clean distclean check install install-strip \
$(foreach targ,all check clean distclean install install-strip,$(foreach build_dir,$(BUILD_DIRS),$(build_dir)/$(target))) \
	$(foreach targ,all check clean distclean install install-strip,$(foreach host_dir,$(HOST_DIRS),$(host_dir)/$(target))) \
	$(foreach targ,all check clean distclean install install-strip,$(foreach target_dir,$(TARGET_DIRS),$(target_dir)/$(target)))

.ONESHELL:

clean: $(foreach host_dir,$(HOST_DIRS),$(host_dir)/clean) $(foreach target_dir,$(TARGET_DIRS),$(target_dir)/clean)

distclean: # $(HOST_DIRS:%/=%/distclean) $(TARGET_DIRS:%/=%/distclean) # binutils and gcc distclean is borked. THe directories need to be deleted anyways, so just rm them
	rm -rf Makefile config.status config.cache $(HOST_DIRS) $(TARGET_DIRS)

check: $(foreach host_dir,$(HOST_DIRS),$(host_dir)/check) $(foreach target_dir,$(TARGET_DIRS),$(target_dir)/check)

install: $(foreach host_dir,$(HOST_DIRS),$(host_dir)/install) $(foreach target_dir,$(TARGET_DIRS),$(target_dir)/install)

install-strip: $(foreach host_dir,$(HOST_DIRS),$(host_dir)/install-strip) $(foreach target_dir,$(TARGET_DIRS),$(target_dir)/install-strip)

# Macros for configure and all

[+ DEFINE configure +]

.PHONY: configure-[+prefix+][+module+]

configure-[+prefix+][+module+]: 
	@r=`${PWD_COMMAND}`; export r; \
	s=`cd $(srcdir); ${PWD_COMMAND}`; export s; \
	[+exports+]; \
	cd [+prefix+][+module+] || exit 1; \
	$(SHELL) @abs_srcdir@/[+module+]/configure --srcdir=../$(srcdir)/[+module+] [+configure_flags+] [+extra_configure_flags+]

[+ ENDDEF +]

[+ DEFINE targs +]

.PHONY: [+prefix+][+module+]/all [+prefix+][+module+]/clean [+prefix+][+module+]/check [+prefix+][+module+]/install [+prefix+][+module+]/install-strip 

.PRECIOUS: [+prefix+][+module+]/config.sub

[+prefix+][+module+]/all: [+prefix+][+module+]/
	+@[+exports+]
	$(MAKE) -C [+prefix+][+module+] all

[+prefix+][+module+]/:
	+@[+exports+]
	$(MKDIR_P) [+prefix+][+module+]/
	$(MAKE) configure-[+prefix+][+module+]

[+prefix+][+module+]/config.status: [+prefix+][+module+]/
	@[+exports+]
	cd [+prefix+][+module+]; ./config.status --recheck

[+prefix+][+module+]/Makefile: [+prefix+][+module+]/config.status $(srcdir)/[+module+]/Makefile.in
	@[+exports+]
	cd [+prefix+][+module+]; ./config.status Makefile

[+prefix+][+module+]/clean:
	+@[+exports+]
	$(MAKE) -C [+prefix+][+module+] clean

[+prefix+][+module+]/check:
	+@[+exports+]
	$(MAKE) -C [+prefix+][+module+] check

[+prefix+][+module+]/install:
	+@[+exports+]
	$(MAKE) -C [+prefix+][+module+] install

[+prefix+][+module+]/install-strip:
	+@[+exports+]; $(MAKE) -C [+prefix+][+module+] install-strip


[+ ENDDEF +]


# stage0 targets (build)

stage0:
ifneq (,$(BUILD_DIRS))
	+$(MAKE) $(BUILD_DIRS:%=%/all)
endif

[+ FOR build_modules +]
[+ configure exports="$(BUILD_DEFS)" configure_flags="$(BUILD_CONFIGURE_OPTS)" prefix="@build_noncanonical@-" +]
[+ targs exports="$(BUILD_DEFS)" prefix="@build_noncanonical@-" +]
[+ ENDFOR +]

# stage1 targets (host)

[+ FOR host_modules +]
[+ configure exports="$(HOST_DEFS)" configure_flags="$(HOST_CONFIGURE_OPTS)" +]
[+ targs exports="$(HOST_DEFS)" +]
[+ ENDFOR +]

stage1: stage0
ifneq (,$(HOST_DIRS))
	+$(MAKE) $(HOST_DIRS:%=%/all)
endif

# stage2 targets (target)

stage2: stage1
ifneq (,$(TARGET_DIRS))
	+$(MAKE) $(TARGET_DIRS:%=%/all)
endif

[+ FOR target_modules +]
[+ configure exports="${TARGET_DEFS}"  configure_flags="$(TARGET_CONFIGURE_OPTS)" +]
[+ targs exports="$(TARGET_DEFS)" +]
[+ ENDFOR +]
