use std::fmt::Display;

use web_sys::MouseEvent;
use yew::{Properties, function_component, Html, html, Callback, Classes, classes};

#[derive(PartialEq, Clone)]
pub enum FontawesomeIcon {
    Rust,
    GitHub,
    Camera,
    ShareNodes,
    Sun,
    Moon,
    House,
    Code,
    Envelope,
    ChevronDown,
    ChevronUp,
    ChevronLeft,
    ChevronRight,
}

impl Display for FontawesomeIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Rust => "fa-brands fa-rust",
            Self::GitHub => "fa-brands fa-github",
            Self::Camera => "fa-camera",
            Self::ShareNodes => "fa-share-nodes",
            Self::Sun => "fa-sun",
            Self::Moon => "fa-moon",
            Self::House => "fa-house",
            Self::Code => "fa-code",
            Self::Envelope => "fa-envelope",
            Self::ChevronDown => "fa-chevron-down",
            Self::ChevronUp => "fa-chevron-up",
            Self::ChevronLeft => "fa-chevron-left",
            Self::ChevronRight => "fa-chevron-right",
        })
    }
}

#[derive(PartialEq, Default)]
pub enum FontawesomeStyle {
    #[default]
    //Regular,
    Solid,
    Light,
    DuoTone,
    Thin,
    Sharp,
}

impl Display for FontawesomeStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            //Self::Regular => "fa-regular",
            Self::Solid => "fa-solid",
            Self::Light => "fa-light",
            Self::DuoTone => "fa-duotone",
            Self::Thin => "fa-thin",
            Self::Sharp => "fa-sharp",
        })
    }
}

#[derive(PartialEq, Default)]
pub enum FontawesomeSize {
    XXS,
    XS,
    S,
    #[default]
    Regular,
    L,
    XL,
    XXL
}

impl Display for FontawesomeSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::XXS => "fa-2xs",
            Self::XS => "fa-xs",
            Self::S => "fa-sm",
            Self::Regular => "",
            Self::L => "fa-lg",
            Self::XL => "fa-xl",
            Self::XXL => "fa-2xl"
        })
    }
}

#[derive(PartialEq, Default)]
pub enum FontawesomeSpinType {
    #[default]
    Regular,
    Pulse
}

#[derive(PartialEq, Default)]
pub enum FontawesomeSpinDirection {
    #[default]
    Clockwise,
    CounterClockwise
}

#[derive(PartialEq)]
pub enum FontawesomeAnimationType {
    Beat {
        scale: f32,
    },
    Fade {
        opacity: f32,
    },
    BeatFade {
        scale: f32,
        opacity: f32,
    },
    Bounce {
        rebound: f32,
        height: f32,
        squish_scale_x: f32,
        squish_scale_y: f32,
        jump_scale_x: f32,
        jump_scale_y: f32,
        land_scale_x: f32,
        land_scale_y: f32,
    },
    Flip {
        x: f32,
        y: f32,
        z: f32,
        angle: f32,
    },
    Shake,
    Spin {
        spin_type: FontawesomeSpinType,
        direction: FontawesomeSpinDirection
    }
}

impl FontawesomeAnimationType {
    fn class(&self) -> &'static str {
        match self {
            Self::Beat { scale: _ } => "fa-beat",
            Self::Fade { opacity: _ } => "fa-fade",
            Self::BeatFade { scale: _, opacity: _ } => "fa-beat-fade",
            Self::Bounce { rebound: _, height: _, squish_scale_x: _, squish_scale_y: _, jump_scale_x: _, jump_scale_y: _, land_scale_x: _, land_scale_y: _ } => "fa-bounce",
            Self::Flip { x: _, y: _, z: _, angle: _ } => "fa-flip",
            Self::Shake => "fa-shake",
            Self::Spin { spin_type, direction } =>
                match direction {
                    FontawesomeSpinDirection::Clockwise => 
                        match spin_type {
                            FontawesomeSpinType::Regular => "fa-spin",
                            FontawesomeSpinType::Pulse => "fa-spin-pulse",
                        },
                    FontawesomeSpinDirection::CounterClockwise =>
                        match spin_type {
                            FontawesomeSpinType::Regular => "fa-spin fa-spin-reverse",
                            FontawesomeSpinType::Pulse => "fa-spin-pulse fa-spin-reverse",
                        },
                }
        }
    }

    fn style(&self) -> String {
        match self {
            Self::Beat { scale } => format!("--fa-beat-scale: {scale};"),
            Self::Fade { opacity } => format!("--fa-fade-opacity: {opacity};"),
            Self::BeatFade { scale, opacity } => format!("--fa-beat-fade-opacity: {opacity}; --fa-beat-fade-scale: {scale};"),
            Self::Bounce {
                rebound,
                height,
                squish_scale_x,
                squish_scale_y,
                jump_scale_x,
                jump_scale_y,
                land_scale_x,
                land_scale_y
            } => format!("--fa-bounce-rebound: {rebound}; --fa-bounce-height: {height}; --fa-bounce-start-scale-x: {squish_scale_x}; --fa-bounce-start-scale-y: {squish_scale_y}; --fa-bounce-jump-scale-x: {jump_scale_x}; --fa-bounce-jump-scale-y: {jump_scale_y}; --fa-bounce-land-scale-x: {land_scale_x}; --fa-bounce-land-scale-y: {land_scale_y};"),
            Self::Flip { x, y, z, angle } => format!("--fa-flip-x: {x}; --fa-flip-y: {y}; --fa-flip-z: {z}; --fa-flip-angle: {angle};"),
            _ => "".to_owned(),
        }
    }
}

