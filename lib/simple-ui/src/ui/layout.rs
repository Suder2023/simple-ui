use crate::{
    ui::{Container, TCtrl},
    utils,
    utils::{IPointOpt, ISizeOpt, ScaleDpi},
    IRect,
};

pub trait TLayout {
    fn update(&self, c: &mut Container, dpi: &ScaleDpi);
}

#[derive(Default)]
pub struct LayoutHorizontal {}

impl LayoutHorizontal {
    pub fn new() -> Box<dyn TLayout> {
        Box::new(Self {})
    }
}

fn get_layout_real_rc(c: &mut Container, dpi: &ScaleDpi) -> Option<IRect> {
    let mut rc = c.get_inner().real_rc.clone();
    let c_padding = dpi.scale(match c.get_inner().padding {
        Some(v) => v,
        None => IRect::new(0, 0, 0, 0),
    });
    rc.left += c_padding.left;
    rc.right -= c_padding.right;
    rc.top += c_padding.top;
    rc.bottom -= c_padding.bottom;

    if rc.width() <= 0 || rc.height() <= 0 {
        for child in c.get_mut_children() {
            child.get_mut_inner().real_rc = IRect::default();
        }
        return None;
    }

    Some(rc)
}

fn layout_uv(
    real_rc: IRect,
    offset: &mut i32,
    default_size: i32,
    empty_count: &mut i32,
    child_point: &IPointOpt,
    child_size: &ISizeOpt,
    dpi: &ScaleDpi,
) -> IRect {
    let mut rc = IRect::default();

    rc.top = match child_point.x {
        Some(v) => dpi.scale(v),
        None => 0,
    } + real_rc.top;

    rc.bottom = match child_point.y {
        Some(v) => {
            let v = dpi.scale(v);
            real_rc.top
                + if v > real_rc.height() {
                    real_rc.bottom
                } else {
                    v
                }
        }
        None => match child_size.height {
            Some(vv) => {
                let vv = dpi.scale(vv);
                if vv + rc.top > real_rc.height() {
                    real_rc.bottom
                } else {
                    vv + real_rc.top
                }
            }
            None => real_rc.bottom,
        },
    };

    rc.left = *offset;
    rc.right = if *offset >= real_rc.right {
        rc.left
    } else {
        let mut v = match child_size.width {
            Some(v) => dpi.scale(v) + rc.left,
            None => {
                *empty_count -= 1;
                if default_size <= 0 {
                    rc.left
                } else {
                    rc.left + default_size
                }
            }
        };
        if v > real_rc.right {
            v = real_rc.right
        }
        *offset = v;
        v
    };

    rc
}

impl TLayout for LayoutHorizontal {
    fn update(&self, c: &mut Container, dpi: &ScaleDpi) {
        let real_rc = get_layout_real_rc(c, dpi);
        if real_rc.is_none() {
            return;
        }
        let real_rc = real_rc.unwrap();

        let children = c.get_mut_children();
        if children.is_empty() {
            return;
        }

        let mut empty_count = 0;
        let mut total_val = 0;
        for child in children.into_iter() {
            match child.get_inner().size.width {
                Some(v) => total_val += dpi.scale(v),
                None => empty_count += 1,
            };
        }

        let default_size = if empty_count == 0 {
            0
        } else {
            (real_rc.width() - total_val) / empty_count
        };

        let mut offset = real_rc.left;
        for child in children {
            let rc = layout_uv(
                real_rc,
                &mut offset,
                default_size,
                &mut empty_count,
                &IPointOpt::new(child.get_inner().pos.top, child.get_inner().pos.bottom),
                &child.get_inner().size,
                dpi,
            );

            let rc = match child.get_inner().margin {
                Some(v) => IRect::new(
                    rc.left + v.left,
                    rc.top + v.top,
                    rc.right - v.right,
                    rc.bottom - v.bottom,
                ),
                None => rc,
            };

            child.get_mut_inner().real_rc = rc;
        }
    }
}

