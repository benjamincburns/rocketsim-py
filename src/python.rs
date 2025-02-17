use pyo3::{exceptions::PyIndexError, prelude::*, types::PyTuple};
use rocketsim_rs::{
    autocxx::prelude::*,
    cxx::UniquePtr,
    glam_ext::glam::{Mat3A, Quat},
    sim as csim,
    sim::math::{Angle, RotMat as CRotMat, Vec3 as CVec3},
};

#[pyfunction]
#[inline]
pub fn init() {
    rocketsim_rs::init();
}

#[pyclass(module = "rocketsim.sim")]
#[derive(Clone, Copy, Debug)]
pub enum Team {
    Blue,
    Orange,
}

impl From<Team> for csim::car::Team {
    #[inline]
    fn from(team: Team) -> Self {
        match team {
            Team::Blue => Self::BLUE,
            Team::Orange => Self::ORANGE,
        }
    }
}

#[pyclass(module = "rocketsim.sim")]
#[derive(Clone, Copy, Debug, Default)]
pub enum GameMode {
    #[default]
    Soccar,
}

impl From<GameMode> for csim::arena::GameMode {
    #[inline]
    fn from(gamemode: GameMode) -> Self {
        match gamemode {
            GameMode::Soccar => Self::SOCCAR,
        }
    }
}

#[pyclass(get_all, set_all, module = "rocketsim.sim")]
#[derive(Clone, Copy, Debug, Default)]
pub struct BallHitInfo {
    car_id: u32,
    relative_pos_on_ball: Vec3,
    ball_pos: Vec3,
    extra_hit_vel: Vec3,
    tick_count_when_hit: u64,
}

impl From<csim::ball::BallHitInfo> for BallHitInfo {
    #[inline]
    fn from(hit: csim::ball::BallHitInfo) -> Self {
        Self {
            car_id: hit.car_id,
            relative_pos_on_ball: hit.relative_pos_on_ball.into(),
            ball_pos: hit.ball_pos.into(),
            extra_hit_vel: hit.extra_hit_vel.into(),
            tick_count_when_hit: hit.tick_count_when_hit,
        }
    }
}

impl From<BallHitInfo> for csim::ball::BallHitInfo {
    #[inline]
    fn from(hit: BallHitInfo) -> Self {
        Self {
            car_id: hit.car_id,
            relative_pos_on_ball: hit.relative_pos_on_ball.into(),
            ball_pos: hit.ball_pos.into(),
            extra_hit_vel: hit.extra_hit_vel.into(),
            tick_count_when_hit: hit.tick_count_when_hit,
        }
    }
}

#[pymethods]
impl BallHitInfo {
    #[new]
    #[inline]
    fn __new__() -> Self {
        Self::default()
    }
}

#[pyclass(get_all, set_all, module = "rocketsim.sim")]
#[derive(Clone, Copy, Debug, Default)]
pub struct Ball {
    pos: Vec3,
    vel: Vec3,
    ang_vel: Vec3,
    hit_info: BallHitInfo,
}

impl From<csim::ball::BallState> for Ball {
    #[inline]
    fn from(ball: csim::ball::BallState) -> Self {
        Self {
            pos: ball.pos.into(),
            vel: ball.vel.into(),
            ang_vel: ball.ang_vel.into(),
            hit_info: ball.hit_info.into(),
        }
    }
}

impl From<Ball> for csim::ball::BallState {
    #[inline]
    fn from(ball: Ball) -> Self {
        Self {
            pos: ball.pos.into(),
            vel: ball.vel.into(),
            ang_vel: ball.ang_vel.into(),
            hit_info: ball.hit_info.into(),
        }
    }
}

