use std::sync::{Arc, RwLock};
use std::collections::VecDeque;
use lazybytes::LazyBytes;

pub struct Segment {
    index: u64,
    duration_ms: u64,
    lazy_bytes: Arc<RwLock<LazyBytes>>,
}

pub struct Hls {
    last_index: u64,
    segments: VecDeque<Segment>,
}

impl Hls {
    pub fn new() -> Arc<RwLock<Hls>> {
        let hls = Hls {
            last_index: 0,
            segments: VecDeque::new(),
        };

        Arc::new(RwLock::new(hls))
    }

    pub fn add_new_segment(&mut self, duration_ms: u64, lazy_bytes: Arc<RwLock<LazyBytes>>) {
        self.last_index += 1;
        self.segments.push_back(Segment {
            index: self.last_index,
            lazy_bytes,
            duration_ms,
        });
        while self.segments.len() > 10 {
            self.segments.pop_front();
        }
    }

    pub fn generate_playlist(&self) -> String {
        let skip = 1;
        let sequence = self.segments
            .iter()
            .rev()
            .skip(skip)
            .next()
            .map(|segment| segment.index)
            .unwrap_or(0);
        let mut playlist = format!(
            r"#EXTM3U
#EXT-X-VERSION:6
#EXT-X-TARGETDURATION:1
#EXT-X-START:TIME-OFFSET=-0.35,PRECISE=NO
#EXT-X-MEDIA-SEQUENCE:{}

",
            sequence
        );
        for segment in &self.segments {
            playlist.push_str(&format!(
                "#EXTINF:{},\nsegment{:09}.ts\n",
                segment.duration_ms as f64 / 1000.0,
                segment.index
            ));
        }
        playlist
    }

    pub fn read_segment(&self, index: u64) -> Option<Arc<RwLock<LazyBytes>>> {
        self.segments
            .iter()
            .find(|segment| segment.index == index)
            .map(|segment| segment.lazy_bytes.clone())
    }
}
