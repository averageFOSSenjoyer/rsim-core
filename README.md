# rsim: Cycle-accurate Simulation Framework with Verilog-like Syntax
---

## Motivation
This project stemmed from the need for a risc-v simulator, one that is cycle accurate with detailed signals, for the course of ECE 120 at the University of Illinois Urbana-Champaign. With flexibility in mind, I figured it would be better to write a framework that can simulate any microarchitecture the course wishes. 

The risc-v simulator mentioned above can be found at [rsim-rv32i](https://github.com/averageFOSSenjoyer/rsim-rv32i).

## Example
---
```
#[ComponentAttribute({
"port": {
    "input": [
        ["data", "u32"],
        ["load", "bool"]
    ],
    "output:" [
        ["out", u32],
    ]
    "clock": true
}
})]
pub struct SimpleRegister {
    data_inner: u32
}

impl SimpleRegister {
    fn on_clock(&mut self) {
        if self.load.get_value() {
            self.data_inner = self.data.get_value();
        }
    }

    fn on_comb(&mut self) {
        self.out.send(self.data_inner);
    }
}
```
This isn't particularly the best example, since it could be done in verilog with significantly less lines. However, imagine black boxes that are significantly harder and time consuming to write in verilog. See [VGA](https://github.com/averageFOSSenjoyer/rsim-rv32i/blob/b66b392b9e57fad20f4eb61098d9dd2f07eaa834/src/backend/component/mem_ctl.rs#L233) and [Keyboard](https://github.com/averageFOSSenjoyer/rsim-rv32i/blob/b66b392b9e57fad20f4eb61098d9dd2f07eaa834/src/backend/component/mem_ctl.rs#L179) MMIO.