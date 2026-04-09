//! Wireshark integration helpers that are independent of runtime state containers.

use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct WiresharkInfo {
    pub found: bool,
    pub path: Option<String>,
    pub version: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct FrameRow {
    pub number: usize,
    pub timestamp: String,
    pub src_ip: String,
    pub dst_ip: String,
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: String,
    pub length: usize,
    pub origin_file: String,
}

pub fn detect_wireshark() -> WiresharkInfo {
    let wireshark_path = find_wireshark_binary();
    if let Some(path) = wireshark_path {
        WiresharkInfo {
            found: true,
            path: Some(path.to_string_lossy().to_string()),
            version: None,
        }
    } else {
        WiresharkInfo {
            found: false,
            path: None,
            version: None,
        }
    }
}

pub fn build_frame_rows(packets: &[gm_types::PacketSummary]) -> Vec<FrameRow> {
    packets
        .iter()
        .enumerate()
        .map(|(i, pkt)| FrameRow {
            number: i + 1,
            timestamp: pkt.timestamp.clone(),
            src_ip: pkt.src_ip.clone(),
            dst_ip: pkt.dst_ip.clone(),
            src_port: pkt.src_port,
            dst_port: pkt.dst_port,
            protocol: pkt.protocol.clone(),
            length: pkt.length,
            origin_file: pkt.origin_file.clone(),
        })
        .collect()
}

pub fn build_frames_csv(packets: &[gm_types::PacketSummary]) -> String {
    let mut csv =
        String::from("No,Timestamp,Source,SrcPort,Destination,DstPort,Protocol,Length,File\n");
    for (i, pkt) in packets.iter().enumerate() {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{}\n",
            i + 1,
            pkt.timestamp,
            pkt.src_ip,
            pkt.src_port,
            pkt.dst_ip,
            pkt.dst_port,
            pkt.protocol,
            pkt.length,
            pkt.origin_file,
        ));
    }
    csv
}

pub fn build_display_filter(conn: &gm_types::ConnectionInfo) -> String {
    let mut parts = Vec::new();
    parts.push(format!(
        "(ip.addr == {} && ip.addr == {})",
        conn.src_ip, conn.dst_ip
    ));
    if conn.src_port > 0 && conn.dst_port > 0 {
        let transport = if conn.transport == "udp" {
            "udp"
        } else {
            "tcp"
        };
        parts.push(format!(
            "({}.port == {} && {}.port == {})",
            transport, conn.src_port, transport, conn.dst_port
        ));
    }
    parts.join(" && ")
}

pub fn find_wireshark_binary() -> Option<PathBuf> {
    let (cmd, arg) = if cfg!(target_os = "windows") {
        ("where.exe", "wireshark")
    } else {
        ("which", "wireshark")
    };

    if let Ok(output) = std::process::Command::new(cmd).arg(arg).output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout)
                .lines()
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
            if !path.is_empty() {
                return Some(PathBuf::from(path));
            }
        }
    }

    let candidates = [
        "/usr/bin/wireshark",
        "/usr/local/bin/wireshark",
        "/snap/bin/wireshark",
        "/Applications/Wireshark.app/Contents/MacOS/Wireshark",
        "/usr/local/bin/wireshark",
        "C:\\Program Files\\Wireshark\\Wireshark.exe",
        "C:\\Program Files (x86)\\Wireshark\\Wireshark.exe",
    ];

    for path in &candidates {
        let p = PathBuf::from(path);
        if p.exists() {
            return Some(p);
        }
    }
    None
}
