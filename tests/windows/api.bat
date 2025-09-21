@echo off
REM ==========================================
REM gymrec testing script
REM ==========================================

@echo on
REM curl -X POST "http://localhost:3000/api/start"
@echo off

@echo on
REM curl -X POST "http://localhost:3000/api/stop"
@echo off

@echo on
curl -X GET "http://localhost:3000/api/videos"
@echo off
