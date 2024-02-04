// 声明一个 UI 组件的特性：绘制 UI 组件
pub trait Draw {
    fn draw(&self);
}

// --------------------------------------------------------------------------------

#[allow(dead_code)]
#[derive(Debug)]
pub struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Button {
    pub fn new(width: u32, height: u32, label: String) -> Button {
        Button { width, height, label }
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }
    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn label(&self) -> &str {
        &self.label
    }
}

impl Draw for Button {
    fn draw(&self) {
        let horizontal_padding = (self.width - 2 - self.label.len() as u32) / 2;
        let vertical_padding = if self.height % 2 == 1 {
            ((self.height + 1) / 2) as usize
        } else {
            (self.height / 2) as usize
        };

        let mut res = format!("┌{:-^width$}┐\n", "-", width = (self.width - 2) as usize);
        for i in 1..(self.height - 1) {
            let mut line = "".to_string();
            if i as usize == (vertical_padding - 1) {
                let print_label = if self.label.len() % 2 == 1 {
                    format!("{} ", self.label.clone())
                } else {
                    self.label.clone()
                };
                line = format!(
                    "|{0}{1}{0}|\n",
                    " ".repeat(horizontal_padding as usize),
                    print_label
                );
            } else {
                let pad = " ".repeat((self.width - 2) as usize);
                line = format!("|{}|\n", pad);
            }
            res.push_str(line.as_str());
        }
        res.push_str(format!("└{:-^width$}┘", "-", width = (self.width - 2) as usize).as_str());

        println!("{}", res);
    }
}

// --------------------------------------------------------------------------------

#[allow(dead_code)]
#[derive(Debug)]
pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl SelectBox {
    pub fn new(width: u32, height: u32, options: Vec<String>) -> SelectBox {
        SelectBox { width, height, options }
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn options(&self) -> &Vec<String> {
        &self.options
    }
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }
    pub fn set_options(&mut self, options: Vec<String>) {
        self.options = options;
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw a select box");
    }
}

// --------------------------------------------------------------------------------

fn draw(x: &dyn Draw) {
    x.draw();
}

// --------------------------------------------------------------------------------

pub struct Screen {
    // dyn，可以理解成 dynamic，它代表【动态特性】
    // 与之相对的，impl trait 代表的是【静态特性】
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn new(components: Vec<Box<dyn Draw>>) -> Screen {
        Screen { components }
    }

    pub fn add_component(&mut self, component: Box<dyn Draw>) {
        self.components.push(component);
    }

    pub fn run(&self) {
        for component in self.components().iter() {
            component.draw();
        }
    }

    pub fn components(&self) -> &Vec<Box<dyn Draw>> {
        &self.components
    }
    pub fn set_components(&mut self, components: Vec<Box<dyn Draw>>) {
        self.components = components;
    }
}

// --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::objects::{Button, draw, Screen, SelectBox};

    #[test]
    fn test_draw() {
        let button = Button::new(10, 5, "OK".to_string());
        draw(&button);
    }

    #[test]
    fn test_screen() {
        let mut screen = Screen::new(vec![]);
        // 在这行，Button 都只是特性 Draw 的一个实例，不再代表具体类型
        screen.add_component(Box::new(Button::new(8, 5, "hello".to_string())));
        // 在这行，SelectBox 都只是特性 Draw 的一个实例，不再代表具体类型
        screen.add_component(Box::new(SelectBox::new(120, 64, vec!["hello, world".to_string()])));
        screen.run();
    }
}
