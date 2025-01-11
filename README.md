# audimage

**audimage** is a rust tool that utilizes [minimodem](https://github.com/kamalmostafa/minimodem) to transmit images from one process to another with computer speakers/microphones.

audimage works best if you use a high-quality microphone and speaker setup.

## Usage

To transmit an image with audimage:

```bash
git clone https://github.com/jovielarue/audimage
cd audimage
cargo build --release

# On one terminal session
cd ./audimage_receive

# This
minimodem --rx 200 | cargo run --release

# In another terminal session
cd ./audimage_transmit
# Before this command, edit src/main.rs and put in the path of the image you wish to transmit
cargo run --release | minimodem --tx 200
```

## Output

audimage_receive will output something similar this:

```bash
.
.
. # spacing lines to allow the receiver to lock in on the signal

50x35 # the dimensions of the image
:21 # a luma value, prefixed by a colon to differentiate it
5,20 # coordinates that the luma value shows up at
:23
3,20
4,20
6,20 # e.g. luma value 23 shows up at (3,20), (4,20), and (6,20)
:24
5,19
:27
6,19
7,20
8,20
:28
0,21
:29
7,19
:31
3,19
```

Once the image has finished transmitting, audimage_transmit will transmit two semicolons which tells audimage_receive to stop receiving and save the image. Only one semicolon is required to stop audimage_receive, but two are transmitted in case of noise.
