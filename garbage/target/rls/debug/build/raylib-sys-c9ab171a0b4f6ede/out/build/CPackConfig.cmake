# This file will be configured to contain variables for CPack. These variables
# should be set in the CMake list file of the project before CPack module is
# included. The list of available CPACK_xxx variables and their associated
# documentation may be obtained using
#  cpack --help-variable-list
#
# Some variables are common to all generators (e.g. CPACK_PACKAGE_NAME)
# and some are specific to a generator
# (e.g. CPACK_NSIS_EXTRA_INSTALL_COMMANDS). The generator specific variables
# usually begin with CPACK_<GENNAME>_xxxx.


set(CPACK_BUILD_SOURCE_DIRS "D:/rusty programs/hat/target/rls/debug/build/raylib-sys-c9ab171a0b4f6ede/out/raylib;D:/rusty programs/hat/target/rls/debug/build/raylib-sys-c9ab171a0b4f6ede/out/build")
set(CPACK_CMAKE_GENERATOR "Visual Studio 16 2019")
set(CPACK_COMPONENT_UNSPECIFIED_HIDDEN "TRUE")
set(CPACK_COMPONENT_UNSPECIFIED_REQUIRED "TRUE")
set(CPACK_DEFAULT_PACKAGE_DESCRIPTION_FILE "D:/programs/cmake/share/cmake-3.23/Templates/CPack.GenericDescription.txt")
set(CPACK_DEFAULT_PACKAGE_DESCRIPTION_SUMMARY "raylib built using CMake")
set(CPACK_DMG_SLA_USE_RESOURCE_FILE_LICENSE "ON")
set(CPACK_GENERATOR "ZIP;TGZ")
set(CPACK_INSTALL_CMAKE_PROJECTS "D:/rusty programs/hat/target/rls/debug/build/raylib-sys-c9ab171a0b4f6ede/out/build;raylib;ALL;/")
set(CPACK_INSTALL_PREFIX "D:/rusty programs/hat/target/rls/debug/build/raylib-sys-c9ab171a0b4f6ede/out")
set(CPACK_MODULE_PATH "D:/rusty programs/hat/target/rls/debug/build/raylib-sys-c9ab171a0b4f6ede/out/raylib/cmake")
set(CPACK_NSIS_DISPLAY_NAME "raylib 3.5.0")
set(CPACK_NSIS_INSTALLER_ICON_CODE "")
set(CPACK_NSIS_INSTALLER_MUI_ICON_CODE "")
set(CPACK_NSIS_INSTALL_ROOT "$PROGRAMFILES64")
set(CPACK_NSIS_PACKAGE_NAME "raylib 3.5.0")
set(CPACK_NSIS_UNINSTALL_NAME "Uninstall")
set(CPACK_OUTPUT_CONFIG_FILE "D:/rusty programs/hat/target/rls/debug/build/raylib-sys-c9ab171a0b4f6ede/out/build/CPackConfig.cmake")
set(CPACK_PACKAGE_DEFAULT_LOCATION "/")
set(CPACK_PACKAGE_DESCRIPTION_FILE "D:/rusty programs/hat/target/rls/debug/build/raylib-sys-c9ab171a0b4f6ede/out/raylib/src/../README.md")
set(CPACK_PACKAGE_DESCRIPTION_SUMMARY "Simple and easy-to-use library to enjoy videogames programming")
set(CPACK_PACKAGE_FILE_NAME "raylib-3.5.0")
set(CPACK_PACKAGE_INSTALL_DIRECTORY "raylib 3.5.0")
set(CPACK_PACKAGE_INSTALL_REGISTRY_KEY "raylib 3.5.0")
set(CPACK_PACKAGE_NAME "raylib")
set(CPACK_PACKAGE_RELOCATABLE "true")
set(CPACK_PACKAGE_VENDOR "Humanity")
set(CPACK_PACKAGE_VERSION "3.5.0")
set(CPACK_PACKAGE_VERSION_MAJOR "")
set(CPACK_PACKAGE_VERSION_MINOR "")
set(CPACK_PACKAGE_VERSION_PATCH "")
set(CPACK_RESOURCE_FILE_LICENSE "D:/rusty programs/hat/target/rls/debug/build/raylib-sys-c9ab171a0b4f6ede/out/raylib/src/../LICENSE")
set(CPACK_RESOURCE_FILE_README "D:/programs/cmake/share/cmake-3.23/Templates/CPack.GenericDescription.txt")
set(CPACK_RESOURCE_FILE_WELCOME "D:/rusty programs/hat/target/rls/debug/build/raylib-sys-c9ab171a0b4f6ede/out/raylib/src/../README.md")
set(CPACK_SET_DESTDIR "OFF")
set(CPACK_SOURCE_7Z "ON")
set(CPACK_SOURCE_GENERATOR "7Z;ZIP")
set(CPACK_SOURCE_OUTPUT_CONFIG_FILE "D:/rusty programs/hat/target/rls/debug/build/raylib-sys-c9ab171a0b4f6ede/out/build/CPackSourceConfig.cmake")
set(CPACK_SOURCE_ZIP "ON")
set(CPACK_SYSTEM_NAME "win64")
set(CPACK_THREADS "1")
set(CPACK_TOPLEVEL_TAG "win64")
set(CPACK_WIX_SIZEOF_VOID_P "8")

if(NOT CPACK_PROPERTIES_FILE)
  set(CPACK_PROPERTIES_FILE "D:/rusty programs/hat/target/rls/debug/build/raylib-sys-c9ab171a0b4f6ede/out/build/CPackProperties.cmake")
endif()

if(EXISTS ${CPACK_PROPERTIES_FILE})
  include(${CPACK_PROPERTIES_FILE})
endif()