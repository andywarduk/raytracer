use raytracer_lib::{camera::Camera, float::*, triple::Vec3};

pub(crate) fn adjust_vfov(cam: &mut Camera, degrees: FltPrim, clear: &mut bool) {
    let vfov = cam.vfov();
    let new_vfov = (vfov + degrees).clamp(0.0, 180.0);

    if new_vfov != vfov {
        cam.set_vfov(new_vfov);
        *clear = true;
    }
}

pub(crate) fn adjust_focus(cam: &mut Camera, degrees: FltPrim, dist: FltPrim, clear: &mut bool) {
    let (defocus_angle, focus_distance) = cam.focus();

    let new_defocus_angle = (defocus_angle + degrees).clamp(0.0, 180.0);
    let new_focus_distance = (focus_distance + dist).max(0.0);

    if new_defocus_angle != defocus_angle || new_focus_distance != focus_distance {
        cam.set_focus(new_defocus_angle, new_focus_distance);
        *clear = true;
    }
}

pub(crate) fn adjust_depth(cam: &mut Camera, adjustment: i64, clear: &mut bool) {
    let depth = cam.max_depth();

    let new_depth = depth.saturating_add_signed(adjustment).max(1);

    if new_depth != depth {
        cam.set_max_depth(new_depth);
        *clear = true;
    }
}

pub(crate) fn adjust_view(
    cam: &mut Camera,
    look_from_adj: Vec3,
    look_at_adj: Vec3,
    vup_adj: Vec3,
    clear: &mut bool,
) {
    let (look_from, look_at, vup) = cam.view();

    let new_look_from = look_from + look_from_adj;
    let new_look_at = look_at + look_at_adj;
    let new_vup = vup + vup_adj;

    cam.set_view(new_look_from, new_look_at, new_vup);
    *clear = true;
}
