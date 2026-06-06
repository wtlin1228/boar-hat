add_executable(GlobalConstants 
    main.cpp
    libs/constants.cpp
)

target_include_directories(GlobalConstants PRIVATE libs)
