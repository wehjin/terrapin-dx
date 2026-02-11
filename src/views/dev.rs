use dioxus::prelude::*;
#[component]
pub fn Dev() -> Element {
    rsx! {
        div { class: "section",
            div { class: "container",
                Chart {}
            }
        }
    }
}

pub struct Value {
    pub portion: f32,
    pub progress: f32,
}

#[component]
fn Chart() -> Element {
    let targets = [0.55, 0.30, 0.15];
    let values = [
        Value {
            portion: 0.65,
            progress: 0.6,
        },
        Value {
            portion: 0.15,
            progress: 0.4,
        },
        Value {
            portion: 0.20,
            progress: 0.8,
        },
    ];

    let target_segments = target_segments(&targets);
    let value_segments = value_segments(&values);
    let bubble_segments = bubble_segments(values.len());
    const ROUND: &'static str = "10";
    rsx! {
        div { class: "title", "Drift Chart" }
        svg {
            width: "640",
            height: "320",
            xmlns: "http://www.w3.org/2000/svg",
            defs {
                linearGradient { id: "targets",
                    x1: "0%", y1: "0%", x2: "0%", y2: "100%",
                    for segment in target_segments.iter() {
                        stop { offset: "{segment.start * 100.0}%", stop_color: "{segment.color}" }
                        stop { offset: "{segment.end * 100.0}%", stop_color: "{segment.color}" }
                    }
                }
                for segment in value_segments.iter() {
                    linearGradient { id: "{segment.id}",
                    x1: "0%", y1: "0%", x2: "100%", y2: "0%",
                        for span in segment.spans.iter() {
                            stop { offset: "{span.start * 100.0}%", stop_color: "{span.color}" }
                            stop { offset: "{span.end * 100.0}%", stop_color: "{span.color}" }
                        }
                    }
                }
            }
            rect {
                x: "0",
                y: "0",
                width: "100%",
                height: "100%",
                fill: "url(#targets)",
            }
            for segment in value_segments.iter() {
                rect {
                    x: "25%",
                    y: "{segment.start_y * 100.0}%",
                    width: "50%",
                    height: "{segment.y_portion * 100.0}%",
                    fill: "url(#{segment.id})",
                    rx: "{ROUND}",
                    ry: "{ROUND}",
                }
            }
            for segment in bubble_segments.iter() {
                rect {
                    x: "76%",
                    y: "{segment.start * 100.0}%",
                    width: "23%",
                    height: "{segment.portion * 100.0}%",
                    style: "{segment.style}",
                    rx: "{ROUND}",
                    ry: "{ROUND}",
                }
            }
        }
    }
}

struct BubbleSegment {
    start: f32,
    portion: f32,
    style: String,
}

fn bubble_segments(count: usize) -> Vec<BubbleSegment> {
    const FILL_COLOR: &'static str = "255,255,255";
    const STROKE_COLORS: &[&'static str] = &["224,64,64", "48,196,48", "64,64,196"];
    const STROKE_WIDTH: &'static str = "2px";
    const MARGIN: f32 = 0.02;
    let full_portion = (1.0 - MARGIN) / (count as f32);
    let portion = full_portion - MARGIN;
    let mut segments = Vec::new();
    let mut start = MARGIN;
    for i in 0..count {
        let stroke_color = STROKE_COLORS[i % STROKE_COLORS.len()];
        let fill = format!("rgb({FILL_COLOR})");
        let stroke = format!("rgb({stroke_color})");
        let style = format!("fill:{fill};stroke:{stroke};stroke-width:{STROKE_WIDTH};");
        segments.push(BubbleSegment {
            start,
            portion,
            style,
        });
        start = (start + full_portion).min(1.0);
    }
    segments
}

struct ValueSpan {
    color: &'static str,
    start: f32,
    end: f32,
}

struct ValueSegment {
    id: String,
    start_y: f32,
    y_portion: f32,
    spans: [ValueSpan; 2],
}

fn value_segments(values: &[Value]) -> Vec<ValueSegment> {
    const COLORS: &[(&str, &str)] = &[
        ("#ff3333b0", "#883333b0"),
        ("#33ff33b0", "#338833b0"),
        ("#5555ffb0", "#222280b0"),
    ];
    let mut segments = Vec::new();
    let mut start_y = 0.0;
    for (i, value) in values.iter().enumerate() {
        let id = format!("value-{}", i);
        let color = COLORS[i % COLORS.len()];
        let y_portion = value.portion.min(1.0 - start_y);
        segments.push(ValueSegment {
            id,
            start_y,
            y_portion,
            spans: [
                ValueSpan {
                    color: color.0,
                    start: 0.0,
                    end: value.progress,
                },
                ValueSpan {
                    color: color.1,
                    start: value.progress,
                    end: 1.0,
                },
            ],
        });
        start_y = (start_y + y_portion).min(1.0);
    }
    segments
}

struct TargetSegment {
    start: f32,
    end: f32,
    color: &'static str,
}

fn target_segments(sizes: &[f32]) -> Vec<TargetSegment> {
    const COLORS: &[&'static str] = &["#ffdddd", "#ddffdd", "#ddddff"];
    let mut segments = Vec::new();
    let mut start = 0.0;
    for (i, size) in sizes.iter().enumerate() {
        let end = (start + size).min(1.0);
        let color = COLORS[i % COLORS.len()];
        segments.push(TargetSegment { start, end, color });
        start = end;
    }
    segments
}
