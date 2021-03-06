cmake_minimum_required(VERSION 3.12)

project(SDL2TemplateCMake)

set(CMAKE_MODULE_PATH "${PROJECT_SOURCE_DIR}/cmake")

add_executable(main main.cpp)

#Default build is to enable all safe optimizations (-O3, LTO)
#If debugging needed, you can override this with
#cmake -D CMAKE_BUILD_TYPE=Debug ..

include(CheckIPOSupported)
# Optional LTO. Do not use LTO if it's not supported by compiler.
check_ipo_supported(RESULT result OUTPUT output)
if(result)
  set_property(TARGET main PROPERTY INTERPROCEDURAL_OPTIMIZATION TRUE)
else()
  message(WARNING "LTO is not supported: ${output}")
endif()

#If no build type set, default to Release build
set(default_build_type "Release")
 
if(NOT CMAKE_BUILD_TYPE AND NOT CMAKE_CONFIGURATION_TYPES)
  message(STATUS "Setting build type to '${default_build_type}' as none was specified.")
  set(CMAKE_BUILD_TYPE "${default_build_type}" CACHE
      STRING "Choose the type of build." FORCE)
  # Set the possible values of build type for cmake-gui
  set_property(CACHE CMAKE_BUILD_TYPE PROPERTY STRINGS
    "Debug" "Release" "MinSizeRel" "RelWithDebInfo")
endif()

#Default to full warnings
target_compile_options(main
  PRIVATE
  $<$<OR:$<CXX_COMPILER_ID:AppleClang>,$<CXX_COMPILER_ID:Clang>,$<CXX_COMPILER_ID:GNU>>:-Wall;-Werror>
  $<$<CXX_COMPILER_ID:MSVC>:/W4>
)

#Default to C++17 -- no effect on C code
target_compile_features(main PUBLIC cxx_std_17)

#Provide option to force O3 for RelWithDebInfo
if (RELWITHDEBINFO_FORCE_OPTIMIZATION_O3)
    string(REPLACE "-O2" "-O3" CMAKE_CXX_FLAGS_RELWITHDEBINFO "${CMAKE_CXX_FLAGS_RELWITHDEBINFO}")
    string(REPLACE "-O2" "-O3" CMAKE_C_FLAGS_RELWITHDEBINFO "${CMAKE_C_FLAGS_RELWITHDEBINFO}")
endif()

#Directory all assets are contained in
set(ASSETS "${PROJECT_SOURCE_DIR}/assets")

#Use Eigen
find_package (Eigen3 3.3.7 REQUIRED NO_MODULE)
target_link_libraries (main PRIVATE Eigen3::Eigen)

#Special: handle emscripten for running in web browser
if ("${CMAKE_SYSTEM_NAME}" MATCHES "Emscripten")
  #Ensure Release builds have full optimization enabled
  string(REPLACE "O2" "O3" CMAKE_C_FLAGS_RELEASE "${CMAKE_C_FLAGS_RELEASE}")
  string(REPLACE "O2" "O3" CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE}")

  string(REPLACE "O2" "O3" CMAKE_C_FLAGS_RELWITHDEBINFO
    "${CMAKE_C_FLAGS_RELWITHDEBINFO}")
  string(REPLACE "O2" "O3" CMAKE_CXX_FLAGS_RELWITHDEBINFO
    "${CMAKE_CXX_FLAGS_RELWITHDEBINFO}")

  string(REPLACE "O2" "O3" CMAKE_EXE_LINKER_FLAGS_RELEASE
    "${CMAKE_EXE_LINKER_FLAGS_RELEASE}")
  string(REPLACE "O2" "O3" CMAKE_EXE_LINKER_FLAGS_RELWITHDEBINFO "${CMAKE_EXE_LINKER_FLAGS_RELWITHDEBINFO}")

  string(REPLACE "O2" "O3" CMAKE_SHARED_LINKER_FLAGS_RELEASE
    "${CMAKE_SHARED_LINKER_FLAGS_RELEASE}")
  string(REPLACE "O2" "O3" CMAKE_SHARED_LINKER_FLAGS_RELWITHDEBINFO
    "${CMAKE_SHARED_LINKER_FLAGS_RELWITHDEBINFO}")

  string(REPLACE "O2" "O3" CMAKE_MODULE_LINKER_FLAGS_RELEASE
    "${CMAKE_MODULE_LINKER_FLAGS_RELEASE}")
  string(REPLACE "O2" "O3" CMAKE_MODULE_LINKER_FLAGS_RELWITHDEBINFO
    "${CMAKE_MODULE_LINKER_FLAGS_RELWITHDEBINFO}")

  #Emscripten supports C++17, but upstream doesn't have the configuration added
  #TODO: make pull request upstream to Emscripten.cmake to add this
  set(CMAKE_CXX17_COMPILE_FEATURES "cxx_std_17")
  
  set(CMAKE_EXECUTABLE_SUFFIX ".html") 
  set(ECXXFLAGS "-s USE_SDL=2 -s USE_SDL_TTF=2 -s EXIT_RUNTIME=1 --preload-file ${ASSETS}@../assets")
  set_target_properties(main PROPERTIES LINK_FLAGS "${ECXXFLAGS} --emrun")
  set_target_properties(main PROPERTIES COMPILE_FLAGS "${ECXXFLAGS}")
else ()

  #Otherwise, do native handling
  find_package(SDL2 REQUIRED)
  find_package(SDL2_ttf REQUIRED)

  target_include_directories(main
    PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}
  )

  target_include_directories(main
    SYSTEM PUBLIC
    ${SDL2_INCLUDE_DIRS}
    ${SDL2_TTF_INCLUDE_DIR}
  )
  
  #CMake-built SDL2 doesn't define SDL2_LIBRARIES while autotools-built SDL2 does
  if (NOT DEFINED SDL2_LIBRARIES)
    set(SDL2_LIBRARIES "SDL2::SDL2")
  endif()

  target_link_libraries(main
    PUBLIC
    ${SDL2_LIBRARIES}
    ${SDL2_TTF_LIBRARY}
  )
endif()

