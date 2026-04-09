//! Capture import use-cases independent of runtime state containers.

use serde::Serialize;
use std::fmt;
use std::time::Instant;

#[derive(Debug, Clone, Serialize)]
pub struct ImportFileResult {
    pub filename: String,
    pub packet_count: usize,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaptureImportError {
    NoPacketsParsed,
}

impl fmt::Display for CaptureImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CaptureImportError::NoPacketsParsed => {
                write!(f, "No packets could be parsed from the provided files")
            }
        }
    }
}

impl std::error::Error for CaptureImportError {}

#[derive(Debug, Clone, Serialize)]
pub struct ImportOutcome {
    pub per_file_results: Vec<ImportFileResult>,
    pub total_packet_count: usize,
}

pub fn filename_from_path(path: &str) -> String {
    std::path::Path::new(path)
        .file_name()
        .map(|f| f.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string())
}

pub fn total_packet_count(per_file_results: &[ImportFileResult]) -> usize {
    per_file_results.iter().map(|r| r.packet_count).sum()
}

pub fn ensure_successful_import(
    total_packet_count: usize,
    per_file_results: &[ImportFileResult],
) -> Result<(), CaptureImportError> {
    if total_packet_count == 0 && !per_file_results.iter().any(|r| r.status == "ok") {
        return Err(CaptureImportError::NoPacketsParsed);
    }
    Ok(())
}

pub fn run_import<ReadFile, ProcessPacket, EmitProgress>(
    paths: &[String],
    start: Instant,
    read_file: ReadFile,
    process_packet: ProcessPacket,
    emit_progress: EmitProgress,
) -> Result<ImportOutcome, CaptureImportError>
where
    ReadFile: FnMut(&str) -> Result<Vec<gm_capture::ParsedPacket>, String>,
    ProcessPacket: FnMut(&gm_capture::ParsedPacket),
    EmitProgress: FnMut(usize, usize, &str, usize, Instant),
{
    let per_file_results =
        process_input_files(paths, start, read_file, process_packet, emit_progress);
    let total_packet_count = total_packet_count(&per_file_results);
    ensure_successful_import(total_packet_count, &per_file_results)?;
    Ok(ImportOutcome {
        per_file_results,
        total_packet_count,
    })
}

pub fn process_input_files<ReadFile, ProcessPacket, EmitProgress>(
    paths: &[String],
    start: Instant,
    mut read_file: ReadFile,
    mut process_packet: ProcessPacket,
    mut emit_progress: EmitProgress,
) -> Vec<ImportFileResult>
where
    ReadFile: FnMut(&str) -> Result<Vec<gm_capture::ParsedPacket>, String>,
    ProcessPacket: FnMut(&gm_capture::ParsedPacket),
    EmitProgress: FnMut(usize, usize, &str, usize, Instant),
{
    let mut per_file_results: Vec<ImportFileResult> = Vec::new();

    for (file_index, path) in paths.iter().enumerate() {
        let filename = filename_from_path(path);
        match read_file(path) {
            Ok(packets) => {
                for packet in &packets {
                    process_packet(packet);
                }
                per_file_results.push(ImportFileResult {
                    filename: filename.clone(),
                    packet_count: packets.len(),
                    status: "ok".to_string(),
                });
                emit_progress(file_index, paths.len(), &filename, packets.len(), start);
            }
            Err(e) => {
                per_file_results.push(ImportFileResult {
                    filename,
                    packet_count: 0,
                    status: format!("error: {}", e),
                });
            }
        }
    }

    per_file_results
}
