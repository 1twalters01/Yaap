# Flac Stream Metadata Retrieval
## Requirements
Subsystem: Audio Playback
Subsystem Requirements Covered: APSSR004, APSSR005, APSSR006
HL Software Requirements Created: APHLSR005, APHLSR006, APHLSR007
LL Software Requirements Created: 

## High Level Overview
Get codec parameters: Need to refactor what I have to expose this?

```
let channels = codec parameters.channels;
let sample rate = codec parameters.sample rate;
let bits per sample = codec parameters.bits per sample;
```

Also get duration, total number of audio frames/samples, stream time base

Are these optionals? Will need error handling.

# Flac Decoding
## Requirements
Subsystem: Audio Playback

## High Level Overview
Initialise Flac decoder
Retrieve encoded audio packets
Decode packets into audio samples
Represent decoded samples as a usable format
Handle end of stream
Handle decoding errors

# Audio Playback
## Requirements
Subsystem: Audio Playback

## High Level Overview
Discover available audio devices
Initialise audio output device: Use CPAL?
Get decoded audio samples some how
Configure output device for sample rate, channel count, sample format
Push buffers to output device: Use CPAL?


# Playback Position Tracking and Seeking
## Requirements
Subsystem: Audio Playback

## High Level Overview
Current position
Submit a position to play from
Make it feasible with the player somehow
    Shared Buffer somehow (atomic/arc (mutex))?

# Playback State Control
## Requirements
Subsystem: Audio Playback

## High Level Overview
Start playback 
Pause playback
Resume playback
Stop playback
Retrieve playback state

# Volume Control
## Requirements
Subsystem: Audio Playback

## High Level Overview
Retrieve current volume
Set playback volume
Handle volume changes during playback
Define volume range

# Speed Control
## Requirements
Subsystem: Audio Playback

## High Level Overview
Retrieve current speed
Set playback speed
Handle speed changes during playback
Define speed range
Maintain playback position with speed changes

# Pitch Control
## Requirements
Subsystem: Audio Playback

## High Level Overview
Retrieve current pitch
Set playback pitch
Handle pitch changes during playback
Define pitch range
Maintain pitch modulation with playback?

# MP3 Handling
## Requirements
Subsystem:: Audio Playback

## High Level Overview
Integrate mp3 for identification, decoding, playback and control

# Track Metadata Retrieval
## Requirements
Subsystem: Track Management

## High Level Overview
Retrieve Title, Artists, Year, etc. from track

# Editing Metadata
Edit metadata for editable fields
Save metadata changes
Undo and Redo metadata changes

# Design UXUI of application
