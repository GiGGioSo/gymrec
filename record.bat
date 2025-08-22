SETLOCAL

SET FPS=30

SET INPUT_NAME=video="USB Camera"
REM SET INPUT_NAME="0"

SET OUTPUT="output.mp4"

SET DEVICE=dshow
REM SET DEVICE="vfwcap"

ffmpeg -y^
    -f %DEVICE%^
    -video_size 1920x1080 -framerate %FPS% -display_rotation 270^
    -i %INPUT_NAME%^
    -audio_device_number true^
    %OUTPUT%

ENDLOCAL
