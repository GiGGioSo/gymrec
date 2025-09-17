use tokio::process;

pub struct RecordingInfo {
    process: Option<process::Child>,
}

pub fn start_recording() -> Result<RecordingInfo, Box<dyn std::error::Error>> {
    // let now_time = chrono::Utc::now().time().format("%H_%M_%S%_3f").to_string();
    // println!("{}", now_time);

    // let output_path = format!("videos/{}/{}/{}.mkv", date, exercise, timestamp);

    let tmp_mkv_path = "tmp.mkv";
    let video_device = "USB Camera";
    let audio_device = "Microfono (AUDIO 2.0)";
    let orientation = 1; // 0 = none, 1 = 90° CW, 2 = 90° CCW

    let input_args = format!("video={}:audio={}", video_device, audio_device);

    let mut ffmpeg_args = vec![
        "-f", "dshow",
        "-i", &input_args,
        "-vcodec", "libx264",
        "-preset", "ultrafast",
        "-crf", "23",
        "-acodec", "aac",
        "-b:a", "128k",
        &tmp_mkv_path,
    ];
    if orientation == 1 {
        ffmpeg_args.push("-vf");
        ffmpeg_args.push("transpose=1");
    } else if orientation == 2 {
        ffmpeg_args.push("-vf");
        ffmpeg_args.push("transpose=2");
    }

    // println!("Starting ffmpeg with following arguments:\n  {:?}", ffmpeg_args);

    let ffmpeg_process = process::Command::new("ffmpeg")
        .args(&ffmpeg_args)
        .spawn()
        .expect("could not spawn ffmpeg process");

    return Ok(RecordingInfo {
        process: Some(ffmpeg_process)
    });
}

pub async fn stop_recording(mut recording: RecordingInfo) -> () {
    if let Some(mut process) = recording.process.take() {
        if let Some(mut stdin) = process.stdin.take() {
            use tokio::io::AsyncWriteExt;
            if let Err(e) = stdin.write_all(b"q").await {
                eprintln!("Failed to send quit to FFmpeg: {}", e);
            }
        }

        // Then wait for FFmpeg to finish gracefully
        if let Err(e) = process.wait().await {
            eprintln!("FFmpeg didn't exit cleanly: {}", e);
        }
    }
}