#[derive(PartialEq)]
pub enum AnimationTime {
    Seconds(f32),
    Milliseconds(f32),
}

impl AnimationTime {
    fn style_unit(&self) -> String {
        match self {
            Self::Seconds(seconds) => format!("{seconds}s"),
            Self::Milliseconds(millis) => format!("{millis}ms"),
        }
    }
}

impl Default for AnimationTime {
    fn default() -> Self {
        Self::Seconds(0.0)
    }
}

impl Display for AnimationTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.style_unit())
    }
}

#[derive(PartialEq)]
pub enum AnimationDuration {
    Time(AnimationTime),
    Infinite
}

impl Default for AnimationDuration {
    fn default() -> Self {
        Self::Time(Default::default())
    }
}

impl Display for AnimationDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::Time(time) => time.style_unit(),
            Self::Infinite => "infinite".to_owned(),
        })
    }
}

#[derive(PartialEq, Default)]
pub enum AnimationDirection {
    #[default]
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

impl Display for AnimationDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Normal => "normal",
            Self::Reverse => "reverse",
            Self::Alternate => "alternate",
            Self::AlternateReverse => "alternate-reverse"
        })
    }
}

#[derive(PartialEq)]
pub enum AnimationStepType {
    JumpStart,
    JumpEnd,
    JumpNone,
    JumpBoth,
    Start,
    End
}

impl Display for AnimationStepType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::JumpStart => "jump-start",
            Self::JumpEnd => "jump-end",
            Self::JumpNone => "jump-none",
            Self::JumpBoth => "jump-both",
            Self::Start => "start",
            Self::End => "end",
        })
    }
}

#[derive(PartialEq, Default)]
pub enum AnimationTiming {
    #[default]
    Ease,
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier(f32, f32, f32, f32),
    Steps(i32, AnimationStepType),
    StepStart,
    StepEnd,
}

impl Display for AnimationTiming {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&
            match self {
                Self::Ease => "ease".to_owned(),
                Self::Linear => "linear".to_owned(),
                Self::EaseIn => "ease-in".to_owned(),
                Self::EaseOut => "ease-out".to_owned(),
                Self::EaseInOut => "ease-in-out".to_owned(),
                Self::CubicBezier(p1, p2, p3, p4) => format!("cubic-bezier({p1}, {p2}, {p3}, {p4})"),
                Self::Steps(count, step_type) => format!("steps({count}, {step_type})"),
                Self::StepStart => "step-start".to_owned(),
                Self::StepEnd => "step-end".to_owned(),
            })
    }
}

#[derive(PartialEq)]
pub struct FontawesomeAnimation {
    duration: AnimationDuration,
    delay: AnimationTime,
    direction: AnimationDirection,
    iterations: AnimationDuration,
    timing: AnimationTiming,
    anim_type: FontawesomeAnimationType
}

impl FontawesomeAnimation {
    fn class(&self) -> &'static str {
        self.anim_type.class()
    }

    fn style(&self) -> String {
        format!(
            "--fa-animation-delay: {}; --fa-animation-direction: {}; --fa-animation-duration: {}; --fa-animation-iteration-count: {}; --fa-animation-timing: {}; {}",
            self.delay,
            self.direction,
            self.duration,
            self.iterations,
            self.timing,
            self.anim_type.style()
        )
    }
}


#[derive(Properties, PartialEq)]
pub struct FontawesomeProperties {
    pub icon: FontawesomeIcon,
    #[prop_or_default]
    pub style: FontawesomeStyle,
    #[prop_or_default]
    pub size: FontawesomeSize,
    #[prop_or_default]
    pub animation: Option<FontawesomeAnimation>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn FAIcon(props: &FontawesomeProperties) -> Html {
    html! {
        <i
            onclick={props.onclick.clone()}
            class={classes!(format!(
                "{} {} {} {}",
                props.icon,
                props.style,
                props.size,
                match props.animation {
                    None => "",
                    Some(ref animation) => animation.class()
                }
            ), props.class.clone())}
            style={format!(
                "{}",
                match props.animation {
                    None => "".to_owned(),
                    Some(ref animation) => animation.style()
                }
            )}></i>
    }
}