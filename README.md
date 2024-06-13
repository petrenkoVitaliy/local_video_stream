# local_video_stream

Local server for video streaming

### API:

- `GET /`
  - return root html
- `GET /chunk`

  - Pipe: return video chunk pipe

- `GET /status`

  - SSE - check video existence status and start converting if needed

- `PUT /source`
  - update videos' source folder
