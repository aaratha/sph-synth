// Plugin macro exports and module declarations.

use nih_plug::prelude::*;

mod params;
mod plugin;
mod sim_thread;

use plugin::SphPlugin;

nih_export_clap!(SphPlugin);
nih_export_vst3!(SphPlugin);
