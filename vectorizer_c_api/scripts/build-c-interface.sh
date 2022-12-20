set -e

#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
# BASICS
#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )


#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
# CONFIGURATION
#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

BUILD_ALL="YES"

SPECIFIC_BUILDS="NO"
BUILD_MACOS_ONLY="NO"
BUILD_IOS_ONLY="NO"
BUILD_CATALYST_ONLY="NO"
BUILD_HEADER_ONLY="NO"

BUILD_RELEASE_MODE="NO"
FORCE_RELINK="NO"

#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
# INPUT OPTIONS
#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
for v in "$@"; do
	if [ "$v" = "debug" ]; then
		BUILD_RELEASE_MODE="NO"
	fi
	if [ "$v" = "release" ]; then
		BUILD_RELEASE_MODE="YES"
	fi
	if [ "$v" = "macos" ]; then
		SPECIFIC_BUILDS="YES"
		BUILD_MACOS_ONLY="YES"
		BUILD_HEADER_ONLY="YES"
	fi
	if [ "$v" = "mac" ]; then
		SPECIFIC_BUILDS="YES"
		BUILD_MACOS_ONLY="YES"
		BUILD_HEADER_ONLY="YES"
	fi
	if [ "$v" = "catalyst" ]; then
		SPECIFIC_BUILDS="YES"
		BUILD_CATALYST_ONLY="YES"
		BUILD_HEADER_ONLY="YES"
	fi
	if [ "$v" = "ios" ]; then
		SPECIFIC_BUILDS="YES"
		BUILD_IOS_ONLY="YES"
		BUILD_HEADER_ONLY="YES"
	fi
	if [ "$v" = "HEADER" ]; then
		SPECIFIC_BUILDS="YES"
		BUILD_HEADER_ONLY="YES"
	fi
	if [ "$v" = "relink" ]; then
		FORCE_RELINK="YES"
	fi
done


#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
# CRATE NAME
#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
CRATE_AR_ARCHIVE_NAME_SRC="libvectorizer_c_api.a"
CRATE_AR_ARCHIVE_NAME_DEST_IOS="libvectorizer_c_api.a"
CRATE_AR_ARCHIVE_NAME_DEST_MACOS="libvectorizer_c_api.a"
CRATE_AR_ARCHIVE_NAME_DEST_CATALYST="libvectorizer_c_api.a"

#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
# FFI DIRS
#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
OUTOUT_DIR="$SCRIPT_DIR/../c-ffi"
RUST_PROJECT_DIR="$SCRIPT_DIR/.."
RUST_TARGET_DIR="$SCRIPT_DIR/../../target"
SWIFT_C_INCLUDE_DIR="$OUTOUT_DIR/include"
SWIFT_C_IOS_LIB_DIR="$OUTOUT_DIR/IOS/lib"
SWIFT_C_MACOS_LIB_DIR="$OUTOUT_DIR/MacOS/lib"
SWIFT_C_CATALYST_LIB_DIR="$OUTOUT_DIR/Catalyst/lib"
SWIFT_INCLUDE_HEADER_FILE_PATH="$SWIFT_C_INCLUDE_DIR/vectorizer-c-api.h"

# We (Maybe) need the SDK Root
# export SDKROOT=`xcrun --sdk macosx --show-sdk-path`

#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
# INTERNAL HELPER FUNCTIONS
#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
to_rel_path () {
	WORKING_DIR=$(pwd)
	perl -le 'use File::Spec; print File::Spec->abs2rel(@ARGV)' $1 $WORKING_DIR
}

canonicalize_file_path () {
	perl -MCwd -e 'print Cwd::abs_path shift' $1
}

#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
# BUILD FUNCTIONS
#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

build_macos () {
	if [ "$BUILD_RELEASE_MODE" = "YES" ];
	then
		echo "Building MacOS (RELEASE MODE)"
		cargo build --target x86_64-apple-darwin --release
	else
		echo "Building MacOS (DEBUG MODE)"
		cargo build --target x86_64-apple-darwin
	fi
}

build_ios () {
	if [ "$BUILD_RELEASE_MODE" = "YES" ];
	then
		echo "Building IOS (RELEASE MODE)"
		cargo build --target aarch64-apple-ios --release
	else
		echo "Building IOS (DEBUG MODE)"
		cargo build --target aarch64-apple-ios
	fi
}

