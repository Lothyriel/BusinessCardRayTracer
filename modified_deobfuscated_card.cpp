#include <stdlib.h>
#include <stdio.h>
#include <math.h>

struct Vector3
{
    float x, y, z;

    Vector3() {}

    Vector3 operator+(Vector3 r)
    {
        return Vector3(x + r.x, y + r.y, z + r.z);
    }

    Vector3 operator*(float r)
    {
        return Vector3(x * r, y * r, z * r);
    }

    float operator%(Vector3 r)
    {
        return x * r.x + y * r.y + z * r.z;
    }

    Vector3 operator^(Vector3 r)
    {
        return Vector3(y * r.z - z * r.y, z * r.x - x * r.z, x * r.y - y * r.x);
    }

    Vector3(float a, float b, float c)
    {
        x = a;
        y = b;
        z = c;
    }

    Vector3 operator!()
    {
        return *this * (1 / sqrt(*this % *this));
    }
};

int spheres_location[] = {
    508418,
    278788,
    278664,
    278608,
    16416,
    16464,
    16520,
    16644,
    522754};

float random()
{
    return (float)rand() / RAND_MAX;
}

int trace(Vector3 o, Vector3 d, float &t, Vector3 &n)
{
    t = 1e9;
    int m = 0;
    float p = -o.z / d.z;

    if (.01 < p)
        t = p, n = Vector3(0, 0, 1), m = 1;

    for (int k = 19; k--;)
        for (int j = 9; j--;)
            if (spheres_location[j] & 1 << k)
            {
                Vector3 p = o + Vector3(-k, 0, -j - 4);
                float b = p % d;
                float c = p % p - 1;
                float q = b * b - c;

                if (q > 0)
                {
                    float s = -b - sqrt(q);
                    if (s < t && s > .01)
                        t = s, n = !(p + d * t), m = 2;
                }
            }
    return m;
}

Vector3 sample(Vector3 o, Vector3 d)
{
    float t;
    Vector3 n;
    int m = trace(o, d, t, n);

    if (!m)
        return Vector3(.7, .6, 1) * pow(1 - d.z, 4);

    Vector3 h = o + d * t;
    Vector3 l = !(Vector3(9 + random(), 9 + random(), 16) + h * -1);
    Vector3 r = d + n * (n % d * -2);

    float b = l % n;

    if (b < 0 || trace(h, l, t, n))
        b = 0;

    float p = pow(l % r * (b > 0), 99);

    if (m & 1)
    {
        h = h * .2;
        return ((int)(ceil(h.x) + ceil(h.y)) & 1 ? Vector3(3, 1, 1) : Vector3(3, 3, 3)) * (b * .2 + .1);
    }

    return Vector3(p, p, p) + sample(h, r) * .5;
}

int main()
{
    const int WIDTH = 512;
    const int HEIGHT = 512;

    Vector3 g = !Vector3(-6, -16, 0);
    Vector3 a = !(Vector3(0, 0, 1) ^ g) * .002;
    Vector3 b = !(g ^ a) * .002;
    Vector3 c = (a + b) * -256 + g;

    for (int y = HEIGHT; y--;)
        for (int x = WIDTH; x--;)
        {
            Vector3 pixel_color(13, 13, 13);
            for (int r = 64; r--;)
            {
                Vector3 t = a * (random() - .5) * 99 + b * (random() - .5) * 99;
                Vector3 o = !(t * -1 + (a * (random() + x) + b * (y + random()) + c) * 16);
                pixel_color = sample(Vector3(17, 16, 8) + t, o) * 3.5 + pixel_color;
            }
            printf("%d %d %d\n", (int)pixel_color.x, (int)pixel_color.y, (int)pixel_color.z);
        }
}