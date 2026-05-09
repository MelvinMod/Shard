#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>

typedef int64_t Int;
typedef int8_t Int8;
typedef int16_t Int16;
typedef int32_t Int32;
typedef int64_t Int64;
typedef uint64_t UInt;
typedef uint8_t UInt8;
typedef uint16_t UInt16;
typedef uint32_t UInt32;
typedef uint64_t UInt64;
typedef float Float32;
typedef double Float64;

// Function declarations
void say(const char* msg);

// Main function
int main(int argc, char** argv) {
    say("Hello, World!");
    return 0;
}

// Standard library
void say(const char* msg) {
    printf("%s\n", msg);
}
