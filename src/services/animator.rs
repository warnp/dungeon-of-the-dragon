use ggez::graphics::Rect;
use keyframe::{AnimationSequence, keyframes, functions};
use keyframe_derive::CanTween;

pub struct Animator {
    sequences: Vec<AnimationSequence<Tweenable>>,
}

impl Animator {
    pub fn new() -> Self {
        let keyframes = vec![keyframes![
            (
                Tweenable::new(0., 0., 1./4.,1./4.),
                0.,
                functions::Step
            ),
            (
                Tweenable::new(1./4., 0., 1./4.,1./4.),
                0.25,
                functions::Step
            ),
            (
                Tweenable::new(2./4., 0., 1./4.,1./4.),
                0.5,
                functions::Step
            ),
            (
                Tweenable::new(3./4., 0., 1./4.,1./4.),
                0.75,
                functions::Step
            ),
            (
                Tweenable::new(0., 0., 1./4.,1./4.),
                1.,
                functions::Step
            )
        ],keyframes![
            (
                Tweenable::new(0., 1./4., 1./4.,1./4.),
                0.,
                functions::Step
            ),
            (
                Tweenable::new(1./4., 1./4., 1./4.,1./4.),
                0.25,
                functions::Step
            ),
            (
                Tweenable::new(2./4., 1./4., 1./4.,1./4.),
                0.5,
                functions::Step
            ),
            (
                Tweenable::new(3./4., 1./4., 1./4.,1./4.),
                0.75,
                functions::Step
            ),
            (
                Tweenable::new(0., 1./4., 1./4.,1./4.),
                1.,
                functions::Step
            )
        ]];
        Self {
            sequences: keyframes
        }
    }

    pub fn advance(&mut self, duration: f64, delta: f64) {
        self.sequences.iter_mut()
            .for_each(|s| { s.advance_and_maybe_wrap(duration * delta); });
    }

    pub fn get_currenct_rect(&self, index: usize) -> Rect {
        self.sequences.get(index).unwrap().now_strict().unwrap().into()
    }
}

#[derive(CanTween, Clone, Copy)]
pub struct Tweenable {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Tweenable {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            x,
            y,
            w,
            h,
        }
    }
}

impl From<Tweenable> for Rect {
    fn from(value: Tweenable) -> Self {
        Rect {
            x: value.x,
            y: value.y,
            w: value.w,
            h: value.h,
        }
    }
}
