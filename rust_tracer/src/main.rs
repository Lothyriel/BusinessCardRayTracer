use rand::Rng;
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

fn main() {
    let width = 512;
    let height = 512;

    render_image(width, height);
}

fn render_image(width: u32, height: u32) {
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

    for y in (0..height).rev() {
        for x in (0..width).rev() {
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
                    .add(camera_position)
                    .scale(16.)
                    .add(offset.scale(-1.))
                    .norm();

                let ray_direction = Vec3::new(17., 16., 8.).add(offset);

                let sampled_pixel_color = sample_pixel_color(ray_direction, ray_origin);
                pixel_color = sampled_pixel_color.scale(3.5).add(pixel_color);
            }

            print!(
                "{} {} {}\n",
                pixel_color.x as u32, pixel_color.y as u32, pixel_color.z as u32
            );
        }
    }
}

fn random() -> f32 {
    rand::thread_rng().gen()
}

fn trace(direction: Vec3, origin: Vec3, distance: &mut f32, surface_normal: &mut Vec3) -> i32 {
    *distance = 1e9;

    let mut material = 0;
    let intersection_distance = -direction.z / origin.z;

    if 0.01 < intersection_distance {
        *distance = intersection_distance;
        *surface_normal = Vec3::new(0., 0., 1.);
        material = 1;
    }

    for k in (0..19).rev() {
        for j in (0..9i32).rev() {
            let render_sphere = SPHERES_LOCATION[j as usize] & 1 << k != 0;
            if render_sphere {
                let p = direction.add(Vec3::new(-k as f32, 0., -j as f32 - 4.));

                let projection = p.dot(origin);
                let quadratic_coefficient = p.dot(p) - 1.;
                let discriminant = projection * projection - quadratic_coefficient;

                if discriminant > 0. {
                    let s = -projection - discriminant.sqrt();
                    if s < *distance && s > 0.01 {
                        *distance = s;
                        *surface_normal = p.add(origin.scale(*distance)).norm();
                        material = 2;
                    }
                }
            }
        }
    }

    material //material intersection 0 = None, 1 = plane, 2 = sphere
}

fn sample_pixel_color(direction: Vec3, origin: Vec3) -> Vec3 {
    let mut distance = 0.;

    let mut surface_normal = Vec3::zero();

    let material = trace(direction, origin, &mut distance, &mut surface_normal);

    if material == 0 {
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
        ) != 0
    {
        brightness = 0.;
    }

    if material & 1 != 0 {
        hit_point = hit_point.scale(0.2);

        let a1 = hit_point.x.ceil() + hit_point.y.ceil();

        let color = if a1 as i32 & 1 != 0 {
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
