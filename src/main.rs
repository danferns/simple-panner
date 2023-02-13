use nih_plug::prelude::*;

use simple_panner::SimplePanner;

fn main() {
    nih_export_standalone::<SimplePanner>();
}
