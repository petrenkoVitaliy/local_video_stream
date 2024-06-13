# local_video_stream

Local server for video streaming

### API:

- > Get `/`
  - return root html
- > Get `/chunk`

  - Pipe: return video chunk pipe

- > Get `/status`

  - SSE - check video existence status and start converting if needed

- > Put `/source`
  - update videos' source folder
