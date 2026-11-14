#[derive(Debug, PartialEq, Eq)]
pub enum AudioType {
    // @relation(APLLSR-007, scope=line)
    Flac,
    // @relation(APLLSR-002, scope=line)
    Unsupported,
    // @relation(APLLSR-003, scope=line)
    UnknownFormat,
    // @relation(APLLSR-004, scope=line)
    UnknownEncoding,
    // @relation(APLLSR-005, scope=line)
    InvalidFormat,
    // @relation(APLLSR-006, scope=line)
    InvalidEncoding,
}

pub fn identify_audio_stream() {
}
