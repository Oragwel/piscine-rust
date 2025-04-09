#[derive(Clone, Copy, Debug)]
pub struct One {
    pub first_layer: Option<Two>,
}

#[derive(Clone, Copy, Debug)]
pub struct Two {
    pub second_layer: Option<Three>,
}

#[derive(Clone, Copy, Debug)]
pub struct Three {
    pub third_layer: Option<Four>,
}

#[derive(Clone, Copy, Debug)]
pub struct Four {
    pub fourth_layer: Option<u16>,
}

impl One {
    pub fn get_fourth_layer(self) -> Option<u16> {
        self.first_layer?.second_layer?.third_layer?.fourth_layer
    }
}