#[pymethods]
impl Ball {
    #[new]
    #[inline]
    fn __new__() -> Self {
        Self::default()
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}

#[pyclass(module = "rocketsim.sim")]
pub enum CarConfig {
    Octane,
    Dominus,
    Plank,
    Breakout,
    Hybrid,
    Merc,
}

impl From<&CarConfig> for &'static csim::car::CarConfig {
    #[inline]
    fn from(config: &CarConfig) -> Self {
        match config {
            CarConfig::Octane => csim::car::CarConfig::octane(),
            CarConfig::Dominus => csim::car::CarConfig::dominus(),
            CarConfig::Plank => csim::car::CarConfig::plank(),
            CarConfig::Breakout => csim::car::CarConfig::breakout(),
            CarConfig::Hybrid => csim::car::CarConfig::hybrid(),
            CarConfig::Merc => csim::car::CarConfig::merc(),
        }
    }
}

#[pyclass(get_all, set_all, module = "rocketsim.sim")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CarControls {
    throttle: f32,
    steer: f32,
    pitch: f32,
    yaw: f32,
    roll: f32,
    jump: bool,
    boost: bool,
    handbrake: bool,
}

impl From<csim::CarControls> for CarControls {
    #[inline]
    fn from(controls: csim::CarControls) -> Self {
        Self {
            throttle: controls.throttle,
            steer: controls.steer,
            pitch: controls.pitch,
            yaw: controls.yaw,
            roll: controls.roll,
            jump: controls.jump,
            boost: controls.boost,
            handbrake: controls.handbrake,
        }
    }
}

impl From<&CarControls> for csim::CarControls {
    #[inline]
    fn from(controls: &CarControls) -> Self {
        Self {
            throttle: controls.throttle,
            steer: controls.steer,
            pitch: controls.pitch,
            yaw: controls.yaw,
            roll: controls.roll,
            jump: controls.jump,
            boost: controls.boost,
            handbrake: controls.handbrake,
        }
    }
}

impl From<CarControls> for csim::CarControls {
    #[inline]
    fn from(controls: CarControls) -> Self {
        Self {
            throttle: controls.throttle,
            steer: controls.steer,
            pitch: controls.pitch,
            yaw: controls.yaw,
            roll: controls.roll,
            jump: controls.jump,
            boost: controls.boost,
            handbrake: controls.handbrake,
        }
    }
}

#[pymethods]
impl CarControls {
    const NAMES: [&str; 8] = ["throttle", "steer", "pitch", "yaw", "roll", "jump", "boost", "handbrake"];