build_catalyst () {
	# export CFLAG="-target x86_64-apple-ios13.0-macabi"
	if [ "$BUILD_RELEASE_MODE" = "YES" ];
	then
		# echo "Building CATALYST (RELEASE MODE)"
		# cargo +nightly build -Z build-std --release --lib --target x86_64-apple-ios-macabi
		echo "Building CATALYST (RELEASE MODE): SKIPPED!"
	else
		# echo "Building CATALYST (DEBUG MODE)"
		# cargo +nightly build -Z build-std --lib --target x86_64-apple-ios-macabi
		echo "Building CATALYST (DEBUG MODE): SKIPPED!"
	fi
}

build_c_header () {
	echo "Building C Header File"
	cbindgen --config cbindgen.toml --output $SWIFT_INCLUDE_HEADER_FILE_PATH
}

#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
# BUILD
#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
cd $RUST_PROJECT_DIR

if [ "$SPECIFIC_BUILDS" = "YES" ]; then
	if [ "$BUILD_MACOS_ONLY" = "YES" ]; then
		build_macos
	fi
	if [ "$BUILD_IOS_ONLY" = "YES" ]; then
		build_ios
	fi
	if [ "$BUILD_CATALYST_ONLY" = "YES" ]; then
		build_catalyst
	fi
else
	build_macos
	build_ios
	build_catalyst
fi

build_c_header


#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
# BUILD - POST-PROCESSING
#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
if [ ! -f $OUTOUT_DIR ]; then
	mkdir -p $OUTOUT_DIR
fi

if [ ! -f $SWIFT_C_INCLUDE_DIR ]; then
	mkdir -p $SWIFT_C_INCLUDE_DIR
fi

if [ ! -f $SWIFT_C_IOS_LIB_DIR ]; then
	mkdir -p $SWIFT_C_IOS_LIB_DIR
fi

if [ ! -f $SWIFT_C_CATALYST_LIB_DIR ]; then
	mkdir -p $SWIFT_C_CATALYST_LIB_DIR
fi

if [ ! -f $SWIFT_C_MACOS_LIB_DIR ]; then
	mkdir -p $SWIFT_C_MACOS_LIB_DIR
fi

#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
# LINK FUNCTIONS
#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

link_macos () {
	MACOS_STATIC_LIB_SRC="$RUST_TARGET_DIR/x86_64-apple-darwin/release/$CRATE_AR_ARCHIVE_NAME_SRC"
	MACOS_STATIC_LIB_DEST="$SWIFT_C_MACOS_LIB_DIR/$CRATE_AR_ARCHIVE_NAME_DEST_MACOS"
	RELINK="NO"
	if [ "$BUILD_RELEASE_MODE" = "NO" ]; then
		MACOS_STATIC_LIB_SRC="$RUST_TARGET_DIR/x86_64-apple-darwin/debug/$CRATE_AR_ARCHIVE_NAME_SRC"
	fi
	if [ ! -f "$MACOS_STATIC_LIB_DEST" ]; then
		RELINK="YES"
	fi
	if [ "$FORCE_RELINK" = "YES" ]; then
		RELINK="YES"
	fi
	if [ "$RELINK" = "YES" ]; then
		if [ -f "$MACOS_STATIC_LIB_DEST" ]; then
			rm "$MACOS_STATIC_LIB_DEST"
		fi
		# NOTE: We want to create symbols links with normalized file paths
		MACOS_STATIC_LIB_SRC=$(canonicalize_file_path $MACOS_STATIC_LIB_SRC)
		MACOS_STATIC_LIB_DEST=$(canonicalize_file_path $MACOS_STATIC_LIB_DEST)
		echo "Creating Symbolic Link [MAC-OS]"
		echo "\t  SRC: $MACOS_STATIC_LIB_SRC"
		echo "\t DEST: $MACOS_STATIC_LIB_DEST"
		ln -s $MACOS_STATIC_LIB_SRC $MACOS_STATIC_LIB_DEST
	fi
}

