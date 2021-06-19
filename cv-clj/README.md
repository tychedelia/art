Java apis are ok for reading from a soundcard. In order to run a Quil sketch, we only need about 30 frames per second, which Java is capable of handling, albeit with a bit of latency. Open question whether writing sound card listener in another language and doing file I/O or some other kind of IPC would be faster than the Java/native boundary.

Only tested on Linux.
