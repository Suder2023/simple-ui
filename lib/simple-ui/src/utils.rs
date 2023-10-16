use crate::{IPoint, IRect, ISize};

////////////////////////////////////////////////////////////
/// DPI
pub trait TScaleDpi {
    fn scale(self, dpi: f64) -> Self;
    fn rescale(self, dpi: f64) -> Self;
}

impl TScaleDpi for i32 {
    fn scale(self, dpi: f64) -> Self {
        (self as f64 * dpi) as i32
    }
    fn rescale(self, dpi: f64) -> Self {
        (self as f64 / dpi) as i32
    }
}

impl TScaleDpi for IRect {
    fn scale(self, dpi: f64) -> Self {
        Self::new(
            (self.left as f64 * dpi) as i32,
            (self.top as f64 * dpi) as i32,
            (self.right as f64 * dpi) as i32,
            (self.bottom as f64 * dpi) as i32,
        )
    }
    fn rescale(self, dpi: f64) -> Self {
        Self::new(
            (self.left as f64 / dpi) as i32,
            (self.top as f64 / dpi) as i32,
            (self.right as f64 / dpi) as i32,
            (self.bottom as f64 / dpi) as i32,
        )
    }
}

impl TScaleDpi for IPoint {
    fn scale(self, dpi: f64) -> Self {
        Self::new((self.x as f64 * dpi) as i32, (self.y as f64 * dpi) as i32)
    }
    fn rescale(self, dpi: f64) -> Self {
        Self::new((self.x as f64 / dpi) as i32, (self.y as f64 / dpi) as i32)
    }
}

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct ScaleDpi {
    dpi: f64,
}

impl ScaleDpi {
    pub fn new(d: f64) -> Self {
        Self { dpi: d }
    }

    pub fn scale<T: TScaleDpi>(&self, v: T) -> T {
        v.scale(self.dpi)
    }

    pub fn rescale<T: TScaleDpi>(&self, v: T) -> T {
        v.rescale(self.dpi)
    }
}

impl Default for ScaleDpi {
    fn default() -> Self {
        Self { dpi: 1. }
    }
}
////////////////////////////////////////////////////////////
/// IRect Option

#[derive(Default)]
pub struct IRectOpt {
    pub left: Option<i32>,
    pub top: Option<i32>,
    pub right: Option<i32>,
    pub bottom: Option<i32>,
}

impl IRectOpt {
    pub fn to_rc(&self, default: i32) -> IRect {
        IRect::new(
            self.left.unwrap_or(default),
            self.top.unwrap_or(default),
            self.right.unwrap_or(default),
            self.bottom.unwrap_or(default),
        )
    }
}

////////////////////////////////////////////////////////////
/// ISize Option

#[derive(Default)]
pub struct ISizeOpt {
    pub width: Option<i32>,
    pub height: Option<i32>,
}

impl ISizeOpt {
    pub fn new(w: Option<i32>, h: Option<i32>) -> Self {
        Self {
            width: w,
            height: h,
        }
    }

    pub fn to_size(&self, default: i32) -> ISize {
        ISize::new(
            self.width.unwrap_or(default),
            self.height.unwrap_or(default),
        )
    }
}

////////////////////////////////////////////////////////////
/// ISize Option

#[derive(Default)]
pub struct IPointOpt {
    pub x: Option<i32>,
    pub y: Option<i32>,
}

impl IPointOpt {
    pub fn new(x: Option<i32>, y: Option<i32>) -> Self {
        Self { x, y }
    }
    pub fn _to_point(self, default: i32) -> IPoint {
        IPoint::new(self.x.unwrap_or(default), self.y.unwrap_or(default))
    }
}

////////////////////////////////////////////////////////////
///

pub fn cal_real_rc(real_rc: &IRect, rc: IRect) -> IRect {
    if rc.width() <= 0 || rc.height() <= 0 {
        return IRect::default();
    }
    let mut rc = rc;

    rc.left += real_rc.left;
    if rc.left >= real_rc.right {
        return IRect::default();
    }
    rc.top += real_rc.top;
    if rc.top >= real_rc.bottom {
        return IRect::default();
    }
    rc.right += real_rc.left;
    if rc.right > real_rc.right {
        rc.right = real_rc.right;
    }
    rc.bottom += real_rc.top;
    if rc.bottom > real_rc.bottom {
        rc.bottom = real_rc.bottom
    }

    rc
}

pub fn in_rc(rc: &IRect, pos: &IPoint) -> bool {
    pos.x >= rc.left && pos.x <= rc.right && pos.y >= rc.top && pos.y <= rc.bottom
}

#[test]
fn test_cal_real_rc() {
    let real_rc = IRect::new(10, 20, 200, 400);
    {
        let rc1 = IRect::new(10, 10, 20, 20);
        let rc2 = IRect::new(20, 30, 30, 40);
        assert_eq!(cal_real_rc(&real_rc, rc1), rc2);
    }
    {
        let rc1 = IRect::new(10, 10, 300, 20);
        let rc2 = IRect::new(20, 30, 200, 40);
        assert_eq!(cal_real_rc(&real_rc, rc1), rc2);
    }
    {
        let rc1 = IRect::new(10, 10, 20, 500);
        let rc2 = IRect::new(20, 30, 30, 400);
        assert_eq!(cal_real_rc(&real_rc, rc1), rc2);
    }
    {
        let rc1 = IRect::new(10, 10, 10, 20);
        let rc2 = IRect::default();
        assert_eq!(cal_real_rc(&real_rc, rc1), rc2);
    }
    {
        let rc1 = IRect::new(200, 10, 220, 20);
        let rc2 = IRect::default();
        assert_eq!(cal_real_rc(&real_rc, rc1), rc2);
    }
    {
        let rc1 = IRect::new(10, 1110, 20, 1120);
        let rc2 = IRect::default();
        assert_eq!(cal_real_rc(&real_rc, rc1), rc2);
    }
}

#[test]
fn test_in_rc() {
    assert!(!in_rc(&IRect::new(10, 10, 20, 20), &IPoint::new(11, 21)));
    assert!(!in_rc(&IRect::new(10, 10, 20, 20), &IPoint::new(9, 12)));
    assert!(!in_rc(&IRect::new(10, 10, 20, 20), &IPoint::new(9, 21)));
    assert!(in_rc(&IRect::new(10, 10, 20, 20), &IPoint::new(11, 20)));
}
