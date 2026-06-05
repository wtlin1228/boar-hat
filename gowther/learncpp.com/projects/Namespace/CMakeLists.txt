add_executable(Namespace
    main.cpp
    libs/foo.cpp
    libs/goo.cpp
)

target_include_directories(Namespace PRIVATE libs)
