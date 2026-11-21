# Flac Stream Metadata Retrieval
## Requirements
Subsystem: Audio Playback
Subsystem Requirements Covered: APSSR-004, APSSR-005, APSSR-006, APSSR-130
HL Software Requirements Created: APHLSR-005, APHLSR-006, APHLSR-007
LL Software Requirements Created: 

## High Level Overview
### Codec Parameters
Retrieve the following properties from the FLAC audio stream's codec 
parameters: number of audio channels, sample rate, bits per sample.

Consider whether the existing implementation needs refactoring to expose these 
properties.

```
let channels = codec_parameters.channels;
let sample_rate = codec_parameters.sample rate;
let bits_per_sample = codec_parameters.bits_per_sample;
```

### Duration
duration = total number of samples / sample rate

Consider how to handle streams where the total sample count is unknown.

### Error Handling
Determine which properties are optional or potentially unavailable.

Consider:
- Missing metadata
- Invalid metadata
- Unknown duration
- Incomplete streams
- Unsupported streams


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
