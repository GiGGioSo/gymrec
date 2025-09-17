<#
.SYNOPSIS
FFmpeg recording script for Windows (MKV → MP4)
#>

param(
    [string]$Output = "output.mp4",             # Output MP4 file (default fallback)
    [string]$VideoDevice = "USB Camera",       # Video capture device
    [string]$AudioDevice = "Microfono (AUDIO 2.0)", # Audio capture device
    [int]$Orientation = 1                      # 0 = none, 1 = 90° CW, 2 = 90° CCW
)

# Temporary MKV file
$TmpMkv = "$Output.mkv"

# Build filter string based on orientation
$Filters = switch ($Orientation) {
    1 { "-vf transpose=1" }
    2 { "-vf transpose=2" }
    default { "" }
}

try {
    Write-Host "Recording to $TmpMkv ... Press Ctrl-C to stop."

    # Start FFmpeg recording and wait for it to finish
    $ffmpegArgs = @(
        "-f", "dshow",
        "-i", "video=`"$VideoDevice`":audio=`"$AudioDevice`"",
        $Filters,
        "-vcodec", "libx264",
        "-preset", "ultrafast",
        "-crf", "23",
        "-acodec", "aac",
        "-b:a", "128k",
        "-y",
        "`"$TmpMkv`""
    )

    Start-Process -FilePath "ffmpeg" -ArgumentList $ffmpegArgs -NoNewWindow -Wait

} finally {
    # Convert MKV → MP4
    if (Test-Path $TmpMkv) {
        Write-Host "Converting $TmpMkv to $Output ..."
        & ffmpeg -i $TmpMkv -c copy -y $Output
        Remove-Item $TmpMkv -Force
    }
}

Write-Host "Done! Output saved to $Output"
