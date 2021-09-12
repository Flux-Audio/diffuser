use crate::parameter_manager::ParameterManager;
use dsp_lab::utils::math::{x_fade};
use dsp_lab::core::reverb::StereoFirDiffuser;
use dsp_lab::core::lin_filter::{BiquadLowPass, BiquadAllPass};
use dsp_lab::core::RawRingBuffer;
use dsp_lab::core::delay::{DelayLine};
use dsp_lab::core::osc::ParOsc;
use dsp_lab::utils::math::var_clip;
use dsp_lab::shared_enums::{InterpMethod, ScaleMethod};
use dsp_lab::traits::{Process, Source, ProcessChain};
use std::sync::Arc;
use std::collections::VecDeque;

pub struct EffectProcessor {
    diff: StereoFirDiffuser,
    fb_lp_l: BiquadLowPass,
    fb_lp_r: BiquadLowPass,
    fb_ap_l: BiquadAllPass,
    fb_ap_r: BiquadAllPass,
    mod_l: ParOsc,
    mod_r: ParOsc,
    //fb_dly_l: Box<RawRingBuffer<16384>>,
    //fb_dly_r: Box<RawRingBuffer<16384>>,
    fb_dly_l: DelayLine,
    fb_dly_r: DelayLine,
    fb: (f64, f64),
}

const RESOLUTION: usize = 512;

impl EffectProcessor {
    pub fn new() -> Self {
        let mut ret = EffectProcessor {
            diff: StereoFirDiffuser::new(),
            fb_lp_l: BiquadLowPass::new(),
            fb_lp_r: BiquadLowPass::new(),
            fb_ap_l: BiquadAllPass::new(),
            fb_ap_r: BiquadAllPass::new(),
            mod_l: ParOsc::new(),
            mod_r: ParOsc::new(),
            //fb_dly_l: Box::new(RawRingBuffer::new()),
            //fb_dly_r: Box::new(RawRingBuffer::new()),
            fb_dly_l: DelayLine::new(),
            fb_dly_r: DelayLine::new(),
            fb: (0.0, 0.0),
        };


        ret.fb_lp_l.cutoff = 12_000.0;
        ret.fb_lp_r.cutoff = 12_000.0;

        ret.fb_ap_l.cutoff = 400.0;
        ret.fb_ap_r.cutoff = 400.0;

        ret.mod_l.oversampling = 4;
        ret.mod_r.oversampling = 4;
        ret.mod_l.set_freq(0.75);
        ret.mod_r.set_freq(1.213525);

        ret.fb_dly_l.add_head(43.0, 1.0);
        ret.fb_dly_r.add_head(57.0, 1.0);
        ret.fb_dly_l.mix_mode = ScaleMethod::Unity;
        ret.fb_dly_r.mix_mode = ScaleMethod::Unity;
        ret.fb_dly_l.interp_mode = InterpMethod::Linear;
        ret.fb_dly_r.interp_mode = InterpMethod::Linear;

        ret
    }

    pub fn set_sr(&mut self, sr: f64) {
        self.fb_lp_l.set_sr(sr);
        self.fb_lp_r.set_sr(sr);
        self.mod_l.set_sr(sr);
        self.mod_r.set_sr(sr);
        self.fb_dly_l.set_sr(sr);
        self.fb_dly_r.set_sr(sr);
    }

    pub fn process_effects(&mut self, param_mngr: Arc<ParameterManager>, l: f64, r: f64) -> (f64, f64) {
        let len = param_mngr.params[0].filtered.get() as f64;
        let cross = param_mngr.params[1].filtered.get() as f64;
        let fb = param_mngr.params[2].filtered.get() as f64;
        let wet = param_mngr.params[3].filtered.get() as f64;

        let dly_l = 43.0 + 2.5 * self.mod_l.step();
        let dly_r = 57.0 + 1.545085 * self.mod_r.step();
        self.fb_dly_l.set_offset(0, dly_l);
        self.fb_dly_r.set_offset(0, dly_r);

        self.fb_lp_l.cutoff = x_fade(5_000.0, fb, 1_000.0);
        self.fb_lp_r.cutoff = self.fb_lp_l.cutoff;

        self.diff.size = len;
        self.diff.crossover = cross;
        let ret = self.diff.step((self.fb.0 * fb * -1.0 + l, self.fb.1 * fb * -1.0 + r));
        let ret_l = x_fade(l, wet, ret.0);
        let ret_r = x_fade(r, wet, ret.1);
        self.fb.0 = var_clip(
            ProcessChain::new(ret.0)
                .pipe(&mut self.fb_lp_l)
                .pipe(&mut self.fb_ap_l)
                .pipe(&mut self.fb_dly_l)
                .consume(),
            0.75);
        self.fb.1 = var_clip(
            ProcessChain::new(ret.1)
                .pipe(&mut self.fb_lp_r)
                .pipe(&mut self.fb_ap_r)
                .pipe(&mut self.fb_dly_r)
                .consume(),
            0.75);
        return (ret_l, ret_r);
    }
}

