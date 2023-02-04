use clap::Args;
use geocalc::{BHOrientationLine, Plane};

// #[derive(ValueEnum, Clone)]
// enum Structure {
//     Plane,
//     Lineation,
// }

#[derive(Args)]
pub struct OrientOne {
    #[arg(long)]
    bearing: f64,

    #[arg(long)]
    inclination: f64,

    #[arg(long)]
    alpha: f64,

    #[arg(long)]
    beta: f64,

    #[arg(long)]
    bottom: bool,
}

pub fn orient_one(cmd: OrientOne) {
    let plane = Plane::alpha_beta(
        cmd.bearing,
        cmd.inclination * -1.0,
        cmd.alpha,
        cmd.beta,
        if cmd.bottom {
            BHOrientationLine::Bottom
        } else {
            BHOrientationLine::Top
        },
    );

    println!("{plane:#?}");
}
