# RFB-hal-blocking

Polls PA0, PA1, PA2, `unsafe`ly counting up a global,
which is read out in the serial RXNE interrupt.

Ironically, includes an interrupt-driven serial port implementation.
Otherwise I'd have to poll the serial registers as well, defeating the purposee.
This highlights the problem with the blocking approach obviously.