    #[new]
    #[pyo3(signature = (*args, **kwargs))]
    fn new(args: &PyTuple, kwargs: Option<&PyAny>) -> Self {
        if let Ok(args) = args.get_item(0).and_then(PyAny::extract) {
            return args;
        }

        let mut vec = [None; Self::NAMES.len()];

        if let Ok(args) = args.get_item(0).and_then(PyAny::extract::<Vec<f32>>) {
            vec.iter_mut().zip(args.into_iter()).for_each(|(a, b)| *a = Some(b));
        } else if let Ok(args) = args.extract::<Vec<f32>>() {
            vec.iter_mut().zip(args.into_iter()).for_each(|(a, b)| *a = Some(b));
        } else {
            for (a, b) in vec.iter_mut().zip(args.into_iter()) {
                if let Ok(x) = b.extract() {
                    *a = Some(x);
                }
            }
        }

        if let Some(kwargs) = kwargs {
            for (a, b) in vec.iter_mut().zip(Self::NAMES.into_iter()) {
                if let Ok(x) = kwargs.get_item(b).and_then(PyAny::extract) {
                    *a = Some(x);
                }
            }
        }

        Self {
            throttle: vec[0].unwrap_or_default(),
            steer: vec[1].unwrap_or_default(),
            pitch: vec[2].unwrap_or_default(),
            yaw: vec[3].unwrap_or_default(),
            roll: vec[4].unwrap_or_default(),
            jump: vec[5].unwrap_or_default() as u8 != 0,
            boost: vec[6].unwrap_or_default() as u8 != 0,
            handbrake: vec[7].unwrap_or_default() as u8 != 0,
        }
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    fn __repr__(&self) -> String {
        format!(
            "CarControls(throttle={}, steer={}, pitch={}, yaw={}, roll={}, jump={}, boost={}, handbrake={})",
            self.throttle, self.steer, self.pitch, self.yaw, self.roll, self.jump, self.boost, self.handbrake
        )
    }
}

#[pyclass(get_all, set_all, module = "rocketsim.sim")]
#[derive(Clone, Copy, Debug, Default)]
pub struct Car {
    pos: Vec3,
    rot_mat: RotMat,
    vel: Vec3,
    ang_vel: Vec3,
    is_on_ground: bool,
    has_jumped: bool,
    has_double_jumped: bool,
    has_flipped: bool,
    last_rel_dodge_torque: Vec3,
    jump_time: f32,
    flip_time: f32,
    is_jumping: bool,
    air_time_since_jump: f32,
    boost: f32,
    time_spent_boosting: f32,
    is_supersonic: bool,
    supersonic_time: f32,
    handbrake_val: f32,
    is_auto_flipping: bool,
    auto_flip_timer: f32,
    auto_flip_torque_scale: f32,
    has_contact: bool,
    contact_normal: Vec3,
    other_car_id: u32,
    cooldown_timer: f32,
    is_demoed: bool,
    demo_respawn_timer: f32,
    last_hit_ball_tick: u64,
    last_controls: CarControls,
}

impl From<csim::car::CarState> for Car {
    #[inline]
    fn from(car: csim::car::CarState) -> Self {
        Self {
            pos: car.pos.into(),
            rot_mat: car.rot_mat.into(),
            vel: car.vel.into(),
            ang_vel: car.ang_vel.into(),
            is_on_ground: car.is_on_ground,
            has_jumped: car.has_jumped,
            has_double_jumped: car.has_double_jumped,
            has_flipped: car.has_flipped,
            last_rel_dodge_torque: car.last_rel_dodge_torque.into(),
            jump_time: car.jump_time,
            flip_time: car.flip_time,
            is_jumping: car.is_jumping,
            air_time_since_jump: car.air_time_since_jump,
            boost: car.boost,
            time_spent_boosting: car.time_spent_boosting,
            is_supersonic: car.is_supersonic,
            supersonic_time: car.supersonic_time,
            handbrake_val: car.handbrake_val,
            is_auto_flipping: car.is_auto_flipping,
            auto_flip_timer: car.auto_flip_timer,
            auto_flip_torque_scale: car.auto_flip_torque_scale,
            has_contact: car.has_contact,
            contact_normal: car.contact_normal.into(),
            other_car_id: car.other_car_id,
            cooldown_timer: car.cooldown_timer,
            is_demoed: car.is_demoed,
            demo_respawn_timer: car.demo_respawn_timer,
            last_hit_ball_tick: car.last_hit_ball_tick,
            last_controls: car.last_controls.into(),
        }
    }
}

impl From<Car> for csim::car::CarState {
    #[inline]
    fn from(car: Car) -> Self {
        Self {
            pos: car.pos.into(),
            rot_mat: car.rot_mat.into(),
            vel: car.vel.into(),
            ang_vel: car.ang_vel.into(),
            is_on_ground: car.is_on_ground,
            has_jumped: car.has_jumped,
            has_double_jumped: car.has_double_jumped,
            has_flipped: car.has_flipped,
            last_rel_dodge_torque: car.last_rel_dodge_torque.into(),
            jump_time: car.jump_time,
            flip_time: car.flip_time,
            is_jumping: car.is_jumping,
            air_time_since_jump: car.air_time_since_jump,
            boost: car.boost,
            time_spent_boosting: car.time_spent_boosting,
            is_supersonic: car.is_supersonic,
            supersonic_time: car.supersonic_time,
            handbrake_val: car.handbrake_val,
            is_auto_flipping: car.is_auto_flipping,
            auto_flip_timer: car.auto_flip_timer,
            auto_flip_torque_scale: car.auto_flip_torque_scale,
            has_contact: car.has_contact,
            contact_normal: car.contact_normal.into(),
            other_car_id: car.other_car_id,
            cooldown_timer: car.cooldown_timer,
            is_demoed: car.is_demoed,
            demo_respawn_timer: car.demo_respawn_timer,
            last_hit_ball_tick: car.last_hit_ball_tick,
            last_controls: car.last_controls.into(),
        }
    }
}

#[pymethods]
impl Car {
    #[new]
    #[inline]
    fn __new__() -> Self {
        Self::default()
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    fn get_contacting_car(&self, arena: &mut Arena) -> Option<Self> {
        csim::car::CarState::from(*self).get_contacting_car(arena.0.pin_mut()).map(Into::into)
    }
}

#[pyclass(get_all, module = "rocketsim.sim")]
#[derive(Clone, Debug)]
pub struct BoostPad {
    pos: Vec3,
    is_big: bool,
}

#[pymethods]
impl BoostPad {
    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}

#[pyclass(set_all, get_all, module = "rocketsim.sim")]
#[derive(Clone, Copy, Debug, Default)]
pub struct BoostPadState {
    pub is_active: bool,
    pub cooldown: f32,
    pub cur_locked_car_id: u32,
    pub prev_locked_car_id: u32,
}

impl From<csim::boostpad::BoostPadState> for BoostPadState {
    #[inline]
    fn from(boost_pad_state: csim::boostpad::BoostPadState) -> Self {
        Self {
            is_active: boost_pad_state.is_active,
            cooldown: boost_pad_state.cooldown,
            cur_locked_car_id: boost_pad_state.cur_locked_car_id,
            prev_locked_car_id: boost_pad_state.prev_locked_car_id,
        }
    }
}

impl From<&BoostPadState> for csim::boostpad::BoostPadState {
    #[inline]
    fn from(boost_pad_state: &BoostPadState) -> Self {
        Self {
            is_active: boost_pad_state.is_active,
            cooldown: boost_pad_state.cooldown,
            cur_locked_car_id: boost_pad_state.cur_locked_car_id,
            prev_locked_car_id: boost_pad_state.prev_locked_car_id,
        }
    }
}

#[pymethods]
impl BoostPadState {
    #[new]
    #[inline]
    fn __new__(is_active: bool, cooldown: f32, cur_locked_car_id: u32, prev_locked_car_id: u32) -> Self {
        Self {
            is_active,
            cooldown,
            cur_locked_car_id,
            prev_locked_car_id,
        }
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    fn __repr__(&self) -> String {
        format!(
            "BoostPadState({}, {}, {}, {})",
            self.is_active, self.cooldown, self.cur_locked_car_id, self.prev_locked_car_id
        )
    }
}

#[pyclass(unsendable, module = "rocketsim.sim")]
#[repr(transparent)]
pub struct Arena(UniquePtr<csim::arena::Arena>);

#[pymethods]
impl Arena {
    #[new]
    #[inline]
    fn __new__(gamemode: Option<GameMode>, tick_rate: Option<f32>) -> Self {
        Self(csim::arena::Arena::new(gamemode.unwrap_or_default().into(), tick_rate.unwrap_or(120.)).within_unique_ptr())
    }

