#![no_std]
#![no_main]

use core::marker::PhantomData;

use panic_halt as _;

use atsamd_hal as hal;
use hal::target_device as pac;

struct PadsT<S, I, P0, P1, P2, P3> {
    sercom: PhantomData<S>,
    ioset: PhantomData<I>,
    pad0: PhantomData<P0>,
    pad1: PhantomData<P1>,
    pad2: PhantomData<P2>,
    pad3: PhantomData<P3>,
}

trait Pads {
    type Sercom;
    type IoSet;
    type Pad0;
    type Pad1;
    type Pad2;
    type Pad3;
}

impl<S, I, P0, P1, P2, P3> Pads for PadsT<S, I, P0, P1, P2, P3> {
    type Sercom = S;
    type IoSet = I;
    type Pad0 = P0;
    type Pad1 = P1;
    type Pad2 = P2;
    type Pad3 = P3;
}

#[rtic::app(device = crate::pac)]
mod app {

    #[resources]
    struct Resources<P: Pads> {
        p: P
    }

    #[init]
    fn init(_: init::Context) -> init::LateResources<impl Pads> {
        let p = PadsT {
            sercom: PhantomData::<u8>,
            ioset: PhantomData::<u16>,
            pad0: PhantomData::<u32>,
            pad1: PhantomData::<i8>,
            pad2: PhantomData::<i16>,
            pad3: PhantomData::<i32>,
        };
        init::LateResources{p}
    }

}
