use num::Float;
use std::f32::consts::PI;
use std::fmt::Debug;

pub fn run() {
    println!("\n*****************************************************************");
    println!("generics");
    println!("*****************************************************************");

    generics_and_lifetimes_tests();
    traits_tests();
}

///////////////////////////////////////////////////////////////////////////////
// lifetime and generics data and tests

#[derive(Debug)]
enum Maybe<T> {
    Some(T),
    None,
}

#[derive(Debug)]
enum MaybeRef<'a, T> {
    Some(&'a T),
    None,
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn get_on_first_some<T>(p1: Maybe<T>, p2: Maybe<T>) -> T {
    match p1 {
        Maybe::Some(p1_val) => p1_val,
        _ => match p2 {
            Maybe::Some(p2_val) => p2_val,
            _ => panic!("oops"),
        },
    }
}

fn get_on_first_someref<'a, T>(p1: MaybeRef<'a, T>, p2: MaybeRef<'a, T>) -> &'a T {
    match p1 {
        MaybeRef::Some(p1_val) => p1_val,
        _ => match p2 {
            MaybeRef::Some(p2_val) => p2_val,
            _ => panic!("oops"),
        },
    }
}

fn generics_and_lifetimes_tests() {
    println!("\n--- generics and lifetimes ---\n");

    let x = Point { x: 1, y: 2 };
    println!(
        "xref: {:?}",
        get_on_first_someref(MaybeRef::None, MaybeRef::Some(&x))
    );
    println!("x: {:?}", get_on_first_some(Maybe::None, Maybe::Some(x)));
}

///////////////////////////////////////////////////////////////////////////////
// traits tests

trait Addition {
    fn translate(&self, other: &Self) -> Self;
}

impl<T: std::ops::Add<Output = T> + Copy> Addition for Point<T> {
    fn translate(&self, other: &Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct SphericalPoint<T: Float> {
    r: T,
    theta: T,
}

impl<T: Copy + Float + std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Debug>
    SphericalPoint<T>
{
    fn to_point(&self) -> Point<T> {
        Point {
            x: self.r * self.theta.cos(),
            y: self.r * self.theta.sin(),
        }
    }

    fn from_point(p: &Point<T>) -> SphericalPoint<T> {
        SphericalPoint {
            r: (p.x * p.x + p.y * p.y).sqrt(),
            theta: p.y.atan2(p.x),
        }
    }
}

impl<T: Float + std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Copy + Debug> Addition
    for SphericalPoint<T>
{
    fn translate(&self, other: &Self) -> Self {
        let p1 = self.to_point();
        let p2 = other.to_point();
        let p = p1.translate(&p2);
        SphericalPoint::from_point(&p)
    }
}

///////////////////////////////////////////////////////////////////////////////
// implementing here my own trait to cover the used methods

trait MyFloatTrait {
    fn my_cos(&self) -> Self;
    fn my_sin(&self) -> Self;
    fn my_sqrt(&self) -> Self;
    fn my_atan2(&self, v: Self) -> Self;
}

impl MyFloatTrait for f32 {
    fn my_cos(&self) -> Self {
        self.cos()
    }
    fn my_sin(&self) -> Self {
        self.sin()
    }
    fn my_sqrt(&self) -> Self {
        self.sqrt()
    }
    fn my_atan2(&self, x: Self) -> Self {
        self.atan2(x)
    }
}

#[derive(Debug)]
struct SphericalPoint2<T: MyFloatTrait> {
    r: T,
    theta: T,
}

impl<T: Copy + MyFloatTrait + std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Debug>
    SphericalPoint2<T>
{
    fn to_point(&self) -> Point<T> {
        Point {
            x: self.r * self.theta.my_cos(),
            y: self.r * self.theta.my_sin(),
        }
    }

    fn from_point(p: &Point<T>) -> Self {
        Self {
            r: (p.x * p.x + p.y * p.y).my_sqrt(),
            theta: p.y.my_atan2(p.x),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// test method

fn traits_tests() {
    println!("\n--- traits ---\n");

    let sp1 = SphericalPoint { r: 10.0, theta: PI };
    let sp2 = SphericalPoint {
        r: 5.8,
        theta: PI * 0.5,
    };
    let sp3 = SphericalPoint2 {
        r: 2.0,
        theta: PI * 0.5,
    };

    println!("sp: {:?}", sp1.translate(&sp2));
    println!("sp3: {:?}", SphericalPoint2::from_point(&sp3.to_point()));
}
