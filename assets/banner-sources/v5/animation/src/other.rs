use crate::{Project, SpotistatsBannerVideo};
use fframes::{Frame, Svgr, Video};
use std::f32::consts::TAU;

impl<const MOBILE: bool> SpotistatsBannerVideo<'_, MOBILE> {
    pub(crate) fn render_other<'a>(&'a self, frame: Frame) -> Svgr<'a> {
        let t = frame.seconds();
        let phase = TAU * t / 6.0;
        let base = match (self.project, MOBILE) {
            (Project::Civica, false) => self.media.civica_desktop_png.href(),
            (Project::Civica, true) => self.media.civica_mobile_png.href(),
            (Project::Keep, false) => self.media.keep_desktop_png.href(),
            (Project::Keep, true) => self.media.keep_mobile_png.href(),
            (Project::Rentify, false) => self.media.rentify_desktop_png.href(),
            (Project::Rentify, true) => self.media.rentify_mobile_png.href(),
            _ => return Svgr::empty(),
        };
        let illustration = if MOBILE {
            "translate(333 185) scale(.57) rotate(-5 210 110)"
        } else {
            "translate(792 36) scale(.81) rotate(-5 210 110)"
        };
        let motion = match self.project {
            Project::Civica => {
                let heights = [40.0_f32, 70.0, 56.0, 102.0];
                let colors = ["#646a73", "#9097a1", "#6e7580", "#d3d8e0"];
                let bars: Vec<Svgr<'a>> = heights.iter().enumerate().map(|(i, base)| {
                    let height = base * (0.86 + 0.14 * (phase + i as f32 * 0.8).sin());
                    let x = 34.0 + i as f32 * 56.0;
                    let y = 182.0 - height;
                    let color = colors[i];
                    fframes::svgr!(<rect x={x} y={y} width="31" height={height} rx="4" fill={color} />)
                }).collect();
                let marker_y = 97.0 + 30.0 * (phase - 0.7).sin();
                fframes::svgr!(
                    <g>
                        {bars}
                        <rect x="311" y={marker_y} width="3" height="18" rx="1.5" fill="#c3ccd8" opacity=".55" />
                    </g>
                )
            }
            Project::Keep => {
                let breath = 0.5 - 0.5 * phase.cos();
                let radius = 17.0 + breath * 4.0;
                let opacity = 0.10 + breath * 0.20;
                let line_x = 190.0 + phase.sin() * 24.0;
                let line_opacity = 0.08 + breath * 0.14;
                fframes::svgr!(
                    <g>
                        <circle cx="41.5" cy="42.5" r={radius} fill="none" stroke="#eeeeee" stroke-width="1" opacity={opacity} />
                        <g transform="translate(31 29)" fill="none" stroke="#e7e7e7" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
                            <rect x="3" y="9" width="15" height="11" rx="2.5" />
                            <path d="M6 9V6a4.5 4.5 0 0 1 9 0v3M10.5 14v2" />
                        </g>
                        <path d="M168 69H288M168 75H259" transform={format!("translate({} 0)", line_x - 190.0)} fill="none" stroke="#eeeeee" stroke-width="1" opacity={line_opacity} />
                    </g>
                )
            }
            Project::Rentify => {
                // A single quiet aperture closure per loop, without a flash.
                let closure = ((1.0 - phase.cos()) * 0.5).powi(12);
                let inner = 17.5 - closure * 13.0;
                let turn = phase.sin() * 0.045;
                let point = |r: f32, a: f32| (137.0 + r * a.cos(), 141.0 + r * a.sin());
                let blades: Vec<Svgr<'a>> = (0..6).map(|i| {
                    let a = i as f32 * TAU / 6.0 + turn;
                    let b = a + TAU / 6.0;
                    let (x1, y1) = point(22.0, a);
                    let (x2, y2) = point(22.0, b);
                    let (x3, y3) = point(inner, b + 0.38);
                    let (x4, y4) = point(inner, a + 0.38);
                    let d = format!("M{x1} {y1}A22 22 0 0 1 {x2} {y2}L{x3} {y3}A{inner} {inner} 0 0 0 {x4} {y4}Z");
                    fframes::svgr!(<path d={d} fill="#353535" stroke="#717171" stroke-width=".65" />)
                }).collect();
                let light = 0.55 + closure * 0.35;
                fframes::svgr!(
                    <g>
                        <circle cx="137" cy="141" r="36" fill="#111111" stroke="#a3a3a3" stroke-width="2" />
                        <circle cx="137" cy="141" r="23" fill="#151515" stroke="#4c4c4c" />
                        {blades}
                        <circle cx="198" cy="105" r="2.6" fill="#ffad6e" opacity={light} />
                    </g>
                )
            }
            Project::Spoti => Svgr::empty(),
        };
        fframes::svgr!(
            <svg xmlns="http://www.w3.org/2000/svg" width={Self::WIDTH} height={Self::HEIGHT}>
                <image x="0" y="0" width={Self::WIDTH} height={Self::HEIGHT} href={base} />
                <g transform={illustration}>{motion}</g>
            </svg>
        )
    }
}
