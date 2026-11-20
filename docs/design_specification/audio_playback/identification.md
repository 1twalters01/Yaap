# File Type Identification

## API
```
use symphonia::core::codecs::AudioCodecId;

pub struct IdentifiedAudioStream<'a> {
    pub classification: AudioStreamClassification,
    pub track_id: u32,
    pub reader: Box<dyn FormatReader + 'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AudioStreamClassification {
    pub format: String,
    pub encoding: AudioCodecId,
}

#[derive(Debug, thiserror::Error)]
pub enum AudioStreamError {
    Unknown(AudioProperty),
    Unsupported(AudioProperty),
    Invalid(AudioProperty),
    NoAudioTrack,
    Io(#[from] std::io::Error),
    Symphonia(#[from] SymphoniaError),
}

#[derive(Debug)]
pub struct AudioProperty {
    pub property: AudioPropertyType,
    pub message: String,
}

#[derive(Debug)]
pub enum AudioPropertyType {
    Format,
    Encoding,
    FormatAndEncoding,
}

pub fn identify_supported_audio_stream<'a, P: AudioProber>(
    media_source_stream: MediaSourceStream<'a>,
    prober: &P,
) -> Result<IdentifiedAudioStream<'a>, AudioStreamError> {
}
```

## Purpose
* Identify the container format of an audio stream.
* Identify the encoding used by the first audio track.
* Determine whether the identified format, encoding and their combination are 
supported.
* Provide the caller with the identified audio stream and its associated format
reader.
* Report identification and validation failures using defined error types.

## Sequence Diagram
```mmd
sequenceDiagram
    autonumber

    actor Caller
    participant API as identify_supported_audio_stream()
    participant Identify as identify_audio_stream()
    participant Prober as AudioProber
    participant Symphonia as SymphoniaProber
    participant Classification as identify_classification()
    participant Extract as extract_classification()
    participant Verify as verify_format_supported()

    Caller->>API: identify_supported_audio_stream(stream, prober)
    API->>Identify: identify_audio_stream(stream, prober)
    Identify->>Prober: probe_stream(stream)
    Prober->>Symphonia: probe_stream(stream)
    Symphonia-->>Prober: ProbedAudioStream
    Prober-->>Identify: ProbedAudioStream

    Identify->>Classification: identify_classification(probed_reader)
    Classification->>Classification: format_short_name()
    Classification->>Classification: first_audio_track()

    alt No audio track
        Classification-->>Identify: Err(NoAudioTrack)
        Identify-->>API: Err(AudioStreamError)
        API-->>Caller: Err(AudioStreamError)
    else Audio track found
        Classification->>Extract: extract_classification(format, codec_params)

        alt Codec parameters unavailable or invalid
            Extract-->>Classification: Err(Unknown(Encoding))
            Classification-->>Identify: Err(AudioStreamError)
            Identify-->>API: Err(AudioStreamError)
            API-->>Caller: Err(AudioStreamError)
        else Classification identified
            Extract-->>Classification: AudioStreamClassification
            Classification-->>Identify: Classification + AudioTrackInfo
            Identify-->>API: IdentifiedAudioStream

            API->>Verify: verify_format_supported(classification)
            Verify->>Verify: validate_audio_format()
            Verify->>Verify: validate_audio_encoding()

            alt Format and encoding supported
                Verify->>Verify: validate_format_encoding_combination()
                Verify-->>API: Ok(())
                API-->>Caller: Ok(IdentifiedAudioStream)
            else Unsupported format or encoding
                Verify-->>API: Err(Unsupported)
                API-->>Caller: Err(AudioStreamError)
            end
        end
    end
```

## Additional Design Considerations
### Track Selection
Currently the first track is selected.

### Stream Ownership 
### Thread Safety and Concurrency
### Format support extensibility