link_ios () {
	IOS_STATIC_LIB_SRC="$RUST_TARGET_DIR/aarch64-apple-ios/release/$CRATE_AR_ARCHIVE_NAME_SRC"
	IOS_STATIC_LIB_DEST="$SWIFT_C_IOS_LIB_DIR/$CRATE_AR_ARCHIVE_NAME_DEST_IOS"
	RELINK="NO"
	if [ "$BUILD_RELEASE_MODE" = "NO" ]; then
		IOS_STATIC_LIB_SRC="$RUST_TARGET_DIR/aarch64-apple-ios/debug/$CRATE_AR_ARCHIVE_NAME_SRC"
	fi
	if [ ! -f "$IOS_STATIC_LIB_DEST" ] && [ "$MACOS_ONLY" != "YES" ] && [ "$BUILD_CATALYST_ONLY" != "YES" ]; then
		RELINK="YES"
	fi
	if [ "$FORCE_RELINK" = "YES" ]; then
		RELINK="YES"
	fi
	if [ "$RELINK" = "YES" ]; then
		if [ -f "$MACOS_STATIC_LIB_DEST" ]; then
			rm "$IOS_STATIC_LIB_DEST"
		fi
		# NOTE: We want to create symbols links with normalized file paths
		IOS_STATIC_LIB_SRC=$(canonicalize_file_path $IOS_STATIC_LIB_SRC)
		IOS_STATIC_LIB_DEST=$(canonicalize_file_path $IOS_STATIC_LIB_DEST)
		echo "Creating Symbolic Link [IOS]"
		echo "\t  SRC: $IOS_STATIC_LIB_SRC"
		echo "\t DEST: $IOS_STATIC_LIB_DEST"
		ln -s $IOS_STATIC_LIB_SRC $IOS_STATIC_LIB_DEST
	fi
}

link_catalyst () {
	echo "Creating Symbolic Link [CATALYST]: SKIPPED!"
	# CATALYST_STATIC_LIB_SRC="$RUST_TARGET_DIR/x86_64-apple-ios-macabi/release/$CRATE_AR_ARCHIVE_NAME_SRC"
	# CATALYST_STATIC_LIB_DEST="$SWIFT_C_CATALYST_LIB_DIR/$CRATE_AR_ARCHIVE_NAME_DEST_CATALYST"
	# RELINK="NO"
	# if [ "$BUILD_RELEASE_MODE" = "NO" ]; then
	# 	CATALYST_STATIC_LIB_SRC="$RUST_TARGET_DIR/x86_64-apple-ios-macabi/debug/$CRATE_AR_ARCHIVE_NAME_SRC"
	# fi
	# if [ ! -f "$CATALYST_STATIC_LIB_DEST" ] && [ "$BUILD_IOS_ONLY" != "YES" ] && [ "$MACOS_ONLY" != "YES" ]; then
	# 	RELINK="YES"
	# fi
	# if [ "$FORCE_RELINK" = "YES" ]; then
	# 	RELINK="YES"
	# fi
	# if [ "$RELINK" = "YES" ]; then
	# 	# NOTE: We want to create symbols links with normalized file paths
	# 	CATALYST_STATIC_LIB_SRC=$(canonicalize_file_path $CATALYST_STATIC_LIB_SRC)
	# 	CATALYST_STATIC_LIB_DEST=$(canonicalize_file_path $CATALYST_STATIC_LIB_DEST)
	# 	echo "Creating Symbolic Link [CATALYST]"
	# 	echo "\t  SRC: $CATALYST_STATIC_LIB_SRC"
	# 	echo "\t DEST: $CATALYST_STATIC_LIB_DEST"
	# 	ln -s $CATALYST_STATIC_LIB_SRC $CATALYST_STATIC_LIB_DEST
	# fi
}

#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
# LINK FILES to FFI DIR
#―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――

if [ "$SPECIFIC_BUILDS" = "YES" ]; then
	if [ "$BUILD_MACOS_ONLY" = "YES" ]; then
		link_macos
	fi
	if [ "$BUILD_IOS_ONLY" = "YES" ]; then
		link_ios
	fi
	if [ "$BUILD_CATALYST_ONLY" = "YES" ]; then
		link_catalyst
	fi
else
	link_macos
	link_ios
	link_catalyst
fi

