use dyn_clone::{DynClone, clone_trait_object};

pub type PropPosition = (f32, f32);
pub type PropColor = (u8, u8, u8, f32);

pub trait ComponentProps: DynClone {
    fn width(&self) -> f32 { 0.0 }
    fn height(&self) -> f32 { 0.0 }
    fn position(&self) -> PropPosition { (0.0, 0.0) }
    fn color(&self) -> PropColor { (0, 0, 0, 0.0) }
}

impl std::fmt::Debug for dyn ComponentProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "ComponentProps {{ width: {:?}, height: {:?}, position: {:?} color: {:?} }}",
            self.width(),
            self.height(),
            self.position(),
            self.color(),
        )
    }
}

impl PartialEq for dyn ComponentProps {
    fn eq(&self, other: &Self) -> bool {
        self.width() == other.width()
        && self.height() == other.height()
        && self.position() == other.position()
        && self.color() == other.color()
    }
}

impl Eq for dyn ComponentProps {}

pub trait Component: DynClone {
    fn name(&self) -> String;
    fn id(&self) -> u32;
    fn props(&self) -> Box<dyn ComponentProps>;
    fn render(&self) -> String;
    fn children(&self) -> Vec<Box<dyn Component>> { Vec::new() }
}

impl std::fmt::Debug for dyn Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "Component {{ name: {:?}, id: {:?}, children: {:?} }}",
            self.name(),
            self.id(),
            self.children(),
        )
    }
}

impl PartialEq for dyn Component {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

clone_trait_object!(Component);
clone_trait_object!(ComponentProps);

pub type Components = Vec<Box<dyn Component>>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_props_have_defaults() {
        #[derive(Debug, Clone)]
        struct TestProps {
            width: f32,
            height: f32,
            position: PropPosition,
            color: PropColor,
        }

        impl ComponentProps for TestProps {}

        let test_props = TestProps {
            width: 200.0,
            height: 100.0,
            position: (20.0, 40.0),
            color: (120, 240, 0, 1.0),
        };

        let expected_width = 0.0;
        let expected_height = 0.0;
        let expected_pos = (0.0, 0.0);
        let expected_color = (0, 0, 0, 0.0);

        assert_eq!(expected_width, test_props.width());
        assert_eq!(expected_height, test_props.height());
        assert_eq!(expected_pos, test_props.position());
        assert_eq!(expected_color, test_props.color());
    }

    #[test]
    fn impl_for_base_props() {
        #[derive(Debug, Clone)]
        struct TestProps {
            width: f32,
            height: f32,
            top: f32,
            left: f32,
            color: (u8, u8, u8, f32),
        }

        impl ComponentProps for TestProps {
            fn width(&self) -> f32 { self.width }
            fn height(&self) -> f32 { self.height }
            fn position(&self) -> (f32, f32) { (self.left, self.top) }
            fn color(&self) -> PropColor { self.color }
        }

        let test_props = TestProps {
            width: 200.0,
            height: 100.0,
            top: 20.0,
            left: 40.0,
            color: (40, 140, 240, 1.0),
        };

        let expected_width = 200.0;
        let expected_height = 100.0;
        let expected_pos = (40.0, 20.0);
        let expected_color = (40, 140, 240, 1.0);

        assert_eq!(expected_width, test_props.width());
        assert_eq!(expected_height, test_props.height());
        assert_eq!(expected_pos, test_props.position());
        assert_eq!(expected_color, test_props.color());
    }

    #[test]
    fn children_props() {
        #[derive(Copy, Clone, Debug, PartialEq)]
        struct TestProps {
            width: f32,
            height: f32,
            top: f32,
            left: f32,
            color: (u8, u8, u8, f32),
        }

        impl ComponentProps for TestProps {
            fn width(&self) -> f32 { self.width }
            fn height(&self) -> f32 { self.height }
            fn position(&self) -> (f32, f32) { (self.left, self.top) }
            fn color(&self) -> PropColor { self.color }
        }

        #[derive(Copy, Clone, Debug, PartialEq)]
        struct TestComponent {
            props: TestProps,
        }

        impl Component for TestComponent {
            fn name(&self) -> String { "TestComponent".to_string() }
            fn id(&self) -> u32 { 1 }
            fn props(&self) -> Box<ComponentProps> { Box::new(self.props) }
            fn render(&self) -> String { "Rendered Component".to_string() }
        }

        let test_props = TestProps {
            width: 200.0,
            height: 100.0,
            top: 20.0,
            left: 40.0,
            color: (40, 140, 240, 1.0),
        };
        let expect_props = test_props.clone();

        let test_component = TestComponent {
            props: test_props,
        };

        let actual_props = test_component.props();
        let expected_props: Box<ComponentProps> = Box::new(expect_props);

        assert_eq!(&expected_props, &actual_props);
    }
}
