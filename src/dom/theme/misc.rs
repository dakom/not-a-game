use std::sync::LazyLock;

use dominator::class;

pub static USER_SELECT_NONE: LazyLock<String> = LazyLock::new(|| {
    class! {
        .style(["-moz-user-select", "user-select"], "none")
    }
});

pub static POINTER_EVENTS_NONE: LazyLock<String> = LazyLock::new(|| {
    class! {
        .style("pointer-events", "none")
    }
});

pub static CURSOR_POINTER: LazyLock<String> = LazyLock::new(|| {
    class! {
        .style("cursor", "pointer")
    }
});

pub static WORD_WRAP_PRE: LazyLock<String> = LazyLock::new(|| {
    class! {
        .style("white-space", "pre-wrap")
    }
});

pub static TEXT_ALIGN_CENTER: LazyLock<String> = LazyLock::new(|| {
    class! {
        .style("text-align", "center")
    }
});

pub static FULL_SCREEN: LazyLock<String> = LazyLock::new(|| {
    class! {
        .style("position", "absolute")
        .style("top", "0")
        .style("left", "0")
        .style("padding", "0")
        .style("margin", "0")
        .style("width", "100vw")
        .style("height", "100vh")
    }
});
