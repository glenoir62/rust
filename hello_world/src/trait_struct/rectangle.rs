pub(crate) struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub(crate) fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    pub(crate) fn area(&self) -> u32 {
        self.width * self.height
    }
}