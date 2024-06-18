# local_video_stream

[![github actions](https://img.shields.io/github/actions/workflow/status/petrenkoVitaliy/local_video_stream/rust.yml?branch=main&style=flat-square&logo=github)](https://github.com/petrenkoVitaliy/local_video_stream/actions)

Local server for video streaming and conversion based on
[Actix](https://github.com/actix/actix-web)
and [FFmpeg](https://github.com/FFmpeg/FFmpeg)

### API:

- `GET /` - return root html

- `GET /chunk` - Pipe: return video chunk pipe

- `GET /status` - SSE - check video existence status and start converting if needed

- `PUT /source` - update videos' source folder

### Flow

`/status` request checks if requested video is in MP4 format or was already converted to MP4 and sends success response.
</br>Otherwise, it starts conversion with FFmpeg in a separate process and sends SSE events to client with current conversion progress.

`/chunk` selects file source and range based on request meta and sends buffer to client.

#### FFmpeg handling

Handler and output parser - [converter_service.rs](src/service/converter_service.rs)
