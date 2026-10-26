use fframes::{AudioMap, Color, Duration, FFramesContext, Frame, Svgr, Video};
use crate::SpotistatsBannerMedia;

#[derive(Debug)]
pub struct ProfileBannerVideo<'m, const MOBILE: bool> {
    media: &'m SpotistatsBannerMedia,
}

impl<'m, const MOBILE: bool> ProfileBannerVideo<'m, MOBILE> {
    pub fn new(media: &'m SpotistatsBannerMedia) -> Self { Self { media } }
}

impl<const MOBILE: bool> Video for ProfileBannerVideo<'_, MOBILE> {
    const FPS: usize = 20;
    const WIDTH: usize = if MOBILE { 640 } else { 1280 };
    const HEIGHT: usize = if MOBILE { 400 } else { 338 };
    const BACKGROUND_COLOR: Color = Color::BLACK;

    fn duration(&self) -> Duration<'_> { Duration::Seconds(6.0) }
    fn audio(&self) -> AudioMap<'_> { AudioMap::none() }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let phase = std::f32::consts::TAU * t / 6.0;
        let ambient = 0.82 + 0.18 * phase.cos();
        let (cx, cy) = if MOBILE { (570, 240) } else { (1050, 140) };
        let waves: Vec<Svgr<'a>> = (0..if MOBILE { 15 } else { 10 }).map(|i| {
            let y = i as f32 * 32.0 + 6.0 * phase.sin();
            let bend = 8.0 * (phase + i as f32 * 0.18).sin();
            let path = if MOBILE {
                format!("M360 {} C460 {} 440 {} 640 {}", y+25.0, y-40.0+bend, y+100.0-bend, y+5.0)
            } else {
                format!("M640 {} C790 {} 1080 {} 1280 {}", y+25.0, y-40.0+bend, y+100.0-bend, y+5.0)
            };
            fframes::svgr!(<path d={path} fill="none" stroke="#dedede" stroke-opacity=".045" />)
        }).collect();
        let (name, layers) = if MOBILE {
            (self.media.header_mobile_name_png.href(), [
                self.media.header_mobile_0_png.href(), self.media.header_mobile_1_png.href(),
                self.media.header_mobile_2_png.href(), self.media.header_mobile_3_png.href(),
            ])
        } else {
            (self.media.header_desktop_name_png.href(), [
                self.media.header_desktop_0_png.href(), self.media.header_desktop_1_png.href(),
                self.media.header_desktop_2_png.href(), self.media.header_desktop_3_png.href(),
            ])
        };
        let colors = ["#3178c6", "#61dafb", "#f0f6fc", "#3ecf8e"];
        let badges: Vec<Svgr<'a>> = layers.into_iter().enumerate().map(|(i, href)| {
            // One badge gently lifts at a time; zero velocity at both loop boundaries.
            let local = (t - i as f32 * 1.5).rem_euclid(6.0);
            let pulse = if local < 1.5 { (std::f32::consts::PI * local / 1.5).sin().powi(2) } else { 0.0 };
            let width = if MOBILE { 278 } else { 216 };
            let x = if MOBILE { 32 + (i % 2) * 298 } else { 752 + (i % 2) * 232 };
            let y = if MOBILE { 218 + (i / 2) * 88 } else { 89 + (i / 2) * 88 };
            let transform = format!("translate({x} {})", y as f32 - 3.0 * pulse);
            let opacity = pulse * 0.42;
            let color = colors[i];
            fframes::svgr!(<g transform={transform}>
                <image x="-4" y="-4" width={width+8} height="80" href={href} />
                <rect x="0" y="0" width={width} height="72" rx="10" fill="none" stroke={color} stroke-width="1.2" opacity={opacity} />
            </g>)
        }).collect();
        fframes::svgr!(<svg xmlns="http://www.w3.org/2000/svg" width={Self::WIDTH} height={Self::HEIGHT}>
            <defs><radialGradient id="header-ambient"><stop stop-color="#262626" /><stop offset="1" stop-color="#101010" /></radialGradient></defs>
            <rect width={Self::WIDTH} height={Self::HEIGHT} fill="#101010" />
            <ellipse cx={cx} cy={cy} rx="470" ry="320" fill="url(#header-ambient)" opacity={ambient} />
            {waves}
            <image x="0" y="0" width={Self::WIDTH} height={Self::HEIGHT} href={name} />
            {badges}
        </svg>)
    }
}
