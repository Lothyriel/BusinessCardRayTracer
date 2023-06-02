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

float random_normalized()
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
    float distance;
    Vector3 surface_normal;
    int material = trace(o, d, distance, surface_normal);

    if (!material)
        return Vector3(.7, .6, 1) * pow(1 - d.z, 4);

    Vector3 hit_point = o + d * distance;
    Vector3 light_direction = !(Vector3(9 + random_normalized(), 9 + random_normalized(), 16) + hit_point * -1);
    Vector3 reflected_ray = d + surface_normal * (surface_normal % d * -2);

    float brightness = light_direction % surface_normal;

    if (brightness < 0 || trace(hit_point, light_direction, distance, surface_normal))
        brightness = 0;

    float contribution = pow(light_direction % reflected_ray * (brightness > 0), 99);

    if (material & 1)
    {
        hit_point = hit_point * .2;
        return ((int)(ceil(hit_point.x) + ceil(hit_point.y)) & 1 ? Vector3(3, 1, 1) : Vector3(3, 3, 3)) * (brightness * .2 + .1);
    }

    return Vector3(contribution, contribution, contribution) + sample(hit_point, reflected_ray) * .5;
}

int main()
{
    const int WIDTH = 512;
    const int HEIGHT = 512;

    Vector3 camera_direction = !Vector3(-6, -16, 0);
    Vector3 camera_right = !(Vector3(0, 0, 1) ^ camera_direction) * .002;
    Vector3 camera_up = !(camera_direction ^ camera_right) * .002;
    Vector3 camera_position = (camera_right + camera_up) * -256 + camera_direction;

    for (int y = HEIGHT; y--;)
        for (int x = WIDTH; x--;)
        {
            Vector3 pixel_color(13, 13, 13);
            for (int r = 64; r--;)
            {
                Vector3 offset = camera_right * (random_normalized() - .5) * 99 + camera_up * (random_normalized() - .5) * 99;
                Vector3 ray_origin = !(offset * -1 + (camera_right * (random_normalized() + x) + camera_up * (y + random_normalized()) + camera_position) * 16);
                Vector3 sampled_color = sample(Vector3(17, 16, 8) + offset, ray_origin);
                pixel_color = sampled_color * 3.5 + pixel_color;
            }
            printf("%d %d %d\n", (int)pixel_color.x, (int)pixel_color.y, (int)pixel_color.z);
        }
}