use minifb::{InputCallback, Window};
use raytracer_lib::{float::*, triple::Vec3};
use std::{cell::RefCell, rc::Rc};

use crate::MainParms;

use super::{
    adjust::{adjust_depth, adjust_focus, adjust_vfov, adjust_view},
    WinState,
};

type KeyVec = Rc<RefCell<Vec<u32>>>;

struct Input {
    keys: KeyVec,
}

impl InputCallback for Input {
    /// Will be called every time a character key is pressed
    fn add_char(&mut self, uni_char: u32) {
        self.keys.borrow_mut().push(uni_char);
    }
}

pub(super) fn setup_keys(window: &mut Window) -> Rc<RefCell<Vec<u32>>> {
    // Set up key buffer
    let keys = KeyVec::new(RefCell::new(Vec::new()));
    window.set_input_callback(Box::new(Input { keys: keys.clone() }));
    keys
}

pub(super) fn print_help() {
    println!("Key bindings:");
    println!("  ? => Print this help again");
    println!("  Camera position:");
    println!("    Q/q W/w E/e => Z- Y+ Z+");
    println!("    A/a S/s D/d => X- Y- X+ ");
    println!("  Camera target:");
    println!("    R/r T/t Y/y => Z- Y+ Z+");
    println!("    F/f G/g H/h => X- Y- X+ ");
    println!("  Camera orientation:");
    println!("    U/u I/i O/o => Z- Y+ Z+");
    println!("    J/j K/k L/l => X- Y- X+ ");
    println!("  Vertical FOV:");
    println!("    z/Z x/X => Decrease / increase vertical field of vision angle");
    println!("  Focus:");
    println!("    c/C v/V => Decrease / increase defocus angle");
    println!("    b/B n/N => Decrease / increase focus distance");
    println!("  Depth:");
    println!("    [/{{ ]/}} => Decrease / increase ray depth (number of bounces)");
}

