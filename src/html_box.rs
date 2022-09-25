use std::fs::File;
use std::io::Write;

pub struct Html {
    buf: std::io::BufWriter<File>,
    pixel_size: u32,
}

impl Html {
    pub fn new(path: &str, pixel_size: u32) -> Html {
        let mut buf = std::io::BufWriter::new(File::create(path).expect("Cant create the HTML File"));
        buf.write_fmt(format_args!(r#"<!DOCTYPE Html><Html lang="en">
    <head>
    <meta charset="UTF-8">
    <title>{}</title>
    <style>div {{
        position: absolute;
        width: 5px;
        height: 5px
    }}</style>
</head>
<body style="padding: 0;margin: 0;">
"#, path)).unwrap();
        Html {
            buf,
            pixel_size,
        }
    }

    pub fn write_pixel_at(&mut self, x: u32, y: u32, color: u32) {
        self.buf.write_fmt(format_args!(r#"<div style="left:{}px;top:{}px;background:#{:0>6x}"></div>"#, x * self.pixel_size, y * self.pixel_size, color)).unwrap();
        self.buf.write(b"\n").unwrap();
    }

    pub fn close(&mut self) {
        self.buf.write(b"</body>\n</Html>").unwrap();
        self.buf.flush().unwrap();
    }
}