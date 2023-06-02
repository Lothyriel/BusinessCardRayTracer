use image::{ImageBuffer, Rgb};
use rand::Rng;
use std::path::Path;
use vector::Vec3;

mod vector;

const GREY_COLOR: Vec3 = Vec3 {
    x: 13.,
    y: 13.,
    z: 13.,
};

const SPHERES_LOCATION: [u32; 9] = [
    508418, 278788, 278664, 278608, 16416, 16464, 16520, 16644, 522754,
];

#[derive(PartialEq, Eq)]
enum RayCollision {
    None,
    Plane,
    Sphere,
}

fn main() {
    let width = 512;
    let height = 512;

    let pixels = render_image(width, height);

    save_as_png(width, height, pixels);
}

fn render_image(width: u32, height: u32) -> Vec<Vec3> {
    let camera_direction = Vec3::new(-6., -16., 0.).norm();

    let camera_right = Vec3::new(0., 0., 1.)
        .cross(camera_direction)
        .norm()
        .scale(0.002);

    let camera_up = camera_direction.cross(camera_right).norm().scale(0.002);

    let camera_position = camera_right
        .add(camera_up)
        .scale(-256.)
        .add(camera_direction);

    let a = (0..height).rev().flat_map(|y| {
        (0..width)
            .rev()
            .map(move |x| get_pixel_color(camera_right, camera_up, x, y, camera_position))
    });

    a.collect()
}

fn get_pixel_color(camera_right: Vec3, camera_up: Vec3, x: u32, y: u32, camera_pos: Vec3) -> Vec3 {
    let mut pixel_color = GREY_COLOR;
    for _ in (0..64).rev() {
        //to create shadows and pespective
        let offset = camera_right
            .scale(random() - 0.5)
            .scale(99.)
            .add(camera_up.scale(random() - 0.5).scale(99.));

        let ray_origin = camera_right
            .scale(random() + x as f32)
            .add(camera_up.scale(random() + y as f32))
            .add(camera_pos)
            .scale(16.)
            .add(offset.scale(-1.))
            .norm();

        let ray_direction = Vec3::new(17., 16., 8.).add(offset);

        let sampled_pixel_color = sample_pixel_color(ray_direction, ray_origin);
        pixel_color = sampled_pixel_color.scale(3.5).add(pixel_color);
    }

    pixel_color
}

fn sample_pixel_color(direction: Vec3, origin: Vec3) -> Vec3 {
    let mut distance = 0.;

    let mut surface_normal = Vec3::zero();

    let collision = trace(direction, origin, &mut distance, &mut surface_normal);

    if collision == RayCollision::None {
        let a1 = 1. - origin.z;
        return Vec3::new(0.7, 0.6, 1.).scale(a1.powf(4.));
    }

    let mut hit_point = direction.add(origin.scale(distance));

    let light_direction = Vec3::new(9. + random(), 9. + random(), 16.)
        .add(hit_point.scale(-1.))
        .norm();

    let reflected_ray = origin.add(surface_normal.scale(surface_normal.dot(origin) * -2.));

    let mut brightness = light_direction.dot(surface_normal);

    if brightness < 0.
        || trace(
            hit_point,
            light_direction,
            &mut distance,
            &mut surface_normal,
        ) != RayCollision::None
    {
        brightness = 0.;
    }

    if collision == RayCollision::Plane {
        hit_point = hit_point.scale(0.2);

        let discriminant = hit_point.x.ceil() + hit_point.y.ceil();

        let color = if discriminant as i32 & 1 != 0 {
            Vec3::new(3., 1., 1.)
        } else {
            Vec3::new(3., 3., 3.)
        };

        return color.scale(brightness * 0.2 + 0.1);
    }

    let color_contribution = if brightness > 0. {
        light_direction.dot(reflected_ray).powf(99.)
    } else {
        0.
    };

    Vec3::new(color_contribution, color_contribution, color_contribution)
        .add(sample_pixel_color(hit_point, reflected_ray).scale(0.5))
}

fn trace(direction: Vec3, origin: Vec3, distance: &mut f32, surf_norm: &mut Vec3) -> RayCollision {
    *distance = 1e9;

    let mut collision = RayCollision::None;
    let intersection_distance = -direction.z / origin.z;

    if 0.01 < intersection_distance {
        *distance = intersection_distance;
        *surf_norm = Vec3::new(0., 0., 1.);
        collision = RayCollision::Plane;
    }

    for k in (0..19).rev() {
        for j in (0..9).rev() {
            if should_render_sphere(j, k) {
                let p = direction.add(Vec3::new(-k as f32, 0., -j as f32 - 4.));

                let projection = p.dot(origin);
                let quadratic_coefficient = p.dot(p) - 1.;
                let discriminant = projection * projection - quadratic_coefficient;

                if discriminant > 0. {
                    let sphere_intersection_distance = -projection - discriminant.sqrt();
                    if sphere_intersection_distance < *distance
                        && sphere_intersection_distance > 0.01
                    {
                        *distance = sphere_intersection_distance;
                        *surf_norm = p.add(origin.scale(*distance)).norm();
                        collision = RayCollision::Sphere;
                    }
                }
            }
        }
    }

    collision
}

fn should_render_sphere(j: i32, k: i32) -> bool {
    //Spheres location are stored as 9x19 bits matrice on this i32 array
    let row = SPHERES_LOCATION[j as usize];

    let collumn_bit_offset = 1 << k;

    let location = row & collumn_bit_offset;
    //0 = empty space
    //non-zero = sphere
    location != 0
}

fn random() -> f32 {
    rand::thread_rng().gen()
}

fn save_as_png(width: u32, height: u32, pixels: Vec<Vec3>) {
    let image_buffer = ImageBuffer::from_fn(width, height, |x, y| {
        let pixel = pixels[(y * width + x) as usize];
        Rgb([pixel.x as u8, pixel.y as u8, pixel.z as u8])
    });

    image_buffer
        .save(Path::new("output_library.png"))
        .expect("Failed to save image");
}
