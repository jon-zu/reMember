use std::{collections::BTreeMap, ops::RangeInclusive};

use euclid::default::Box2D;
use geo::CoordNum;
use geo_svg::ToSvg;
use itertools::Itertools;
use rstar::{RTree, RTreeNode, RTreeObject, SelectionFunction, AABB};
use serde::{Deserialize, Serialize};

use crate::id::FootholdId;

type FhScalar = f32;

fn clamp_range(r: RangeInclusive<FhScalar>, v: FhScalar) -> f32 {
    v.clamp(*r.start(), *r.end())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line<T> {
    pub start: euclid::default::Point2D<T>,
    pub end: euclid::default::Point2D<T>,
}

impl<T: CoordNum> Line<T> {
    pub fn from_points(
        start: euclid::default::Point2D<T>,
        end: euclid::default::Point2D<T>,
    ) -> Self {
        Self { start, end }
    }

    pub fn slope(&self) -> T {
        let delta_y = self.end.y - self.start.y;
        let delta_x = self.end.x - self.start.x;
        delta_y / delta_x
    }

    fn to_geo_line(&self) -> geo::Line<T> {
        geo::Line::new(self.start.to_tuple(), self.end.to_tuple())
    }
}

impl<T: CoordNum> geo_svg::ToSvgStr for Line<T> {
    fn to_svg_str(&self, style: &geo_svg::Style) -> String {
        self.to_geo_line().to_svg_str(style)
    }

    fn viewbox(&self, style: &geo_svg::Style) -> geo_svg::ViewBox {
        self.to_geo_line().viewbox(style)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhSlope {
    line: Line<FhScalar>,
    slope: FhScalar,
}

impl FhSlope {
    pub fn new(line: Line<FhScalar>) -> Self {
        Self {
            slope: line.slope(),
            line,
        }
    }

    pub fn calc_y(&self, x: FhScalar) -> FhScalar {
        let x = x - self.line.start.x;
        // Basic linear interpolation
        x.mul_add(self.slope, self.line.start.y)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Foothold {
    Wall(Line<FhScalar>),
    Platform(Line<FhScalar>),
    Slope(FhSlope),
}

impl Foothold {
    pub fn from_points(
        low: euclid::default::Point2D<FhScalar>,
        high: euclid::default::Point2D<FhScalar>,
    ) -> Self {
        let line = Line::from_points(low, high);
        if high.x == low.x {
            return Self::Wall(line);
        }
        if high.y == low.y {
            return Self::Platform(line);
        }
        Self::Slope(FhSlope::new(line))
    }

    pub fn get_line(&self) -> &Line<FhScalar> {
        match self {
            Self::Wall(l) => l,
            Self::Platform(l) => l,
            Self::Slope(l) => &l.line,
        }
    }

    pub fn get_x_range(&self) -> RangeInclusive<FhScalar> {
        let line = self.get_line();
        line.start.x..=line.end.x
    }

    pub fn calc_y(&self, x: FhScalar) -> FhScalar {
        match self {
            Self::Wall(l) => l.start.y.max(l.end.y),
            Self::Platform(l) => l.start.y,
            Self::Slope(slope) => slope.calc_y(x),
        }
    }

    pub fn get_coord_below(
        &self,
        c: euclid::default::Point2D<FhScalar>,
    ) -> Option<euclid::default::Point2D<FhScalar>> {
        let x_range = self.get_x_range();
        if !x_range.contains(&c.x) {
            return None;
        }

        let y = match self {
            Self::Wall(l) => l.start.y.max(l.end.y),
            Self::Platform(l) => l.start.y,
            Self::Slope(slope) => slope.calc_y(c.x),
        };

        if y < c.y {
            return None;
        }

        Some((c.x, y).into())
    }

    pub fn clamp_x(&self, x: f32) -> f32 {
        let r_x = self.get_x_range();
        x.clamp(*r_x.start(), *r_x.end())
    }

    pub fn get_item_spread(
        &self,
        x: FhScalar,
        items: usize,
    ) -> impl Iterator<Item = geo::Coord<FhScalar>> + '_ {
        //TODO make this config-able
        const STEP: FhScalar = 20.0;
        let start_x = if items > 0 {
            ((items - 1) as FhScalar).mul_add(-STEP, x)
        } else {
            x
        };

        (0..items)
            .map(move |i| self.clamp_x((i as f32).mul_add(STEP, start_x)))
            .map(|x| (x, self.calc_y(x)).into())
    }
}

impl RTreeObject for Foothold {
    type Envelope = AABB<(FhScalar, FhScalar)>;

    fn envelope(&self) -> Self::Envelope {
        let line = self.get_line();
        AABB::from_corners(line.start.to_tuple(), line.end.to_tuple())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FhTree {
    tree: RTree<Foothold, rstar::DefaultParams>,
    bounds: euclid::default::Box2D<FhScalar>,
}

pub struct BelowPointSelector {
    p: euclid::default::Point2D<FhScalar>,
}

impl SelectionFunction<Foothold> for BelowPointSelector {
    fn should_unpack_parent(&self, envelope: &<Foothold as RTreeObject>::Envelope) -> bool {
        let u = envelope.upper();
        let l = envelope.lower();

        // Check that x is in range
        if !(l.0..=u.0).contains(&self.p.x) {
            return false;
        }

        let min_y = u.1.max(l.1);
        // Check that y is below
        min_y > self.p.y
    }

    fn should_unpack_leaf(&self, leaf: &Foothold) -> bool {
        leaf.get_coord_below(self.p).is_some()
    }
}

impl FhTree {
    pub fn create_render_data_for_tree_2d(&self) -> String {
        fn get_color_for_depth(depth: usize) -> geo_svg::Color {
            match depth {
                0 => geo_svg::Color::Rgb(0, 0, 0),
                1 => geo_svg::Color::Rgb(216, 0, 216),
                2 => geo_svg::Color::Rgb(0, 216, 216),
                3 => geo_svg::Color::Rgb(0, 0, 130),
                4 => geo_svg::Color::Rgb(216, 0, 0),
                _ => geo_svg::Color::Rgb(0, 216, 216),
            }
        }

        let line0 = geo::Point::new(0, 0);
        let mut svg = line0.to_svg();
        let mut rects = Vec::new();

        let mut to_visit = vec![(self.tree.root(), 0)];
        while let Some((cur, depth)) = to_visit.pop() {
            let env = &cur.envelope();
            rects.push((geo::Rect::new(env.lower(), env.upper()), depth));

            for child in cur.children() {
                match child {
                    RTreeNode::Leaf(fh) => {
                        let svg_line = fh
                            .get_line()
                            .to_svg()
                            .with_radius(5.0)
                            .with_stroke_width(5.)
                            .with_fill_opacity(1.);
                        let color = match fh {
                            Foothold::Wall(_) => "black",
                            Foothold::Platform(_) => "green",
                            Foothold::Slope(_) => "red",
                        };

                        svg = svg.and(
                            svg_line
                                .with_fill_color(geo_svg::Color::Named(color))
                                .with_stroke_color(geo_svg::Color::Named(color)),
                        );
                    }
                    RTreeNode::Parent(ref data) => {
                        to_visit.push((data, depth + 1));
                    }
                }
            }
        }

        let mut rect_svg = line0.to_svg();
        for (rect, depth) in rects.iter() {
            let c = get_color_for_depth(*depth);

            rect_svg = rect_svg.and(
                rect.to_svg()
                    .with_stroke_color(c)
                    .with_fill_color(c)
                    .with_stroke_width(3.0)
                    .with_stroke_opacity(0.3)
                    //.with_opacity(0.)
                    .with_fill_opacity((*depth as f32 + 1.0) * 0.03),
            );
        }

        rect_svg.and(svg).to_string()
    }

    pub fn from_meta(
        footholds: &BTreeMap<
            FootholdId,
            BTreeMap<FootholdId, BTreeMap<FootholdId, super::Foothold>>,
        >,
        rect: Box2D<i16>,
    ) -> Self {
        let fhs = footholds
            .values()
            .flat_map(|v| v.values())
            .flat_map(|v| v.iter())
            .map(|(_id, fh)| {
                Foothold::from_points(fh.pt1.to_f32().to_point(), fh.pt2.to_f32().to_point())
            })
            .collect_vec();

        Self {
            tree: rstar::RTree::bulk_load(fhs),
            bounds: rect.to_f32(),
        }
    }

    pub fn get_foothold_below(&self, p: euclid::default::Point2D<FhScalar>) -> Option<&Foothold> {
        let (_x, y) = p.to_tuple();
        let fh_below = self
            .tree
            .locate_with_selection_function(BelowPointSelector { p });

        fh_below.min_by_key(|fh| (y - fh.get_coord_below(p).unwrap().y).abs() as i32)
    }

    pub fn x_range(&self) -> RangeInclusive<FhScalar> {
        self.bounds.min.x..=self.bounds.max.x
    }

    pub fn y_range(&self) -> RangeInclusive<FhScalar> {
        self.bounds.min.y..=self.bounds.max.y
    }

    pub fn clamp(
        &self,
        coord: euclid::default::Point2D<FhScalar>,
    ) -> euclid::default::Point2D<FhScalar> {
        (
            clamp_range(self.x_range(), coord.x),
            clamp_range(self.y_range(), coord.y),
        )
            .into()
    }

    pub fn contains(&self, p: euclid::default::Point2D<FhScalar>) -> bool {
        self.bounds.contains(p)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use crate::id::FieldId;
    use geo_svg::ToSvg;

    use crate::MetaService;

    use super::*;

    #[test]
    fn map_fh() -> anyhow::Result<()> {
        let meta = MetaService::load_from_dir("../../shroom-metadata", crate::MetaOption::Full)?;
        let field_1 = meta.get_field(FieldId(450002000)).unwrap();

        let fh_tree = FhTree::from_meta(&field_1.footholds, field_1.rect);
        dbg!(&fh_tree.bounds);
        let line0 = geo::Point::new(0, 0);
        let mut svg = line0.to_svg();

        for fh in fh_tree.tree.iter() {
            let svg_line = fh
                .get_line()
                .to_svg()
                .with_radius(2.0)
                .with_stroke_width(5.)
                .with_fill_opacity(0.7);
            let color = match fh {
                Foothold::Wall(_) => "black",
                Foothold::Platform(_) => "green",
                Foothold::Slope(_) => "red",
            };

            dbg!(fh);

            svg = svg.and(
                svg_line
                    .with_fill_color(geo_svg::Color::Named(color))
                    .with_stroke_color(geo_svg::Color::Named(color)),
            );
        }

        let mut lines = Vec::new();

        let bounds = &fh_tree.bounds;
        for test_pt_x in (bounds.min.x as i32..bounds.max.x as i32).step_by(50) {
            let test_pt_x = test_pt_x as f32;
            let pt = (test_pt_x, 0.).into();

            if let Some(fh) = fh_tree.get_foothold_below(pt) {
                let intersec = fh.get_coord_below(pt).unwrap();
                let line = Line::from_points(pt, intersec);
                lines.push(line);
            } else {
                dbg!(pt);
            }
        }

        for line in lines.iter() {
            svg = svg.and(
                line.to_svg()
                    .with_radius(2.0)
                    .with_fill_color(geo_svg::Color::Named("blue"))
                    .with_stroke_color(geo_svg::Color::Named("blue"))
                    .with_stroke_width(5.)
                    .with_fill_opacity(0.7),
            );
        }

        let mut f = File::create("sp2.svg")?;
        writeln!(&mut f, "{svg}")?;

        let extra = fh_tree.create_render_data_for_tree_2d();

        let mut f = File::create("extra.svg")?;
        writeln!(&mut f, "{extra}")?;

        Ok(())
    }
}
