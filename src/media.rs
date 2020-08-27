use yew::services::resize::WindowDimensions;

pub fn is_mobile(dim: &WindowDimensions) -> bool {
  dim.height < 1000 && dim.width < 500
}

pub fn is_tablet(dim: &WindowDimensions) -> bool {
  dim.height > 700 && dim.height < 100 && dim.width > 500 && dim.width < 800
}

pub fn is_laptop(dim: &WindowDimensions) -> bool {
  dim.height > 700 && dim.height < 1000 && dim.width > 1200 && dim.width < 1400
}

pub fn is_screen(dim: &WindowDimensions) -> bool {
  dim.height > 1000 && dim.width > 1400
}