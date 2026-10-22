use fframes::{EncoderOptions, RenderOptions, StaticMediaProvider, cli};
use fframes::cli::clap;
use fframes_skia_renderer::{
    SkiaFFramesRenderer, SkiaPipelineConcurrencyPolicy, SkiaPipelineConfig,
    vulkan::SkiaVulkanCtx,
};
use spotistats_banner::{Project, SpotistatsBannerMedia, SpotistatsBannerVideo};
use std::process::ExitCode;

#[derive(Debug, clap::Args)]
struct BannerArgs {
    #[arg(long, global = true)]
    mobile: bool,
    #[arg(long, global = true, default_value = "spoti", value_parser = ["spoti", "civica", "keep", "rentify"])]
    project: String,
}

fn main() -> ExitCode {
    let args = cli::parse::<BannerArgs>();
    let media = SpotistatsBannerMedia::prepare().expect("banner media");
    let project = match args.app.project.as_str() {
        "civica" => Project::Civica,
        "keep" => Project::Keep,
        "rentify" => Project::Rentify,
        _ => Project::Spoti,
    };
    let (width, height) = if args.app.mobile { (640, 334) } else { (1280, 252) };
    let gpu = SkiaVulkanCtx::new(width, height).expect("GPU context");
    macro_rules! run_banner {
        ($mobile:literal) => {{
            let video = SpotistatsBannerVideo::<$mobile>::new(&media, project);
            cli::new(
                &video,
                RenderOptions {
                    media: Some(&media),
                    video_encoder_options: EncoderOptions {
                        preferred_encoder: Some("libx264"),
                        codec_params: Some(&[("crf", "0"), ("preset", "fast"), ("tune", "animation")]),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
            .args(args)
            .backend(SkiaFFramesRenderer::new_vulkan(
                &gpu,
                SkiaPipelineConfig {
                    concurrency_policy: SkiaPipelineConcurrencyPolicy::MaxPerformance,
                    ..Default::default()
                },
            ).expect("Skia renderer"))
            .preview(fframes_native_player::cli_preview)
            .run()
        }};
    }
    if args.app.mobile { run_banner!(true) } else { run_banner!(false) }
}