pub(super) fn process_keys(
    state: &mut MainParms,
    winstate: &WinState,
    keys: &Rc<RefCell<Vec<u32>>>,
) -> bool {
    let mut clear = false;

    let mut keys = keys.borrow_mut();

    for t in keys.iter() {
        match char::from_u32(*t) {
            // Vertical FOV
            Some('z') => adjust_vfov(&mut state.cam, -1.0, &mut clear),
            Some('Z') => adjust_vfov(&mut state.cam, -5.0, &mut clear),
            Some('x') => adjust_vfov(&mut state.cam, 1.0, &mut clear),
            Some('X') => adjust_vfov(&mut state.cam, 5.0, &mut clear),
            // Focus
            Some('c') => adjust_focus(&mut state.cam, -0.1, 0.0, &mut clear),
            Some('C') => adjust_focus(&mut state.cam, -1.0, 0.0, &mut clear),
            Some('v') => adjust_focus(&mut state.cam, 0.1, 0.0, &mut clear),
            Some('V') => adjust_focus(&mut state.cam, 1.0, 0.0, &mut clear),
            Some('b') => adjust_focus(&mut state.cam, 0.0, -1.0, &mut clear),
            Some('B') => adjust_focus(&mut state.cam, 0.0, -10.0, &mut clear),
            Some('n') => adjust_focus(&mut state.cam, 0.0, 1.0, &mut clear),
            Some('N') => adjust_focus(&mut state.cam, 0.0, 10.0, &mut clear),
            // Depth
            Some('[') => adjust_depth(&mut state.cam, -1, &mut clear),
            Some('{') => adjust_depth(&mut state.cam, -5, &mut clear),
            Some(']') => adjust_depth(&mut state.cam, 1, &mut clear),
            Some('}') => adjust_depth(&mut state.cam, 5, &mut clear),
            // Camera position
            Some('q') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, flt_prim(-winstate.move_delta_small)),
                Vec3::default(),
                Vec3::default(),
                &mut clear,
            ),
            Some('Q') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, flt_prim(-winstate.move_delta_big)),
                Vec3::default(),
                Vec3::default(),
                &mut clear,
            ),
            Some('w') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, flt_prim(winstate.move_delta_small), 0.0),
                Vec3::default(),
                Vec3::default(),
                &mut clear,
            ),
            Some('W') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, flt_prim(winstate.move_delta_big), 0.0),
                Vec3::default(),
                Vec3::default(),
                &mut clear,
            ),
            Some('e') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, flt_prim(winstate.move_delta_small)),
                Vec3::default(),
                Vec3::default(),
                &mut clear,
            ),
            Some('E') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, flt_prim(winstate.move_delta_big)),
                Vec3::default(),
                Vec3::default(),
                &mut clear,
            ),
            Some('a') => adjust_view(
                &mut state.cam,
                Vec3::new(flt_prim(-winstate.move_delta_small), 0.0, 0.0),
                Vec3::default(),
                Vec3::default(),
                &mut clear,
            ),
            Some('A') => adjust_view(
                &mut state.cam,
                Vec3::new(flt_prim(-winstate.move_delta_big), 0.0, 0.0),
                Vec3::default(),
                Vec3::default(),
                &mut clear,
            ),
            Some('s') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, flt_prim(-winstate.move_delta_small), 0.0),
                Vec3::default(),
                Vec3::default(),
                &mut clear,
            ),
            Some('S') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, flt_prim(-winstate.move_delta_big), 0.0),
                Vec3::default(),
                Vec3::default(),
                &mut clear,
            ),
            Some('d') => adjust_view(
                &mut state.cam,
                Vec3::new(flt_prim(winstate.move_delta_small), 0.0, 0.0),
                Vec3::default(),
                Vec3::default(),
                &mut clear,
            ),
            Some('D') => adjust_view(
                &mut state.cam,
                Vec3::new(flt_prim(winstate.move_delta_big), 0.0, 0.0),
                Vec3::default(),
                Vec3::default(),
                &mut clear,
            ),
            // Camera target
            Some('r') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::new(0.0, 0.0, flt_prim(-winstate.move_delta_small)),
                Vec3::default(),
                &mut clear,
            ),
            Some('R') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::new(0.0, 0.0, flt_prim(-winstate.move_delta_big)),
                Vec3::default(),
                &mut clear,
            ),
            Some('t') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::new(0.0, flt_prim(winstate.move_delta_small), 0.0),
                Vec3::default(),
                &mut clear,
            ),
            Some('T') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::new(0.0, flt_prim(winstate.move_delta_big), 0.0),
                Vec3::default(),
                &mut clear,
            ),
            Some('y') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::new(0.0, 0.0, flt_prim(winstate.move_delta_small)),
                Vec3::default(),
                &mut clear,
            ),
            Some('Y') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::new(0.0, 0.0, flt_prim(winstate.move_delta_big)),
                Vec3::default(),
                &mut clear,
            ),
            Some('f') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::new(flt_prim(-winstate.move_delta_small), 0.0, 0.0),
                Vec3::default(),
                &mut clear,
            ),
            Some('F') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::new(flt_prim(-winstate.move_delta_big), 0.0, 0.0),
                Vec3::default(),
                &mut clear,
            ),
            Some('g') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::new(0.0, flt_prim(-winstate.move_delta_small), 0.0),
                Vec3::default(),
                &mut clear,
            ),
            Some('G') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::new(0.0, flt_prim(-winstate.move_delta_big), 0.0),
                Vec3::default(),
                &mut clear,
            ),
            Some('h') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::new(flt_prim(winstate.move_delta_small), 0.0, 0.0),
                Vec3::default(),
                &mut clear,
            ),
            Some('H') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::new(flt_prim(winstate.move_delta_big), 0.0, 0.0),
                Vec3::default(),
                &mut clear,
            ),
            // Camera orienation
            Some('u') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::default(),
                Vec3::new(0.0, 0.0, -1.0),
                &mut clear,
            ),
            Some('U') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::default(),
                Vec3::new(0.0, 0.0, -10.0),
                &mut clear,
            ),
            Some('i') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::default(),
                Vec3::new(0.0, 1.0, 0.0),
                &mut clear,
            ),
            Some('I') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::default(),
                Vec3::new(0.0, 10.0, 0.0),
                &mut clear,
            ),
            Some('o') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::default(),
                Vec3::new(0.0, 0.0, 1.0),
                &mut clear,
            ),
            Some('O') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::default(),
                Vec3::new(0.0, 0.0, 10.0),
                &mut clear,
            ),
            Some('j') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::default(),
                Vec3::new(-1.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('J') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::default(),
                Vec3::new(-10.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('k') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::default(),
                Vec3::new(0.0, -1.0, 0.0),
                &mut clear,
            ),
            Some('K') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::default(),
                Vec3::new(0.0, -10.0, 0.0),
                &mut clear,
            ),
            Some('l') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::default(),
                Vec3::new(1.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('L') => adjust_view(
                &mut state.cam,
                Vec3::default(),
                Vec3::default(),
                Vec3::new(10.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('?') => print_help(),
            // Catch others
            _ => (),
        }
    }

    keys.clear();

    clear
}
