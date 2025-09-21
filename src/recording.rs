use tokio::process;
use std::process::Stdio;

pub struct RecordingInfo {
    process: Option<process::Child>,
}

pub fn start_recording() -> Result<RecordingInfo, Box<dyn std::error::Error>> {
    let tmp_mkv_path = "tmp.mkv";
    let orientation = 1; // 0 = none, 1 = 90° CW, 2 = 90° CCW

    let os = std::env::consts::OS;
    let mut ffmpeg_args: Vec<String>;

    if os == "windows" {
        let video_device = "USB Camera";
        let audio_device = "Microfono (AUDIO 2.0)";
        let input_args = format!("video={}:audio={}", video_device, audio_device);

        ffmpeg_args = vec![
            "-f".into(), "dshow".into(),
            "-i".into(), input_args,
            "-vcodec".into(), "libx264".into(),
            "-preset".into(), "ultrafast".into(),
            "-crf".into(), "23".into(),
            "-acodec".into(), "aac".into(),
            "-b:a".into(), "128k".into(),
            tmp_mkv_path.into(),
        ];
    } else if os == "linux" {
        let video_device = "/dev/video0";
        let audio_device = "default"; // puoi cambiare in "alsa" se serve
        ffmpeg_args = vec![
            "-f".into(), "v4l2".into(),
            "-framerate".into(), "30".into(),
            "-video_size".into(), "1280x720".into(),
            "-input_format".into(), "mjpeg".into(),
            "-i".into(), video_device.into(),
            "-f".into(), "pulse".into(),
            "-i".into(), audio_device.into(),
            "-vcodec".into(), "libx264".into(),
            "-preset".into(), "ultrafast".into(),
            "-crf".into(), "23".into(),
            "-acodec".into(), "aac".into(),
            "-b:a".into(), "128k".into(),
            tmp_mkv_path.into(),
        ];
    } else {
        return Err(format!("Unsupported OS: {}", os).into());
    }

    if orientation == 1 {
        ffmpeg_args.push("-vf".into());
        ffmpeg_args.push("transpose=1".into());
    } else if orientation == 2 {
        ffmpeg_args.push("-vf".into());
        ffmpeg_args.push("transpose=2".into());
    }
    ffmpeg_args.push("-y".into());

    let ffmpeg_process = process::Command::new("ffmpeg")
        .args(&ffmpeg_args)
        .stdin(Stdio::piped())
        .spawn()
        .expect("could not spawn ffmpeg process");

    Ok(RecordingInfo {
        process: Some(ffmpeg_process),
    })
}

pub async fn stop_recording(mut recording: RecordingInfo) -> () {
    if let Some(mut process) = recording.process.take() {
        if let Some(mut stdin) = process.stdin.take() {
            use tokio::io::AsyncWriteExt;
            if let Err(e) = stdin.write_all(b"q\n").await {
                eprintln!("Failed to send quit to FFmpeg: {}", e);
            }
        }

        // Then wait for FFmpeg to finish gracefully
        if let Err(e) = process.wait().await {
            eprintln!("FFmpeg didn't exit cleanly: {}", e);
        }
    }
}
