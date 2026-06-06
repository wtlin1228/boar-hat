add_executable(InlineFunction 
    main.cpp
    libs/math.cpp
)

target_include_directories(InlineFunction PRIVATE libs)
