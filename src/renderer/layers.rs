extern crate nanoid;

use nanoid::nanoid;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub enum LayerKind {
    Background,
    Generic,
    Foreground,
    UI,
    Overlay,
}

impl Default for LayerKind {
    fn default() -> LayerKind {
        LayerKind::Generic
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Default)]
pub struct Layer {
    kind: LayerKind,
    ordinal: i32,
    name: String,
    id: String,
}

impl Layer {
    pub fn new(name: &str, kind: LayerKind, ordinal: i32) -> Layer {
        Layer {
            id: nanoid!(),
            name: name.to_string(),
            kind,
            ordinal,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn on_attach() {

    }

    pub fn on_detach() {

    }

    pub fn on_update() {

    }

    pub fn on_event() {

    }
}

#[derive(Default)]
pub struct Layers {
    layers: Vec<Layer>
}

impl Layers {
    pub fn new() -> Layers {
        Layers {
            layers: Vec::new(),
        }
    }

    pub fn add(& mut self, layer: Layer) {
        self.layers.push(layer);
        self.layers.sort();
    }

    pub fn remove(& mut self, id: &str) {
        self.layers.retain(|layer| {
            layer.id != id
        });
    }

    pub fn clear(&mut self) {
        self.layers.clear();
    }

    pub fn sort(&mut self) {
        self.layers.sort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_empty_layers() {
        let actual_layers = Layers::new();
        let expected_layers = Layers::new();

        assert_eq!(expected_layers.layers, actual_layers.layers);
    }

    #[test]
    fn can_add_layer_to_layers() {
        let test_layer = Layer::new("test_layer", LayerKind::Generic, 1);
        let mut actual_layers = Layers::new();
        let expected_layers = vec![test_layer.clone()];

        actual_layers.add(test_layer);

        assert_eq!(expected_layers, actual_layers.layers);
    }

    #[test]
    fn can_remove_layer_from_layers() {
        let test_layer = Layer::new("test_layer", LayerKind::Generic, 1);
        let test_layer_two = Layer::new("test_layer", LayerKind::Generic, 1);
        let mut actual_layers = Layers::new();
        let expected_layers = vec![test_layer_two.clone()];

        let test_layer_id = test_layer.get_id();

        actual_layers.add(test_layer);
        actual_layers.add(test_layer_two);
        actual_layers.remove(&test_layer_id);

        assert_eq!(expected_layers, actual_layers.layers);
    }

    #[test]
    fn can_sort_layers_by_enums_then_ordinals() {
        let test_layer_one = Layer::new("test_one", LayerKind::Background, 1);
        let test_layer_two = Layer::new("test_one", LayerKind::Background, 2);
        let test_layer_three = Layer::new("test_two", LayerKind::Generic, 1);
        let test_layer_four = Layer::new("test_two", LayerKind::Generic, 2);
        let test_layer_five = Layer::new("test_three", LayerKind::Foreground, 1);
        let test_layer_six = Layer::new("test_three", LayerKind::Foreground, 2);
        let test_layer_seven = Layer::new("test_four", LayerKind::UI, 1);
        let test_layer_eight = Layer::new("test_four", LayerKind::UI, 2);
        let test_layer_nine = Layer::new("test_five", LayerKind::Overlay, 1);
        let test_layer_ten = Layer::new("test_five", LayerKind::Overlay, 2);

        let mut actual_layers = Layers::new();
        let expected_layers = vec![
            test_layer_one.clone(),
            test_layer_two.clone(),
            test_layer_three.clone(),
            test_layer_four.clone(),
            test_layer_five.clone(),
            test_layer_six.clone(),
            test_layer_seven.clone(),
            test_layer_eight.clone(),
            test_layer_nine.clone(),
            test_layer_ten.clone(),
        ];

        actual_layers.add(test_layer_four);
        actual_layers.add(test_layer_three);
        actual_layers.add(test_layer_two);
        actual_layers.add(test_layer_ten);
        actual_layers.add(test_layer_one);
        actual_layers.add(test_layer_five);
        actual_layers.add(test_layer_seven);
        actual_layers.add(test_layer_nine);
        actual_layers.add(test_layer_six);
        actual_layers.add(test_layer_eight);

        actual_layers.sort();

        assert_eq!(expected_layers, actual_layers.layers);
    }

    #[test]
    fn should_clear_layers() {
        let test_layer_one = Layer::new("test_one", LayerKind::Generic, 1);
        let test_layer_two = Layer::new("test_two", LayerKind::Generic, 2);
        let mut actual_layers = Layers::new();
        let expected_layers: Vec<Layer> = Vec::new();

        actual_layers.add(test_layer_one);
        actual_layers.add(test_layer_two);

        actual_layers.clear();

        assert_eq!(expected_layers, actual_layers.layers);
    }
}
