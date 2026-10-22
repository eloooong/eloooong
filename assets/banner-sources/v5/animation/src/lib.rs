use fframes::{
    AudioMap, Color, Duration, FFramesContext, Frame, Svgr, Video,
    animation::{Easing, KeyFramesAnimation}, include_media_dir,
};

include_media_dir!(pub struct SpotistatsBannerMedia, "media");

mod other;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Project { Spoti, Civica, Keep, Rentify }

pub struct SpotistatsBannerVideo<'m, const MOBILE: bool> {
    media: &'m SpotistatsBannerMedia,
    rotation: KeyFramesAnimation<f32>,
    project: Project,
}

impl<'m, const MOBILE: bool> SpotistatsBannerVideo<'m, MOBILE> {
    pub fn new(media: &'m SpotistatsBannerMedia, project: Project) -> Self {
        Self { media, project, rotation: fframes::timeline!(
            at 0.0 => 6.0, animate 0.0_f32 => 360.0_f32, Easing::Linear,
        ) }
    }
}

impl<const MOBILE: bool> std::fmt::Debug for SpotistatsBannerVideo<'_, MOBILE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpotistatsBanner").field("mobile", &MOBILE).finish()
    }
}

impl<const MOBILE: bool> Video for SpotistatsBannerVideo<'_, MOBILE> {
    const FPS: usize = 20;
    const WIDTH: usize = if MOBILE { 640 } else { 1280 };
    const HEIGHT: usize = if MOBILE { 334 } else { 252 };
    const BACKGROUND_COLOR: Color = Color::BLACK;

    fn duration(&self) -> Duration<'_> { Duration::Seconds(6.0) }
    fn audio(&self) -> AudioMap<'_> { AudioMap::none() }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        if self.project != Project::Spoti { return self.render_other(frame); }
        let t = frame.seconds();
        let bases = [15.0_f32, 26.0, 42.0, 61.0, 35.0, 51.0, 70.0, 44.0, 25.0, 38.0, 16.0];
        let bars: Vec<Svgr<'a>> = bases.iter().enumerate().map(|(i, base)| {
            let phase = i as f32 * 0.68;
            let height = base * (0.78
                + 0.20 * (std::f32::consts::TAU * t / 1.5 + phase).sin()
                + 0.08 * (std::f32::consts::TAU * t / 6.0 + phase * 0.75).sin());
            let x = 210.0 + i as f32 * 13.0;
            let y = 114.0 - height / 2.0;
            let opacity = 0.58 + i as f32 * 0.035;
            fframes::svgr!(<rect x={x} y={y} width="6" height={height} rx="3" fill="#1ed760" opacity={opacity} />)
        }).collect();
        let angle = frame.animate(&self.rotation);
        let record_rotation = format!("rotate({angle} 105 110)");
        let illustration = if MOBILE {
            "translate(333 185) scale(.57) rotate(-5 210 110)"
        } else {
            "translate(792 36) scale(.81) rotate(-5 210 110)"
        };
        let base = if MOBILE { self.media.mobile_base_png.href() } else { self.media.desktop_base_png.href() };
        fframes::svgr!(
            <svg xmlns="http://www.w3.org/2000/svg" width={Self::WIDTH} height={Self::HEIGHT}>
                <image x="0" y="0" width={Self::WIDTH} height={Self::HEIGHT} href={base} />
                <g transform={illustration}>
                    <circle cx="105" cy="110" r="73" fill="#111512" stroke="#424a44" />
                    <circle cx="105" cy="110" r="59" fill="none" stroke="#28302a" />
                    <circle cx="105" cy="110" r="45" fill="none" stroke="#28302a" />
                    <g transform={record_rotation}>
                        <path d="M49 98A57 57 0 0 1 79 59M158 123A57 57 0 0 1 139 156" fill="none" stroke="#66816d" stroke-width="3" stroke-linecap="round" opacity=".3" />
                        <circle cx="105" cy="110" r="26" fill="#1ed760" />
                        <path d="M91 102H108M91 109H99M114 117H121" stroke="#0c6c32" stroke-width="3" stroke-linecap="round" />
                        <circle cx="105" cy="110" r="5" fill="#111512" />
                    </g>
                    {bars}
                </g>
            </svg>
        )
    }
}
