My attempt to play with Andrew Kensler's Business Card Raytracer algorithm

https://fabiensanglard.net/rayTracing_back_of_business_card/

build: c++ -O3 -o card deobfuscated_card.cpp
run: ./card > card.ppm


- deobfuscated some names and typedefs to make it cleared to read
- altered the original "AEK" letters bit-encoded in the numbers to my initials "JX"
- altered the final output encoding to a custom format of one pixel colors in RGB per line
- created a program in C# to convert from this custom format to bitmap then .PNG


# ![3D Rendered output with my initials](https://github.com/Lothyriel/BusinessCardRayTracer/blob/main/output.png)