#[derive(Default)]
pub struct LayoutVertical {}

impl LayoutVertical {
    pub fn new() -> Box<dyn TLayout> {
        Box::new(Self {})
    }
}

impl TLayout for LayoutVertical {
    fn update(&self, c: &mut Container, dpi: &ScaleDpi) {
        let real_rc = get_layout_real_rc(c, dpi);
        if real_rc.is_none() {
            return;
        }
        let real_rc = real_rc.unwrap();

        let children = c.get_mut_children();
        if children.is_empty() {
            return;
        }

        let mut empty_count = 0;
        let mut total_val = 0;
        for child in children.into_iter() {
            match child.get_inner().size.height {
                Some(v) => total_val += dpi.scale(v) as i32,
                None => empty_count += 1,
            };
        }

        let default_size = if empty_count == 0 {
            0
        } else {
            (real_rc.height() - total_val) / empty_count
        };

        let mut offset = real_rc.top;
        for child in children {
            let rc = layout_uv(
                IRect::new(real_rc.top, real_rc.left, real_rc.bottom, real_rc.right),
                &mut offset,
                default_size,
                &mut empty_count,
                &IPointOpt::new(child.get_inner().pos.left, child.get_inner().pos.right),
                &ISizeOpt::new(child.get_inner().size.height, child.get_inner().size.width),
                dpi,
            );

            let rc = IRect::new(rc.top, rc.left, rc.bottom, rc.right);
            let rc = match child.get_inner().margin {
                Some(v) => IRect::new(
                    rc.left + v.left,
                    rc.top + v.top,
                    rc.right - v.right,
                    rc.bottom - v.bottom,
                ),
                None => rc,
            };

            child.get_mut_inner().real_rc = rc;
        }
    }
}

pub struct LayoutTable {
    row: usize,
    line: usize,
}

impl LayoutTable {
    pub fn new(row: usize, line: usize) -> Box<dyn TLayout> {
        Box::new(Self { row, line })
    }
}

impl TLayout for LayoutTable {
    fn update(&self, c: &mut Container, _dpi: &ScaleDpi) {
        let rect_rc = c.get_inner().real_rc;

        let row_size = rect_rc.width() as usize / self.row;
        let line_size = rect_rc.height() as usize / self.line;

        let mut count = 0usize;

        for child in c.get_mut_children() {
            let child = child.as_mut();

            let cur_row = count % self.row;
            let cur_line = count / self.row;
            count += 1;
            if cur_line > self.line {
                child.get_mut_inner().real_rc = IRect::default();
                continue;
            }

            let crc = &mut child.get_mut_inner().real_rc;

            crc.left = rect_rc.left + (row_size * cur_row) as i32;
            crc.top = rect_rc.top + (line_size * cur_line) as i32;
            crc.right = crc.left + cur_row as i32;
            crc.bottom = crc.top + cur_line as i32;
        }
    }
}

pub struct Layout {}

impl Layout {
    pub fn new() -> Box<dyn TLayout> {
        Box::new(Self {})
    }
}

impl TLayout for Layout {
    fn update(&self, c: &mut Container, dpi: &ScaleDpi) {
        let real_rc = c.get_inner().real_rc.clone();
        if real_rc.width() <= 0 || real_rc.height() <= 0 {
            return;
        }
        for child in c.get_mut_children() {
            let pos = &child.get_inner().pos;
            let size = &child.get_inner().size;

            let mut rc = IRect::default();
            rc.left = pos.left.unwrap_or(0);

            rc.top = pos.top.unwrap_or(0);

            rc.right = pos.right.unwrap_or(size.width.unwrap_or(0) + rc.left);
            rc.bottom = pos.bottom.unwrap_or(size.height.unwrap_or(0) + rc.top);

            child.get_mut_inner().real_rc = utils::cal_real_rc(&real_rc, dpi.scale(rc));
        }
    }
}
