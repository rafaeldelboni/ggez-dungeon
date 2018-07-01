use std::collections::HashMap;

use texture_packer;

use serde_json;
use serde_json::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Frame {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Atlas {
    frames: HashMap<String, Frame>,
}

pub fn to_atlas(frames: &HashMap<String, texture_packer::Frame>) -> Atlas {

    let frames_map = frames
        .iter()
        .map(|(name, frame)| (
                name.clone(),
                Frame {
                    x: frame.frame.x,
                    y: frame.frame.y,
                    w: frame.frame.w,
                    h: frame.frame.h
                }
            )
        )
        .collect();

    return Atlas { frames: frames_map };
}

pub fn to_json(atlas: Atlas) -> Result<(String), Error> {
    let json = serde_json::to_string(&atlas)?;
    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_to_atlas() {
        let mut converted_frames: HashMap<String, texture_packer::Frame> = HashMap::new();
        converted_frames.insert(
            "test1".to_string(),
            texture_packer::Frame {
                key: "test1".to_string(),
                frame: texture_packer::Rect{ x: 0, y: 0, w: 0, h: 0},
                source: texture_packer::Rect{ x: 0, y: 0, w: 0, h: 0},
                rotated: false,
                trimmed: false,
            }
        );
        let atlas = to_atlas(&converted_frames);

        let mut created_frames: HashMap<String, Frame> = HashMap::new();
        created_frames.insert("test1".to_string(), Frame { x: 0, y: 0, w: 0, h: 0 });
        created_frames.insert("test2".to_string(), Frame { x: 1, y: 1, w: 1, h: 1 });

        let converted = atlas.frames.get("test1").unwrap();
        let created = created_frames.get("test1").unwrap();

        assert_eq!(converted.x, created.x);
        assert_eq!(converted.y, created.y);
        assert_eq!(converted.w, created.w);
        assert_eq!(converted.h, created.h);
    }

    #[test]
    fn should_convert_to_json() {
        let mut created_frames: HashMap<String, Frame> = HashMap::new();
        created_frames.insert("test1".to_string(), Frame { x: 0, y: 0, w: 0, h: 0 });

        let atlas = to_json(Atlas { frames: created_frames} ).unwrap();
        let json = "{\"frames\":{\"test1\":{\"x\":0,\"y\":0,\"w\":0,\"h\":0}}}";

        assert_eq!(atlas, json);
    }
}