    #[inline]
    fn get_tick_rate(&self) -> f32 {
        self.0.get_tick_rate()
    }

    #[inline]
    fn get_tick_count(&self) -> u64 {
        self.0.get_tick_count()
    }

    #[inline]
    fn step(&mut self, ticks_to_simulate: Option<i32>) {
        self.0.pin_mut().step(ticks_to_simulate.unwrap_or(1));
    }

    #[inline]
    fn get_ball(&mut self) -> Ball {
        self.0.pin_mut().get_ball().into()
    }

    #[inline]
    fn set_ball(&mut self, ball: Ball) {
        self.0.pin_mut().set_ball(ball.into());
    }

    #[inline]
    fn num_cars(&self) -> usize {
        self.0.num_cars()
    }

    #[inline]
    fn add_car(&mut self, team: Team, config: &CarConfig) -> u32 {
        self.0.pin_mut().add_car(team.into(), config.into())
    }

    #[inline]
    fn set_car_controls(&mut self, id: u32, controls: &CarControls) -> PyResult<()> {
        self.0.pin_mut().set_car_controls(id, controls.into()).map_err(|e| PyIndexError::new_err(e.to_string()))
    }

    #[inline]
    fn set_all_controls(&mut self, controls: Vec<(u32, CarControls)>) -> PyResult<()> {
        self.0
            .pin_mut()
            .set_all_controls(&controls.iter().map(|(id, controls)| (*id, controls.into())).collect::<Vec<_>>())
            .map_err(|e| PyIndexError::new_err(e.to_string()))
    }

