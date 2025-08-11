
pub struct LineIcons;

impl LineIcons
{
    const VERTICAL_START : char = '┬';
    const VERTICAL_MIDDLE : char = '│';
    const VERTICAL_END : char = '┴';

    const HORIZONTAL_START : char = '├';
    const HORIZONTAL_MIDDLE : char = '─';
    const HORIZONTAL_END : char = '┤';

    const HORIZONTAL_DOTTED_START : char = '│';
    const HORIZONTAL_DOTTED_MIDDLE : char = '┄';
    const HORIZONTAL_DOTTED_END : char = '│';

    const HORIZONTAL_SELECTED_START : char = '░';
    const HORIZONTAL_SELECTED_MIDDLE : char = '┄';
    const HORIZONTAL_SELECTED_END : char = '░';

    // const BOTTOM : char = '▁';
    // const TOP : char = '▔';
    // const LEFT : char = '▏';
    // const RIGHT : char = '▕';

    // const TOP_LEFT : char = '▛';
    // const TOP_RIGHT : char = '▜';

    // const BOTTOM_LEFT : char = '▙';
    // const BOTTOM_RIGHT : char = '▟';

    // const FULL_BLOCK : char = '█';

    // const UPPER_HALF : char = '▀';
    // const LOWER_HALF : char = '▄';

    // pub fn vertical_full() -> ( char, char, char )
    // {
    //     ( LineIcons::FULL_BLOCK, LineIcons::FULL_BLOCK, LineIcons::FULL_BLOCK )
    // }

    // pub fn horizontal_external_top() -> ( char, char, char )
    // {
    //     ( LineIcons::LEFT, LineIcons::TOP, LineIcons::RIGHT )
    // }

    // pub fn horizontal_external_bottom() -> ( char, char, char )
    // {
    //     ( LineIcons::LEFT, LineIcons::BOTTOM, LineIcons::RIGHT )
    // }

    // pub fn vertical_external_left() -> ( char, char, char )
    // {
    //     ( LineIcons::LEFT, LineIcons::LEFT, LineIcons::LEFT )
    // }

    // pub fn vertical_external_right() -> ( char, char, char )
    // {
    //     ( LineIcons::RIGHT, LineIcons::RIGHT, LineIcons::RIGHT )
    // }

    pub fn vertical_icons() -> ( char, char, char )
    {
        ( LineIcons::VERTICAL_START, LineIcons::VERTICAL_MIDDLE, LineIcons::VERTICAL_END )
    }

    pub fn horizontal_icons() -> ( char, char, char )
    {
        ( LineIcons::HORIZONTAL_START, LineIcons::HORIZONTAL_MIDDLE, LineIcons::HORIZONTAL_END )
    }

    pub fn horizontal_dotted_icons() -> ( char, char, char )
    {
        ( LineIcons::HORIZONTAL_DOTTED_START, LineIcons::HORIZONTAL_DOTTED_MIDDLE, LineIcons::HORIZONTAL_DOTTED_END )
    }

    pub fn horizontal_selected_icons() -> ( char, char, char )
    {
        ( LineIcons::HORIZONTAL_SELECTED_START, LineIcons::HORIZONTAL_SELECTED_MIDDLE, LineIcons::HORIZONTAL_SELECTED_END )
    }
}