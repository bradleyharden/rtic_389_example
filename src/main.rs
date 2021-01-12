#![no_std]
#![no_main]

use panic_halt as _;
use atsamd_hal::target_device as pac;

#[rtic::app(device = crate::pac)]
mod app {

    #[resources]
    struct Resources {
        #[init(0)]
        one: u8,
        #[init(0)]
        two: u16,
    }

    #[init]
    fn init(_: init::Context) -> init::LateResources {
        init::LateResources{}
    }

    #[task(binds = PAC, priority = 1, resources = [one, two])]
    fn task_1(ctx: task_1::Context) {
        let task_1::Resources { one, two } = ctx.resources;
        (one, two).lock(|one, two| {
            *one = 5;
            *two = 10;
        });
        //(one, two).lock(|one, two| {
        //    *one = 5;
        //    *two = 10;
        //});
    }

    #[task(binds = AC, priority = 2, resources = [one, two])]
    fn task_2(ctx: task_2::Context) {
        let task_2::Resources { mut one, mut two } = ctx.resources;
        one.lock(|one| {
            two.lock(|two| {
                *one = 5;
                *two = 10;
            });
        });
        one.lock(|one| {
            two.lock(|two| {
                *one = 5;
                *two = 10;
            });
        });
    }

}