    #[inline]
    fn get_car(&mut self, id: u32) -> Car {
        self.0.pin_mut().get_car(id).into()
    }

    #[inline]
    fn set_car(&mut self, id: u32, car: Car) -> PyResult<()> {
        self.0.pin_mut().set_car(id, car.into()).map_err(|e| PyIndexError::new_err(e.to_string()))
    }

    #[inline]
    fn num_pads(&self) -> usize {
        self.0.num_pads()
    }

    #[inline]
    fn get_pad_static(&self, index: usize) -> BoostPad {
        BoostPad {
            pos: self.0.get_pad_pos(index).into(),
            is_big: self.0.get_pad_is_big(index),
        }
    }

    #[inline]
    fn get_pad_state(&self, index: usize) -> BoostPadState {
        self.0.get_pad_state(index).into()
    }

    #[inline]
    fn set_pad_state(&mut self, index: usize, state: &BoostPadState) {
        self.0.pin_mut().set_pad_state(index, state.into())
    }
}

#[pyclass(get_all, set_all, module = "rocketsim")]
#[derive(Clone, Copy, Debug, Default)]
pub struct RotMat {
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
}

impl From<CRotMat> for RotMat {
    #[inline]
    fn from(rot_mat: CRotMat) -> Self {
        Self {
            forward: rot_mat.forward.into(),
            right: rot_mat.right.into(),
            up: rot_mat.up.into(),
        }
    }
}

impl From<RotMat> for CRotMat {
    #[inline]
    fn from(rot_mat: RotMat) -> Self {
        Self {
            forward: rot_mat.forward.into(),
            right: rot_mat.right.into(),
            up: rot_mat.up.into(),
        }
    }
}

#[pymethods]
impl RotMat {
    #[new]
    #[inline]
    fn __new__(forward: Vec3, right: Vec3, up: Vec3) -> Self {
        Self { forward, right, up }
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    fn __repr__(&self) -> String {
        format!("RotMat({}, {}, {})", self.forward.__repr__(), self.right.__repr__(), self.up.__repr__())
    }

    #[inline]
    #[staticmethod]
    fn from_angles(pitch: f32, yaw: f32, roll: f32) -> Self {
        CRotMat::from(Mat3A::from_quat(Quat::from(Angle { pitch, yaw, roll }))).into()
    }
}

#[pyclass(get_all, set_all, module = "rocketsim")]
#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<CVec3> for Vec3 {
    #[inline]
    fn from(vec3: CVec3) -> Self {
        Self { x: vec3.x, y: vec3.y, z: vec3.z }
    }
}

impl From<Vec3> for CVec3 {
    #[inline]
    fn from(vec3: Vec3) -> Self {
        CVec3 {
            x: vec3.x,
            y: vec3.y,
            z: vec3.z,
            _w: 0.,
        }
    }
}

#[pymethods]
impl Vec3 {
    #[new]
    #[inline]
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    fn with_x(&mut self, x: f32) -> Self {
        Self::new(x, self.y, self.z)
    }

    #[inline]
    fn with_y(&mut self, y: f32) -> Self {
        Self::new(self.x, y, self.z)
    }

    #[inline]
    fn with_z(&self, z: f32) -> Self {
        Self::new(self.x, self.y, z)
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    fn __repr__(&self) -> String {
        format!("Vec3({}, {}, {})", self.x, self.y, self.z)
    }
}
