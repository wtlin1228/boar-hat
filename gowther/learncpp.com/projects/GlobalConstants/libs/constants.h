#ifndef CONSTANT_H
#define CONSTANT_H

namespace constants {
// Use inline constexpr for constants that rarely change. The value lives in
// the header, so changing it forces every file that includes this header to
// be recompiled.
inline constexpr double pi{3.14159};
inline constexpr double avogadro{6.0221413e23};
inline constexpr double myGravity{9.2};

// Use extern const for constants that change frequently. The value lives in
// a single .cpp file, so changing it only recompiles that file, not every
// includer. Switch these to inline constexpr before release for better
// optimization.
extern const int colorR;
extern const int colorG;
extern const int colorB;
} // namespace constants

#endif
